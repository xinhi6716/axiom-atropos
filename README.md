<p align="center">
  <img src="assets/readme-cover.svg" width="960" alt="Axiom Forge — research and engineering skills" />
</p>

# Axiom Forge

**Research and engineering skills for the agent you already use.**

<p align="center"><a href="README.md">English</a> · <a href="README.zh-TW.md">繁體中文</a> · <a href="README.zh-CN.md">简体中文</a></p>

Axiom Forge brings **65 skills** into **Codex · Claude Code · DeepSeek Harness · Antigravity · Pi**. Use them to work through a research question, review evidence, or plan and check a code change.

## What are you working on?

### A research question

Start with [research-suite](skills/research-suite/SKILL.md). It selects workflows for literature searches, hypothesis design, statistics and evidence critique.

> Use Axiom Forge to investigate this question. Start with research-suite and select the skills the problem needs.

### A code change

Start with [poteto-mode](skills/poteto-mode/SKILL.md). It routes engineering work through Pstack planning, implementation, review and verification.

> Use poteto-mode to investigate this bug. Read the relevant code, reproduce the failure, and explain how you will check the fix.

Pick the skills the task needs. You do not need to load all 65.

## Install for your host

Requires **Python 3.10+**. Codex and Claude Code also need their own CLI.

**Windows · PowerShell**

```powershell
& ([scriptblock]::Create((irm https://raw.githubusercontent.com/xinhi6716/axiom-forge/main/install.ps1))) -Agent codex
```

**macOS / Linux**

```sh
curl -fsSL https://raw.githubusercontent.com/xinhi6716/axiom-forge/main/install.sh | sh -s -- --host codex
```

Choose your host by replacing `codex`:

| Host | Selector | Integration |
|---|---|---|
| Codex | `codex` | Native plugin |
| Claude Code | `claude` | Native plugin |
| DeepSeek Harness | `deepseek` | Skills |
| Antigravity | `antigravity` | Skills |
| Pi | `pi` | Skills |

**Update:** repeat the same command with `-Update` on Windows or `--update` on macOS/Linux. Restart or reload the host afterward.

These commands download and run an installer from `main`. Read [the shell installer](install.sh), [the PowerShell installer](install.ps1) and [the Python adapter](scripts/install.py) before running them.

## Before you rely on it

- **Skills are instructions and supporting scripts.** Installation does not bundle a Python environment, third-party MCP servers, model weights or credentials. RSI is included as a research method, not a deployed model or runtime.

- **Host support has different levels of verification.** Antigravity and Pi have installer tests, but no model-session verification. Files landing in the right directory does not establish scientific quality. See [the full compatibility table](docs/compatibility.md).

- **Directory installs stop on same-name conflicts or modified owned content.** Codex and Claude Code use their native plugin managers, which manage their own caches. Personal instructions, model settings and API keys are not edited. Pi uses a shared skills directory that other applications may also discover.

## Inside the repository

```text
skills/             65 reusable workflows
pstack-resources/   Shared Pstack references and host mappings
docs/               Compatibility and maintenance
scripts/            Installer, validation and packaging
licenses/           Upstream copyright and MIT notices
```

[Compatibility & verification](docs/compatibility.md) · [Maintenance](docs/maintaining.md) · [Releases](https://github.com/xinhi6716/axiom-forge/releases)

---

**[MIT License](LICENSE).** Built on Pstack by Lauren Tan, codex-research by LIU-31415 and scientific skills by K-Dense. An independent integration; see [credits and licenses](licenses/README.md).

