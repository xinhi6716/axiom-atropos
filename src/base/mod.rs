use serde::Serialize;
use std::{
    io::Write,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};
pub mod proc;
pub mod schema;
#[derive(Serialize, Debug, Clone)]
pub struct Failure {
    pub kind: String,
    pub message: String,
}
impl Failure {
    pub fn new(kind: &str, message: impl Into<String>) -> Self {
        Self {
            kind: kind.into(),
            message: message.into(),
        }
    }
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({"kind": self.kind, "message": self.message})
    }
}
impl std::fmt::Display for Failure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.kind, self.message)
    }
}
impl std::error::Error for Failure {}
pub type Res<T> = Result<T, Failure>;
pub fn resolve_command(env_name: &str, executable: &str) -> Res<Vec<String>> {
    if let Some(raw) = std::env::var_os(env_name).filter(|v| !v.is_empty()) {
        let parsed = raw
            .to_str()
            .and_then(|s| serde_json::from_str::<Vec<String>>(s).ok());
        return parsed
            .filter(|v| !v.is_empty() && v.iter().all(|s| !s.is_empty()))
            .ok_or_else(|| {
                Failure::new(
                    "config",
                    format!("{env_name} must be a non-empty JSON array of non-empty strings"),
                )
            });
    }
    let paths = std::env::var_os("PATH").unwrap_or_default();
    for dir in std::env::split_paths(&paths) {
        #[cfg(unix)]
        let candidates = vec![dir.join(executable)];
        #[cfg(windows)]
        let candidates = {
            let extensions =
                std::env::var_os("PATHEXT").unwrap_or_else(|| ".COM;.EXE;.BAT;.CMD".into());
            let extensions = extensions.to_string_lossy();
            let extensions: Vec<_> = extensions.split(';').filter(|x| !x.is_empty()).collect();
            if extensions.iter().any(|ext| {
                executable
                    .to_ascii_lowercase()
                    .ends_with(&ext.to_ascii_lowercase())
            }) {
                vec![dir.join(executable)]
            } else {
                extensions
                    .iter()
                    .map(|ext| dir.join(format!("{executable}{ext}")))
                    .collect()
            }
        };
        for path in candidates {
            if path.is_file() {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    if path
                        .metadata()
                        .map(|m| m.permissions().mode() & 0o111 == 0)
                        .unwrap_or(true)
                    {
                        continue;
                    }
                }
                return Ok(vec![path.to_string_lossy().into_owned()]);
            }
        }
    }
    Err(Failure::new(
        "not_installed",
        format!("{executable} is not on PATH; install it or set {env_name}"),
    ))
}
pub fn axiom_home() -> PathBuf {
    std::env::var_os("AXIOM_HOME")
        .filter(|s| !s.is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(
                std::env::var_os("USERPROFILE")
                    .or_else(|| std::env::var_os("HOME"))
                    .unwrap_or_default(),
            )
            .join(".axiom")
        })
}
pub fn data_home() -> Res<PathBuf> {
    std::env::var_os("AXIOM_DATA_HOME")
        .filter(|s| !s.is_empty())
        .or_else(|| std::env::var_os("CLAUDE_PLUGIN_DATA").filter(|s| !s.is_empty()))
        .map(PathBuf::from)
        .ok_or_else(|| {
            Failure::new(
                "config",
                "AXIOM_DATA_HOME or CLAUDE_PLUGIN_DATA must be set",
            )
        })
}
pub fn require_slug(name: &str, value: &str) -> Res<()> {
    let b = value.as_bytes();
    if !b.is_empty()
        && b.len() <= 64
        && (b[0].is_ascii_lowercase() || b[0].is_ascii_digit())
        && b.iter()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == b'_' || *c == b'-')
    {
        Ok(())
    } else {
        Err(Failure::new(
            "invalid_argument",
            format!("invalid {name}: {value}"),
        ))
    }
}
pub fn slug_for_dir(dir: &Path) -> String {
    let name = dir
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_lowercase();
    let mut s = String::new();
    for c in name.chars() {
        let c = if c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-' {
            c
        } else {
            '-'
        };
        if c != '-' || !s.ends_with('-') {
            s.push(c);
        }
    }
    let s = s.trim_start_matches(['-', '_']).trim_end_matches('-');
    let s = &s[..s.len().min(64)];
    let s = s.trim_end_matches('-');
    if s.is_empty() {
        "project".into()
    } else {
        s.into()
    }
}
pub fn require_directory(path: &str) -> Res<PathBuf> {
    let p = PathBuf::from(path);
    if p.is_absolute() && p.is_dir() {
        Ok(p)
    } else {
        Err(Failure::new(
            "invalid_argument",
            format!("not an absolute directory: {path}"),
        ))
    }
}
static COUNTER: AtomicU64 = AtomicU64::new(0);
pub fn unique() -> String {
    use sha2::{Digest, Sha256};
    let mut h = Sha256::new();
    h.update(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
            .to_le_bytes(),
    );
    h.update(std::process::id().to_le_bytes());
    h.update(COUNTER.fetch_add(1, Ordering::Relaxed).to_le_bytes());
    h.finalize()[..16]
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}
pub fn atomic_write(path: &Path, contents: &[u8]) -> std::io::Result<()> {
    let parent = path.parent().unwrap_or(Path::new("."));
    let name = path
        .file_name()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "missing filename"))?
        .to_string_lossy();
    let temp = parent.join(format!(".{name}.{}.{}.tmp", std::process::id(), unique()));
    let result = (|| {
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp)?;
        f.write_all(contents)?;
        drop(f);
        std::fs::rename(&temp, path)
    })();
    if result.is_err() {
        let _ = std::fs::remove_file(&temp);
    }
    result
}
fn civil(days: i64) -> (i64, i64, i64) {
    let z = days + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = mp + if mp < 10 { 3 } else { -9 };
    (y + if m <= 2 { 1 } else { 0 }, m, d)
}
pub fn now_iso() -> String {
    let ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let sec = (ms / 1000) as i64;
    let (y, m, d) = civil(sec / 86400);
    let t = sec % 86400;
    format!(
        "{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}.{:03}Z",
        t / 3600,
        t % 3600 / 60,
        t % 60,
        ms % 1000
    )
}
pub fn today() -> String {
    now_iso()[..10].to_string()
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Family {
    OpenAi,
    Google,
    Anthropic,
    Unknown,
}
pub fn family_of(model: &str) -> Family {
    if model.starts_with("gpt-")
        || model.starts_with("codex-")
        || (model.as_bytes().first() == Some(&b'o')
            && model.as_bytes().get(1).is_some_and(u8::is_ascii_digit))
    {
        Family::OpenAi
    } else if model.starts_with("gemini-") {
        Family::Google
    } else if model.starts_with("claude-")
        || model.starts_with("chronos:")
        || ["opus", "sonnet", "haiku"].contains(&model)
    {
        Family::Anthropic
    } else {
        Family::Unknown
    }
}
pub mod progress;
