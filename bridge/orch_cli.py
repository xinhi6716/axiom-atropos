"""Run Pstack's orch bookkeeping CLI under Bun with a per-project store outside the plugin cache."""
import os
import re
import subprocess
from pathlib import Path

from procs import BridgeError

ORCH_SCRIPT = Path(__file__).resolve().parents[1] / 'skills' / 'poteto-mode' / 'scripts' / 'orch' / 'orch.ts'
PROJECT = re.compile(r'[a-z0-9][a-z0-9-]{0,63}')


def store_for(project):
    if not PROJECT.fullmatch(project or ''):
        raise BridgeError('invalid_argument', 'project must be a lowercase slug: letters, digits and hyphens')
    # The plugin root changes on every update; CLAUDE_PLUGIN_DATA survives it.
    home = os.environ.get('AXIOM_ORCH_HOME') or os.environ.get('CLAUDE_PLUGIN_DATA')
    if not home:
        raise BridgeError('config', 'neither AXIOM_ORCH_HOME nor CLAUDE_PLUGIN_DATA is set, so there is no place for orch stores')
    return Path(home) / 'orchestrate' / project


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
