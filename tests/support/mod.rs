use std::{
    path::PathBuf,
    process::{Command, Output},
};

pub fn scratch(name: &str) -> PathBuf {
    let root = PathBuf::from(
        std::env::var_os("AXIOM_SCRATCH").unwrap_or_else(|| std::env::temp_dir().into_os_string()),
    );
    assert!(root.is_absolute());
    let dir = root.join(format!("cli-contract-{}-{name}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}
pub fn axiom(
    args: &[&str],
    primary: Option<&std::path::Path>,
    fallback: Option<&std::path::Path>,
) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_atropos"));
    command
        .args(args)
        .env_remove("AXIOM_DATA_HOME")
        .env_remove("CLAUDE_PLUGIN_DATA");
    if let Some(value) = primary {
        command.env("AXIOM_DATA_HOME", value);
    }
    if let Some(value) = fallback {
        command.env("CLAUDE_PLUGIN_DATA", value);
    }
    command.output().unwrap()
}
