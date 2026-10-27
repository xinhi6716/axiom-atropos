"""Drive Antigravity (agy) print mode with NDJSON in and out."""
import json
import os
import subprocess
import tempfile
import threading

from procs import BridgeError, kill_tree, require_directory


def run(command, prompt, cwd, model, json_schema=None, timeout_s=600):
    """agy exits 0 even when the turn fails, so success is read from the result event, never the exit code."""
    require_directory(cwd)
    if not prompt or not prompt.strip():
        raise BridgeError('invalid_argument', 'prompt is empty')
    schema_path = None
    # Plan mode keeps the preview role read-only.
    args = command + ['--input-format', 'stream-json', '--output-format', 'stream-json', '--model', model, '--mode', 'plan']
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
    succeeded = result.get('status') == 'SUCCESS'
    return {
        'status': 'completed' if succeeded else 'failed',
        'response': result.get('response'),
        'structured_output': result.get('structured_output'),
        'conversation_id': result.get('conversation_id'),
        'usage': result.get('usage'),
        'error': None if succeeded else {'kind': 'agy_' + str(result.get('status', 'unknown')).lower(), 'message': result.get('error') or ''},
    }


def _collect(stream, sink, parse):
    for line in stream:
        if not parse:
            sink.append(line.rstrip())
            continue
        try:
            sink.append(json.loads(line))
        except json.JSONDecodeError:
            pass
