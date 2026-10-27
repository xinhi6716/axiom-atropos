"""Run Pstack's orch bookkeeping CLI under Bun with a per-project store outside the plugin cache."""
import subprocess
from pathlib import Path

from procs import BridgeError, data_home, require_slug

ORCH_SCRIPT = Path(__file__).resolve().parents[1] / 'skills' / 'poteto-mode' / 'scripts' / 'orch' / 'orch.ts'


def store_for(project):
    require_slug('project', project)
    return data_home() / 'orchestrate' / project


def run(command, project, args, timeout_s=120):
    if not isinstance(args, list) or not args or not all(isinstance(arg, str) for arg in args):
        raise BridgeError('invalid_argument', 'args must be a non-empty array of strings, e.g. ["unit", "add", "u1"]')
    store = store_for(project)
    store.parent.mkdir(parents=True, exist_ok=True)
    try:
        done = subprocess.run(command + [str(ORCH_SCRIPT), '--store', str(store)] + args, cwd=ORCH_SCRIPT.parent.parent,
                              stdin=subprocess.DEVNULL, capture_output=True, text=True, encoding='utf-8', errors='replace', timeout=timeout_s)
    except subprocess.TimeoutExpired:
        raise BridgeError('timeout', f'orch {" ".join(args)} exceeded {timeout_s}s') from None
    return {'status': 'completed' if done.returncode == 0 else 'failed', 'store': str(store), 'exit_code': done.returncode,
            'stdout': done.stdout.strip(), 'stderr': done.stderr.strip(),
            'error': None if done.returncode == 0 else {'kind': 'orch_failed', 'message': (done.stderr or done.stdout).strip()[-2000:]}}
