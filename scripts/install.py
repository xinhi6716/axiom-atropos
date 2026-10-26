"""Install/update Axiom Forge without replacing user-owned skills or settings."""
import argparse
import hashlib
import io
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import urllib.request
import zipfile

REPO = 'xinhi6716/axiom-forge'
HOSTS = {
    'deepseek': '.dsh', 'cursor': '.cursor', 'gemini': '.gemini',
    'copilot': '.copilot', 'opencode': '.config/opencode',
    'cline': '.cline', 'hermes': '.hermes',
    'windsurf': '.codeium/windsurf', 'roo': '.roo', 'kilo': '.kilo',
    'antigravity': '.gemini/antigravity', 'amp': '.config/agents',
    'goose': '.config/goose', 'kimi': '.agents', 'qwen': '.qwen',
    'pi': '.agents', 'openclaw': '.openclaw', 'continue': '.continue',
    'droid': '.factory', 'kiro': '.kiro', 'trae': '.trae',
    'trae-cn': '.trae-cn', 'qoder': '.qoder', 'qoder-cn': '.qoder-cn',
    'openhands': '.openhands', 'mistral-vibe': '.vibe',
    'augment': '.augment', 'junie': '.junie', 'crush': '.config/crush',
    'warp': '.agents', 'zed': '.agents', 'minimax': '.minimax',
    'codebuddy': '.codebuddy', 'devin': '.config/devin',
    'deepagents': '.deepagents/agent', 'universal': '.agents',
}

ALIASES = {'deepseek-harness': 'deepseek', 'claude-code': 'claude',
           'gemini-cli': 'gemini', 'github-copilot': 'copilot',
           'qwen-code': 'qwen', 'kimi-code-cli': 'kimi', 'hermes-agent': 'hermes'}


def host_base(host, home):
    overrides = {'deepseek': 'DSH_HOME', 'hermes': 'HERMES_HOME', 'mistral-vibe': 'VIBE_HOME'}
    if host in overrides and os.environ.get(overrides[host]):
        return Path(os.environ[overrides[host]]).expanduser().resolve()
    relative = HOSTS[host]
    if relative.startswith('.config/') and os.environ.get('XDG_CONFIG_HOME'):
        return Path(os.environ['XDG_CONFIG_HOME']).expanduser().resolve() / relative.removeprefix('.config/')
    return home / relative


def run(*args):
    executable = shutil.which(args[0])
    if not executable:
        raise RuntimeError(f'{args[0]} CLI is required; install it first.')
    subprocess.run([executable, *args[1:]], check=True)


def digest(path):
    if path.is_symlink():
        raise RuntimeError(f'Refusing symbolic link: {path}')
    result = {}
    for file in sorted(path.rglob('*')):
        if file.is_symlink():
            raise RuntimeError(f'Refusing symbolic link: {file}')
        if file.is_file():
            result[file.relative_to(path).as_posix()] = hashlib.sha256(file.read_bytes()).hexdigest()
    return result


