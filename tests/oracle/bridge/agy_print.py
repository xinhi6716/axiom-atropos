"""Drive Antigravity (agy) print mode with NDJSON in and out."""
import json
import os
import subprocess
import tempfile
import threading

from procs import BridgeError, kill_tree, require_directory


def run(command, prompt, cwd, model, json_schema=None, timeout_s=600):
    """agy exits 0 even when the turn fails, so success is read from the result event, never the exit code.

    Without `cwd` the turn runs in a fresh empty directory, which keeps a translation read-only by leaving it nothing to
    touch. Plan mode is not used: it answers a direct request with a plan document and a question instead of the answer.
    """
    if not prompt or not prompt.strip():
        raise BridgeError('invalid_argument', 'prompt is empty')
    if cwd is not None:
        require_directory(cwd)
        return _run(command, prompt, cwd, model, json_schema, timeout_s)
    with tempfile.TemporaryDirectory(prefix='axiom-agy-') as empty:
        return _run(command, prompt, empty, model, json_schema, timeout_s)


def _run(command, prompt, cwd, model, json_schema, timeout_s):
    schema_path = None
    args = command + ['--input-format', 'stream-json', '--output-format', 'stream-json', '--model', model]
    try:
        if json_schema is not None:
            # A file avoids quoting JSON through Windows command lines.
            with tempfile.NamedTemporaryFile('w', suffix='.json', delete=False, encoding='utf-8') as handle:
                json.dump(json_schema, handle)
                schema_path = handle.name
            args += ['--json-schema', schema_path]
        args.append('-p=')
        proc = subprocess.Popen(args, cwd=cwd, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                                text=True, encoding='utf-8', errors='replace')
        events, stderr = [], []
        readers = [threading.Thread(target=_collect, args=(proc.stdout, events, True), daemon=True),
                   threading.Thread(target=_collect, args=(proc.stderr, stderr, False), daemon=True)]
        for reader in readers:
            reader.start()
        proc.stdin.write(json.dumps({'event': 'user', 'message': {'role': 'user', 'content': prompt}}, ensure_ascii=False) + '\n')
        proc.stdin.close()
        try:
            proc.wait(timeout=timeout_s)
        except subprocess.TimeoutExpired:
            kill_tree(proc)
            proc.stdout.close()
            proc.stderr.close()
            return {'status': 'timeout', 'error': {'kind': 'timeout', 'message': f'agy exceeded {timeout_s}s and was killed'}}
        for reader in readers:
            reader.join(timeout=5)
        proc.stdout.close()
        proc.stderr.close()
    finally:
        if schema_path:
            os.unlink(schema_path)
    result = next((event['result'] for event in reversed(events) if event.get('event') == 'result'), None)
    if result is None:
        tail = '\n'.join(stderr[-10:])
        return {'status': 'bridge_error', 'error': {'kind': 'agy_no_result', 'message': f'agy exited {proc.returncode} without a result event\n{tail}'.strip()}}
    report = {
        'status': 'completed',
        'response': result.get('response'),
        'structured_output': result.get('structured_output'),
        'conversation_id': result.get('conversation_id'),
        'usage': result.get('usage'),
        'error': None,
    }
    if result.get('status') != 'SUCCESS':
        report['status'] = 'failed'
        report['error'] = {'kind': 'agy_' + str(result.get('status', 'unknown')).lower(), 'message': result.get('error') or ''}
    elif not (report['response'] or '').strip() and report['structured_output'] is None:
        # A SUCCESS turn can spend its whole budget on thinking and tool calls and say nothing.
        report['status'] = 'failed'
        report['error'] = {'kind': 'empty_response', 'message': 'agy reported SUCCESS but returned no text'}
    return report


def _collect(stream, sink, parse):
    for line in stream:
        if not parse:
            sink.append(line.rstrip())
            continue
        try:
            sink.append(json.loads(line))
        except json.JSONDecodeError:
            pass
