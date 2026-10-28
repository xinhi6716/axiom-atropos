<p align="center"><picture>
  <source media="(prefers-color-scheme: dark) and (max-width: 600px)" srcset="assets/readme-cover-dark-mobile.svg" />
  <source media="(max-width: 600px)" srcset="assets/readme-cover-mobile.svg" />
  <source media="(prefers-color-scheme: dark)" srcset="assets/readme-cover-dark.svg" />
  <img src="assets/readme-cover.svg" width="1200" alt="Axiom Atropos：Moirai 系列的研究与工程技能包。" />
</picture></p>

<h1 align="center">Axiom Atropos</h1>

<p align="center"><a href="README.md">English</a> · <a href="README.zh-TW.md">繁體中文</a> · <a href="README.zh-CN.md">简体中文</a></p>

<p align="center"><a href="#install">安装</a> · <a href="#begin">技能</a> · <a href="#boundaries">状态</a></p>

**66 个研究与工程工作流**，用于你现有的 agent。选择技能、定义检查，再审阅证据。


<a id="install"></a>
<a id="install-for-your-host"></a>
<a id="选择你的使用环境"></a>

## 安装

需要 **Python 3.10+**；Codex 与 Claude Code 另需各自 CLI。

**Windows · PowerShell**

```powershell
& ([scriptblock]::Create((irm https://raw.githubusercontent.com/xinhi6716/axiom-forge/main/install.ps1))) -Agent codex
```

**macOS / Linux**

```sh
curl -fsSL https://raw.githubusercontent.com/xinhi6716/axiom-forge/main/install.sh | sh -s -- --host codex
```

安装命令与插件标识保留 **`axiom-forge`**，以维持兼容性。已安装的主机仍可能显示 **Axiom Forge**；名称调整不需要重装或迁移缓存。详见[名称说明](docs/naming-transition.md)。

这些命令会下载并执行 `main` 上的安装程序。执行前请阅读 [Shell](install.sh)、[PowerShell](install.ps1) 与 [Python 适配器](scripts/install.py)。

<details>
<summary><strong>选择主机与更新</strong></summary>

将 `codex` 换成你使用的主机参数：

| 主机 | 参数 | 集成方式 |
|---|---|---|
| Codex | `codex` | 原生插件 |
| Claude Code | `claude` | 原生插件 |
| DeepSeek Harness | `deepseek` | 技能目录 |
| Antigravity | `antigravity` | 技能目录 |
| Pi | `pi` | 技能目录 |

更新时重新运行同一命令，Windows 加 `-Update`，macOS/Linux 加 `--update`，再重启或重新加载主机。[原生插件命令与兼容性](docs/compatibility.md)。

</details>

<a id="begin"></a>
<a id="what-are-you-working-on"></a>
<a id="你现在要做什么"></a>

## 选择工作流

**研究：**[research-suite](skills/research-suite/SKILL.md) 按问题选用文献检索、假说设计、统计分析与证据评估技能。

> 使用 research-suite 研究这个问题。选用需要的技能，说明哪些证据会改变结论。

**工程：**[poteto-mode](skills/poteto-mode/SKILL.md) 以 Pstack 的规划、实现、审查与验证流程处理代码修改。

> 使用 poteto-mode 调查这个 bug。先复现失败，再说明要如何验证修复。

按需要加载技能，不必一次读全部 66 个。个别研究技能也可直接使用，不是每个任务都需要协调器。

<a id="evidence"></a>
<details>
<summary><strong>流程与进阶工具</strong></summary>

<p align="center"><picture>
  <source media="(prefers-color-scheme: dark) and (max-width: 600px)" srcset="assets/atropos-process-dark-mobile.svg" />
  <source media="(max-width: 600px)" srcset="assets/atropos-process-mobile.svg" />
  <source media="(prefers-color-scheme: dark)" srcset="assets/atropos-process-dark.svg" />
  <img src="assets/atropos-process.svg" width="1200" alt="建议流程：选择符合任务的技能，界定范围与交付成果，依工作流执行并保留证据，审阅结果并核对标准。" />
</picture></p>

图中是使用指南，不是执行环境强制保证。现有 [bridge](bridge/README.md) 与 [agents](agents/) 在兼容主机中支持协作，bridge 需要 Python 3.11+ 与相应后端 CLI，使用前请查看各自要求。

</details>

<a id="boundaries"></a>
<a id="before-you-rely-on-it"></a>
<a id="使用前要知道的事"></a>

## 状态与限制

- 技能包含指南与辅助程序；安装不提供 Python、第三方 MCP server、模型权重或凭证。RSI 是研究方法，不是已部署模型或自我进化运行环境。
- 各主机验证程度不同。Antigravity 与 Pi 只有安装程序测试，没有模型会话验证。详见[兼容性表](docs/compatibility.md)。
- 技能目录安装遇到同名冲突或已修改的软件包内容会停止。原生插件管理器维护各自缓存；不修改个人指令、模型设置或 API key。其他应用程序也可能读取 Pi 的共享技能目录。

**Lachesis 仍在规划中。** 它是独立协调器，不是本技能包已附带的功能。

<a id="inside-the-repository"></a>
<a id="仓库导览"></a>
<details>
<summary><strong>仓库导览</strong></summary>

```text
skills/             66 reusable workflows
pstack-resources/   Shared Pstack references and host mappings
bridge/             Existing tool bridge and tests
docs/               Compatibility and maintenance
scripts/            Installer, validation and packaging
licenses/           Upstream copyright and MIT notices
```

[维护流程](docs/maintaining.md) · [来源版本](sources.json) · [版本下载](https://github.com/xinhi6716/axiom-atropos/releases)

</details>

---

[MIT 许可](LICENSE) · [来源与许可](licenses/README.md) · [素材声明](assets/ASSET_NOTICES.txt)

本项目独立集成 Lauren Tan 的 Pstack、LIU-31415 的 codex-research 与 K-Dense 科研技能。Atropos 是 Axiom 旗下 Moirai 系列的技能包。