def install_skills(source, host, home):
    """Preflight every collision before writing; update only unchanged owned bundles."""
    base = host_base(host, home)
    record = base / '.axiom-forge-install.json'
    previous = json.loads(record.read_text(encoding='utf-8')) if record.exists() else {}
    if previous and previous.get('repository') != REPO:
        raise RuntimeError(f'Unrecognized ownership record: {record}')
    bundles = {f'skills/{p.name}': p for p in (source / 'skills').iterdir() if p.is_dir() and (p / 'SKILL.md').is_file()}
    bundles['pstack-resources'] = source / 'pstack-resources'
    old = previous.get('bundles', {})
    for relative in set(old) | set(bundles):
        if relative != 'pstack-resources' and not (relative.startswith('skills/') and len(Path(relative).parts) == 2):
            raise RuntimeError('Invalid ownership path')
        target = base / relative
        if target.parent.is_symlink() or target.is_symlink() or base.is_symlink():
            raise RuntimeError(f'Refusing linked destination: {target}')
        if target.exists() and (relative not in old or digest(target) != old[relative]):
            raise RuntimeError(f'Existing or locally modified content preserved: {target}')
    base.mkdir(parents=True, exist_ok=True)
    # Keep rollback material beside the destinations, on the same filesystem.
    with tempfile.TemporaryDirectory(prefix='.axiom-forge-stage-', dir=base) as tmp:
        stage = Path(tmp)
        hashes = {}
        for relative, origin in bundles.items():
            shutil.copytree(origin, stage / 'new' / relative)
            hashes[relative] = digest(stage / 'new' / relative)
        moved, installed = [], []
        try:
            for relative in sorted(set(old) | set(bundles)):
                target = base / relative
                if target.exists():
                    backup = stage / 'old' / relative
                    backup.parent.mkdir(parents=True, exist_ok=True)
                    target.rename(backup)
                    moved.append(relative)
                if relative in bundles:
                    target.parent.mkdir(parents=True, exist_ok=True)
                    (stage / 'new' / relative).rename(target)
                    installed.append(relative)
            data = {'repository': REPO, 'version': json.loads((source / 'plugin.json').read_text())['version'], 'bundles': hashes}
            temporary_record = stage / 'record.json'
            temporary_record.write_text(json.dumps(data, indent=2) + '\n', encoding='utf-8')
            os.replace(temporary_record, record)
        except BaseException:
            for relative in reversed(installed):
                shutil.rmtree(base / relative)
            for relative in reversed(moved):
                (stage / 'old' / relative).rename(base / relative)
            raise
    print(f'{host}: installed {len(bundles) - 1} skills. Restart/reload the host. Personal settings unchanged.')


def download_source(destination):
    request = urllib.request.Request(f'https://api.github.com/repos/{REPO}/commits/main', headers={'User-Agent': 'axiom-forge-installer'})
    with urllib.request.urlopen(request, timeout=60) as response:
        commit = json.load(response)['sha']
    with urllib.request.urlopen(f'https://codeload.github.com/{REPO}/zip/{commit}', timeout=60) as response:
        archive = zipfile.ZipFile(io.BytesIO(response.read()))
    for info in archive.infolist():
        path = destination / info.filename
        if not path.resolve().is_relative_to(destination.resolve()) or (info.external_attr >> 16) & 0o170000 == 0o120000:
            raise RuntimeError('Unsafe archive member')
    archive.extractall(destination)
    print('Source commit:', commit)
    return next(destination.iterdir())


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--host', choices=['codex', 'claude', *HOSTS, *ALIASES], default='codex')
    parser.add_argument('--list-hosts', action='store_true', help='Show supported host selectors without installing')
    parser.add_argument('--update', action='store_true')
    parser.add_argument('--source', type=Path, help='Use a local checkout (for development/offline installation)')
    args = parser.parse_args()
    if args.list_hosts:
        print('\n'.join(['codex (native plugin)', 'claude (native plugin)', *[f'{host}: ~/{path}/skills' for host, path in HOSTS.items()]]))
        return
    args.host = ALIASES.get(args.host, args.host)
    if args.host == 'codex':
        if args.update:
            run('codex', 'plugin', 'marketplace', 'upgrade', 'axiom-forge')
        else:
            run('codex', 'plugin', 'marketplace', 'add', REPO, '--ref', 'main')
        run('codex', 'plugin', 'add', 'axiom-forge@axiom-forge')
    elif args.host == 'claude':
        run('claude', 'plugin', 'marketplace', 'update' if args.update else 'add', 'axiom-forge' if args.update else REPO)
        run('claude', 'plugin', 'update' if args.update else 'install', 'axiom-forge@axiom-forge', '--scope', 'user')
    elif args.source:
        install_skills(args.source.resolve(), args.host, Path.home())
    else:
        with tempfile.TemporaryDirectory(prefix='axiom-forge-download-') as tmp:
            install_skills(download_source(Path(tmp)), args.host, Path.home())


if __name__ == '__main__':
    try:
        main()
    except (RuntimeError, subprocess.CalledProcessError) as error:
        raise SystemExit(str(error))
