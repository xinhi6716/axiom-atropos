# Maintaining the Rust release

Change Cargo.toml and plugin.json together. Run `cargo run -- metadata`, then rebuild so embedded manifests match. Add distribution assets explicitly to assets.txt. Keep all three README files aligned. Preserve upstream notices and optional companion tools.

Run formatting, Clippy and all-feature tests as documented in [rust-runtime.md](rust-runtime.md). Commit before formal packaging. CI creates Windows/Linux native archives, their checksums, and the Cargo source package. Inspect CI and artifacts before publishing a prerelease from the tested branch commit.

Never include credentials, personal AGENTS.md, model weights, environments, build outputs or arbitrary workspace files in assets.txt. The fixed inventory defines publication; ambient files are not scanned into archives.
