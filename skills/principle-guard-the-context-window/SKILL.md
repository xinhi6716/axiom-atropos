---
name: principle-guard-the-context-window
description: 'Apply when context is filling up: large outputs, long files, repeated
  reads, fan-out planning. Route bulk to subagents; keep summaries in the main thread,
  not raw payloads.'
---

Host integration: first read [HOSTS.md](../../pstack-resources/HOSTS.md); it maps the upstream workflow to the current agent host.

# Guard the Context Window

**Why:** Context overflow degrades reasoning quality, creates compression artifacts, and halts progress.

**Pattern:**
- **Size phases and cap scope.** Limit files per phase, set turn budgets, account for mechanism costs.
