# Axiom Atropos bridge

Overview: [English](../README.md) · [繁體中文](../README.zh-TW.md) · [简体中文](../README.zh-CN.md)

A dependency-free MCP stdio server for Claude Code to call Codex and Antigravity (`agy`) through typed tools. The Claude plugin registers it as **`axiom-bridge`**. The bridge needs **Python 3.11+** and the relevant backend CLIs; installing the skill pack does not supply them.

[Tools](#tools) · [Contracts](#contracts) · [Configuration](#configuration) · [Tests](#tests)

## Tools

- **`codex_run`** sends a brief to a `terra` worker or `sol` integrator. One long-lived `codex app-server` uses JSON-RPC over stdio; each call gets an ephemeral thread. Roles resolve to models listed by the signed-in account.
- **`gemini_preview`** uses `agy` print mode with NDJSON to turn worker reports into plain language. It runs in an empty directory unless given `cwd`. It avoids plan mode, which responds with a plan and a question.
- **`orch`** calls Pstack’s `orch.ts` through Bun for units, inbox, gates and program status. Its store lives under `CLAUDE_PLUGIN_DATA` or `AXIOM_DATA_HOME`, outside plugin updates.
- **`memory_record`, `memory_search`, `memory_get`, `memory_digest`** use SQLite FTS5 trigram support in the same data directory. They retain reusable work from verified units and a one-line-per-wish project history. Search matches stored terms, including the original wish; CJK runs are split into bigrams. This is lexical retrieval, not translation. Referenced-file hashes let results report `stale` when files change.
- **`bridge_status`** probes executable versions, Java/TLC readiness and app-server state.

Every result includes `status`. A non-`completed` result includes `error.kind`, such as `usageLimitExceeded`, `timeout`, `codex_exited` or `agy_error`. Because `agy` can exit 0 on failed turns, the bridge checks its result event rather than trusting the process exit code.

### Coordinated work

[`/god-mode <wish>`](../skills/god-mode/SKILL.md) starts the workflow from the main conversation. The [dispatcher](../agents/dispatcher.md), **`axiom-forge:dispatcher`** (Sonnet), writes unit briefs, includes relevant memory, sends work to `terra` and verifies command records. A `sol` integrator is used only when the full suite fails. The [memory keeper](../agents/memory-keeper.md), **`axiom-forge:memory-keeper`** (Haiku), records reusable work afterward.

These existing `axiom-forge` identifiers are retained for compatibility. [Naming details](../docs/naming-transition.md).

## Contracts

- A `brief` follows [GOAL, SCOPE and VERIFY](DISPATCH.md), agreed before dispatch. The worker sees the brief, not the parent conversation. Map its report to `orch inbox push` like any other worker completion.
- `cwd` must be an existing absolute directory. Give each writer its own worktree.
- `sandbox` is `read-only` or `workspace-write`; `danger-full-access` is not exposed. `approvalPolicy` is fixed to `never`. Unexpected approval requests are refused and listed in `refused_requests`.
- Codex threads are ephemeral; workers do not retain memory between calls.
- Lean mode is on by default and changes only the worker thread. It disables enabled plugins and configured MCP servers, plus the apps connector, goals, skills catalog, personality section and web search. `web_search: true` retains search; `keep_plugins` retains named plugins. The report lists disabled items, MCP servers that actually started and token `usage`.
- Available models depend on the signed-in account; `model/list` is authoritative. Provider errors such as `model_not_found` are reported even when Codex groups them under `other`.

<details>
<summary><strong>Recorded lean-mode comparison</strong></summary>

The repository records these two comparisons using `gpt-5.6-terra` at low effort. They describe those runs, not a general performance guarantee.

| Brief | Inherited config | Lean |
|---|---|---|
| One-word reply | 19,863 input tokens | 8,457 |
| Write a module and tests, then run them | 149,414 input tokens; failed after a Pstack detour and sandbox block | 29,084; passed |

Codex’s base instructions (about 3.5k tokens) remain because they describe shell and patch tools.

</details>

## Configuration

The server launches with `python` and needs Python 3.11+. It resolves `codex` and `agy` from `PATH`; override a command with a JSON array, for example `AXIOM_CODEX_CMD='["/opt/codex/bin/codex"]'`. Missing executables return `not_installed`.

### Windows PowerShell

Codex’s Windows sandbox cannot directly launch Microsoft Store (MSIX) `pwsh` or `python`; the recorded failure is `CreateProcessAsUserW failed: 5`. System32 `powershell.exe` and `cmd.exe` work. If PATH contains only Store PowerShell 7, workers cannot run VERIFY through it; inspect the dispatcher’s verification result rather than treating the worker’s completion as success.

Use PowerShell 7’s MSI build. The recorded workaround removes the Store package first because winget will not replace it with an MSI, then installs:

```powershell
winget install --id Microsoft.PowerShell --exact --installer-type wix
```

The existing record reports 10 of 10 sandboxed `python -m unittest` runs passing with Store Python invoked through MSI `pwsh`; it did not require replacing Python. Restart Claude Code after changing PATH so the app-server receives the new environment.

## Tests

```sh
python bridge/test_bridge.py
```

This checks backends and the MCP session against scripted stand-ins in [`fakes/`](fakes/). It does not establish live-model quality or universal host compatibility. See the [host compatibility table](../docs/compatibility.md).
