use crate::base::{Failure, Res};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::Write,
    path::{Component, Path},
};
pub fn io(e: impl std::fmt::Display) -> Failure {
    Failure::new("io", e.to_string())
}
pub fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
pub fn arg<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.windows(2)
        .find(|p| p[0] == name)
        .map(|p| p[1].as_str())
}
pub fn safe_relative(path: &str) -> bool {
    !path.is_empty()
        && !path.contains('\\')
        && !path.contains(':')
        && Path::new(path)
            .components()
            .all(|p| matches!(p, Component::Normal(_)))
}
pub fn no_links(path: &Path) -> Res<()> {
    for parent in path.ancestors() {
        if let Ok(m) = fs::symlink_metadata(parent) {
            #[cfg(windows)]
            {
                use std::os::windows::fs::MetadataExt;
                if m.file_attributes() & 0x400 != 0 {
                    return Err(io("linked path refused"));
                }
            }
            if m.file_type().is_symlink() {
                return Err(io("linked path refused"));
            }
        }
    }
    Ok(())
}
pub fn tree(path: &Path) -> Res<BTreeMap<String, String>> {
    fn walk(root: &Path, p: &Path, out: &mut BTreeMap<String, String>) -> Res<()> {
        no_links(p)?;
        for e in fs::read_dir(p).map_err(io)? {
            let p = e.map_err(io)?.path();
            no_links(&p)?;
            if p.is_dir() {
                walk(root, &p, out)?;
            } else if p.is_file() {
                out.insert(
                    p.strip_prefix(root)
                        .unwrap()
                        .to_string_lossy()
                        .replace('\\', "/"),
                    hash(&fs::read(p).map_err(io)?),
                );
            } else {
                return Err(io("non-regular asset"));
            }
        }
        Ok(())
    }
    let mut out = BTreeMap::new();
    walk(path, path, &mut out)?;
    Ok(out)
}
pub fn materialize(destination: &Path, source: Option<&Path>) -> Res<()> {
    no_links(destination)?;
    for (name, _) in crate::assets::FILES {
        let p = destination.join(name);
        no_links(&p)?;
        if p.exists() {
            return Err(io(format!("destination already contains {}", p.display())));
        }
    }
    fs::create_dir_all(destination).map_err(io)?;
    for (name, embedded) in crate::assets::FILES {
        if !safe_relative(name) {
            return Err(io("invalid embedded path"));
        }
        let bytes = if let Some(root) = source {
            let p = root.join(name);
            no_links(&p)?;
            fs::read(p).map_err(io)?
        } else {
            embedded.to_vec()
        };
        let p = destination.join(name);
        fs::create_dir_all(p.parent().unwrap()).map_err(io)?;
        fs::write(p, bytes).map_err(io)?;
    }
    Ok(())
}
pub fn metadata(root: &Path, check: bool) -> Res<Value> {
    let m: Value =
        serde_json::from_slice(&fs::read(root.join("plugin.json")).map_err(io)?).map_err(io)?;
    let mut common = serde_json::Map::new();
    for k in [
        "name",
        "version",
        "description",
        "author",
        "license",
        "repository",
        "homepage",
    ] {
        common.insert(k.into(), m[k].clone());
    }
    let mut codex = common.clone();
    codex.insert("skills".into(), json!("./skills/"));
    if let Some(o) = m["extensions"]["com.openai"].as_object() {
        codex.extend(o.clone());
    }
    let mut claude = common;
    if let Some(o) = m["extensions"]["com.anthropic"].as_object() {
        claude.extend(o.clone());
    }
    let outputs = [
        (".codex-plugin/plugin.json", Value::Object(codex)),
        (".claude-plugin/plugin.json", Value::Object(claude)),
        (
            ".agents/plugins/marketplace.json",
            json!({"name":m["name"],"interface":{"displayName":"Axiom Atropos"},"plugins":[{"name":m["name"],"source":{"source":"local","path":"./"},"policy":{"installation":"AVAILABLE","authentication":"ON_INSTALL"},"category":"Productivity"}]}),
        ),
        (
            ".claude-plugin/marketplace.json",
            json!({"name":m["name"],"owner":m["author"],"metadata":{"description":m["description"]},"plugins":[{"name":m["name"],"source":"./","description":m["description"],"version":m["version"]}]}),
        ),
    ];
    for (path, value) in outputs {
        let p = root.join(path);
        if check {
            let actual: Value = serde_json::from_slice(&fs::read(&p).map_err(io)?).map_err(io)?;
            if actual != value {
                return Err(io(format!("stale metadata: {path}")));
            }
        } else {
            fs::create_dir_all(p.parent().unwrap()).map_err(io)?;
            fs::write(
                p,
                format!("{}\n", serde_json::to_string_pretty(&value).unwrap()),
            )
            .map_err(io)?;
        }
    }
    Ok(json!({"status":"completed","metadata":"consistent"}))
}
pub fn validate(root: &Path) -> Res<Value> {
    metadata(root, true)?;
    let m: Value =
        serde_json::from_slice(&fs::read(root.join("plugin.json")).map_err(io)?).map_err(io)?;
    if m["name"] != "axiom-forge" {
        return Err(io("plugin identity changed"));
    }
    if m["version"] != env!("CARGO_PKG_VERSION") {
        return Err(io("asset version does not match runtime"));
    }
    let source: Value =
        serde_json::from_slice(&fs::read(root.join("sources.json")).map_err(io)?).map_err(io)?;
    let mut count = 0;
    for entry in fs::read_dir(root.join("skills")).map_err(io)? {
        let p = entry.map_err(io)?.path();
        let skill = p.join("SKILL.md");
        if !skill.is_file() {
            continue;
        }
        count += 1;
        let text = fs::read_to_string(skill).map_err(io)?;
        let front = text
            .split("---")
            .nth(1)
            .ok_or_else(|| io("missing frontmatter"))?;
        let v: Value = serde_yaml::from_str(front).map_err(io)?;
        if v["name"].as_str() != p.file_name().and_then(|n| n.to_str())
            || v["description"].as_str().unwrap_or("").is_empty()
        {
            return Err(io(format!("invalid skill {}", p.display())));
        }
    }
    if Some(count) != source["skill_count"].as_u64() {
        return Err(io("skill count differs from inventory"));
    }
    let home_pattern = regex::Regex::new(r"(?i)[A-Z]:[/\\]Users[/\\][a-z0-9_.-]+").unwrap();
    let secret_pattern=regex::Regex::new(r"-----BEGIN (?:RSA |EC |OPENSSH )?PRIVATE KEY-----|gh[pousr]_[A-Za-z0-9]{30,}|github_pat_[A-Za-z0-9_]{40,}").unwrap();
    let code = regex::Regex::new(r"(?s)```.*?```").unwrap();
    let links = regex::Regex::new(r"\]\(([^)]+)\)").unwrap();
    for (name, _) in crate::assets::FILES {
        let path = root.join(name);
        no_links(&path)?;
        let bytes = fs::read(&path).map_err(|e| io(format!("{name}: {e}")))?;
        let leaf = path.file_name().unwrap().to_string_lossy().to_lowercase();
        if [
            "agents.md",
            "agents.override.md",
            ".env",
            "credentials.json",
            "auth.json",
            "id_rsa",
            "id_ed25519",
        ]
        .contains(&leaf.as_str())
            || (leaf.starts_with(".env.") && leaf != ".env.example")
            || ["pem", "key", "pt", "pth", "onnx", "safetensors", "pyc"]
                .contains(&path.extension().and_then(|s| s.to_str()).unwrap_or(""))
            || name
                .split('/')
                .any(|s| ["node_modules", ".venv", "venv"].contains(&s))
        {
            return Err(io(format!("private/runtime asset refused: {name}")));
        }
        if let Ok(text) = std::str::from_utf8(&bytes) {
            if secret_pattern.is_match(text)
                || home_pattern
                    .find_iter(text)
                    .any(|m| !m.as_str().ends_with("username") && !m.as_str().contains("YOUR_"))
            {
                return Err(io(format!("private content in {name}")));
            }
            if name.ends_with(".md") {
                let prose = code.replace_all(text, "");
                for cap in links.captures_iter(&prose) {
                    let target = cap[1].split('#').next().unwrap_or("");
                    if target.is_empty()
                        || target.contains(':')
                        || target.starts_with('/')
                        || (*name == "skills/why/references/synthesizer-prompt.md"
                            && target == "url")
                    {
                        continue;
                    }
                    if !path.parent().unwrap().join(target).exists() {
                        return Err(io(format!("{name}: missing link {target}")));
                    }
                }
            }
        }
    }
    for key in ["logo", "logoDark", "composerIcon"] {
        let icon = m["extensions"]["com.openai"]["interface"][key]
            .as_str()
            .ok_or_else(|| io("missing icon metadata"))?;
        if !root.join(icon).is_file() {
            return Err(io(format!("missing icon {key}")));
        }
    }

    Ok(json!({"skills":count,"errors":[],"version":m["version"]}))
}
pub fn package(root: &Path, output: &Path, candidate: bool) -> Res<Value> {
    validate(root)?;
    no_links(output)?;
    if !cfg!(target_arch = "x86_64") || !(cfg!(windows) || cfg!(target_os = "linux")) {
        return Err(io("native packages support Windows/Linux x86-64 only"));
    }
    if output.exists() {
        let output_path = fs::canonicalize(output).map_err(io)?;
        for (name, _) in crate::assets::FILES {
            if fs::canonicalize(root.join(name)).map_err(io)? == output_path {
                return Err(io("archive output would overwrite a distribution asset"));
            }
        }
    }
    let git = |args: &[&str]| -> Res<String> {
        let o = std::process::Command::new("git")
            .args(args)
            .current_dir(root)
            .output()
            .map_err(io)?;
        if !o.status.success() {
            return Err(io(String::from_utf8_lossy(&o.stderr)));
        }
        Ok(String::from_utf8_lossy(&o.stdout).trim().into())
    };
    let commit = git(&["rev-parse", "HEAD"])?;
    if !candidate
        && fs::canonicalize(git(&["rev-parse", "--show-toplevel"])?).map_err(io)?
            != fs::canonicalize(root).map_err(io)?
    {
        return Err(io("formal packaging requires the repository root"));
    }
    if !candidate && !git(&["status", "--porcelain", "--untracked-files=no"])?.is_empty() {
        return Err(io("commit tracked changes before publishing"));
    }
    if !candidate {
        let tracked = git(&["ls-files"])?;
        for (name, _) in crate::assets::FILES {
            if !tracked.lines().any(|p| p == *name) {
                return Err(io(format!("uncommitted asset {name}")));
            }
        }
    }
    let exe = fs::read(std::env::current_exe().map_err(io)?).map_err(io)?;
    let platform = if cfg!(windows) {
        "x86_64-pc-windows-msvc"
    } else if cfg!(target_os = "linux") {
        "x86_64-unknown-linux-gnu"
    } else {
        "unsupported"
    };
    let bin = if cfg!(windows) {
        "bin/atropos.exe"
    } else {
        "bin/atropos"
    };
    let mut entries = BTreeMap::new();
    for (name, embedded) in crate::assets::FILES {
        let bytes = fs::read(root.join(name)).map_err(io)?;
        if bytes.as_slice() != *embedded {
            return Err(io(format!("rebuild runtime after changing asset {name}")));
        }
        entries.insert(name.to_string(), bytes);
    }
    entries.insert(bin.into(), exe);
    let hashes: BTreeMap<_, _> = entries.iter().map(|(p, b)| (p.clone(), hash(b))).collect();
    let manifest = json!({"version":env!("CARGO_PKG_VERSION"),"target":platform,"source_commit":commit,"candidate":candidate,"files":hashes});
    entries.insert(
        "MANIFEST.json".into(),
        serde_json::to_vec_pretty(&manifest).unwrap(),
    );
    if let Some(p) = output.parent() {
        fs::create_dir_all(p).map_err(io)?;
    }
    let f = fs::File::create(output).map_err(io)?;
    if output.to_string_lossy().ends_with(".tar.gz") {
        let mut archive = tar::Builder::new(flate2::write::GzEncoder::new(
            f,
            flate2::Compression::default(),
        ));
        for (name, bytes) in entries {
            let mut h = tar::Header::new_gnu();
            h.set_size(bytes.len() as u64);
            h.set_mode(if name == bin || name.ends_with(".sh") {
                0o755
            } else {
                0o644
            });
            h.set_mtime(0);
            h.set_cksum();
            archive
                .append_data(&mut h, name, bytes.as_slice())
                .map_err(io)?;
        }
        archive.into_inner().map_err(io)?.finish().map_err(io)?;
    } else {
        let mut archive = zip::ZipWriter::new(f);
        for (name, bytes) in entries {
            archive
                .start_file(
                    name.clone(),
                    zip::write::SimpleFileOptions::default()
                        .compression_method(zip::CompressionMethod::Deflated)
                        .unix_permissions(if name == bin || name.ends_with(".sh") {
                            0o755
                        } else {
                            0o644
                        }),
                )
                .map_err(io)?;
            archive.write_all(&bytes).map_err(io)?;
        }
        archive.finish().map_err(io)?;
    }
    let checksum = hash(&fs::read(output).map_err(io)?);
    fs::write(
        output.with_extension(format!(
            "{}.sha256",
            output.extension().unwrap_or_default().to_string_lossy()
        )),
        format!(
            "{checksum}  {}\n",
            output.file_name().unwrap().to_string_lossy()
        ),
    )
    .map_err(io)?;
    Ok(json!({"package":output,"sha256":checksum,"source_commit":commit,"candidate":candidate}))
}
