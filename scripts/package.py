"""Build a validated, deterministic ZIP from this repository."""
import json
from pathlib import Path
import subprocess
import sys
import zipfile

sys.dont_write_bytecode = True
from validate import content_files

root = Path(__file__).resolve().parents[1]
subprocess.run([sys.executable, str(root / 'scripts/validate.py')], check=True)
manifest = json.loads((root / 'plugin.json').read_text(encoding='utf-8'))
output = root / 'dist' / f"{manifest['name']}-{manifest['version']}.zip"
output.parent.mkdir(exist_ok=True)
with zipfile.ZipFile(output, 'w', compression=zipfile.ZIP_DEFLATED) as archive:
    for path in content_files():
        info = zipfile.ZipInfo(path.relative_to(root).as_posix(), (2026, 1, 1, 0, 0, 0))
        info.compress_type = zipfile.ZIP_DEFLATED
        info.external_attr = (0o100755 if path.suffix == '.sh' else 0o100644) << 16
        archive.writestr(info, path.read_bytes())
print(output)
