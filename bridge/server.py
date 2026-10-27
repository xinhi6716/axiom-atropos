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
import orch_cli  # noqa: E402
from codex_app import SANDBOXES, CodexAppServer  # noqa: E402
from memory import Memory  # noqa: E402
from procs import BridgeError, data_home, resolve_command  # noqa: E402

SERVER_INFO = {'name': 'axiom-bridge', 'version': '0.1.0'}
ROLES = ('terra', 'sol')
DEFAULT_AGY_MODEL = 'gemini-4-argon-medium'

TOOLS = [
    {
        'name': 'codex_run',
        'description': (
            'Run one Codex turn in a fresh ephemeral thread and return a structured report. '
            'Write `brief` per bridge/DISPATCH.md (GOAL, SCOPE, VERIFY): the worker sees nothing else. '
            'Use role terra for modular units and role sol for integration. '
            'Status is completed, failed, interrupted, timeout or bridge_error; anything but completed carries `error`.'),
        'inputSchema': {
            'type': 'object',
            'properties': {
                'brief': {'type': 'string', 'description': 'Full worker brief; sent as the only user message.'},
                'cwd': {'type': 'string', 'description': 'Absolute working directory, ideally the unit\'s own worktree.'},
                'role': {'type': 'string', 'enum': list(ROLES), 'description': 'Resolved to a model this Codex account lists; the report names it.'},
                'model': {'type': 'string', 'description': 'Explicit model id; overrides role.'},
                'sandbox': {'type': 'string', 'enum': list(SANDBOXES), 'default': 'workspace-write'},
                'effort': {'type': 'string', 'description': 'Reasoning effort advertised by the model, e.g. low, medium, high.'},
                'developer_instructions': {'type': 'string', 'description': 'Role contract layered over the user\'s Codex AGENTS.md.'},
                'output_schema': {'type': 'object', 'description': 'JSON Schema for the final message; parsed into structured_output.'},
                'timeout_s': {'type': 'integer', 'minimum': 10, 'default': 1800},
                'lean': {'type': 'boolean', 'default': True,
                         'description': ('For this thread only, disable every Codex plugin, configured MCP server, the apps connector, '
                                         'goals, the skills catalog, the personality section and (unless web_search) the web search tool.')},
                'web_search': {'type': 'boolean', 'default': False, 'description': 'Keep the web search tool in lean mode; costs ~2.5k input tokens per call.'},
                'keep_plugins': {'type': 'array', 'items': {'type': 'string'}, 'description': 'Plugin ids such as name@marketplace that stay enabled in lean mode.'},
            },
            'required': ['brief', 'cwd'],
        },
    },
    {
        'name': 'gemini_preview',
        'description': (
            'Have a Gemini model (through Antigravity, agy) rewrite worker reports into plain language for the user: '
            'what was done, how it turned out, what the user must decide. Put the reports in `prompt`. Without `cwd` it runs '
            'in an empty directory, so there is nothing for it to change. Status is completed, failed, timeout or bridge_error; '
            'an empty reply is failed with kind empty_response.'),
        'inputSchema': {
            'type': 'object',
            'properties': {
                'prompt': {'type': 'string'},
                'cwd': {'type': 'string', 'description': 'Absolute directory agy may read; omit to run in an empty directory.'},
                'model': {'type': 'string', 'default': DEFAULT_AGY_MODEL},
                'json_schema': {'type': 'object', 'description': 'Optional JSON Schema; result returned as structured_output.'},
                'timeout_s': {'type': 'integer', 'minimum': 10, 'default': 600},
            },
            'required': ['prompt'],
        },
    },
    {
        'name': 'orch',
        'description': (
            "Run Pstack's orch bookkeeping CLI (units, inbox, ledger, gates, standing orders, status) on the store for `project`. "
            'The store lives in the plugin data directory and survives plugin updates; its path is returned as `store`.'),
        'inputSchema': {
            'type': 'object',
            'properties': {
                'project': {'type': 'string', 'description': 'Lowercase slug naming the program, e.g. todo-stats.'},
                'args': {'type': 'array', 'items': {'type': 'string'}, 'description': 'orch arguments, e.g. ["unit", "add", "u1", "--track", "build"].'},
            },
            'required': ['project', 'args'],
        },
    },
    {
        'name': 'memory_record',
        'description': ('Store one verified unit as long-run memory: what it built, its interface, and what a later worker reusing '
                        'it must know. Files are hashed so later searches can flag the entry stale once they change.'),
        'inputSchema': {
            'type': 'object',
            'properties': {
                'project': {'type': 'string', 'description': 'Program slug, the same one orch uses.'},
                'project_dir': {'type': 'string', 'description': 'Absolute project directory; files are relative to it.'},
                'unit': {'type': 'string'},
                'title': {'type': 'string', 'description': 'The module, function or feature, as a later brief would name it.'},
                'summary': {'type': 'string', 'description': 'One or two lines: what it does and its interface.'},
                'body': {'type': 'string', 'description': 'What a reuser needs: signatures, behaviour decisions, verify command, pitfalls met.'},
                'symbols': {'type': 'array', 'items': {'type': 'string'}},
                'files': {'type': 'array', 'items': {'type': 'string'}},
                'wish': {'type': 'string', 'description': "The wish verbatim, in the user's words; indexed so searches in their language find the entry."},
            },
            'required': ['project', 'project_dir', 'unit', 'title', 'summary', 'body', 'wish'],
        },
    },
    {
        'name': 'memory_search',
        'description': ('Find earlier units related to `query` (English words and Chinese both match). Returns titles and '
                        'summaries only, with `stale` true when a recorded file has changed since; fetch a body with memory_get.'),
        'inputSchema': {
            'type': 'object',
            'properties': {
                'query': {'type': 'string'},
                'project': {'type': 'string', 'description': 'Limit to one program; omit to search every project.'},
                'limit': {'type': 'integer', 'minimum': 1, 'maximum': 20, 'default': 5},
            },
            'required': ['query'],
        },
    },
    {
        'name': 'memory_get',
        'description': 'Return one memory entry in full, with its files and whether it is stale.',
        'inputSchema': {'type': 'object', 'properties': {'id': {'type': 'integer'}}, 'required': ['id']},
    },
    {
        'name': 'memory_digest',
        'description': ("Read a project's short history of wishes and outcomes, one line each; pass `add` to append a line first."),
        'inputSchema': {
            'type': 'object',
            'properties': {'project': {'type': 'string'}, 'add': {'type': 'string', 'description': 'One line: the wish, the outcome, memory ids.'}},
            'required': ['project'],
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
            role = args.get('role')
            if role is not None and role not in ROLES:
                raise BridgeError('invalid_argument', f'role must be one of {ROLES}')
            codex = self.codex()
            model = args.get('model') or (codex.resolve_model(role) if role else None)
            return codex.run(
                args.get('brief'), args.get('cwd'), model,
                args.get('sandbox') or 'workspace-write', args.get('effort'), args.get('output_schema'),
                args.get('developer_instructions'), int(args.get('timeout_s') or 1800),
                args.get('lean', True) is not False, args.get('keep_plugins') or (), args.get('web_search') is True)
        if name == 'gemini_preview':
            return agy_print.run(
                resolve_command('AXIOM_AGY_CMD', 'agy'), args.get('prompt'), args.get('cwd'),
                args.get('model') or DEFAULT_AGY_MODEL, args.get('json_schema'), int(args.get('timeout_s') or 600))
        if name.startswith('memory_'):
            memory = Memory(data_home())
            if name == 'memory_record':
                return memory.record(args.get('project'), args.get('unit'), args.get('title'), args.get('summary'), args.get('body'),
                                     args.get('symbols'), args.get('files'), args.get('project_dir'), args.get('wish'))
            if name == 'memory_search':
                return memory.search(args.get('query'), args.get('project'), args.get('limit') or 5)
            if name == 'memory_get':
                return memory.get(args.get('id'))
            if name == 'memory_digest':
                return memory.digest(args.get('project'), args.get('add'))
        if name == 'orch':
            return orch_cli.run(resolve_command('AXIOM_BUN_CMD', 'bun'), args.get('project'), args.get('args'))
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
