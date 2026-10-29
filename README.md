<p align="center"><picture>
  <source media="(prefers-color-scheme: dark) and (max-width: 600px)" srcset="assets/readme-cover-dark-mobile.svg" />
  <source media="(max-width: 600px)" srcset="assets/readme-cover-mobile.svg" />
  <source media="(prefers-color-scheme: dark)" srcset="assets/readme-cover-dark.svg" />
  <img src="assets/readme-cover.svg" width="1200" alt="Axiom Atropos. Research and engineering skills in the Moirai series." />
</picture></p>

<h1 align="center">Axiom Atropos</h1>

<p align="center"><a href="README.md">English</a> · <a href="README.zh-TW.md">繁體中文</a> · <a href="README.zh-CN.md">简体中文</a></p>

<p align="center"><a href="#install">Install</a> · <a href="#begin">Skills</a> · <a href="#boundaries">Status</a></p>

**66 research and engineering workflows** for your agent. Choose a skill, define its checks and review the evidence.


| Research a question | Change a codebase | Inspect compatibility |
| --- | --- | --- |
| [Research suite](skills/research-suite/SKILL.md) | [Poteto mode](skills/poteto-mode/SKILL.md) | [Host support](docs/compatibility.md) |

<a id="install"></a>
<a id="install-for-your-host"></a>

## Installation

Requires **Python 3.10+**. Codex and Claude Code also need their own CLI.

**Windows · PowerShell**

```powershell
& ([scriptblock]::Create((irm https://raw.githubusercontent.com/xinhi6716/axiom-forge/main/install.ps1))) -Agent codex
```

**macOS / Linux**

```sh
curl -fsSL https://raw.githubusercontent.com/xinhi6716/axiom-forge/main/install.sh | sh -s -- --host codex
```

The install commands and plugin IDs still use **`axiom-forge`** for compatibility. An installed host may show **Axiom Forge**; no reinstall or cache migration is needed for the name change. [Naming details](docs/naming-transition.md).

These commands download and execute an installer from `main`. Inspect the [shell](install.sh), [PowerShell](install.ps1) and [Python adapter](scripts/install.py) before running them.

<details>
<summary><strong>Choose a host or update</strong></summary>

Replace `codex` with your host selector:

| Host | Selector | Integration |
|---|---|---|
| Codex | `codex` | Native plugin |
| Claude Code | `claude` | Native plugin |
| DeepSeek Harness | `deepseek` | Skills directory |
| Antigravity | `antigravity` | Skills directory |
| Pi | `pi` | Skills directory |

To update, repeat the command with `-Update` on Windows or `--update` on macOS/Linux. Restart or reload the host afterward. [Native plugin commands and compatibility](docs/compatibility.md).

</details>

<a id="begin"></a>
<a id="what-are-you-working-on"></a>

## Choose a workflow

**Research:** [research-suite](skills/research-suite/SKILL.md) routes a question to literature search, hypothesis design, statistical analysis and evidence critique.

> Use research-suite to investigate this question. Choose the skills it needs and state what evidence would change the conclusion.

**Engineering:** [poteto-mode](skills/poteto-mode/SKILL.md) brings Pstack planning, implementation, review and verification to a code change.

> Use poteto-mode to investigate this bug. Reproduce the failure and explain how you will check the fix.

Load the skills you need, rather than all 66. Individual research skills can also be used directly; not every task needs a coordinator.

<a id="evidence"></a>
<details>
<summary><strong>Workflow and advanced tools</strong></summary>

<p align="center"><picture>
  <source media="(prefers-color-scheme: dark) and (max-width: 600px)" srcset="assets/atropos-process-dark-mobile.svg" />
  <source media="(max-width: 600px)" srcset="assets/atropos-process-mobile.svg" />
  <source media="(prefers-color-scheme: dark)" srcset="assets/atropos-process-dark.svg" />
  <img src="assets/atropos-process.svg" width="1200" alt="Suggested workflow: choose the relevant skill, work with explicit checks, inspect the evidence, review the result." />
</picture></p>

The diagram is a usage guide, not an enforced runtime guarantee. The existing [bridge](bridge/README.md) and [agents](agents/) support coordinated work in compatible hosts; the bridge needs Python 3.11+ and its backend CLIs. Consult its requirements before use.

</details>

<a id="boundaries"></a>
<a id="before-you-rely-on-it"></a>

## Status and limits

- Skills contain instructions and supporting scripts. Installation does not supply Python, third-party MCP servers, model weights or credentials. RSI is a research method, not a deployed model or self-evolution runtime.
- Host verification differs. Antigravity and Pi have installer tests, not model-session verification. See the [compatibility table](docs/compatibility.md).
- Directory installs stop on same-name conflicts or modified owned content. Native plugin managers own their caches. Personal instructions, model settings and API keys are not edited. Pi’s shared skills directory may also be discovered by other applications.

**Lachesis remains planned.** It is a separate coordinator, not a feature bundled into this pack.

<a id="inside-the-repository"></a>
<details>
<summary><strong>Repository guide</strong></summary>

```text
skills/             66 reusable workflows
pstack-resources/   Shared Pstack references and host mappings
bridge/             Existing tool bridge and tests
docs/               Compatibility and maintenance
scripts/            Installer, validation and packaging
licenses/           Upstream copyright and MIT notices
```

[Maintenance](docs/maintaining.md) · [Source pins](sources.json) · [Releases](https://github.com/xinhi6716/axiom-atropos/releases)

</details>

---

[MIT License](LICENSE) · [Credits and licenses](licenses/README.md) · [Asset notices](assets/ASSET_NOTICES.txt)

An independent integration of Lauren Tan’s Pstack, LIU-31415’s codex-research and K-Dense scientific skills. Atropos is the Moirai skill pack within Axiom.

