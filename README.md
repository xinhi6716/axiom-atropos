<p align="center"><img src="assets/icon.svg" width="112" alt="Axiom Forge" /></p>

# Axiom Forge

[English](README.md) · [繁體中文](README.zh-TW.md) · [简体中文](README.zh-CN.md)

**Evidence into insight. Insight into verified work.**

A portable research and engineering toolkit: 58 Pstack skills, five scientific research skills, RSI methodological guidance and one research coordinator — 65 skills in one distribution. Select workflows relevant to the task; installing the suite does not run all of them.

| Work | Entry |
|---|---|
| Research coordination | [research-suite](skills/research-suite/SKILL.md) |
| Engineering | [poteto-mode](skills/poteto-mode/SKILL.md) |
| Literature research | codex-research, literature-review |
| Hypotheses and evidence | hypothesis-generation, scientific-critical-thinking |
| Statistics | statistical-analysis |
| Iterative reasoning | rsi-anything; methodology only, no model deployment |

## One-command installation and updates

Requires Python 3.10+. Codex and Claude Code also require their respective CLIs. The installer manages Axiom Forge, not the host application.

**Windows PowerShell:**

```powershell
& ([scriptblock]::Create((irm https://raw.githubusercontent.com/xinhi6716/axiom-forge/main/install.ps1))) -Agent codex
```

**macOS / Linux:**

```sh
curl -fsSL https://raw.githubusercontent.com/xinhi6716/axiom-forge/main/install.sh | sh -s -- --host codex
```

To update, run the same command with `-Update` on Windows or `--update` on macOS/Linux. Replace `codex` with a host from the table below.

| Host | Selector | Installation |
|---|---|---|
| Codex | `codex` | Native Git marketplace/plugin |
| Claude Code | `claude` | Native marketplace/plugin, user scope |
| DeepSeek Harness | `deepseek` | `~/.dsh/skills/` |
| Cursor | `cursor` | `~/.cursor/skills/` |
| Gemini CLI | `gemini` | `~/.gemini/skills/` |
| GitHub Copilot | `copilot` | `~/.copilot/skills/`, local CLI/IDE |
| OpenCode | `opencode` | `~/.config/opencode/skills/` |
| Cline | `cline` | `~/.cline/skills/` |
| Hermes | `hermes` | `~/.hermes/skills/` |

The installer preserves personal instructions, models, API keys and general settings. Native CLIs register the plugin; directory adapters write skills, adjacent shared resources and an ownership/hash record. Existing same-name skills stop installation. Local edits to owned files stop updates. Maintain native-plugin changes in this repo, not host-managed caches.

Directory adapters target default user locations. Custom homes, profiles, containers and remote hosts need installation in the corresponding environment. Local installation does not sync cloud agents. Enable skills or grant trust through the host's own UI when required; the installer does not change those preferences.

Codex installation/update and isolated Claude Code installation/update were verified. The complete 65-skill GitHub payload was also installed/updated in an isolated DeepSeek directory layout. Other adapters have filesystem tests; this does not claim end-to-end research execution in every host. Official format sources and verification details: [host support](HOST-SUPPORT.md).

## Native marketplace commands

Codex installation:

```sh
codex plugin marketplace add xinhi6716/axiom-forge --ref main
codex plugin add axiom-forge@axiom-forge
```

Codex update:

```sh
codex plugin marketplace upgrade axiom-forge
codex plugin add axiom-forge@axiom-forge
```

Claude Code, inside a session:

```text
/plugin marketplace add xinhi6716/axiom-forge
/plugin install axiom-forge@axiom-forge
```

Use a published tag instead of `main` in Codex's `--ref` to pin a release. Restart/reload the host if its skill catalog is stale. Other SKILL.md hosts can load this checkout's `skills/` while retaining adjacent `pstack-resources/`. Read [host mappings](pstack-resources/HOSTS.md).

## Maintenance

