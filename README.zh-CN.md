<p align="center">
  <img src="assets/icon.svg" width="88" alt="Axiom Forge" />
</p>
<h1 align="center">Axiom Forge</h1>
<p align="center"><strong>科研推理，工程验证。</strong></p>
<p align="center">
  <a href="README.md">English</a> · <a href="README.zh-TW.md">繁體中文</a> · <a href="README.zh-CN.md">简体中文</a>
</p>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-FFAD47.svg?style=flat-square" alt="License: MIT" /></a>
</p>

---

为 **Codex · Claude Code · DeepSeek Harness · Antigravity · Pi** 整合的科研与工程工具箱。

**65 个 skills** — Pstack 工作流、五个科研技能、统一研究入口，以及 RSI 推理方法。

## 安装

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

已有同名 skill 或本地修改时会停止，不直接覆盖。个人指令、模型设置与 API key 保持原样。

## 从这里开始

- **科研 → [research-suite](skills/research-suite/SKILL.md)** — 界定问题、检索文献、评估证据、设计假说与分析数据。
- **工程 → [poteto-mode](skills/poteto-mode/SKILL.md)** — 通过 Pstack 规划、实现、审查与验证成果。

> 使用 Axiom Forge 研究这个问题，从 research-suite 开始，选用问题需要的技能。

按任务选用技能。RSI 提供迭代研究思路；安装不会部署模型或运行环境。

## 仓库导览

```text
skills/             65 个可重用工作流
pstack-resources/   Pstack 共享参考与宿主映射
docs/               兼容性与维护文档
scripts/            安装、验证与打包工具
licenses/           上游版权与 MIT 声明
```

[兼容性与验证](docs/compatibility.md) · [维护流程](docs/maintaining.md) · [版本下载](https://github.com/xinhi6716/axiom-forge/releases)

---

**[MIT 协议](LICENSE)。** 整合 Lauren Tan 的 Pstack、LIU-31415 的 codex-research 与 K-Dense 科研技能。本项目为独立整合发行；详见[来源与协议](licenses/README.md)。
