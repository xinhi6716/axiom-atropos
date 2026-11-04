mod support;
use std::{path::PathBuf, process::Command};

#[test]
fn version_help_and_unknown_argument() {
    let version = support::axiom(&["--version"], None, None);
    assert!(version.status.success());
    assert_eq!(
        String::from_utf8(version.stdout).unwrap(),
        "atropos 2.0.0-rust.1\n"
    );
    let help = support::axiom(&["--help"], None, None);
    let text = String::from_utf8(help.stdout).unwrap();
    for name in [
        "bridge",
        "install",
        "package",
        "validate",
        "metadata",
        "memory",
        "orch",
        "check-plan",
        "work-log",
        "worktree-audit",
    ] {
        assert!(text.contains(name));
    }
    let unknown = support::axiom(&["--unknown"], None, None);
    assert_eq!(unknown.status.code(), Some(2));
    assert!(!unknown.stderr.is_empty());
}

#[test]
fn data_root_is_lazy_and_preserves_unicode_paths() {
    let root = support::scratch("資料 space");
    let primary = root.join("主要 path");
    let fallback = root.join("備用 path");
    // Run in a child so environment changes never race other tests.
    let probe = Command::new(env!("CARGO_BIN_EXE_atropos"))
        .args(["orch", "status"])
        .env_remove("AXIOM_DATA_HOME")
        .env_remove("CLAUDE_PLUGIN_DATA")
        .output()
        .unwrap();
    assert!(String::from_utf8(probe.stderr)
        .unwrap()
        .contains("neither AXIOM_DATA_HOME nor CLAUDE_PLUGIN_DATA"));
    let configured = support::axiom(&["orch", "status"], Some(&primary), Some(&fallback));
    assert!(!String::from_utf8(configured.stderr)
        .unwrap()
        .contains("neither AXIOM_DATA_HOME"));
    assert!(primary.is_absolute());
}

#[test]
fn data_root_precedence() {
    const PROBE: &str = "AXIOM_CONTRACT_PROBE";
    if std::env::var_os(PROBE).is_some() {
        let expected = PathBuf::from(std::env::var_os("AXIOM_EXPECTED_ROOT").unwrap());
        assert_eq!(
            axiom_atropos::Context::default().data_root().unwrap(),
            expected
        );
        return;
    }
    let root = support::scratch("precedence 資料");
    let primary = root.join("first 根");
    let fallback = root.join("second 根");
    for (first, expected) in [(Some(&primary), &primary), (None, &fallback)] {
        let mut cmd = Command::new(std::env::current_exe().unwrap());
        cmd.args(["--exact", "data_root_precedence"])
            .env(PROBE, "1")
            .env("AXIOM_EXPECTED_ROOT", expected)
            .env("CLAUDE_PLUGIN_DATA", &fallback)
            .env_remove("AXIOM_DATA_HOME");
        if let Some(first) = first {
            cmd.env("AXIOM_DATA_HOME", first);
        }
        assert!(cmd.status().unwrap().success());
    }
}

#[test]
fn basic_cli_runs_without_interpreter_path() {
    let output = Command::new(env!("CARGO_BIN_EXE_atropos"))
        .args(["--version"])
        .env("PATH", "")
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout, b"atropos 2.0.0-rust.1\n");
}
