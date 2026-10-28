# Updating Axiom Atropos

[Axiom Atropos](naming-transition.md) · [English overview](../README.md) · [繁體中文總覽](../README.zh-TW.md) · [简体中文概览](../README.zh-CN.md)

The repository owns skills, host adapters, scripts, manifests and upstream notices. Global AGENTS.md and installed plugin caches are not inputs.

## Upstream updates

1. Read the current commit for the relevant upstream in sources.json. Fetch its repository into a separate checkout, and compare that commit with the target commit.
2. Merge changed files into this repo, preserving host adapters, relative links, invocation policy and the RSI method-only default. Review removed files and newly required runtimes. Do not run an upstream installer over this checkout.
3. For Pstack, its skills map to skills/; agents, docs, assets and dormant automations map to pstack-resources/. Preserve the frontmatter normalization (folder-aligned name; no Cursor-only icon/color/disable-model-invocation keys), HOSTS.md entry and agents/openai.yaml policies. Maintain local tool mappings in pstack-resources/CODEX.md.
4. Update sources.json's pinned commit and skill inventory. Preserve upstream licenses. Validate metadata and links, then run tests appropriate to changed executable code. Tests that install dependencies should run in an isolated environment.

## Release

Change the version in root plugin.json, run scripts/sync_metadata.py and scripts/validate.py, then scripts/test_install.py and scripts/test_package.py. Review and commit the changes before running scripts/package.py: packaging requires clean tracked files and archives HEAD through Git. Untracked and ignored files are never included. Push, check CI, tag v<VERSION>, and attach dist/axiom-forge-<VERSION>.zip to the GitHub release. Do not publish personal instructions, credentials, environments or model weights.

Consumers following main refresh the marketplace and install the new version; consumers pinned to a tag explicitly choose a newer tag. These operations do not synchronize changes from installed caches back to the source.

Keep README.md, README.zh-TW.md and README.zh-CN.md aligned when changing installation or support. The five supported host paths live in scripts/install.py; update its tests and the support tables together. The upstream skills-registry commit in sources.json records the directory reference, not end-to-end certification by each host vendor.

## Repository layout

Keep user-facing introductions in the three root READMEs. Detailed compatibility and release procedures belong in docs/; upstream notices belong in licenses/. Preserve skills/ and pstack-resources/ relative paths. Root host manifests are generated from plugin.json, not separately maintained.

## Responsibility boundaries

- `skills/research-suite/` owns scientific workflow selection and its routing reference. Leaf research skills may run independently; they do not require the coordinator or Pstack for every task.
- `skills/poteto-mode/` owns engineering workflows. The research coordinator selects it only for engineering work or an explicit user request. Host/user defaults still take precedence.
- `pstack-resources/HOSTS.md` and `CODEX.md` map host capabilities; they refer to the coordinator instead of duplicating scientific routing.
- `scripts/validate.py` checks repository contents. `scripts/package.py` calls that check as a command, then publishes a committed Git snapshot; it does not import validator internals to choose publication files.

1.2.0 moves the scientific routing reference out of codex-research and removes its old path after migrating callers. All 65 skills and the five supported hosts are retained. These are structural and instruction-level changes, not a measured improvement in scientific quality or model behavior.
