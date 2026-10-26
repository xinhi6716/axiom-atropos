<p align="center"><img src="assets/icon.svg" width="112" alt="Axiom Forge" /></p>

# Axiom Forge

[English](README.md) · [繁體中文](README.zh-TW.md) · [简体中文](README.zh-CN.md)

**Evidence into insight. Insight into verified work.**

公理工坊：把科研方法與工程驗證放進同一套可攜的代理技能庫。包含 58 個 Pstack skills、五個科研 skills、RSI 思路與統一研究入口，共 65 個技能。依問題選用，不一次執行全部流程。

| 工作 | 入口 |
|---|---|
| 科研協調 | [research-suite](skills/research-suite/SKILL.md) |
| 工程執行 | [poteto-mode](skills/poteto-mode/SKILL.md) |
| 文獻與回顧 | codex-research、literature-review |
| 假說與證據 | hypothesis-generation、scientific-critical-thinking |
| 統計 | statistical-analysis |
| 迭代思路 | rsi-anything；需要實作時自行查官方文件，預設不部署模型 |

## 一行安裝與更新

需要 Python 3.10+；Codex、Claude Code 另需各自 CLI。安裝器只安裝本套件，不安裝或更新主機軟體。

Windows PowerShell（預設 Codex）：

```powershell
& ([scriptblock]::Create((irm https://raw.githubusercontent.com/xinhi6716/axiom-forge/main/install.ps1))) -Agent codex
```

macOS / Linux：

```sh
curl -fsSL https://raw.githubusercontent.com/xinhi6716/axiom-forge/main/install.sh | sh -s -- --host codex
```

更新使用同一行，Windows 加 `-Update`；macOS/Linux 加 `--update`。將 `codex` 換成 `claude`、`deepseek`、`cursor`、`gemini`、`copilot`、`opencode`、`cline` 或 `hermes` 即可指定主機。

安裝器不修改 AGENTS.md、模型、API key 或一般個人設定。Codex/Claude 透過官方 CLI 登記插件；其餘主機只寫 skills、共用資源與本套件的版本／檔案雜湊紀錄。同名既有 skill 會使安裝停止；更新遇到本機改動也會停止，不覆蓋。CLI 管理的插件快取由原生管理器更新，請在 repo 維護修改。

相容方式、範圍與官方來源見 [主機支援表](HOST-SUPPORT.md)。

## Codex 原生指令

加入公開 Git marketplace 後安裝：

```sh
codex plugin marketplace add xinhi6716/axiom-forge --ref main
codex plugin add axiom-forge@axiom-forge
```

更新時先刷新來源，再安裝更新版本：

```sh
codex plugin marketplace upgrade axiom-forge
codex plugin add axiom-forge@axiom-forge
```

需要固定版本時，在 marketplace add 的 `--ref` 使用已發布 tag，例如 `v1.0.0`；固定 tag 不會跟隨 main。安裝後若技能清單仍舊，重啟主機。

## Claude Code 與其他代理

Claude Code 的插件格式與 marketplace 另附在 `.claude-plugin/`：

```text
/plugin marketplace add xinhi6716/axiom-forge
/plugin install axiom-forge@axiom-forge
```

其他支援 SKILL.md 的代理可 clone 本 repo，載入 `skills/`；保留相鄰的 `pstack-resources/`，避免複製單一技能造成連結失效。先讀 [主機對應](pstack-resources/HOSTS.md)。格式相容不代表每個主機都具備全部工具；各環境需驗證實際使用的功能。

## 維護與發布

**此 repo 是唯一主來源。** 直接修改 repo 的 skills 與相容層，不從個人技能目錄重建，也不修改安裝快取。上游版本與來源見 [sources.json](sources.json)，更新程序見 [UPDATING.md](UPDATING.md)。

```sh
python -m pip install -r requirements-dev.txt
python scripts/sync_metadata.py
python scripts/validate.py
python scripts/test_install.py
python scripts/package.py
```

`plugin.json` 是品牌、版本與 Codex 顯示資訊的主來源；sync_metadata 生成相容 manifest 與 marketplace。package 只把 repo 內容打成 ZIP，輸出到 dist。CI 檢查 metadata、所有技能與連結，拒絕個人 AGENTS.md、憑證檔及模型／環境。

插件不含模型、Python 環境、第三方 MCP 或憑證，也不會自動安裝它們。個人的語言、全域預設與 AGENTS.md 留在各自主機。Pstack 的自動化模板只是參考，不會因安裝而建立排程。

## 來源

採用 **MIT**，見 [LICENSE](LICENSE)。保留上游作者著作權與 MIT 聲明。三份 README 提供不同語言的安裝與使用說明；技能正文保留上游原文，避免維護三份不同步的流程。

Pstack by Lauren Tan、codex-research by LIU-31415、四個科研 skills by K-Dense。這是個人整合發行，並非上游作者的官方聯合產品。完整授權見 [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) 與 [LICENSE](LICENSE)。

## 更多 Harness

目錄式安裝另涵蓋 Windsurf、Roo Code、Kilo Code、Antigravity、Amp、Goose、Kimi、Qwen Code、Pi、OpenClaw、Continue、Factory Droid、Kiro、Trae、Qoder、OpenHands、Mistral Vibe、Augment、Junie、Crush、Warp、Zed、MiniMax Code、CodeBuddy、Devin for Terminal 與 Deep Agents。使用同一安裝／更新指令選擇下表參數；這是檔案部署相容，不代表已逐一驗證原生插件或模型會話。

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

路徑依據：[主機文件](HOST-SUPPORT.md) 與 [Vercel skills 安裝器登錄表](https://github.com/vercel-labs/skills/blob/main/src/agents.ts)。`universal` 寫入共用的 `~/.agents/skills`，可能被多個主機讀取；同名衝突會停止。安裝器不會自動對全部軟體進行安裝。
