"""Axiom Forge bridge: a dependency-free MCP stdio server exposing Codex and agy as typed tools."""
import atexit
import json
import os
import subprocess
import sys
import threading
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import agy_print  # noqa: E402
from codex_app import SANDBOXES, CodexAppServer  # noqa: E402
from procs import BridgeError, resolve_command  # noqa: E402

SERVER_INFO = {'name': 'axiom-bridge', 'version': '0.1.0'}
DEFAULT_CODEX_MODEL = 'gpt-6.1-terra'
DEFAULT_AGY_MODEL = 'gemini-4-argon-medium'

TOOLS = [
    {
        'name': 'codex_run',
        'description': (
            'Run one Codex turn in a fresh ephemeral thread and return a structured report. '
            'Pass a complete Pstack brief (GOAL, SCOPE, CONTEXT, ACCEPTANCE, VERIFY, TIMEBOX, FORBIDDEN, REPORT, STANDING) '
            'as `brief`: the worker cannot see this chat. Use a terra model for modular units and a sol model for integration. '
            'Status is completed, failed, interrupted, timeout or bridge_error; anything but completed carries `error`.'),
        'inputSchema': {
            'type': 'object',
            'properties': {
                'brief': {'type': 'string', 'description': 'Full worker brief; sent as the only user message.'},
                'cwd': {'type': 'string', 'description': 'Absolute working directory, ideally the unit\'s own worktree.'},
                'model': {'type': 'string', 'default': DEFAULT_CODEX_MODEL},
                'sandbox': {'type': 'string', 'enum': list(SANDBOXES), 'default': 'workspace-write'},
                'effort': {'type': 'string', 'description': 'Reasoning effort advertised by the model, e.g. low, medium, high.'},
                'developer_instructions': {'type': 'string', 'description': 'Role contract layered over the user\'s Codex AGENTS.md.'},
                'output_schema': {'type': 'object', 'description': 'JSON Schema for the final message; parsed into structured_output.'},
                'timeout_s': {'type': 'integer', 'minimum': 10, 'default': 1800},
                'lean': {'type': 'boolean', 'default': True,
                         'description': 'Disable every Codex plugin, configured MCP server and the apps connector for this thread to cut per-call context.'},
                'keep_plugins': {'type': 'array', 'items': {'type': 'string'}, 'description': 'Plugin ids such as name@marketplace that stay enabled in lean mode.'},
            },
            'required': ['brief', 'cwd'],
        },
    },
    {
        'name': 'gemini_preview',
        'description': (
            'Ask a Gemini model through Antigravity (agy) for a read-only, human-facing explanation of finished work. '
            'Runs in plan mode, so it does not edit files. Status is completed, failed, timeout or bridge_error.'),
        'inputSchema': {
            'type': 'object',
            'properties': {
                'prompt': {'type': 'string'},
                'cwd': {'type': 'string', 'description': 'Absolute directory agy may read.'},
                'model': {'type': 'string', 'default': DEFAULT_AGY_MODEL},
                'json_schema': {'type': 'object', 'description': 'Optional JSON Schema; result returned as structured_output.'},
                'timeout_s': {'type': 'integer', 'minimum': 10, 'default': 600},
            },
            'required': ['prompt', 'cwd'],
        },
    },
    {
        'name': 'bridge_status',
        'description': ('Report which Codex, agy and Bun executables the bridge resolved and their versions, whether Java and '
                        'tla2tools.jar are configured for statechart, and whether the Codex app-server is running.'),
        'inputSchema': {'type': 'object', 'properties': {}},
    },
]


