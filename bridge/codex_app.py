"""Drive Codex through one long-lived `codex app-server` over stdio JSON-RPC."""
import itertools
import json
import queue
import subprocess
import sys
import threading
import time
from collections import defaultdict

from procs import BridgeError, kill_tree, require_directory

SANDBOXES = ('read-only', 'workspace-write')
CLIENT_INFO = {'name': 'axiom-bridge', 'title': 'Axiom Forge bridge', 'version': '0.1.0'}
_DEAD = object()


class CodexAppServer:
    def __init__(self, command):
        self.command = command
        self._proc = None
        self._eof = threading.Event()
        self._ids = itertools.count(1)
        self._write_lock = threading.Lock()
        self._start_lock = threading.Lock()
        self._pending = {}
        self._threads = defaultdict(queue.Queue)
        self._stderr = []

    def alive(self):
        # EOF on stdout can precede poll() noticing the exit, especially on Windows.
        return self._proc is not None and not self._eof.is_set() and self._proc.poll() is None

    def ensure_started(self):
        with self._start_lock:
            if self.alive():
                return
            self.close()
            self._proc = subprocess.Popen(
                self.command + ['app-server', '--listen', 'stdio://'],
                stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                text=True, encoding='utf-8', errors='replace', bufsize=1)
            self._pending = {}
            self._threads = defaultdict(queue.Queue)
            self._stderr = []
            self._eof = threading.Event()
            threading.Thread(target=self._read_stdout, args=(self._proc, self._pending, self._threads, self._eof), daemon=True).start()
            threading.Thread(target=self._read_stderr, args=(self._proc,), daemon=True).start()
            self._request('initialize', {'clientInfo': CLIENT_INFO}, timeout=60)
            self._send({'method': 'initialized'})

    def close(self):
        if self._proc is not None:
            kill_tree(self._proc)
            for stream in (self._proc.stdin, self._proc.stdout, self._proc.stderr):
                try:
                    stream.close()
                except OSError:
                    pass

    def _send(self, message):
        line = json.dumps(message, ensure_ascii=False) + '\n'
        with self._write_lock:
            try:
                self._proc.stdin.write(line)
                self._proc.stdin.flush()
            except (BrokenPipeError, OSError, ValueError) as exc:
                raise BridgeError('codex_exited', f'codex app-server is not accepting input: {exc}{self._stderr_tail()}') from exc

    def _request(self, method, params, timeout):
        request_id = next(self._ids)
        reply = queue.Queue(maxsize=1)
        self._pending[request_id] = reply
        self._send({'id': request_id, 'method': method, 'params': params})
        try:
            message = reply.get(timeout=timeout)
        except queue.Empty:
            raise BridgeError('timeout', f'{method} got no reply within {timeout}s') from None
        finally:
            self._pending.pop(request_id, None)
        if message is _DEAD:
            raise BridgeError('codex_exited', f'codex app-server exited during {method}{self._stderr_tail()}')
        if 'error' in message:
            error = message['error']
            raise BridgeError('codex_rpc', f"{method}: {error.get('message', error)}")
        return message['result']

    def _read_stdout(self, proc, pending, threads, eof):
        for line in proc.stdout:
            try:
                message = json.loads(line)
            except json.JSONDecodeError:
                self._stderr.append('non-JSON stdout: ' + line.strip())
                continue
            method = message.get('method')
            if method is None:
                waiter = pending.get(message.get('id'))
                if waiter is not None:
                    waiter.put(message)
                continue
            params = message.get('params') or {}
            if 'id' in message:
                # approvalPolicy=never should prevent these; refusing keeps a stray request from hanging the turn.
                try:
                    self._send({'id': message['id'], 'error': {'code': -32601, 'message': 'axiom bridge grants no interactive approvals'}})
                except BridgeError:
                    pass
                method = 'bridge/refusedServerRequest:' + method
            thread_id = params.get('threadId')
            if thread_id:
                threads[thread_id].put((method, params))
        eof.set()
        for waiter in list(pending.values()):
            waiter.put(_DEAD)
        for events in list(threads.values()):
            events.put(_DEAD)

    def _read_stderr(self, proc):
        for line in proc.stderr:
            self._stderr.append(line.rstrip())
            del self._stderr[:-40]

    def _stderr_tail(self):
        tail = '\n'.join(self._stderr[-10:])
        return f'\nstderr:\n{tail}' if tail else ''

    def run(self, brief, cwd, model, sandbox, effort=None, output_schema=None, developer_instructions=None, timeout_s=1800,
            lean=True, keep_plugins=(), web_search=False):
        require_directory(cwd)
        if sandbox not in SANDBOXES:
            raise BridgeError('invalid_argument', f'sandbox must be one of {SANDBOXES}; danger-full-access is not exposed by the bridge')
        if not brief or not brief.strip():
            raise BridgeError('invalid_argument', 'brief is empty')
        self.ensure_started()
        thread_params = {'model': model, 'ephemeral': True, 'sandbox': sandbox, 'approvalPolicy': 'never', 'cwd': cwd}
        if developer_instructions:
            thread_params['developerInstructions'] = developer_instructions
        disabled = None
        if lean:
            thread_params['config'], disabled = self._lean_config(cwd, set(keep_plugins), web_search)
        thread_id = self._request('thread/start', thread_params, timeout=120)['thread']['id']
        try:
            turn_params = {'threadId': thread_id, 'input': [{'type': 'text', 'text': brief}]}
            if effort:
                turn_params['effort'] = effort
            if output_schema is not None:
                turn_params['outputSchema'] = output_schema
            turn_id = self._request('turn/start', turn_params, timeout=120)['turn']['id']
            return self._collect(thread_id, turn_id, output_schema is not None, timeout_s, disabled)
        finally:
            self._threads.pop(thread_id, None)

    def _lean_config(self, cwd, keep_plugins, web_search):
        """A brief carries everything a worker needs, so drop context it would otherwise pay for on every call.

        Measured on a one-word brief: plugins, MCP servers and apps cost ~6.1k input tokens, the web_search tool
        ~2.5k, the skills catalog ~1.9k, goals ~0.6k and the personality section ~0.4k. The base instructions
        (~3.5k) stay, because they teach the shell and patch tools the worker relies on.
        """
        installed = self._request('plugin/installed', {'cwds': [cwd]}, timeout=60)
        plugins = sorted(plugin['id'] for market in installed.get('marketplaces') or [] for plugin in market.get('plugins') or []
                         if plugin.get('enabled') and plugin['id'] not in keep_plugins)
        servers = sorted((self._request('config/read', {'cwd': cwd}, timeout=60)['config'].get('mcp_servers') or {}))
        config = {'features': {'apps': False, 'goals': False},
                  'plugins': {plugin_id: {'enabled': False} for plugin_id in plugins},
                  'mcp_servers': {name: {'enabled': False} for name in servers},
                  # Positive values only; 1 leaves no room for any catalog entry.
                  'skills': {'max_context_tokens': 1},
                  'personality': 'none'}
        sections = ['apps', 'goals', 'skills_catalog', 'personality']
        if not web_search:
            config['web_search'] = 'disabled'
            sections.append('web_search')
        return config, {'sections': sections, 'plugins': plugins, 'mcp_servers': servers}

    def _collect(self, thread_id, turn_id, structured, timeout_s, disabled=None):
        events = self._threads[thread_id]
        deadline = time.monotonic() + timeout_s
        report = {'status': None, 'thread_id': thread_id, 'turn_id': turn_id, 'final_message': None,
                  'structured_output': None, 'commands': [], 'file_changes': [], 'refused_requests': [],
                  'usage': None, 'disabled': disabled, 'started_mcp_servers': [], 'error': None}
        while True:
            remaining = deadline - time.monotonic()
            try:
                event = events.get(timeout=min(remaining, 5)) if remaining > 0 else None
            except queue.Empty:
                event = None if remaining <= 5 else (_DEAD if not self.alive() else False)
            if event is False:
                continue
            if event is None:
                self._interrupt(thread_id, turn_id)
                report['status'] = 'timeout'
                report['error'] = {'kind': 'timeout', 'message': f'turn exceeded {timeout_s}s and was interrupted'}
                return report
            if event is _DEAD:
                report['status'] = 'bridge_error'
                report['error'] = {'kind': 'codex_exited', 'message': 'codex app-server exited mid-turn' + self._stderr_tail()}
                return report
            method, params = event
            if method == 'mcpServer/startupStatus/updated' and params.get('status') == 'ready':
                report['started_mcp_servers'].append(params.get('name'))
                continue
            if params.get('turnId', turn_id) != turn_id:
                continue
            if method == 'thread/tokenUsage/updated':
                report['usage'] = params.get('tokenUsage', {}).get('total')
            elif method == 'item/completed':
                self._record_item(report, params['item'])
            elif method == 'error' and not params.get('willRetry'):
                report['error'] = _turn_error(params.get('error') or {})
            elif method.startswith('bridge/refusedServerRequest:'):
                report['refused_requests'].append(method.split(':', 1)[1])
            elif method == 'turn/completed' and params['turn']['id'] == turn_id:
                turn = params['turn']
                report['status'] = turn['status']
                if turn.get('error'):
                    report['error'] = _turn_error(turn['error'])
                if structured and report['final_message'] is not None:
                    try:
                        report['structured_output'] = json.loads(report['final_message'])
                    except json.JSONDecodeError as exc:
                        report['error'] = report['error'] or {'kind': 'invalid_structured_output', 'message': str(exc)}
                return report

    def _interrupt(self, thread_id, turn_id):
        try:
            self._request('turn/interrupt', {'threadId': thread_id, 'turnId': turn_id}, timeout=30)
        except BridgeError as exc:
            print(f'axiom-bridge: interrupt failed: {exc.message}', file=sys.stderr)

    @staticmethod
    def _record_item(report, item):
        kind = item.get('type')
        if kind == 'agentMessage':
            report['final_message'] = item.get('text')
        elif kind == 'commandExecution':
            report['commands'].append({'command': item.get('command'), 'status': item.get('status'), 'exit_code': item.get('exitCode')})
        elif kind == 'fileChange':
            for change in item.get('changes') or []:
                report['file_changes'].append({'path': change.get('path'), 'kind': (change.get('kind') or {}).get('type'), 'status': item.get('status')})


def _turn_error(error):
    """codexErrorInfo is either a bare tag such as usageLimitExceeded or a one-key object."""
    info = error.get('codexErrorInfo')
    if isinstance(info, str):
        kind = info
    elif isinstance(info, dict) and info:
        kind = next(iter(info))
    else:
        kind = 'codex_error'
    message = error.get('message', '')
    if kind == 'other':
        # Upstream API rejections such as model_not_found arrive as `other` with the provider's JSON body as the message.
        try:
            kind = json.loads(message)['error']['code'] or kind
        except (ValueError, KeyError, TypeError):
            pass
    return {'kind': kind, 'message': message}
