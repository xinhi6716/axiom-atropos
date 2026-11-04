use crate::base::{self, Failure, Res};
use regex::Regex;
use rusqlite::{params, params_from_iter, Connection, Row};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::HashSet,
    io::Write,
    path::{Component, Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS entries (
    id INTEGER PRIMARY KEY, project TEXT NOT NULL, unit TEXT NOT NULL, title TEXT NOT NULL, summary TEXT NOT NULL,
    body TEXT NOT NULL, symbols TEXT NOT NULL, wish TEXT NOT NULL, files TEXT NOT NULL, project_dir TEXT NOT NULL, created REAL NOT NULL);
CREATE VIRTUAL TABLE IF NOT EXISTS entries_fts USING fts5(
    title, summary, body, symbols, wish, content='entries', content_rowid='id', tokenize='trigram');
CREATE TRIGGER IF NOT EXISTS entries_ai AFTER INSERT ON entries BEGIN
    INSERT INTO entries_fts(rowid, title, summary, body, symbols, wish)
    VALUES (new.id, new.title, new.summary, new.body, new.symbols, new.wish);
END;
";
fn err(e: impl std::fmt::Display) -> Failure {
    Failure::new("io", e.to_string())
}

pub fn query_terms(query: &str) -> Vec<String> {
    let word = Regex::new(r"[A-Za-z0-9_./-]{2,}").expect("constant regex");
    let cjk = Regex::new(r"[\u{3040}-\u{30ff}\u{3400}-\u{9fff}\u{f900}-\u{faff}]+")
        .expect("constant regex");
    let mut terms: Vec<String> = word
        .find_iter(query)
        .map(|m| m.as_str().to_ascii_lowercase())
        .collect();
    for run in cjk.find_iter(query) {
        let chars: Vec<char> = run.as_str().chars().collect();
        if chars.len() == 1 {
            terms.push(chars[0].to_string());
        } else {
            terms.extend(chars.windows(2).map(|w| w.iter().collect()));
        }
    }
    let mut seen = HashSet::new();
    terms.retain(|s| seen.insert(s.clone()));
    terms
}

pub struct Record {
    pub project: String,
    pub project_dir: String,
    pub unit: String,
    pub title: String,
    pub summary: String,
    pub body: String,
    pub symbols: Vec<String>,
    pub files: Vec<String>,
    pub wish: String,
}
pub struct Memory {
    root: PathBuf,
    db_path: PathBuf,
}
impl Memory {
    pub fn open(home: &Path) -> Res<Self> {
        let root = home.join("memory");
        std::fs::create_dir_all(&root).map_err(err)?;
        let m = Self {
            db_path: root.join("memory.db"),
            root,
        };
        m.connect()?.execute_batch(SCHEMA).map_err(err)?;
        Ok(m)
    }
    fn connect(&self) -> Res<Connection> {
        Connection::open(&self.db_path).map_err(err)
    }
    pub fn record(&self, args: &Record) -> Res<Value> {
        base::require_slug("project", &args.project)?;
        for (name, value) in [
            ("unit", &args.unit),
            ("title", &args.title),
            ("summary", &args.summary),
            ("body", &args.body),
            ("wish", &args.wish),
        ] {
            if value.trim().is_empty() {
                return Err(Failure::new(
                    "invalid_argument",
                    format!("{name} is required"),
                ));
            }
        }
        let root = Path::new(&args.project_dir);
        if !root.is_absolute() || !root.is_dir() {
            return Err(Failure::new(
                "invalid_argument",
                format!(
                    "project_dir must be an existing absolute directory: {:?}",
                    args.project_dir
                ),
            ));
        }
        let mut files = Vec::new();
        for rel in &args.files {
            let relative = Path::new(rel);
            if relative.is_absolute()
                || relative
                    .components()
                    .any(|c| matches!(c, Component::ParentDir))
            {
                return Err(Failure::new(
                    "invalid_argument",
                    format!("invalid relative file path: {rel}"),
                ));
            }
            let path = root.join(rel);
            if !path.is_file() {
                return Err(Failure::new(
                    "invalid_argument",
                    format!("file not found in project_dir: {rel}"),
                ));
            }
            files.push(json!({"path": rel, "sha256": hash(&path)?}));
        }
        let mut db = self.connect()?;
        let tx = db
            .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
            .map_err(err)?;
        let created = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();
        tx.execute("INSERT INTO entries (project, unit, title, summary, body, symbols, wish, files, project_dir, created) VALUES (?,?,?,?,?,?,?,?,?,?)", params![args.project, args.unit, args.title.trim(), args.summary.trim(), args.body.trim(), args.symbols.join(" "), args.wish.trim(), serde_json::to_string(&files).map_err(err)?, args.project_dir, created]).map_err(err)?;
        let id = tx.last_insert_rowid();
        tx.commit().map_err(err)?;
        Ok(json!({"status":"completed", "id":id, "files":files, "error":null}))
    }
    pub fn search(&self, query: &str, project: Option<&str>, limit: i64) -> Res<Value> {
        let terms = query_terms(query);
        if terms.is_empty() {
            return Err(Failure::new(
                "invalid_argument",
                "query has no searchable terms",
            ));
        }
        let hit = "(f.title LIKE ? OR f.summary LIKE ? OR f.body LIKE ? OR f.symbols LIKE ? OR f.wish LIKE ?)";
        let hits = vec![hit; terms.len()].join(" + ");
        let filter = project.filter(|p| !p.is_empty());
        let sql = format!("SELECT * FROM (SELECT e.*, {hits} AS matched FROM entries e JOIN entries_fts f ON f.rowid = e.id) WHERE matched > 0{} ORDER BY matched DESC, created DESC LIMIT ?", if filter.is_some() { " AND project = ?" } else { "" });
        let mut values: Vec<rusqlite::types::Value> = terms
            .iter()
            .flat_map(|t| (0..5).map(move |_| format!("%{t}%").into()))
            .collect();
        if let Some(p) = filter {
            values.push(p.to_string().into());
        }
        values.push(limit.into());
        let db = self.connect()?;
        let mut stmt = db.prepare(&sql).map_err(err)?;
        let mut rows = stmt.query(params_from_iter(values)).map_err(err)?;
        let mut entries = Vec::new();
        while let Some(row) = rows.next().map_err(err)? {
            entries.push(json!({"id":row.get::<_, i64>("id").map_err(err)?, "project":row.get::<_, String>("project").map_err(err)?, "unit":row.get::<_, String>("unit").map_err(err)?, "title":row.get::<_, String>("title").map_err(err)?, "summary":row.get::<_, String>("summary").map_err(err)?, "matched_terms":row.get::<_, i64>("matched").map_err(err)?, "stale":stale(row)?}));
        }
        Ok(json!({"status":"completed", "terms":terms, "entries":entries, "error":null}))
    }
    pub fn get(&self, id: i64) -> Res<Value> {
        let db = self.connect()?;
        let mut stmt = db
            .prepare("SELECT * FROM entries WHERE id = ?")
            .map_err(err)?;
        let mut rows = stmt.query([id]).map_err(err)?;
        let row = rows
            .next()
            .map_err(err)?
            .ok_or_else(|| Failure::new("not_found", format!("no memory entry {id}")))?;
        let files: String = row.get("files").map_err(err)?;
        let symbols: String = row.get("symbols").map_err(err)?;
        let entry = json!({"id":id, "project":row.get::<_, String>("project").map_err(err)?, "unit":row.get::<_, String>("unit").map_err(err)?, "title":row.get::<_, String>("title").map_err(err)?, "summary":row.get::<_, String>("summary").map_err(err)?, "body":row.get::<_, String>("body").map_err(err)?, "symbols":symbols.split_whitespace().collect::<Vec<_>>(), "wish":row.get::<_, String>("wish").map_err(err)?, "files":serde_json::from_str::<Value>(&files).map_err(err)?, "project_dir":row.get::<_, String>("project_dir").map_err(err)?, "created":row.get::<_, f64>("created").map_err(err)?, "stale":stale(row)?});
        Ok(json!({"status":"completed", "entry":entry, "error":null}))
    }
    pub fn digest(&self, project: &str, add: Option<&str>) -> Res<Value> {
        base::require_slug("project", project)?;
        let path = self.root.join(format!("{project}.md"));
        if let Some(add) = add.filter(|s| !s.is_empty()) {
            let line = add.split_whitespace().collect::<Vec<_>>().join(" ");
            let mut f = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&path)
                .map_err(err)?;
            writeln!(f, "- {} {line}", base::today()).map_err(err)?;
        }
        let digest = match std::fs::read_to_string(path) {
            Ok(s) => s.replace("\r\n", "\n"),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => String::new(),
            Err(e) => return Err(err(e)),
        };
        Ok(json!({"status":"completed", "digest":digest, "error":null}))
    }
}

fn hash(path: &Path) -> Res<String> {
    Ok(format!(
        "{:x}",
        Sha256::digest(std::fs::read(path).map_err(err)?)
    ))
}
fn stale(row: &Row<'_>) -> Res<Option<bool>> {
    let root: String = row.get("project_dir").map_err(err)?;
    let root = Path::new(&root);
    if !root.is_dir() {
        return Ok(None);
    }
    let files: String = row.get("files").map_err(err)?;
    let files: Vec<Value> = serde_json::from_str(&files).map_err(err)?;
    for item in files {
        let path = item
            .get("path")
            .and_then(Value::as_str)
            .ok_or_else(|| err("invalid file path"))?;
        let expected = item
            .get("sha256")
            .and_then(Value::as_str)
            .ok_or_else(|| err("invalid file hash"))?;
        let path = root.join(path);
        if !path.is_file() || hash(&path)? != expected {
            return Ok(Some(true));
        }
    }
    Ok(Some(false))
}
