"""Build a validated, deterministic ZIP from this repository."""
import json
from pathlib import Path
import subprocess
import sys


def build_archive(root, output):
    """Publish a committed snapshot, never ambient files from the working directory."""
    top = subprocess.check_output(['git', 'rev-parse', '--show-toplevel'], cwd=root, text=True).strip()
    if Path(top).resolve() != root.resolve():
        raise RuntimeError('Packaging requires this repository root.')
    if subprocess.check_output(['git', 'status', '--porcelain', '--untracked-files=no'], cwd=root):
        raise RuntimeError('Commit tracked changes before packaging.')
    output.parent.mkdir(parents=True, exist_ok=True)
    subprocess.run(['git', 'archive', '--format=zip', f'--output={output.resolve()}', 'HEAD'], cwd=root, check=True)


def main():
    root = Path(__file__).resolve().parents[1]
    subprocess.run([sys.executable, str(root / 'scripts/validate.py')], check=True)
    manifest = json.loads((root / 'plugin.json').read_text(encoding='utf-8'))
    output = root / 'dist' / f"{manifest['name']}-{manifest['version']}.zip"
    build_archive(root, output)
    print(output)


if __name__ == '__main__':
    main()
