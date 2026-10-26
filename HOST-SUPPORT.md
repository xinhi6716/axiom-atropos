# 主機支援

安裝與更新入口統一為 install.ps1 / install.sh；來源是本 repo 的 main。以下支援的是 Axiom Forge，不會升級主機本身，也不改模型或個人指令。

| `--host` / `-Agent` | 安裝方式 | 官方格式依據 |
|---|---|---|
| codex | 原生 Git marketplace + 插件 | [Codex plugins](https://developers.openai.com/plugins/build/plugins) |
| claude | 原生 marketplace + user scope 插件 | [Claude Code plugins](https://code.claude.com/docs/en/plugins-reference) |
| deepseek | `~/.dsh/skills/` | [DeepSeek Harness skills](https://github.com/deepseek-ai/deepseek-harness/blob/master/docs/subsystems/skills.md) |
| cursor | `~/.cursor/skills/` | [Cursor skills](https://cursor.com/help/customization/skills) |
| gemini | `~/.gemini/skills/` | [Gemini CLI skills](https://geminicli.com/docs/cli/using-agent-skills/) |
| copilot | `~/.copilot/skills/`，本機 CLI/IDE | [Copilot skills](https://docs.github.com/en/copilot/concepts/customizations/agent-skills) |
| opencode | `~/.config/opencode/skills/` | [OpenCode skills](https://docs.opencode.ai/docs/skills/) |
| cline | `~/.cline/skills/` | [Cline skills](https://github.com/cline/skills) |
| hermes | `~/.hermes/skills/` | [Hermes skills](https://hermes-agent.nousresearch.com/docs/user-guide/features/skills) |

DeepSeek Harness 以原生 skill 目錄載入指令；這不等同安裝 Cordis runtime plugin。使用 DeepSeek 模型的其他軟體，依實際主機選擇安裝目標。

目錄式安裝只對預設使用者目錄生效。自訂 DSH_HOME、Hermes profile 或遠端／容器主機需在對應環境安裝；本機安裝不代表雲端代理已同步。若主機需要啟用 skills、信任來源或授予工具權限，沿用其原生介面處理，安裝器不改這些偏好。

驗證分層：安裝器測試涵蓋所有目錄式目標、重複執行、版本更新、保護既有 skills、本機修改與失敗回復；原生插件另以主機 CLI 驗證。這些不代表所有主機、模型與 65 個工作流都已跑過完整科研任務。各主機只使用實際具備的工具，見 [HOSTS.md](pstack-resources/HOSTS.md)。
