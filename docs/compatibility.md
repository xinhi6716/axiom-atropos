# Compatibility

[English](../README.md) · [繁體中文](../README.zh-TW.md) · [简体中文](../README.zh-CN.md)

Axiom Forge supports five hosts. Other host selectors were removed in 1.1.0; existing installations in those applications are left untouched.

| Host | Integration | Verification |
|---|---|---|
| Codex | Native Git marketplace/plugin | GitHub installation and update tested |
| Claude Code | Native marketplace/plugin, user scope | Installation and update tested in an isolated config directory |
| DeepSeek Harness | `~/.dsh/skills/`; honors `DSH_HOME` | Full payload deployment and installer tests |
| Antigravity | `~/.gemini/antigravity/skills/` | Installer tests; no model-session verification |
| Pi | `~/.agents/skills/` | Installer tests; no model-session verification |

Pi uses a shared skill location that other applications may also discover. Installing for Pi does not register or configure those applications. Same-name collisions stop installation before files are replaced.

Directory adapters write skills, adjacent `pstack-resources/`, and an ownership/hash record. Updates preserve local edits by stopping when owned content differs. Native plugin managers own their caches; maintain changes in this repository.

Personal instructions, models, credentials and general preferences are not edited. Host trust and permission prompts remain the host's responsibility. Install inside the relevant remote or container environment when needed; local installation does not sync cloud agents.

## Native commands

Codex:

```sh
codex plugin marketplace add xinhi6716/axiom-forge --ref main
codex plugin add axiom-forge@axiom-forge
```

Update with `codex plugin marketplace upgrade axiom-forge`, then `codex plugin add axiom-forge@axiom-forge`. Replace `main` with a release tag to pin a version.

Claude Code:

```text
/plugin marketplace add xinhi6716/axiom-forge
/plugin install axiom-forge@axiom-forge
```

## Sources and limits

- [Codex plugin format](https://developers.openai.com/plugins/build/plugins)
- [Claude Code plugin format](https://code.claude.com/docs/en/plugins-reference)
- [DeepSeek Harness skill discovery](https://github.com/deepseek-ai/deepseek-harness/blob/master/docs/subsystems/skills.md)
- [Antigravity and Pi directory mappings](https://github.com/vercel-labs/skills/blob/a80e4c9d12d607631249d4d7d059f835d4b2f13a/src/agents.ts)

Filesystem compatibility does not establish successful scientific work in every host/model. Skills use only available tools; see [host mappings](../pstack-resources/HOSTS.md). RSI supplies methodology and does not deploy models.
