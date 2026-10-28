# 科研與 Pstack 分工

依研究問題選用技能，共用來源與紀錄；不因安裝套件而執行全部流程。

| 需求 | 主責 |
|---|---|
| 查找原文、理解領域與綜合證據 | [codex-research](../../codex-research/SKILL.md) |
| 系統性／範疇回顧與搜尋篩選紀錄 | [literature-review](../../literature-review/SKILL.md) |
| 假說、替代解釋與區辨實驗 | [hypothesis-generation](../../hypothesis-generation/SKILL.md) |
| 統計方法、效果量與不確定性 | [statistical-analysis](../../statistical-analysis/SKILL.md) |
| 科研有效性、混淆與證據強度 | [scientific-critical-thinking](../../scientific-critical-thinking/SKILL.md) |
| 多輪研究的候選、預測、回饋與更新思路 | [rsi-anything](../../rsi-anything/SKILL.md) |

Pstack 負責任務需要的工程實作與驗證；科研 skills 負責研究方法與證據判斷。工程測試或代理共識不能替代科學驗證。資料隔離、驗收與使用者偏好依 AGENTS.md，不在此另立門檻。

## 工具

插件不附 Python 環境、模型、憑證或第三方 MCP。依所選 skill 檢查當前工具與依賴，優先重用可用環境。Windows 與 POSIX 命令須按主機調整。

文獻工具可使用現有搜尋、公開資料庫與合法全文來源。literature-review 的 search_databases.py 僅整理已取得的紀錄；DOI metadata 查核不能代替原文核對。

## 有程式實作時：執行邊界與成本紀錄

沿用上述環境與授權限制，把 [真實產物驗證](../../principle-prove-it-works/SKILL.md) 與 [行為測試](../../principle-test-behavior-not-implementation/SKILL.md) 落在研究程式的實際邊界。

- 不要只驗 mock 或 import。以小型合成資料，從正式 entrypoint 經相同初始化順序與 guard，使用已安裝的真實依賴完成最小運算及輸出。若依賴探索需要被 guard 禁止的 subprocess，保留可定位的失敗，不移除 guard 或用 mock 成功替代；初始化邊界的變更另行審核。
- 彙總成本時，依既有 contract／manifest 的明確清單或類型欄位選取成本紀錄，不只靠檔名前綴或 glob。對納入的紀錄核對類型、run／phase 身分、完成狀態與數值；重複、缺漏或無效紀錄不能靜默略過後宣稱完整總成本。沿用既有 schema，不另建紀錄系統。

在既有測試工具中加入適用的合成案例，預期值不要由受測程式自行產生。

| 案例 | 可觀察的驗收結果 |
|---|---|
| 真實已安裝依賴、合成輸入、正式 guard | 完成最小運算並產生可核對輸出；缺依賴則明列未驗證 |
| 依賴探索觸及禁止的 subprocess | 明確失敗且 guard 保持有效，不產生完成標記；保留真實失敗或部分紀錄 |
| 兩筆預期成本為 1.25 與 2.75 秒，另有同前綴非成本 receipt | 只計兩筆成本，總和為 4.0 秒 |
| 預期成本重複、缺漏、run 不符、未完成或數值無效 | 拒絕完整加總，不把部分證據標成完成 |
