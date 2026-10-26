# 科研與 Pstack 分工

依研究問題選用技能，共用來源與紀錄；不因安裝套件而執行全部流程。

| 需求 | 主責 |
|---|---|
| 查找原文、理解領域與綜合證據 | [codex-research](../SKILL.md) |
| 系統性／範疇回顧與搜尋篩選紀錄 | [literature-review](../../literature-review/SKILL.md) |
| 假說、替代解釋與區辨實驗 | [hypothesis-generation](../../hypothesis-generation/SKILL.md) |
| 統計方法、效果量與不確定性 | [statistical-analysis](../../statistical-analysis/SKILL.md) |
| 科研有效性、混淆與證據強度 | [scientific-critical-thinking](../../scientific-critical-thinking/SKILL.md) |
| 多輪研究的候選、預測、回饋與更新思路 | [rsi-anything](../../rsi-anything/SKILL.md) |

Pstack 負責任務需要的工程實作與驗證；科研 skills 負責研究方法與證據判斷。工程測試或代理共識不能替代科學驗證。資料隔離、驗收與使用者偏好依 AGENTS.md，不在此另立門檻。

## 工具

插件不附 Python 環境、模型、憑證或第三方 MCP。依所選 skill 檢查當前工具與依賴，優先重用可用環境。Windows 與 POSIX 命令須按主機調整。

文獻工具可使用現有搜尋、公開資料庫與合法全文來源。literature-review 的 search_databases.py 僅整理已取得的紀錄；DOI metadata 查核不能代替原文核對。