**This repo is the source of truth.** Edit skills and host integration here. Never rebuild from a personal skills directory or edit installation caches. [sources.json](sources.json) pins upstream versions; [UPDATING.md](UPDATING.md) describes upstream merges and releases.

```sh
python -m pip install -r requirements-dev.txt
python scripts/sync_metadata.py
python scripts/validate.py
python scripts/test_install.py
python scripts/package.py
```

`plugin.json` owns branding/version metadata. Generated host manifests and marketplaces follow it; ZIPs go to `dist/`. CI checks metadata, skill links, privacy exclusions, installation protection and packaging. Personal AGENTS.md, credentials, environments and models are excluded. Automation templates are dormant references. RSI is a reasoning framework; consult upstream documentation if implementation is needed.

The three READMEs cover installation and use. Upstream skill bodies retain their original language so workflow maintenance has one source.

## License and credits

**MIT** — see [LICENSE](LICENSE). Upstream copyright and MIT notices remain in [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) and `licenses/`.

Pstack by Lauren Tan; codex-research by LIU-31415; four scientific skills by K-Dense. This is an independently maintained integration, not an official joint release by the upstream authors.

## Extended host support

Directory adapters also cover Windsurf, Roo Code, Kilo Code, Antigravity, Amp, Goose, Kimi, Qwen Code, Pi, OpenClaw, Continue, Factory Droid, Kiro, Trae, Qoder, OpenHands, Mistral Vibe, Augment, Junie, Crush, Warp, Zed, MiniMax Code, CodeBuddy, Devin for Terminal and Deep Agents. Select a host below with the same install/update command. These are filesystem adapters, not claims of native plugin or model-session testing.

| Host | Skills directory |
|---|---|
| `deepseek` | `~/.dsh/skills/` |
| `cursor` | `~/.cursor/skills/` |
| `gemini` | `~/.gemini/skills/` |
| `copilot` | `~/.copilot/skills/` |
| `opencode` | `~/.config/opencode/skills/` |
| `cline` | `~/.cline/skills/` |
| `hermes` | `~/.hermes/skills/` |
| `windsurf` | `~/.codeium/windsurf/skills/` |
| `roo` | `~/.roo/skills/` |
| `kilo` | `~/.kilo/skills/` |
| `antigravity` | `~/.gemini/antigravity/skills/` |
| `amp` | `~/.config/agents/skills/` |
| `goose` | `~/.config/goose/skills/` |
| `kimi` | `~/.agents/skills/` |
| `qwen` | `~/.qwen/skills/` |
| `pi` | `~/.agents/skills/` |
| `openclaw` | `~/.openclaw/skills/` |
| `continue` | `~/.continue/skills/` |
| `droid` | `~/.factory/skills/` |
| `kiro` | `~/.kiro/skills/` |
| `trae` | `~/.trae/skills/` |
| `trae-cn` | `~/.trae-cn/skills/` |
| `qoder` | `~/.qoder/skills/` |
| `qoder-cn` | `~/.qoder-cn/skills/` |
| `openhands` | `~/.openhands/skills/` |
| `mistral-vibe` | `~/.vibe/skills/` |
| `augment` | `~/.augment/skills/` |
| `junie` | `~/.junie/skills/` |
| `crush` | `~/.config/crush/skills/` |
| `warp` | `~/.agents/skills/` |
| `zed` | `~/.agents/skills/` |
| `minimax` | `~/.minimax/skills/` |
| `codebuddy` | `~/.codebuddy/skills/` |
| `devin` | `~/.config/devin/skills/` |
| `deepagents` | `~/.deepagents/agent/skills/` |
| `universal` | `~/.agents/skills/` |

Path mapping sources: [host documentation](HOST-SUPPORT.md) and the [Vercel skills installer registry](https://github.com/vercel-labs/skills/blob/main/src/agents.ts). `universal` writes the shared `~/.agents/skills` location and may be discovered by multiple hosts. Conflicts stop installation. No automatic installation into every application.
