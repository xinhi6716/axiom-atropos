# Axiom bridge

A dependency-free MCP stdio server that lets Claude Code drive Codex and Antigravity (agy) through typed tools instead of ad-hoc shell calls. The Claude plugin manifest registers it as `axiom-bridge`.

| Tool | Backend | Use |
| --- | --- | --- |
| `codex_run` | one long-lived `codex app-server` (JSON-RPC over stdio), one ephemeral thread per call | Send a full Pstack brief to a terra worker or a sol integrator |
| `gemini_preview` | `agy` print mode, NDJSON in and out, plan mode | Read-only, human-facing explanation of finished work |
| `bridge_status` | `--version` probes | Which executables were resolved and whether the app-server is up |

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
