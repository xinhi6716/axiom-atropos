"""Validate portable plugin contents before packaging or publication."""
import json
from pathlib import Path
import re
import subprocess
import sys
import yaml

ROOT = Path(__file__).resolve().parents[1]


def content_files():
    return sorted(p for p in ROOT.rglob('*') if p.is_file() and not {'.git', 'dist', '__pycache__'}.intersection(p.relative_to(ROOT).parts))


def main():
    subprocess.run([sys.executable, str(ROOT / 'scripts/sync_metadata.py'), '--check'], check=True)
    manifest = json.loads((ROOT / 'plugin.json').read_text(encoding='utf-8'))
    assert manifest['name'] == 'axiom-forge'
    sources = json.loads((ROOT / 'sources.json').read_text(encoding='utf-8'))
    skills = sorted((ROOT / 'skills').glob('*/SKILL.md'))
    errors = []
    if len(skills) != sources['skill_count']:
        errors.append('Skill count differs from sources.json')
    for skill in skills:
        try:
            meta = yaml.safe_load(skill.read_text(encoding='utf-8').split('---', 2)[1])
            assert meta['name'] == skill.parent.name and meta['description']
        except (IndexError, KeyError, TypeError, AssertionError, yaml.YAMLError):
            errors.append(f'{skill.relative_to(ROOT)}: invalid skill metadata')
    for path in content_files():
        rel = path.relative_to(ROOT).as_posix()
        if path.name.lower() in {'agents.md', 'agents.override.md', '.env', 'credentials.json', 'auth.json', 'id_rsa', 'id_ed25519'} or path.name.startswith('.env.') and path.name != '.env.example':
            errors.append(f'{rel}: private configuration file')
        if path.suffix.lower() in {'.safetensors', '.pth', '.pt', '.onnx', '.pyc', '.pem', '.key'} or {'.venv', 'venv', 'node_modules'}.intersection(path.parts):
            errors.append(f'{rel}: runtime, model or credential material')
        try:
            content = path.read_text(encoding='utf-8')
        except UnicodeDecodeError:
            continue
        if re.search(r'[A-Za-z]:[/\\]Users[/\\](?!<|\{|YOUR_|username)[\w.-]+', content, re.I):
            errors.append(f'{rel}: absolute user home path')
        if re.search(r'-----BEGIN (?:RSA |EC |OPENSSH )?PRIVATE KEY-----|gh[pousr]_[A-Za-z0-9]{30,}|github_pat_[A-Za-z0-9_]{40,}', content):
            errors.append(f'{rel}: credential-like content')
        if path.suffix != '.md':
            continue
        prose = re.sub(r'```.*?```', '', content, flags=re.S)
        for target in re.findall(r'\]\(([^)]+)\)', prose):
            target = target.split('#', 1)[0]
            if target == 'url' and rel == 'skills/why/references/synthesizer-prompt.md':
                continue
            if not target or re.match(r'[a-zA-Z]+:|/', target):
                continue
            if not (path.parent / target).exists():
                errors.append(f'{rel}: missing link {target}')
    for key in ('logo', 'logoDark', 'composerIcon'):
        if not (ROOT / manifest['extensions']['com.openai']['interface'][key]).is_file():
            errors.append(f'Missing icon: {key}')
    print(json.dumps({'skills': len(skills), 'errors': errors}, ensure_ascii=False, indent=2))
    return bool(errors)


if __name__ == '__main__':
    sys.exit(main())
