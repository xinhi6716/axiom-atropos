"""Scripted stand-in for `agy` print mode; the prompt text selects the scenario."""
import json
import sys
import time

sys.stdin.reconfigure(encoding='utf-8')
sys.stdout.reconfigure(encoding='utf-8')
if '--version' in sys.argv:
    print('1.4.7-fake')
    sys.exit(0)

args = sys.argv[1:]
assert args[-1] == '-p=' and '--mode' not in args, args
schema = None
if '--json-schema' in args:
    with open(args[args.index('--json-schema') + 1], encoding='utf-8') as handle:
        schema = json.load(handle)
event = json.loads(sys.stdin.readline())
prompt = event['message']['content']


def emit(payload):
    sys.stdout.write(json.dumps(payload, ensure_ascii=False) + '\n')
    sys.stdout.flush()


emit({'event': 'init', 'conversation_id': 'c1'})
if 'AGY_HANG' in prompt:
    time.sleep(60)
if 'AGY_SILENT' in prompt:
    sys.stderr.write('agy: crashed before result\n')
    sys.exit(3)
if 'AGY_EMPTY' in prompt:
    emit({'event': 'result', 'result': {'conversation_id': 'c1', 'status': 'SUCCESS', 'response': ''}})
    sys.exit(0)
if 'AGY_CWD' in prompt:
    import os
    emit({'event': 'result', 'result': {'conversation_id': 'c1', 'status': 'SUCCESS', 'response': json.dumps(sorted(os.listdir('.')))}})
    sys.exit(0)
if 'AGY_ERROR' in prompt:
    emit({'event': 'result', 'result': {'conversation_id': 'c1', 'status': 'ERROR', 'response': '', 'error': 'model refused'}})
    sys.exit(0)
result = {'conversation_id': 'c1', 'status': 'SUCCESS', 'response': f'echo: {prompt}', 'usage': {'total_tokens': 1}}
if schema is not None:
    result['structured_output'] = {'answer': prompt, 'schema_keys': sorted(schema.get('properties', {}))}
emit({'event': 'result', 'result': result})
