#!/bin/sh
set -eu
command -v python3 >/dev/null 2>&1 || { echo 'Python 3.10+ is required.' >&2; exit 1; }
installer_file="$(mktemp)"
trap 'rm -f "$installer_file"' EXIT HUP INT TERM
curl -fsSL https://raw.githubusercontent.com/xinhi6716/axiom-forge/main/scripts/install.py -o "$installer_file"
python3 "$installer_file" "$@"
