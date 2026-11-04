"""Stand-in for `bun orch.ts ...`: echoes its arguments, fails when asked to."""
import json
import sys

if '--version' in sys.argv:
    print('1.4.4-fake')
    sys.exit(0)
args = sys.argv[1:]
if 'boom' in args:
    sys.stderr.write('orch: unknown command boom\n')
    sys.exit(2)
print(json.dumps({'script': args[0], 'store': args[args.index('--store') + 1], 'rest': args[args.index('--store') + 2:]}))
