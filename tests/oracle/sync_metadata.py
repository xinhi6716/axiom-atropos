"""Derive host metadata from plugin.json; --check detects drift."""
import json
from pathlib import Path
import sys

root = Path(__file__).resolve().parents[1]
manifest = json.loads((root / 'plugin.json').read_text(encoding='utf-8'))
common = {k: manifest[k] for k in ('name', 'version', 'description', 'author', 'license', 'repository', 'homepage')}
name = common['name']
outputs = {
    '.codex-plugin/plugin.json': {**common, 'skills': './skills/', **manifest['extensions']['com.openai']},
    '.claude-plugin/plugin.json': {**common, **manifest['extensions'].get('com.anthropic', {})},
    '.agents/plugins/marketplace.json': {
        'name': name, 'interface': {'displayName': 'Axiom Forge'},
        'plugins': [{'name': name, 'source': {'source': 'local', 'path': './'},
                     'policy': {'installation': 'AVAILABLE', 'authentication': 'ON_INSTALL'}, 'category': 'Productivity'}]},
    '.claude-plugin/marketplace.json': {
        'name': name, 'owner': common['author'], 'metadata': {'description': common['description']},
        'plugins': [{'name': name, 'source': './', 'description': common['description'], 'version': common['version']}]},
}
for relative, value in outputs.items():
    path = root / relative
    content = json.dumps(value, ensure_ascii=False, indent=2) + '\n'
    if '--check' in sys.argv:
        if not path.exists() or path.read_text(encoding='utf-8') != content:
            raise SystemExit('Stale metadata: ' + relative)
    else:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(content, encoding='utf-8', newline='\n')
print('Host metadata OK.')
