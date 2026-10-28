<p align="center"><picture>
  <source media="(prefers-color-scheme: dark) and (max-width: 600px)" srcset="assets/readme-cover-dark-mobile.svg" />
  <source media="(max-width: 600px)" srcset="assets/readme-cover-mobile.svg" />
  <source media="(prefers-color-scheme: dark)" srcset="assets/readme-cover-dark.svg" />
  <img src="assets/readme-cover.svg" width="1200" alt="Axiom Atropos：Moirai 系列的研究與工程技能包。" />
</picture></p>

<h1 align="center">Axiom Atropos</h1>

<p align="center"><a href="README.md">English</a> · <a href="README.zh-TW.md">繁體中文</a> · <a href="README.zh-CN.md">简体中文</a></p>

<p align="center"><a href="#install">安裝</a> · <a href="#begin">技能</a> · <a href="#boundaries">狀態</a></p>

**66 個研究與工程工作流**，用於你現有的 agent。選擇技能、定義檢查，再審閱證據。


<a id="install"></a>
<a id="install-for-your-host"></a>
<a id="選擇你的使用環境"></a>

## 安裝

需要 **Python 3.10+**；Codex 與 Claude Code 另需各自 CLI。

**Windows · PowerShell**

```powershell
& ([scriptblock]::Create((irm https://raw.githubusercontent.com/xinhi6716/axiom-forge/main/install.ps1))) -Agent codex
```

**macOS / Linux**

```sh
curl -fsSL https://raw.githubusercontent.com/xinhi6716/axiom-forge/main/install.sh | sh -s -- --host codex
```

安裝指令與插件識別碼保留 **`axiom-forge`**，以維持相容性。已安裝的主機仍可能顯示 **Axiom Forge**；名稱調整不需要重裝或搬移快取。詳見[名稱說明](docs/naming-transition.md)。

這些指令會下載並執行 `main` 上的安裝程式。執行前請閱讀 [Shell](install.sh)、[PowerShell](install.ps1) 與 [Python 轉接程式](scripts/install.py)。

<details>
<summary><strong>選擇主機與更新</strong></summary>

將 `codex` 換成你使用的主機參數：

| 主機 | 參數 | 整合方式 |
|---|---|---|
| Codex | `codex` | 原生插件 |
| Claude Code | `claude` | 原生插件 |
| DeepSeek Harness | `deepseek` | 技能目錄 |
| Antigravity | `antigravity` | 技能目錄 |
| Pi | `pi` | 技能目錄 |

更新時重跑同一指令，Windows 加 `-Update`，macOS/Linux 加 `--update`，再重啟或重新載入主機。[原生插件指令與相容性](docs/compatibility.md)。

</details>

<a id="begin"></a>
<a id="what-are-you-working-on"></a>
<a id="你現在要做什麼"></a>

## 選擇工作流

**研究：**[research-suite](skills/research-suite/SKILL.md) 按問題選用文獻檢索、假說設計、統計分析與證據評估技能。

> 使用 research-suite 研究這個問題。選用需要的技能，說明哪些證據會改變結論。

**工程：**[poteto-mode](skills/poteto-mode/SKILL.md) 以 Pstack 的規劃、實作、審查與驗證流程處理程式修改。

> 使用 poteto-mode 調查這個 bug。先重現失敗，再說明要如何驗證修正。

按需要載入技能，不必一次讀全部 66 個。個別研究技能也可直接使用，不是每個任務都需要協調器。

<a id="evidence"></a>
<details>
<summary><strong>流程與進階工具</strong></summary>

<p align="center"><picture>
  <source media="(prefers-color-scheme: dark) and (max-width: 600px)" srcset="assets/atropos-process-dark-mobile.svg" />
  <source media="(max-width: 600px)" srcset="assets/atropos-process-mobile.svg" />
  <source media="(prefers-color-scheme: dark)" srcset="assets/atropos-process-dark.svg" />
  <img src="assets/atropos-process.svg" width="1200" alt="建議流程：選擇符合任務的技能，界定範圍與交付成果，依工作流執行並保留證據，審閱結果並核對標準。" />
</picture></p>

圖中是使用指南，不是執行環境強制保證。現有 [bridge](bridge/README.md) 與 [agents](agents/) 在相容主機中支援協作，bridge 需要 Python 3.11+ 與相應後端 CLI，使用前請查看各自需求。

</details>

<a id="boundaries"></a>
<a id="before-you-rely-on-it"></a>
<a id="使用前要知道的事"></a>

## 狀態與限制

- 技能包含指引與輔助程式；安裝不提供 Python、第三方 MCP server、模型權重或憑證。RSI 是研究方法，不是已部署模型或自我進化執行環境。
- 各主機驗證程度不同。Antigravity 與 Pi 只有安裝程式測試，沒有模型對話驗證。詳見[相容性表](docs/compatibility.md)。
- 技能目錄安裝遇到同名衝突或已修改的套件內容會停止。原生插件管理器維護各自快取；不修改個人指令、模型設定或 API key。其他應用程式也可能讀取 Pi 的共用技能目錄。

**Lachesis 仍在規劃中。** 它是獨立協調器，不是本技能包已附帶的功能。

<a id="inside-the-repository"></a>
<a id="倉庫導覽"></a>
<details>
<summary><strong>倉庫導覽</strong></summary>

```text
skills/             66 reusable workflows
pstack-resources/   Shared Pstack references and host mappings
bridge/             Existing tool bridge and tests
docs/               Compatibility and maintenance
scripts/            Installer, validation and packaging
licenses/           Upstream copyright and MIT notices
```

[維護流程](docs/maintaining.md) · [來源版本](sources.json) · [版本下載](https://github.com/xinhi6716/axiom-atropos/releases)

</details>

---

[MIT 授權](LICENSE) · [來源與授權](licenses/README.md) · [素材聲明](assets/ASSET_NOTICES.txt)

本專案獨立整合 Lauren Tan 的 Pstack、LIU-31415 的 codex-research 與 K-Dense 科研技能。Atropos 是 Axiom 旗下 Moirai 系列的技能包。
