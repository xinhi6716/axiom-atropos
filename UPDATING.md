# Updating Axiom Forge

The repository owns skills, host adapters, scripts, manifests and upstream notices. Global AGENTS.md and installed plugin caches are not inputs.

## Upstream updates

1. Read the current commit for the relevant upstream in sources.json. Fetch its repository into a separate checkout, and compare that commit with the target commit.
2. Merge changed files into this repo, preserving host adapters, relative links, invocation policy and the RSI method-only default. Review removed files and newly required runtimes. Do not run an upstream installer over this checkout.
3. For Pstack, its skills map to skills/; agents, docs, assets and dormant automations map to pstack-resources/. Preserve the frontmatter normalization (folder-aligned name; no Cursor-only icon/color/disable-model-invocation keys), HOSTS.md entry and agents/openai.yaml policies. Maintain local tool mappings in pstack-resources/CODEX.md.
4. Update sources.json's pinned commit and skill inventory. Preserve upstream licenses. Validate metadata and links, then run tests appropriate to changed executable code. Tests that install dependencies should run in an isolated environment.

## Release

Change the version in root plugin.json, run scripts/sync_metadata.py, scripts/validate.py and scripts/package.py. Review the diff and CI, commit and push, tag v<VERSION>, and attach dist/axiom-forge-<VERSION>.zip to the GitHub release. Do not publish personal instructions, credentials, environments or model weights.

Consumers following main refresh the marketplace and install the new version; consumers pinned to a tag explicitly choose a newer tag. These operations do not synchronize changes from installed caches back to the source.
