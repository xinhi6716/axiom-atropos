"""Resolve host CLIs and stop their process trees."""
import json
import os
import re
import shutil
import subprocess
from pathlib import Path

SLUG = re.compile(r'[a-z0-9][a-z0-9-]{0,63}')


class BridgeError(Exception):
    """A failure the caller must see; never converted into an empty success."""

    def __init__(self, kind, message):
        super().__init__(message)
        self.kind = kind
        self.message = message

    def as_dict(self):
        return {'kind': self.kind, 'message': self.message}


def resolve_command(env_name, executable):
    """An env override is a JSON array; otherwise the executable must be on PATH."""
    override = os.environ.get(env_name)
    if override:
        try:
            command = json.loads(override)
        except json.JSONDecodeError as exc:
            raise BridgeError('config', f'{env_name} must be a JSON array of strings: {exc}') from exc
        if not isinstance(command, list) or not command or not all(isinstance(part, str) for part in command):
            raise BridgeError('config', f'{env_name} must be a non-empty JSON array of strings')
        return command
    found = shutil.which(executable)
    if not found:
        raise BridgeError('not_installed', f'{executable} is not on PATH; install it or set {env_name}')
    return [found]


def kill_tree(proc):
    """npm shims put cmd.exe and node between us and the real binary, so kill the whole tree."""
    if proc.poll() is not None:
        return
    if os.name == 'nt':
        subprocess.run(['taskkill', '/T', '/F', '/PID', str(proc.pid)], capture_output=True)
    else:
        proc.kill()
    try:
        proc.wait(timeout=10)
    except subprocess.TimeoutExpired:
        pass


def require_slug(name, value):
    if not SLUG.fullmatch(value or ''):
        raise BridgeError('invalid_argument', f'{name} must be a lowercase slug: letters, digits and hyphens')


def data_home():
    """Where orch stores and memory live. The plugin root changes on every update; CLAUDE_PLUGIN_DATA survives it."""
    home = os.environ.get('AXIOM_DATA_HOME') or os.environ.get('CLAUDE_PLUGIN_DATA')
    if not home:
        raise BridgeError('config', 'neither AXIOM_DATA_HOME nor CLAUDE_PLUGIN_DATA is set, so there is nowhere to keep state')
    return Path(home)


def require_directory(cwd):
    if not cwd or not os.path.isabs(cwd) or not os.path.isdir(cwd):
        raise BridgeError('invalid_argument', f'cwd must be an existing absolute directory: {cwd!r}')