class Bridge:
    def __init__(self):
        self._codex = None
        self._codex_lock = threading.Lock()

    def codex(self):
        with self._codex_lock:
            if self._codex is None:
                self._codex = CodexAppServer(resolve_command('AXIOM_CODEX_CMD', 'codex'))
                atexit.register(self._codex.close)
            return self._codex

    def call(self, name, args):
        if name == 'codex_run':
            return self.codex().run(
                args.get('brief'), args.get('cwd'), args.get('model') or DEFAULT_CODEX_MODEL,
                args.get('sandbox') or 'workspace-write', args.get('effort'), args.get('output_schema'),
                args.get('developer_instructions'), int(args.get('timeout_s') or 1800),
                args.get('lean', True) is not False, args.get('keep_plugins') or ())
        if name == 'gemini_preview':
            return agy_print.run(
                resolve_command('AXIOM_AGY_CMD', 'agy'), args.get('prompt'), args.get('cwd'),
                args.get('model') or DEFAULT_AGY_MODEL, args.get('json_schema'), int(args.get('timeout_s') or 600))
        if name == 'bridge_status':
            return self.status()
        raise BridgeError('unknown_tool', f'unknown tool {name}')

    def status(self):
        report = {}
        for key, env_name, executable in (('codex', 'AXIOM_CODEX_CMD', 'codex'), ('agy', 'AXIOM_AGY_CMD', 'agy'), ('bun', 'AXIOM_BUN_CMD', 'bun')):
            try:
                command = resolve_command(env_name, executable)
                probe = subprocess.run(command + ['--version'], stdin=subprocess.DEVNULL, capture_output=True, text=True, encoding='utf-8', errors='replace', timeout=30)
                report[key] = {'command': command, 'version': (probe.stdout or probe.stderr).strip().splitlines()[-1:] or None, 'exit_code': probe.returncode}
            except (BridgeError, OSError, subprocess.TimeoutExpired) as exc:
                report[key] = {'error': exc.as_dict() if isinstance(exc, BridgeError) else {'kind': 'probe_failed', 'message': str(exc)}}
        java, jar = os.environ.get('JAVA'), os.environ.get('TLA2TOOLS')
        report['statechart_tlc'] = {'JAVA': java, 'java_exists': bool(java and os.path.isfile(java)),
                                    'TLA2TOOLS': jar, 'jar_exists': bool(jar and os.path.isfile(jar))}
        report['codex_app_server_running'] = self._codex is not None and self._codex.alive()
        return report


class StdioServer:
    def __init__(self, bridge, stdin=sys.stdin, stdout=sys.stdout):
        self.bridge = bridge
        self.stdin = stdin
        self.stdout = stdout
        self._write_lock = threading.Lock()

    def write(self, message):
        with self._write_lock:
            self.stdout.write(json.dumps(message, ensure_ascii=False) + '\n')
            self.stdout.flush()

    def serve(self):
        for line in self.stdin:
            if not line.strip():
                continue
            try:
                message = json.loads(line)
            except json.JSONDecodeError as exc:
                self.write({'jsonrpc': '2.0', 'id': None, 'error': {'code': -32700, 'message': f'parse error: {exc}'}})
                continue
            if 'id' not in message:
                continue
            if message.get('method') == 'tools/call':
                # Tool calls can run for many minutes; keep the read loop free for concurrent calls and pings.
                threading.Thread(target=self.handle, args=(message,), daemon=True).start()
            else:
                self.handle(message)

    def handle(self, message):
        request_id, method, params = message['id'], message.get('method'), message.get('params') or {}
        if method == 'initialize':
            result = {'protocolVersion': params.get('protocolVersion', '2025-06-18'), 'capabilities': {'tools': {}}, 'serverInfo': SERVER_INFO}
        elif method == 'ping':
            result = {}
        elif method == 'tools/list':
            result = {'tools': TOOLS}
        elif method == 'tools/call':
            result = self.call_tool(params.get('name'), params.get('arguments') or {})
        else:
            self.write({'jsonrpc': '2.0', 'id': request_id, 'error': {'code': -32601, 'message': f'method not found: {method}'}})
            return
        self.write({'jsonrpc': '2.0', 'id': request_id, 'result': result})

    def call_tool(self, name, args):
        try:
            payload = self.bridge.call(name, args)
            failed = payload.get('status') not in (None, 'completed')
        except BridgeError as exc:
            payload, failed = {'status': 'bridge_error', 'error': exc.as_dict()}, True
        except Exception as exc:  # noqa: BLE001 - an unexpected crash must reach the caller as an error, not kill the server
            payload, failed = {'status': 'bridge_error', 'error': {'kind': type(exc).__name__, 'message': str(exc)}}, True
        return {'content': [{'type': 'text', 'text': json.dumps(payload, ensure_ascii=False, indent=2)}],
                'structuredContent': payload, 'isError': failed}


def main():
    sys.stdin.reconfigure(encoding='utf-8')
    sys.stdout.reconfigure(encoding='utf-8')
    StdioServer(Bridge()).serve()


if __name__ == '__main__':
    main()
