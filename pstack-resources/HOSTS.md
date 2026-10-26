# Host integration

Keep the native skill layout intact: relative links connect skills to shared resources.

- Codex: read [CODEX.md](CODEX.md) for tool mapping and model inheritance.
- Claude Code: use native tools and the [upstream harness notes](docs/harness/claude-code.md). Current user instructions and model settings override upstream author's model names and defaults; do not assume subscriptions or unavailable models.
- Other agents: load the selected SKILL.md and linked resources from this checkout. Map read/write/search/shell and delegation to capabilities actually exposed by the host. Missing delegation tools do not authorize inventing tool calls or spawning separate app tasks.

Use the host's language and output preferences. Never treat installation as permission to schedule work, deploy models, call paid services or change unrelated settings. A skill that needs an absent runtime must first establish that runtime's availability.

Start research at [research-suite](../skills/research-suite/SKILL.md), engineering at [poteto-mode](../skills/poteto-mode/SKILL.md). Read only the workflows the task needs. Personal global instructions stay outside this repository.
