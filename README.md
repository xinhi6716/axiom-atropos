<p align="center">
  <img src="assets/icon.svg" width="88" alt="Axiom Forge" />
</p>
<h1 align="center">Axiom Forge</h1>
<p align="center"><strong>Scientific reasoning. Verified engineering.</strong></p>
<p align="center">
  <a href="README.md">English</a> · <a href="README.zh-TW.md">繁體中文</a> · <a href="README.zh-CN.md">简体中文</a>
</p>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-FFAD47.svg?style=flat-square" alt="License: MIT" /></a>
</p>

---

A research and engineering toolkit for **Codex · Claude Code · DeepSeek Harness · Antigravity · Pi**.

**65 skills** — Pstack workflows, five research skills, a research coordinator, and RSI as a reasoning framework.

## Install

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

Existing same-name skills and local edits are preserved: installation stops on conflicts. Personal instructions, model settings and API keys stay untouched.

## Start here

- **Research → [research-suite](skills/research-suite/SKILL.md)** — Frame questions, find literature, assess evidence, design hypotheses and analyze data.
- **Engineering → [poteto-mode](skills/poteto-mode/SKILL.md)** — Plan, implement, review and verify the work using Pstack.

> Use Axiom Forge to investigate this question. Start with research-suite and select the skills the problem needs.

Skills are selected as needed. RSI supplies an iterative research method; no models or runtimes are deployed by installation.

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
