---
name: rsi-anything
description: >-
  Use RSI-Anything as a reasoning framework for iterative scientific research: propose alternatives, state predictions, compare external feedback, and retain supported improvements. Use for multi-round research or explicit RSI requests. No model deployment by default.
---

# RSI 科研迭代思路

本 skill 是依據 RSI-Anything 建立的方法指引，不是官方執行框架。按[科研分工](../research-suite/references/routing.md)與其他技能配合。

- 先明確研究問題、限制與判定條件，再提出候選與可區辨的預測。
- 選擇能減少關鍵未知的檢驗；區分預測、模擬與實測回饋。
- 根據外部證據檢查更新，保留有效做法，否決無支持的改動；下一輪使用已保留的版本。
- 檢查替代解釋與資料曝光。增加紀錄或改寫提示不代表模型已改善。

沿用任務既有紀錄與驗收規範，不強制新增模板或完整迴圈。預設不安裝依賴、下載模型或啟動官方 RSI 實驗。

需要理解方法或實作選項時，代理可自行查閱[方法與來源](references/method.md)及[執行路線](references/deployment.md)；實際部署依當次任務授權。
