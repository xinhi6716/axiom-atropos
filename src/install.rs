use crate::{
    base::{Failure, Res},
    distribution::{self, arg, io, no_links, tree},
};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};
fn home() -> Res<PathBuf> {
    std::env::var_os("USERPROFILE")
        .or_else(|| std::env::var_os("HOME"))
        .map(PathBuf::from)
        .ok_or_else(|| io("home directory unavailable"))
}
pub fn skills(source: &Path, base: &Path) -> Res<Value> {
    no_links(base)?;
    fs::create_dir_all(base).map_err(io)?;
    no_links(&base.join(".axiom-forge-install.lock"))?;
    let lock = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(base.join(".axiom-forge-install.lock"))
        .map_err(io)?;
    fs2::FileExt::try_lock_exclusive(&lock).map_err(|_| io("another installation is running"))?;
    let record = base.join(".axiom-forge-install.json");
    no_links(&record)?;
    let previous: Value = if record.exists() {
        serde_json::from_slice(&fs::read(&record).map_err(io)?).map_err(io)?
    } else {
        json!({})
    };
    if record.exists() && (!previous["bundles"].is_object() || !previous["repository"].is_string())
    {
        return Err(io("invalid ownership record"));
    }
    if previous.get("repository").is_some() && previous["repository"] != "xinhi6716/axiom-forge" {
        return Err(io("unrecognized ownership record"));
    }
    let mut bundles = BTreeMap::new();
    for e in fs::read_dir(source.join("skills")).map_err(io)? {
        let p = e.map_err(io)?.path();
        if p.join("SKILL.md").is_file() {
            bundles.insert(
                format!("skills/{}", p.file_name().unwrap().to_string_lossy()),
                p,
            );
        }
    }
    bundles.insert("pstack-resources".into(), source.join("pstack-resources"));
    let mut names: BTreeSet<String> = bundles.keys().cloned().collect();
    if let Some(old) = previous["bundles"].as_object() {
        names.extend(old.keys().cloned());
    }
    for relative in &names {
        if !distribution::safe_relative(relative)
            || (relative != "pstack-resources"
                && !(relative.starts_with("skills/") && relative.split('/').count() == 2))
        {
            return Err(io("invalid ownership path"));
        }
        let p = base.join(relative);
        no_links(&p)?;
        if p.exists() {
            let current = serde_json::to_value(tree(&p)?).unwrap();
            if previous["bundles"].get(relative) != Some(&current) {
                return Err(io(format!(
                    "existing or locally modified content preserved: {}",
                    p.display()
                )));
            }
        }
    }
    let stage = tempfile::Builder::new()
        .prefix(".axiom-forge-stage-")
        .tempdir_in(base)
        .map_err(io)?;
    let mut hashes = BTreeMap::new();
    for (relative, origin) in &bundles {
        let inventory = tree(origin)?;
        let dest = stage.path().join("new").join(relative);
        fs::create_dir_all(&dest).map_err(io)?;
        for name in inventory.keys() {
            let p = dest.join(name);
            fs::create_dir_all(p.parent().unwrap()).map_err(io)?;
            fs::copy(origin.join(name), p).map_err(io)?;
        }
        hashes.insert(relative.clone(), inventory);
    }
    let mut moved = Vec::new();
    let mut installed = Vec::new();
    let result = (|| -> Res<()> {
        for relative in &names {
            let target = base.join(relative);
            if target.exists() {
                let backup = stage.path().join("old").join(relative);
                fs::create_dir_all(backup.parent().unwrap()).map_err(io)?;
                fs::rename(&target, backup).map_err(io)?;
                moved.push(relative.clone());
            }
            if bundles.contains_key(relative) {
                fs::create_dir_all(target.parent().unwrap()).map_err(io)?;
                fs::rename(stage.path().join("new").join(relative), &target).map_err(io)?;
                installed.push(relative.clone());
            }
        }
        let manifest: Value =
            serde_json::from_slice(&fs::read(source.join("plugin.json")).map_err(io)?)
                .map_err(io)?;
        crate::base::atomic_write(&record,&serde_json::to_vec_pretty(&json!({"repository":"xinhi6716/axiom-forge","version":manifest["version"],"bundles":hashes})).unwrap()).map_err(io)?;
        Ok(())
    })();
    if let Err(error) = result {
        let rollback = (|| -> std::io::Result<()> {
            for relative in installed.iter().rev() {
                fs::remove_dir_all(base.join(relative))?;
            }
            for relative in moved.iter().rev() {
                fs::rename(stage.path().join("old").join(relative), base.join(relative))?;
            }
            Ok(())
        })();
        if let Err(recovery) = rollback {
            let saved = stage.keep();
            return Err(io(format!(
                "{error}; rollback failed: {recovery}; backups retained at {}",
                saved.display()
            )));
        }
        return Err(error);
    }
    Ok(json!({"status":"completed","skills":bundles.len()-1,"destination":base}))
}
fn host_command(cmd: &[String], args: &[String]) -> Res<String> {
    let mut c = crate::base::proc::command(cmd);
    c.args(args);
    let o = crate::process::capture(&mut c, None, std::time::Duration::from_secs(120))?;
    if o.exit_code != 0 {
        return Err(io(format!("{}{}", o.stdout, o.stderr)));
    }
    Ok(o.stdout)
}
pub fn run(args: &[String]) -> Res<Value> {
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--list-hosts" | "--update" => i += 1,
            "--host" | "--source" => {
                if args.get(i + 1).is_none_or(|v| v.starts_with("--")) {
                    return Err(Failure::new(
                        "invalid_argument",
                        format!("{} requires a value", args[i]),
                    ));
                }
                i += 2;
            }
            _ => {
                return Err(Failure::new(
                    "invalid_argument",
                    format!("unknown install argument {}", args[i]),
                ))
            }
        }
    }

    if args.iter().any(|s| s == "--list-hosts") {
        return Ok(
            json!({"hosts":["codex","claude","deepseek","antigravity","pi"],"aliases":{"deepseek-harness":"deepseek","claude-code":"claude"}}),
        );
    }
    let host = match arg(args, "--host").unwrap_or("codex") {
        "deepseek-harness" => "deepseek",
        "claude-code" => "claude",
        s => s,
    };
    if !["codex", "claude", "deepseek", "antigravity", "pi"].contains(&host) {
        return Err(Failure::new("invalid_argument", "unknown host"));
    }
    let scratch = tempfile::tempdir().map_err(io)?;
    distribution::materialize(scratch.path(), arg(args, "--source").map(Path::new))?;
    distribution::validate(scratch.path())?;
    let h = home()?;
    if matches!(host, "codex" | "claude") {
        let versioned = h
            .join(".local/share/axiom-atropos")
            .join(env!("CARGO_PKG_VERSION"));
        no_links(&versioned)?;
        let cmd = crate::base::resolve_command(
            if host == "codex" {
                "AXIOM_CODEX_CMD"
            } else {
                "AXIOM_CLAUDE_CMD"
            },
            host,
        )?;
        fs::create_dir_all(versioned.parent().unwrap()).map_err(io)?;
        let stage = tempfile::Builder::new()
            .prefix(".install-")
            .tempdir_in(versioned.parent().unwrap())
            .map_err(io)?;
        let payload = stage.path().join("payload");
        distribution::materialize(&payload, Some(scratch.path()))?;
        fs::create_dir_all(payload.join("bin")).map_err(io)?;
        fs::copy(
            std::env::current_exe().map_err(io)?,
            payload.join("bin").join(if cfg!(windows) {
                "atropos.exe"
            } else {
                "atropos"
            }),
        )
        .map_err(io)?;
        if versioned.exists() {
            if tree(&versioned)? != tree(&payload)? {
                return Err(io(
                    "versioned installation differs; existing content preserved",
                ));
            }
        } else {
            fs::rename(&payload, &versioned).map_err(io)?;
        }
        let mut calls = vec![vec![
            "plugin".into(),
            "marketplace".into(),
            "add".into(),
            versioned.to_string_lossy().into_owned(),
        ]];
        calls.push(if host == "codex" {
            vec![
                "plugin".into(),
                "add".into(),
                "axiom-forge@axiom-forge".into(),
            ]
        } else {
            vec![
                "plugin".into(),
                if args.iter().any(|s| s == "--update") {
                    "update".into()
                } else {
                    "install".into()
                },
                "axiom-forge@axiom-forge".into(),
                "--scope".into(),
                "user".into(),
            ]
        });
        let previous = if host == "codex" && args.iter().any(|s| s == "--update") {
            let listed = host_command(
                &cmd,
                &[
                    "plugin".into(),
                    "marketplace".into(),
                    "list".into(),
                    "--json".into(),
                ],
            )?;
            let listed: Value = serde_json::from_str(&listed).map_err(io)?;
            let rows = listed["marketplaces"]
                .as_array()
                .ok_or_else(|| io("invalid marketplace list"))?;
            match rows.iter().find(|v| v["name"] == "axiom-forge") {
                Some(row) => {
                    let root = PathBuf::from(
                        row["root"]
                            .as_str()
                            .ok_or_else(|| io("missing previous marketplace root"))?,
                    );
                    if !root.is_dir() {
                        return Err(io(
                            "previous marketplace root is unavailable; cannot recover an update",
                        ));
                    }
                    if fs::canonicalize(&root).map_err(io)?
                        != fs::canonicalize(&versioned).map_err(io)?
                    {
                        Some(root)
                    } else {
                        None
                    }
                }
                None => None,
            }
        } else {
            None
        };
        let remove = vec![
            "plugin".into(),
            "marketplace".into(),
            "remove".into(),
            "axiom-forge".into(),
        ];
        if previous.is_some() {
            host_command(&cmd, &remove)?;
        }
        let result = (|| -> Res<()> {
            for args in calls {
                host_command(&cmd, &args)?;
            }
            Ok(())
        })();
        if let Err(error) = result {
            if let Some(root) = previous {
                let _ = host_command(&cmd, &remove);
                let restored = host_command(
                    &cmd,
                    &[
                        "plugin".into(),
                        "marketplace".into(),
                        "add".into(),
                        root.to_string_lossy().into_owned(),
                    ],
                );
                return Err(io(format!(
                    "{host}: {error}; previous local marketplace {}: {}",
                    root.display(),
                    match restored {
                        Ok(_) => "restored as a local snapshot".into(),
                        Err(e) => format!("restore failed: {e}"),
                    }
                )));
            }
            return Err(io(format!(
                "{host}: {error}; when replacing an existing source, use --update"
            )));
        }

        Ok(
            json!({"status":"completed","host":host,"source":versioned,"version":env!("CARGO_PKG_VERSION")}),
        )
    } else {
        let base = match host {
            "deepseek" => std::env::var_os("DSH_HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|| h.join(".dsh")),
            "antigravity" => h.join(".gemini/antigravity"),
            _ => h.join(".agents"),
        };
        skills(scratch.path(), &base)
    }
}
