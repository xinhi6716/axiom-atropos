<p align="center"><picture>
  <source media="(prefers-color-scheme: dark) and (max-width: 600px)" srcset="assets/readme-cover-dark-mobile.svg" />
  <source media="(max-width: 600px)" srcset="assets/readme-cover-mobile.svg" />
  <source media="(prefers-color-scheme: dark)" srcset="assets/readme-cover-dark.svg" />
  <img src="assets/readme-cover.svg" width="1200" alt="Axiom Atropos. Research and engineering skills in the Moirai series." />
</picture></p>

<p align="center"><a href="README.md">English</a> · <a href="README.zh-TW.md">繁體中文</a> · <a href="README.zh-CN.md">简体中文</a></p>

<h1 align="center">Axiom Atropos</h1>

<p align="center"><a href="#begin">Choose the work</a> · <a href="#install">Install</a> · <a href="#evidence">Follow the evidence</a> · <a href="#boundaries">Know the boundaries</a></p>

Axiom Atropos brings **66 research and engineering skills** to the agent you already use. Start with a question or a code change, load the workflows it needs, and review the result.

**Codex · Claude Code · DeepSeek Harness · Antigravity · Pi**

<p align="center"><picture><source media="(prefers-color-scheme: dark)" srcset="assets/atropos-threshold-dark.svg" /><img src="assets/atropos-threshold.svg" width="1200" alt="" /></picture></p>

<a id="begin"></a>
<a id="what-are-you-working-on"></a>

## I · Choose the work

### A research question

Start with [research-suite](skills/research-suite/SKILL.md). It selects workflows for literature searches, hypothesis design, statistics and evidence critique.

> Use Axiom Atropos to investigate this question. Start with research-suite and select the skills the problem needs.

### A code change

Start with [poteto-mode](skills/poteto-mode/SKILL.md). It routes engineering work through Pstack planning, implementation, review and verification.

> Use poteto-mode to investigate this bug. Read the relevant code, reproduce the failure, and explain how you will check the fix.

Pick the skills the task needs. You do not need to load all 66.

<p align="center"><picture><source media="(prefers-color-scheme: dark)" srcset="assets/atropos-threshold-dark.svg" /><img src="assets/atropos-threshold.svg" width="1200" alt="" /></picture></p>

<a id="install"></a>
<a id="install-for-your-host"></a>

## II · Install

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

<p align="center"><picture><source media="(prefers-color-scheme: dark)" srcset="assets/atropos-threshold-dark.svg" /><img src="assets/atropos-threshold.svg" width="1200" alt="" /></picture></p>

<a id="evidence"></a>

## III · Follow the evidence

Choose a workflow, make its checks explicit, and keep the result open to review. The diagram describes how to use the pack, not an enforced runtime guarantee.

<p align="center"><picture>
  <source media="(prefers-color-scheme: dark) and (max-width: 600px)" srcset="assets/atropos-process-dark-mobile.svg" />
  <source media="(max-width: 600px)" srcset="assets/atropos-process-mobile.svg" />
  <source media="(prefers-color-scheme: dark)" srcset="assets/atropos-process-dark.svg" />
  <img src="assets/atropos-process.svg" width="1200" alt="Suggested workflow: choose the relevant skill, work with explicit checks, inspect the evidence, review the result." />
</picture></p>

<p align="center"><picture><source media="(prefers-color-scheme: dark)" srcset="assets/atropos-threshold-dark.svg" /><img src="assets/atropos-threshold.svg" width="1200" alt="" /></picture></p>

<a id="boundaries"></a>
<a id="before-you-rely-on-it"></a>

## IV · Know the boundaries

- **Skills are instructions and supporting scripts.** Installation does not bundle a Python environment, third-party MCP servers, model weights or credentials. RSI is included as a research method, not a deployed model or runtime.

- **Host support has different levels of verification.** Antigravity and Pi have installer tests, but no model-session verification. Files landing in the right directory does not establish scientific quality. See [the full compatibility table](docs/compatibility.md).

- **Directory installs stop on same-name conflicts or modified owned content.** Codex and Claude Code use their native plugin managers, which manage their own caches. Personal instructions, model settings and API keys are not edited. Pi uses a shared skills directory that other applications may also discover.

**Axiom Lachesis remains planned.** This pack contains its existing skills, bridge and supporting agent integrations; it does not bundle that separate coordinator.

<a id="inside-the-repository"></a>
<details>
<summary><strong>Repository guide & compatibility</strong></summary>

Axiom Atropos is the Moirai skill pack, formerly Axiom Forge. The repository is now `xinhi6716/axiom-atropos`; existing plugin IDs, host labels and install commands retain `axiom-forge` for compatibility. [Naming & compatibility](docs/naming-transition.md).

```text
skills/             66 reusable workflows
pstack-resources/   Shared Pstack references and host mappings
docs/               Compatibility and maintenance
scripts/            Installer, validation and packaging
licenses/           Upstream copyright and MIT notices
```

[Compatibility & verification](docs/compatibility.md) · [Maintenance](docs/maintaining.md) · [Releases](https://github.com/xinhi6716/axiom-atropos/releases)


</details>

---

**[MIT License](LICENSE).** Built on Pstack by Lauren Tan, codex-research by LIU-31415 and scientific skills by K-Dense. An independent integration; see [credits and licenses](licenses/README.md).
