# Axiom bridge

[Axiom Atropos](../docs/naming-transition.md) · [English overview](../README.md) · [繁體中文總覽](../README.zh-TW.md) · [简体中文概览](../README.zh-CN.md)

A dependency-free MCP stdio server that lets Claude Code drive Codex and Antigravity (agy) through typed tools instead of ad-hoc shell calls. The Claude plugin manifest registers it as `axiom-bridge`.

| Tool | Backend | Use |
| --- | --- | --- |
| `codex_run` | one long-lived `codex app-server` (JSON-RPC over stdio), one ephemeral thread per call | Send a brief to a `terra` worker or a `sol` integrator; the role resolves to a model the signed-in account lists |
| `gemini_preview` | `agy` print mode, NDJSON in and out | Rewrite worker reports into plain language: what was done, how it turned out, what the user must decide. Runs in an empty directory unless given `cwd`; plan mode is avoided because it answers with a plan document and a question |
| `orch` | Pstack's `orch.ts` under Bun | Units, inbox, gates and status for one program; the store lives under `CLAUDE_PLUGIN_DATA` (or `AXIOM_DATA_HOME`) so it survives plugin updates |
| `memory_record`, `memory_search`, `memory_get`, `memory_digest` | SQLite FTS5 (trigram) in the same data directory | Long-run memory: verified units distilled for later briefs, and a one-line-per-wish history per project. Entries index the wish verbatim, so English entries are found by Chinese queries; CJK query runs are split into bigrams. Each entry hashes its files and reports `stale` once they change |
| `bridge_status` | `--version` probes | Resolved Codex, agy and Bun executables, Java/TLC readiness, and whether the app-server is up |

[`/god-mode <wish>`](../skills/god-mode/SKILL.md) runs the whole pipeline from the main conversation. The [dispatcher agent](../agents/dispatcher.md) (`axiom-forge:dispatcher`, Sonnet) splits the wish into units, pastes relevant memory into each brief, dispatches terra workers, verifies from the command record, and only calls a sol integrator when the full suite fails. The [memory keeper](../agents/memory-keeper.md) (`axiom-forge:memory-keeper`, Haiku) then records what later units can reuse.

On Windows, the Codex sandbox cannot start Microsoft Store (MSIX) apps: launching the Store builds of `pwsh` or `python` directly fails with `CreateProcessAsUserW failed: 5` every time, while System32 `powershell.exe` and `cmd.exe` work. Codex runs worker commands through `pwsh` when one is on PATH, so a machine whose only PowerShell 7 is the Store build leaves workers unable to run VERIFY; the dispatcher then reruns it, so results stay trustworthy.

The fix is PowerShell 7 from its MSI. winget installs the MSIX build by default and refuses to replace a Store install with an MSI, so remove the Store package first, then:

```powershell
winget install --id Microsoft.PowerShell --exact --installer-type wix
```

With the MSI `pwsh`, a Store-installed Python works when invoked through the shell (10 of 10 sandboxed `python -m unittest` runs passed), so Python itself does not need replacing. Restart Claude Code afterwards so the bridge's Codex app-server sees the new PATH.

Every result carries `status`; anything other than `completed` carries `error.kind` (for example `usageLimitExceeded`, `timeout`, `codex_exited`, `agy_error`). agy exits 0 on failed turns, so the bridge reads the result event rather than the exit code.

## Contracts

- `brief` follows the [dispatch policy](DISPATCH.md): GOAL, SCOPE and VERIFY, decided before dispatch. The worker sees nothing else. Map a returned report onto `orch inbox push` the same way as any other worker completion.
- `cwd` must be an existing absolute directory; give each writer its own worktree.
- `sandbox` is `read-only` or `workspace-write`. The bridge never exposes `danger-full-access`, and `approvalPolicy` is fixed to `never`; stray approval requests are refused and listed in `refused_requests`.
- Codex threads are ephemeral, so workers keep no memory between calls.
- `lean` (default on) changes that thread only: it asks the app-server which plugins and configured MCP servers are enabled and turns them all off, along with the apps connector, goals, the skills catalog, the personality section and the web search tool (`web_search: true` keeps it). `keep_plugins` exempts named plugins. The report lists what was disabled, which MCP servers actually started, and the thread's token `usage`.

  Measured with `gpt-5.6-terra` at low effort:

  | Brief | Inherited config | Lean |
  | --- | --- | --- |
  | One-word reply | 19,863 input tokens | 8,457 |
  | Write a module and its tests, then run them | 149,414 input tokens, FAIL (detoured into Pstack skills, then sandbox-blocked) | 29,084, PASS |

  Codex's base instructions (~3.5k tokens) stay in place because they teach the shell and patch tools.
- Upstream API rejections that Codex labels `other` are reported by their provider code, such as `model_not_found`. Available models depend on the signed-in account; `model/list` is the source of truth.

## Configuration

The bridge resolves `codex` and `agy` from `PATH`. Override either with a JSON array, e.g. `AXIOM_CODEX_CMD='["/opt/codex/bin/codex"]'`. A missing executable fails the call with `not_installed`. The server is launched with `python`; Python 3.11 or newer is required.

## Tests

`python bridge/test_bridge.py` runs the backends and the MCP session against scripted stand-ins in `fakes/`.
