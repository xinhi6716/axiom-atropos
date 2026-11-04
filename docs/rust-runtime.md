# Rust runtime

`atropos` is the native execution core for Axiom Atropos. Plugin identity stays `axiom-forge`, preserving existing skill references. This prerelease targets Windows x86-64 (MSVC) and Linux x86-64 (glibc, built on Ubuntu 22.04), with all 66 skills and bundled SQLite.

## Install

Download the matching archive and `.sha256` from the GitHub release, verify the checksum, extract it, and run `bin/atropos install --host codex` (Windows: `bin/atropos.exe`). Hosts: `codex`, `claude`, `deepseek`, `antigravity`, `pi`. `--list-hosts` lists aliases. Add the extracted `bin` directory to PATH for native skill helpers.

Codex/Claude use their official plugin CLI and a local marketplace at `~/.local/share/axiom-atropos/<version>`, including the runtime. `--update` selects Claude's update command. For Codex migration from an existing marketplace, it switches the named source through the official CLI. If registration fails, it restores the previous marketplace root as a local snapshot; restoring a former remote tracking source may then require adding that Git source again. Existing versioned content with different hashes is preserved. No running plugin cache is edited.

Directory hosts receive skills and `pstack-resources` in `.dsh`, `.gemini/antigravity`, or `.agents`; `DSH_HOME` overrides `.dsh`. Ownership records identify installed files. Updates refuse unowned or locally edited bundles, acquire a lock, and roll back failed file moves. If rollback fails, backups remain at the reported path. Managed paths cannot be symlinks or Windows reparse points.

`install.ps1` / `install.sh` launch the adjacent extracted binary. Source installation: `cargo build --release --locked`, then `target/release/atropos install --host <host>`. `cargo install --path . --locked` installs the embedded CLI. Building bundled SQLite needs a C compiler.

## Commands

```text
atropos --version
atropos bridge
atropos install --host pi [--update] [--source DIR]
atropos assets EMPTY_DIRECTORY
atropos metadata --source DIR [--check]
atropos validate --source DIR
atropos package --source DIR --output FILE.zip [--candidate]
atropos memory record '{"project":"example",...}'
atropos memory search '{"query":"storage","limit":5}'
atropos memory get '{"id":1}'
atropos memory digest '{"project":"example","add":"A durable finding"}'
atropos orch --project example init
atropos orch --project example unit add core --track rust
atropos orch --project example unit set core --state pending --branch feature/core
atropos orch --project example status --json
atropos check-plan PLAN.md
atropos work-log LOG.tsv PHASE DECISION WHY EVIDENCE RESULT
atropos worktree-audit
```

Use the shell's JSON quoting conventions. Storage resolves `AXIOM_DATA_HOME`, then `CLAUDE_PLUGIN_DATA`; memory/project-based orch fail if neither is set. `orch --store DIR` or `ORCH_STORE` chooses an explicit store without requiring a data-home variable, `--project` defaults to `default`. Existing SQLite entries, hashes, digests, orchestrator TSV and JSON remain readable. Native `worktree-audit` reports paths, HEAD, branches and changed files; the richer upstream shell audit remains separate.

## MCP and backends

Stdio exposes `codex_run`, `gemini_preview`, `orch`, `memory_record`, `memory_search`, `memory_get`, `memory_digest`, `bridge_status`. See `src/bridge/tools.json` for schemas. Standard output is newline-delimited JSON-RPC. Invalid requests receive protocol errors; slow tools do not block ping. Failures set `isError` with structured diagnostics.

Codex uses its app-server, ephemeral threads and approval policy `never`. Sandboxes: `read-only`, `workspace-write`. Explicit models override roles; roles resolve matching advertised families. Omitting both uses the account default. Lean mode changes only the child thread; `keep_plugins` / `web_search` opt resources back in. Timeouts interrupt turns, crashes are reported, subsequent calls reconnect.

Gemini preview uses installed `agy`, in an empty temporary directory unless `cwd` is set. No models or credentials are bundled. `AXIOM_CODEX_CMD`, `AXIOM_AGY_CMD`, `AXIOM_CLAUDE_CMD` accept JSON arrays of executable and initial arguments. `frontier set` needs Git and `gh` or `origin`.

## Migration boundary

Installation, packaging, metadata, validation, MCP, Codex/Gemini adapters, memory, orchestrator bookkeeping, plan checking and work logs run natively in Rust without launching Python, Node or Bun.

Standalone scientific helpers, PR watchers, statechart/TLC tools, shell audits and their tests remain in skill bundles with their original invocation and interpreter requirements. These optional companions may need Python, Node/Bun or Java/TLC. This release does not claim every upstream program has been ported. The old project-owned Python core is retained only in `tests/oracle/`, excluded from native releases and the `.crate`.

## Verification and publication

```sh
cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked --all-features
cargo build --locked --release --bin atropos
target/release/atropos validate
target/release/atropos package --output dist/axiom-atropos.zip
cargo package --locked
```

Formal packaging requires committed tracked changes, tracked inventory assets, matching runtime/asset versions, and identical embedded/source assets. `--candidate` allows development snapshots and marks the manifest accordingly. Archives include source commit, target and per-file SHA-256 hashes, plus an external archive checksum. The `.crate` is a release attachment, not a crates.io publication.

CI runs Windows and Linux. Native scripted backend tests cover protocol behavior, including empty-PATH execution. They do not call paid models or certify live accounts or host vendor behavior. Live use depends on the installed backend and account.
