"""Exercise the bridge against scripted Codex and agy stand-ins, end to end over MCP stdio."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))

import agy_print  # noqa: E402
import orch_cli  # noqa: E402
from codex_app import CodexAppServer  # noqa: E402
from procs import BridgeError, resolve_command  # noqa: E402

FAKE_CODEX = [sys.executable, '-I', str(HERE / 'fakes' / 'fake_codex.py')]
FAKE_AGY = [sys.executable, '-I', str(HERE / 'fakes' / 'fake_agy.py')]
FAKE_BUN = [sys.executable, '-I', str(HERE / 'fakes' / 'fake_bun.py')]


class CodexAppServerTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.cwd = self.temp.name
        self.codex = CodexAppServer(FAKE_CODEX)
        self.addCleanup(self.codex.close)

    def run_brief(self, brief, **kwargs):
        options = {'model': 'gpt-6.1-terra', 'sandbox': 'workspace-write', 'timeout_s': 20, **kwargs}
        return self.codex.run(brief, self.cwd, **options)

    def test_completed_turn_reports_work(self):
        report = self.run_brief('GOAL 寫一個模組', effort='low')
        self.assertEqual(report['status'], 'completed')
        self.assertIsNone(report['error'])
        echo = json.loads(report['final_message'])
        self.assertEqual((echo['model'], echo['effort'], echo['brief']), ('gpt-6.1-terra', 'low', 'GOAL 寫一個模組'))
        self.assertEqual(report['commands'], [{'command': 'pytest -q', 'status': 'completed', 'exit_code': 0}])
        self.assertEqual(report['file_changes'], [{'path': 'src/mod.py', 'kind': 'add', 'status': 'completed'}])

    def test_lean_mode_disables_plugins_servers_and_apps(self):
        report = self.run_brief('GOAL lean')
        self.assertEqual(report['disabled'], {'sections': ['apps', 'goals', 'skills_catalog', 'personality', 'web_search'],
                                              'plugins': ['keepme@personal', 'panel@personal'], 'mcp_servers': ['node_repl']})
        self.assertEqual(json.loads(report['final_message'])['config']['skills'], {'max_context_tokens': 1})
        self.assertEqual(json.loads(report['final_message'])['config']['web_search'], 'disabled')
        self.assertEqual(report['started_mcp_servers'], [])
        self.assertEqual(report['usage']['totalTokens'], 1240)

    def test_web_search_can_stay_on_in_lean_mode(self):
        report = self.run_brief('GOAL search', web_search=True)
        self.assertNotIn('web_search', report['disabled']['sections'])
        self.assertNotIn('web_search', json.loads(report['final_message'])['config'])

    def test_keep_plugins_and_inherit_mode(self):
        kept = self.run_brief('GOAL keep', keep_plugins=['keepme@personal'])
        self.assertEqual(kept['started_mcp_servers'], ['keepme'])
        inherited = self.run_brief('GOAL inherit', lean=False)
        self.assertIsNone(inherited['disabled'])
        self.assertEqual(sorted(inherited['started_mcp_servers']), ['codex_apps', 'keepme', 'node_repl', 'panel'])

    def test_output_schema_is_parsed(self):
        report = self.run_brief('GOAL structured', output_schema={'type': 'object'})
        self.assertEqual(report['structured_output'], {'answer': 'ok'})

    def test_usage_limit_surfaces_as_failure(self):
        report = self.run_brief('LIMIT')
        self.assertEqual(report['status'], 'failed')
        self.assertEqual(report['error']['kind'], 'usageLimitExceeded')

    def test_provider_rejection_names_its_code(self):
        self.assertEqual(self.run_brief('BADMODEL')['error']['kind'], 'model_not_found')

    def test_server_request_is_refused_not_hung(self):
        report = self.run_brief('APPROVAL')
        self.assertEqual(report['status'], 'completed')
        self.assertEqual(report['refused_requests'], ['item/commandExecution/requestApproval'])

    def test_timeout_interrupts_turn(self):
        report = self.run_brief('HANG', timeout_s=1)
        self.assertEqual(report['status'], 'timeout')
        self.assertEqual(report['error']['kind'], 'timeout')

    def test_crash_is_reported_then_server_restarts(self):
        report = self.run_brief('CRASH')
        self.assertEqual(report['status'], 'bridge_error')
        self.assertEqual(report['error']['kind'], 'codex_exited')
        self.assertEqual(self.run_brief('GOAL after crash')['status'], 'completed')

    def test_one_app_server_serves_sequential_turns(self):
        first = self.run_brief('GOAL one')
        self.assertEqual(self.run_brief('GOAL two')['status'], 'completed')
        self.assertNotEqual(first['thread_id'], self.run_brief('GOAL three')['thread_id'])

    def test_roles_resolve_from_the_account_model_list(self):
        self.assertEqual(self.codex.resolve_model('terra'), 'gpt-5.6-terra')
        self.assertEqual(self.codex.resolve_model('sol'), 'gpt-6-luna')

    def test_rejects_missing_model(self):
        with self.assertRaises(BridgeError) as caught:
            self.codex.run('GOAL', self.cwd, None, 'read-only')
        self.assertEqual(caught.exception.kind, 'invalid_argument')

    def test_rejects_full_access_and_bad_cwd(self):
        with self.assertRaises(BridgeError) as caught:
            self.run_brief('GOAL', sandbox='danger-full-access')
        self.assertEqual(caught.exception.kind, 'invalid_argument')
        with self.assertRaises(BridgeError):
            self.codex.run('GOAL', 'relative/dir', 'm', 'read-only')


class AgyTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)

    def run_prompt(self, prompt, **kwargs):
        return agy_print.run(FAKE_AGY, prompt, self.temp.name, 'gemini-4-argon-medium', timeout_s=kwargs.pop('timeout_s', 20), **kwargs)

    def test_success_with_schema(self):
        result = self.run_prompt('解釋這個模組', json_schema={'type': 'object', 'properties': {'answer': {'type': 'string'}}})
        self.assertEqual(result['status'], 'completed')
        self.assertEqual(result['structured_output'], {'answer': '解釋這個模組', 'schema_keys': ['answer']})

    def test_error_status_with_exit_zero_is_a_failure(self):
        result = self.run_prompt('AGY_ERROR')
        self.assertEqual(result['status'], 'failed')
        self.assertEqual(result['error'], {'kind': 'agy_error', 'message': 'model refused'})

    def test_missing_result_event_is_explicit(self):
        result = self.run_prompt('AGY_SILENT')
        self.assertEqual(result['status'], 'bridge_error')
        self.assertIn('crashed before result', result['error']['message'])

    def test_timeout_kills_process(self):
        result = self.run_prompt('AGY_HANG', timeout_s=1)
        self.assertEqual(result['status'], 'timeout')


class OrchTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        os.environ['AXIOM_ORCH_HOME'] = self.temp.name
        self.addCleanup(os.environ.pop, 'AXIOM_ORCH_HOME')

    def test_store_and_args_reach_orch(self):
        result = orch_cli.run(FAKE_BUN, 'todo-stats', ['unit', 'add', 'u1'])
        echoed = json.loads(result['stdout'])
        self.assertEqual(result['status'], 'completed')
        self.assertEqual(Path(echoed['store']), Path(self.temp.name) / 'orchestrate' / 'todo-stats')
        self.assertEqual(echoed['rest'], ['unit', 'add', 'u1'])
        self.assertTrue(echoed['script'].endswith('orch.ts'))

    def test_failures_and_bad_input_are_explicit(self):
        failed = orch_cli.run(FAKE_BUN, 'p', ['boom'])
        self.assertEqual((failed['status'], failed['error']['kind']), ('failed', 'orch_failed'))
        self.assertIn('unknown command boom', failed['error']['message'])
        for project, args in (('Bad Slug', ['status']), ('p', []), ('p', 'status')):
            with self.assertRaises(BridgeError):
                orch_cli.run(FAKE_BUN, project, args)

    def test_store_location_must_be_configured(self):
        os.environ.pop('AXIOM_ORCH_HOME')
        os.environ.pop('CLAUDE_PLUGIN_DATA', None)
        with self.assertRaises(BridgeError) as caught:
            orch_cli.store_for('p')
        self.assertEqual(caught.exception.kind, 'config')
        os.environ['AXIOM_ORCH_HOME'] = self.temp.name

    @unittest.skipUnless(shutil.which('bun'), 'bun is not installed')
    def test_real_orch_round_trip(self):
        bun = [shutil.which('bun')]
        for args in (['init'], ['unit', 'add', 'u1', '--track', 'build'], ['inbox', 'push', 'terra', 'u1', 'done'], ['inbox', 'drain']):
            self.assertEqual(orch_cli.run(bun, 'round-trip', args)['status'], 'completed', args)
        listed = orch_cli.run(bun, 'round-trip', ['unit', 'list'])
        self.assertIn('u1', listed['stdout'])


class ResolveCommandTests(unittest.TestCase):
    def test_override_must_be_json_array(self):
        os.environ['AXIOM_TEST_CMD'] = 'not json'
        self.addCleanup(os.environ.pop, 'AXIOM_TEST_CMD')
        with self.assertRaises(BridgeError) as caught:
            resolve_command('AXIOM_TEST_CMD', 'definitely-not-installed')
        self.assertEqual(caught.exception.kind, 'config')

    def test_missing_executable_is_explicit(self):
        with self.assertRaises(BridgeError) as caught:
            resolve_command('AXIOM_UNSET_CMD', 'definitely-not-installed-axiom')
        self.assertEqual(caught.exception.kind, 'not_installed')


class McpStdioTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        env = {**os.environ, 'AXIOM_CODEX_CMD': json.dumps(FAKE_CODEX), 'AXIOM_AGY_CMD': json.dumps(FAKE_AGY),
               'AXIOM_BUN_CMD': json.dumps(FAKE_BUN), 'AXIOM_ORCH_HOME': self.temp.name}
        self.proc = subprocess.Popen([sys.executable, '-I', str(HERE / 'server.py')], stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                     text=True, encoding='utf-8', env=env)
        self.addCleanup(self.shutdown)
        self.next_id = 0

    def shutdown(self):
        self.proc.stdin.close()
        self.proc.wait(timeout=20)
        self.proc.stdout.close()

    def rpc(self, method, params=None):
        self.next_id += 1
        self.proc.stdin.write(json.dumps({'jsonrpc': '2.0', 'id': self.next_id, 'method': method, 'params': params or {}}) + '\n')
        self.proc.stdin.flush()
        reply = json.loads(self.proc.stdout.readline())
        self.assertEqual(reply['id'], self.next_id)
        return reply

    def test_full_session(self):
        init = self.rpc('initialize', {'protocolVersion': '2025-06-18', 'capabilities': {}, 'clientInfo': {'name': 't', 'version': '0'}})
        self.assertEqual(init['result']['serverInfo']['name'], 'axiom-bridge')
        self.proc.stdin.write(json.dumps({'jsonrpc': '2.0', 'method': 'notifications/initialized'}) + '\n')
        names = [tool['name'] for tool in self.rpc('tools/list')['result']['tools']]
        self.assertEqual(names, ['codex_run', 'gemini_preview', 'orch', 'bridge_status'])

        done = self.rpc('tools/call', {'name': 'codex_run', 'arguments': {'brief': 'GOAL x', 'cwd': self.temp.name, 'role': 'terra'}})['result']
        self.assertFalse(done['isError'])
        self.assertEqual((done['structuredContent']['status'], done['structuredContent']['model']), ('completed', 'gpt-5.6-terra'))

        no_model = self.rpc('tools/call', {'name': 'codex_run', 'arguments': {'brief': 'GOAL x', 'cwd': self.temp.name}})['result']
        self.assertEqual(no_model['structuredContent']['error']['kind'], 'invalid_argument')

        orch = self.rpc('tools/call', {'name': 'orch', 'arguments': {'project': 'demo', 'args': ['status']}})['result']['structuredContent']
        self.assertEqual(json.loads(orch['stdout'])['rest'], ['status'])

        limited = self.rpc('tools/call', {'name': 'codex_run', 'arguments': {'brief': 'LIMIT', 'cwd': self.temp.name, 'model': 'm'}})['result']
        self.assertTrue(limited['isError'])
        self.assertEqual(limited['structuredContent']['error']['kind'], 'usageLimitExceeded')

        preview = self.rpc('tools/call', {'name': 'gemini_preview', 'arguments': {'prompt': 'hi', 'cwd': self.temp.name}})['result']
        self.assertEqual(preview['structuredContent']['response'], 'echo: hi')

        bad = self.rpc('tools/call', {'name': 'codex_run', 'arguments': {'brief': 'GOAL', 'cwd': 'nope', 'model': 'm'}})['result']
        self.assertTrue(bad['isError'])
        self.assertEqual(bad['structuredContent']['error']['kind'], 'invalid_argument')

        status = self.rpc('tools/call', {'name': 'bridge_status', 'arguments': {}})['result']['structuredContent']
        self.assertTrue(status['codex_app_server_running'])
        self.assertEqual(status['agy'].get('version'), ['1.4.7-fake'], status)

        self.assertEqual(self.rpc('no/such/method')['error']['code'], -32601)


if __name__ == '__main__':
    unittest.main()
