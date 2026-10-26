# Pstack on Codex

This is poteto's cursor/plugins Pstack, bundled in the axiom-forge plugin for Codex. Apply these host mappings before the upstream workflow. The user's task and global AGENTS.md remain authoritative.

- Resolve `/skill-name` as the installed `$skill-name`; read sibling SKILL.md files as needed. When Pstack engineering work is selected, apply poteto-mode and its relevant bundled skills. Global default activation remains a host AGENTS.md setting. This overrides upstream manual-invocation-only wording; it does not mean running every workflow on every task.
- Cursor Task/Task tool means the available Codex collaboration tools. For a poteto-agent delegate, use an available worker/default agent and include the full installed poteto-mode path and the upstream agent instructions from this resources directory. For comment-sicko, include agents/comment-sicko.md and keep its read-only scope. Do not invent unavailable agent types. Interrupt finished child agents.
- Use the parent chat's model and reasoning effort unless the user or this file's role overrides explicitly choose an available model. Cursor Opus/Grok defaults are not installed or entitled by this package. Omit model overrides to inherit. Distinct-model review requires actually available distinct models; report a limitation instead of calling same-model reviewers independent model families.
- `/setup-pstack` configures this file's Role overrides section, not ~/.cursor/rules. Enumerate currently exposed Codex model/effort choices. Ask only for the desired changes when unspecified, preserve other roles, and never change app-wide model settings. Default budget: inherit parent; all roles: inherit parent.
- Map AskQuestion to the available user-input tool; TodoWrite to available plan tracking or a concise local plan; Read/Grep/Glob/Shell to filesystem and shell tools; create-skill to skill-creator. Use app-native thread/worktree/automation tools where applicable. A Cursor /loop is not a Codex automation: only schedule when the user requests it.
- Cursor-only tools (deslop, control-ui/control-cli, MCP services, Grok Bot webhooks) are not supplied by Pstack. Use a genuinely equivalent installed capability if available; otherwise state the specific missing capability. Do not claim those integrations were installed or tested.
- Follow the user's language and writing preferences over upstream narration requirements. Preserve factual limitations and necessary causal analysis.
- Paths to docs, agents, assets and automation templates resolve under this directory. The bundled automation templates are dormant reference material, not registered jobs. Never execute shell-specific examples unchanged on Windows.

## Scientific research routing

For work that also needs scientific methods, use [research-suite](../skills/research-suite/SKILL.md) as the coordinator. It owns scientific routing; this file only maps Pstack tools to Codex.

## Role overrides

All roles: inherit-parent
Reasoning effort: inherit-parent

## Packaged path resolution

Resolve skill paths inside this plugin first. The `pstack/skills/` prefix in upstream examples means this plugin's `skills/` directory. Playbooks and references live under `skills/poteto-mode/`; agents and docs live under `pstack-resources/`. This repository is the maintained source; installed caches are generated copies.
