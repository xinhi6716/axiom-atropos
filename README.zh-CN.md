<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark) and (max-width: 600px)" srcset="assets/readme-cover-dark-mobile.svg" />
    <source media="(max-width: 600px)" srcset="assets/readme-cover-mobile.svg" />
    <source media="(prefers-color-scheme: dark)" srcset="assets/readme-cover-dark.svg" />
    <img src="assets/readme-cover.svg" width="960" alt="Axiom Atropos — research and engineering skill pack in the Axiom Moirai family" />
  </picture>
</p>

# Axiom Atropos

**把研究与工程工作交给你正在使用的 agent。**

<p align="center"><a href="README.md">English</a> · <a href="README.zh-TW.md">繁體中文</a> · <a href="README.zh-CN.md">简体中文</a></p>

Axiom Atropos 为 **Codex · Claude Code · DeepSeek Harness · Antigravity · Pi** 整合 **66 个 skills**。你可以用它拆解研究问题、核对证据，或规划与检查代码修改。

**名称过渡：** **Axiom Atropos** 是本 **Axiom Forge** 技能包选定的新名称，属于 Axiom 品牌下的 **Moirai** 系列。本次仅更新品牌与文档；仓库网址、安装命令、插件标识及宿主显示名称均保持不变。[名称与兼容性说明](docs/naming-transition.md)。

此套件包含技能与现有 agent 的辅助集成。**Axiom Lachesis 仍在规划中**；本次品牌更新不会实现或附带该独立产品。

## 你现在要做什么？

### 研究一个问题

从 [research-suite](skills/research-suite/SKILL.md) 开始，按问题选用文献检索、假说设计、统计分析与证据评估技能。

> 使用 Axiom Atropos 研究这个问题，从 research-suite 开始，选用问题需要的技能。

### 修改一段代码

从 [poteto-mode](skills/poteto-mode/SKILL.md) 开始，通过 Pstack 规划、实现、审查与验证。

> 使用 poteto-mode 调查这个 bug。先读相关代码、复现失败，再说明要如何验证修复。

需要哪个技能，再读哪个。不必一次加载全部 66 个。

## 选择你的使用环境

需要 **Python 3.10+**；Codex、Claude Code 另需各自 CLI。

**Windows · PowerShell**

```powershell
& ([scriptblock]::Create((irm https://raw.githubusercontent.com/xinhi6716/axiom-forge/main/install.ps1))) -Agent codex
```

**macOS / Linux**

```sh
curl -fsSL https://raw.githubusercontent.com/xinhi6716/axiom-forge/main/install.sh | sh -s -- --host codex
```

把 `codex` 换成你使用的宿主：

| 宿主 | 参数 | 整合方式 |
|---|---|---|
| Codex | `codex` | 原生插件 |
| Claude Code | `claude` | 原生插件 |
| DeepSeek Harness | `deepseek` | 技能目录 |
| Antigravity | `antigravity` | 技能目录 |
| Pi | `pi` | 技能目录 |

**更新：**重新运行同一行，Windows 加 `-Update`，macOS/Linux 加 `--update`。完成后重启或重新加载宿主。

上述命令会下载并运行 `main` 上的安装程序。运行前可先读 [Shell 安装程序](install.sh)、[PowerShell 安装程序](install.ps1)与 [Python 适配程序](scripts/install.py)。

## 使用前要知道的事

- **技能包含操作指引与辅助程序。** 安装包不附 Python 环境、第三方 MCP、模型权重或凭证。RSI 提供研究方法，不会替你部署模型或运行环境。

- **各环境的验证程度不同。** Antigravity 与 Pi 目前只有安装程序测试，没有模型对话验证；文件放对位置，不代表研究质量已获验证。详见[完整兼容性表](docs/compatibility.md)。

- **技能目录安装遇到同名冲突或已修改的套件内容会停止。** Codex、Claude Code 则由各自插件管理器维护缓存。安装不修改个人指令、模型设置与 API key。Pi 使用共享技能目录，其他应用也可能读到它。

## 仓库导览

```text
skills/             66 个可重用工作流
pstack-resources/   Pstack 共享参考与宿主映射
docs/               兼容性与维护文档
scripts/            安装、验证与打包工具
licenses/           上游版权与 MIT 声明
```

[兼容性与验证](docs/compatibility.md) · [维护流程](docs/maintaining.md) · [版本下载](https://github.com/xinhi6716/axiom-forge/releases)

---

**[MIT 协议](LICENSE)。** 整合 Lauren Tan 的 Pstack、LIU-31415 的 codex-research 与 K-Dense 科研技能。本项目为独立整合发行；详见[来源与协议](licenses/README.md)。

