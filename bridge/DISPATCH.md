# Dispatch policy

For whoever writes `codex_run` briefs. Workers never see this file; they see only the brief.

A worker stays small when the brief leaves nothing to infer. In 18 runs of delete, bug-fix and rename tasks with three-field briefs, `gpt-5.6-terra` finished every task in 2–7 commands and 1–13 changed lines, with no writes outside SCOPE, no backups and no added tests, under both the stock and a trimmed system prompt. Where a brief left a choice open, runs diverged: "rename `calc` everywhere" also renamed the test method `test_calc` in 2 of 6 runs.

## The brief

```
GOAL    one observable outcome, one sentence
SCOPE   the exact paths the worker may create, change or delete
VERIFY  one command; exit code 0 means done
```

Add `CONTEXT` only when the unit depends on something outside SCOPE: paste the upstream report or retrieved snippet itself, not a pointer to it. This is Pstack's collapsed brief for a one-command unit ([orchestrate](../skills/poteto-mode/playbooks/orchestrate.md)); the full template is for coordinators.

## Before dispatch

- Decide every open choice in GOAL or SCOPE. If two reasonable workers could deliver different results, the brief is not finished.
- A field you cannot fill precisely means the unit is not ready. Split it, or ask the human; do not dispatch it.
- For deletions, SCOPE lists each path to delete.
- VERIFY runs a test file, never inline code. Put the check in a test file listed in SCOPE and make VERIFY the test runner, such as `python -m unittest test_todo`. In the first end-to-end wish, both units used `python -c "..."` checks that broke on Windows shell quoting; the workers spent 106k and 157k input tokens working around them, one wrote and deleted a file outside SCOPE, and no test survived the unit.

## Write by subtraction

- No prohibitions, warnings or reminders. Naming a behaviour puts it in the worker's context. SCOPE and the bridge (sandbox, lean mode) bound what the worker can do.
- No persona, tone or background the unit does not use.

## After the worker returns

The worker's report is a claim. Treat the unit as verified only when `commands` shows VERIFY ran with exit code 0, or when you rerun VERIFY yourself. In 2 of the 18 runs the Windows sandbox refused to start PowerShell, VERIFY never ran, and the work was still correct. The report said so, but only the command record is evidence.
