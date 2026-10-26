<p align="center"><img src="assets/icon.svg" width="112" alt="Axiom Forge" /></p>

# Axiom Forge

[English](README.md) · [繁體中文](README.zh-TW.md) · [简体中文](README.zh-CN.md)

**Evidence into insight. Insight into verified work.**

公理工坊：把科研方法与工程验证放进同一套可移植的代理技能库。包含 58 个 Pstack skills、五个科研 skills、RSI 思路与统一研究入口，共 65 个技能。按问题选用，安装不代表同时执行全部流程。

| 工作 | 入口 |
|---|---|
| 科研协调 | [research-suite](skills/research-suite/SKILL.md) |
| 工程执行 | [poteto-mode](skills/poteto-mode/SKILL.md) |
| 文献研究 | codex-research、literature-review |
| 假说与证据 | hypothesis-generation、scientific-critical-thinking |
| 统计 | statistical-analysis |
| 迭代思路 | rsi-anything；仅提供方法，需要实现时查官方文档，默认不部署模型 |

## 一行安装与更新

需要 Python 3.10+；Codex、Claude Code 另需各自 CLI。安装器只管理 Axiom Forge，不安装或升级宿主软件。

**Windows PowerShell：**

```powershell
& ([scriptblock]::Create((irm https://raw.githubusercontent.com/xinhi6716/axiom-forge/main/install.ps1))) -Agent codex
```

**macOS / Linux：**

```sh
curl -fsSL https://raw.githubusercontent.com/xinhi6716/axiom-forge/main/install.sh | sh -s -- --host codex
```

更新使用同一行，Windows 加 `-Update`，macOS/Linux 加 `--update`。将 `codex` 换成下表中的目标。

| 软件 | 参数 | 安装方式 |
|---|---|---|
| Codex | `codex` | 原生 Git marketplace / 插件 |
| Claude Code | `claude` | 原生 marketplace / 插件，用户范围 |
| DeepSeek Harness | `deepseek` | `~/.dsh/skills/` |
| Cursor | `cursor` | `~/.cursor/skills/` |
| Gemini CLI | `gemini` | `~/.gemini/skills/` |
| GitHub Copilot | `copilot` | `~/.copilot/skills/`，本地 CLI/IDE |
| OpenCode | `opencode` | `~/.config/opencode/skills/` |
| Cline | `cline` | `~/.cline/skills/` |
| Hermes | `hermes` | `~/.hermes/skills/` |

安装器保留个人指令、模型、API key 和一般设置。Codex/Claude 通过官方 CLI 注册插件；其他目标只写 skills、相邻的共享资源和版本／文件哈希记录。发现已有同名 skill 时停止；更新遇到本地修改也停止，不覆盖。原生插件的修改请在 repo 中维护，不修改宿主管理的缓存。

目录式安装面向默认用户位置。自定义 DSH_HOME、Hermes profile、容器或远程主机需要在对应环境安装；本地安装不会自动同步云端代理。若软件要求启用 skills、信任来源或授予权限，通过其原生界面处理，安装器不修改这些偏好。

已验证 Codex 安装／更新、隔离设置目录中的 Claude Code 安装／更新，以及 GitHub 完整 65 skills 在隔离 DeepSeek 目录布局中的部署。其他适配器有文件系统测试；不代表每个软件和模型都跑过完整科研任务。官方依据与验证细节见 [主机支持表](HOST-SUPPORT.md)。

## 原生 marketplace 指令

Codex 安装：

```sh
codex plugin marketplace add xinhi6716/axiom-forge --ref main
codex plugin add axiom-forge@axiom-forge
```

Codex 更新：

```sh
codex plugin marketplace upgrade axiom-forge
codex plugin add axiom-forge@axiom-forge
```

Claude Code 会话内：

```text
/plugin marketplace add xinhi6716/axiom-forge
/plugin install axiom-forge@axiom-forge
```

Codex 的 `--ref` 可改成已发布 tag 以固定版本；固定 tag 不跟随 main。技能列表未刷新时重启或重新加载软件。其他支持 SKILL.md 的代理可加载此 repo 的 `skills/`，同时保留相邻的 `pstack-resources/`。先读 [主机映射](pstack-resources/HOSTS.md)。

## 维护

**此 repo 是唯一主来源。** 直接修改这里的 skills 与兼容层，不从个人技能目录重新构建，也不修改安装缓存。上游版本见 [sources.json](sources.json)，合并和发布流程见 [UPDATING.md](UPDATING.md)。

```sh
python -m pip install -r requirements-dev.txt
python scripts/sync_metadata.py
python scripts/validate.py
python scripts/test_install.py
python scripts/package.py
```

`plugin.json` 维护品牌与版本；sync_metadata 生成宿主 manifest 和 marketplace，ZIP 输出到 `dist/`。CI 检查元数据、技能链接、隐私排除、安装保护与打包。个人 AGENTS.md、凭证、环境和模型不会打包。自动化模板只是参考，不创建任务。RSI 作为推理方法，需要实现时再查上游文档。

三份 README 提供不同语言的安装与使用说明；技能正文保留上游原文，避免维护三份不同步的流程。

## 协议与来源

采用 **MIT**，见 [LICENSE](LICENSE)。上游作者的版权与 MIT 声明保留于 [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) 和 `licenses/`。

Pstack by Lauren Tan；codex-research by LIU-31415；四个科研 skills by K-Dense。这是独立维护的整合版本，不是上游作者的官方联合发行。

## 更多 Harness

目录式安装另覆盖 Windsurf、Roo Code、Kilo Code、Antigravity、Amp、Goose、Kimi、Qwen Code、Pi、OpenClaw、Continue、Factory Droid、Kiro、Trae、Qoder、OpenHands、Mistral Vibe、Augment、Junie、Crush、Warp、Zed、MiniMax Code、CodeBuddy、Devin for Terminal 与 Deep Agents。使用同一安装／更新命令选择下表参数；这是文件部署兼容，不代表已逐一验证原生插件或模型会话。

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

路径依据：[主机文档](HOST-SUPPORT.md) 与 [Vercel skills 安装器注册表](https://github.com/vercel-labs/skills/blob/main/src/agents.ts)。`universal` 写入共享的 `~/.agents/skills`，可能被多个软件读取；同名冲突会停止。安装器不会自动安装到全部软件。
