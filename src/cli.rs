use crate::{
    base::{Failure, Res},
    distribution::{arg, io},
    CommandOutput, Context,
};
use serde_json::{json, Value};
use std::path::Path;
const HELP:&str="atropos 2.0.0-rust.1\nUsage: atropos <COMMAND> [ARGS...]\nCommands: bridge install package validate metadata memory orch check-plan work-log worktree-audit assets\nUse --source DIR for metadata/validate/package; install embeds its versioned assets.\n";
pub fn run(args: &[String], context: &Context) -> CommandOutput {
    if args.is_empty() || matches!(args[0].as_str(), "--help" | "-h") {
        return CommandOutput::success(HELP);
    }
    if args[0] == "--version" || args[0] == "-V" {
        return CommandOutput::success(concat!("atropos ", env!("CARGO_PKG_VERSION"), "\n"));
    }
    let known = [
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
        "assets",
    ];
    if !known.contains(&args[0].as_str()) {
        return CommandOutput::error(2, format!("unknown argument or command: {}\n", args[0]));
    }
    if args.get(1).is_some_and(|a| a == "--help") {
        return CommandOutput::success(format!(
            "Usage: atropos {} [ARGS...]\nSee docs/rust-runtime.md for the command contract.\n",
            args[0]
        ));
    }
    if args[0] == "memory" {
        if let Err(e) = context.data_root() {
            return CommandOutput::error(1, format!("{e}\n"));
        }
    }
    if args[0] == "bridge" {
        return match crate::bridge::serve() {
            Ok(()) => CommandOutput::success(""),
            Err(e) => CommandOutput::error(1, format!("{e}\n")),
        };
    }
    if args[0] == "orch" {
        let mut a = args[1..].to_vec();
        for flag in ["--store", "--project"] {
            if let Some(i) = a.iter().position(|v| v == flag) {
                if a.get(i + 1).is_none_or(|s| s.starts_with("--")) {
                    return CommandOutput::error(2, format!("{flag} requires a value\n"));
                }
            }
        }
        let path = if let Some(p) = arg(&a, "--store") {
            std::path::PathBuf::from(p)
        } else {
            let configured = std::env::var_os("ORCH_STORE").filter(|v| !v.is_empty());
            if let Some(p) = configured.filter(|_| arg(&a, "--project").is_none()) {
                std::path::PathBuf::from(p)
            } else {
                let root = match context.data_root() {
                    Ok(p) => p,
                    Err(e) => return CommandOutput::error(1, format!("{e}\n")),
                };
                match crate::orch::store_for(&root, arg(&a, "--project").unwrap_or("default")) {
                    Ok(p) => p,
                    Err(e) => return CommandOutput::error(1, e.to_string()),
                }
            }
        };
        for flag in ["--store", "--project"] {
            if let Some(i) = a.iter().position(|v| v == flag) {
                a.drain(i..=i + 1);
            }
        }
        let json = a.iter().any(|s| s == "--json");
        let force = a.iter().any(|s| s == "--force");
        a.retain(|s| s != "--json" && s != "--force");
        let o = crate::orch::run(&path, &a, json, force);
        return CommandOutput {
            exit_code: o.exit_code,
            stdout: if o.stdout.is_empty() {
                o.stdout
            } else {
                format!("{}\n", o.stdout.trim_end())
            },
            stderr: o.stderr,
        };
    }
    let result = (|| -> Res<Value> {
        let a = &args[1..];
        for flag in ["--source", "--output", "--host"] {
            if let Some(i) = a.iter().position(|v| v == flag) {
                if a.get(i + 1).is_none_or(|s| s.starts_with("--")) {
                    return Err(Failure::new(
                        "invalid_argument",
                        format!("{flag} requires a value"),
                    ));
                }
            }
        }
        let current = std::env::current_dir().map_err(io)?;
        let root = arg(a, "--source").map(Path::new).unwrap_or(&current);
        match args[0].as_str() {
            "install" => crate::install::run(a),
            "metadata" => crate::distribution::metadata(root, a.iter().any(|s| s == "--check")),
            "validate" => crate::distribution::validate(root),
            "package" => crate::distribution::package(
                root,
                Path::new(arg(a, "--output").unwrap_or("dist/axiom-atropos-2.0.0-rust.1.zip")),
                a.iter().any(|s| s == "--candidate"),
            ),
            "assets" => {
                let dest = Path::new(a.first().ok_or_else(|| {
                    Failure::new("invalid_argument", "assets requires destination")
                })?);
                crate::distribution::materialize(dest, None)?;
                Ok(json!({"destination":dest,"files":crate::assets::FILES.len()}))
            }
            "memory" => {
                let name = a.first().ok_or_else(|| {
                    Failure::new(
                        "invalid_argument",
                        "memory requires record/search/get/digest",
                    )
                })?;
                let data: Value =
                    serde_json::from_str(a.get(1).map(String::as_str).unwrap_or("{}"))
                        .map_err(io)?;
                crate::bridge::Bridge::default().call(&format!("memory_{name}"), &data)
            }
            name => crate::helpers::run(name, a),
        }
    })();
    match result {
        Ok(v) => CommandOutput::success(format!("{}\n", serde_json::to_string_pretty(&v).unwrap())),
        Err(e) => CommandOutput::error(1, format!("{e}\n")),
    }
}
