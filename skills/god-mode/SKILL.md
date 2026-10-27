---
name: god-mode
description: Grant a wish on the current project. The axiom-forge dispatcher sends Codex workers to do the work, and Gemini rewrites their reports in plain language. Needs Claude Code with the axiom-bridge MCP server. Invoke as /god-mode <wish>.
disable-model-invocation: true
argument-hint: <wish>
---

# God mode

Wish: $ARGUMENTS

1. If the wish is empty, ask for it and stop.
2. Hand the wish, verbatim, and the absolute path of the current project directory to the `axiom-forge:dispatcher` agent. Do not plan, scope or edit the project yourself.
3. If the dispatcher returns a question, put it to the user and stop.
4. Read every file in `<store>/reports/` from the dispatcher's report. Call the axiom-bridge `gemini_preview` tool without `cwd`. Its prompt is this line, then the wish, the dispatcher's report and those files:

   Rewrite the work reports below for the person who made the wish, in the wish's language and in plain words: what was done, how it turned out, and what they need to decide.

5. Show Gemini's text. Under it, list from the dispatcher's report the changed files, the full test result and the choices the dispatcher decided. Where Gemini's text disagrees with those facts, correct it from the facts. If `gemini_preview` fails, show the dispatcher's report and say the plain-language preview failed.
