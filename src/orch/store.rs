use crate::base::{self, Failure, Res};
use crate::orch::{parse_brief, Brief};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::Write,
    path::{Path, PathBuf},
    sync::{Arc, Mutex, OnceLock},
};

const HEADER: &str = "id\ttrack\tstate\tbranch\tpr\tsha\tbrief";
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unit {
    pub id: String,
    pub track: String,
    pub state: String,
    pub branch: String,
    pub pr: String,
    pub sha: String,
    pub brief: String,
}
#[derive(Debug, Clone)]
pub struct Output {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}
pub fn store_for(home: &Path, project: &str) -> Res<PathBuf> {
    base::require_slug("project", project)?;
    Ok(home.join("orchestrate").join(project))
}
fn io(e: std::io::Error) -> Failure {
    Failure::new("io", e.to_string())
}
fn fail(message: impl Into<String>) -> Failure {
    Failure::new("invalid_argument", message)
}
fn clean(s: &str) -> String {
    let s = s.replace(['\t', '\r', '\n'], " ");
    if s.starts_with(['=', '+', '-', '@']) {
        format!("'{s}")
    } else {
        s
    }
}
fn required(s: &str, name: &str) -> Res<String> {
    let s = clean(s);
    if s.trim().is_empty() {
        Err(fail(format!("{name} must not be empty")))
    } else {
        Ok(s)
    }
}
fn read_file(store: &Path, name: &str) -> Res<String> {
    std::fs::read_to_string(store.join(name)).map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            fail(format!(
                "store is not initialized at {}; run orch init",
                store.display()
            ))
        } else {
            io(e)
        }
    })
}
fn write(store: &Path, name: &str, text: &str) -> Res<()> {
    base::atomic_write(&store.join(name), text.as_bytes()).map_err(io)
}
pub fn read_units(store: &Path) -> Res<Vec<Unit>> {
    let text = read_file(store, "units.tsv")?.replace('\r', "");
    let mut lines = text.split('\n');
    if lines.next() != Some(HEADER) {
        return Err(fail("units.tsv has an invalid header"));
    }
    lines
        .filter(|l| !l.is_empty())
        .map(|l| {
            let c: Vec<_> = l.split('\t').collect();
            if c.len() != 7 {
                return Err(fail("units.tsv has a malformed row"));
            }
            Ok(Unit {
                id: c[0].into(),
                track: c[1].into(),
                state: c[2].into(),
                branch: c[3].into(),
                pr: c[4].into(),
                sha: c[5].into(),
                brief: c[6].into(),
            })
        })
        .collect()
}
fn unit_line(u: &Unit) -> String {
    [
        &u.id, &u.track, &u.state, &u.branch, &u.pr, &u.sha, &u.brief,
    ]
    .iter()
    .map(|s| clean(s))
    .collect::<Vec<_>>()
    .join("\t")
}
fn save_units(store: &Path, rows: &[Unit]) -> Res<()> {
    let mut s = format!("{HEADER}\n");
    for row in rows {
        s.push_str(&unit_line(row));
        s.push('\n');
    }
    write(store, "units.tsv", &s)
}
pub fn unit_brief(store: &Path, unit: &str) -> Res<Brief> {
    let row = read_units(store)?
        .into_iter()
        .find(|u| u.id == unit)
        .ok_or_else(|| Failure::new("not_found", format!("unit {unit} not found")))?;
    if row.brief.is_empty() {
        return Err(Failure::new(
            "no_brief",
            format!("unit {unit} has no brief"),
        ));
    }
    let text = std::fs::read_to_string(&row.brief).map_err(io)?;
    parse_brief(&text).map_err(|field| fail(format!("brief {} is missing {field}", row.brief)))
}
static LOCKS: OnceLock<Mutex<HashMap<PathBuf, Arc<Mutex<()>>>>> = OnceLock::new();
fn locked<T>(store: &Path, force: bool, note: &mut String, f: impl FnOnce() -> Res<T>) -> Res<T> {
    let key = std::fs::canonicalize(store).unwrap_or_else(|_| store.to_path_buf());
    let m = LOCKS
        .get_or_init(|| Mutex::new(HashMap::new()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .entry(key)
        .or_insert_with(|| Arc::new(Mutex::new(())))
        .clone();
    let _guard = m.lock().unwrap_or_else(|e| e.into_inner());
    let path = store.join(".orch.lock");
    let pid = std::process::id().to_string();
    let create = || -> std::io::Result<()> {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)?;
        writeln!(file, "{pid}")
    };
    if let Err(e) = create() {
        if e.kind() != std::io::ErrorKind::AlreadyExists {
            return Err(io(e));
        }
        let holder = std::fs::read_to_string(&path).unwrap_or_default();
        let holder = if holder.trim().is_empty() {
            "unknown"
        } else {
            holder.trim()
        };
        let dead = holder
            .parse::<u32>()
            .ok()
            .filter(|p| *p > 0 && p.to_string() == holder)
            .is_some_and(|p| p > i32::MAX as u32 || !base::proc::pid_alive(p));
        if dead {
            note.push_str(&format!(
                "replacing stale store lock (pid {holder} is dead)\n"
            ));
        } else if force {
            note.push_str(&format!("stealing store lock held by pid {holder}\n"));
        } else {
            return Err(fail(format!("store lock held by pid {holder}")));
        }
        std::fs::remove_file(&path).map_err(io)?;
        create().map_err(io)?;
    }
    struct Guard {
        path: PathBuf,
        pid: String,
    }
    impl Drop for Guard {
        fn drop(&mut self) {
            if std::fs::read_to_string(&self.path).is_ok_and(|s| s.trim() == self.pid) {
                let _ = std::fs::remove_file(&self.path);
            }
        }
    }
    let _file_guard = Guard { path, pid };
    f()
}
pub fn with_lock<T>(store: &Path, force: bool, f: impl FnOnce() -> Res<T>) -> Res<T> {
    locked(store, force, &mut String::new(), f)
}
pub fn set_unit_state(store: &Path, unit: &str, state: &str) -> Res<()> {
    with_lock(store, false, || {
        let mut rows = read_units(store)?;
        let row = rows
            .iter_mut()
            .find(|u| u.id == unit)
            .ok_or_else(|| Failure::new("not_found", format!("unit {unit} not found")))?;
        row.state = required(state, "state")?;
        save_units(store, &rows)
    })
}
fn push(
    store: &Path,
    agent: &str,
    unit: &str,
    status: &str,
    report: Option<&str>,
) -> Res<(serde_json::Value, String)> {
    let ts = base::now_iso();
    let a = required(agent, "agent")?;
    let u = required(unit, "unit")?;
    let s = required(status, "status")?;
    let r = report
        .map(|x| required(x, "report"))
        .transpose()?
        .unwrap_or_default();
    let filename = format!(
        "{}-{}-{}.tsv",
        ts.replace([':', '.'], "-"),
        std::process::id(),
        base::unique()
    );
    write(
        store,
        &format!("inbox/{filename}"),
        &format!(
            "{}\t{}\t{}\t{}\t{}\n",
            clean(&ts),
            clean(&a),
            clean(&u),
            clean(&s),
            clean(&r)
        ),
    )?;
    Ok((
        serde_json::json!({"ts":ts,"agent":a,"unit":u,"status":s,"report":r}),
        filename,
    ))
}
pub fn append_inbox(
    store: &Path,
    agent: &str,
    unit: &str,
    status: &str,
    report: Option<&str>,
) -> Res<()> {
    with_lock(store, false, || {
        push(store, agent, unit, status, report)?;
        Ok(())
    })
}
fn option<'a>(args: &'a [String], name: &str) -> Res<Option<&'a str>> {
    match args.iter().position(|x| x == name) {
        Some(i) => Ok(Some(
            args.get(i + 1)
                .ok_or_else(|| fail(format!("{name} requires a value")))?,
        )),
        None => Ok(None),
    }
}
fn arg(args: &[String], i: usize) -> Res<&str> {
    args.get(i)
        .map(String::as_str)
        .ok_or_else(|| fail("a valid command is required"))
}
fn emit(value: serde_json::Value, compact: String, json: bool) -> String {
    let mut s = if json {
        serde_json::to_string_pretty(&value).unwrap_or_default()
    } else {
        compact
    };
    if !s.ends_with('\n') {
        s.push('\n');
    }
    s
}

const LEDGER_HEADER: &str = "pr\tsha\tverdict\tevidence\tverifier\tts";
const VERDICTS: [&str; 5] = [
    "live-ui-verified",
    "unit-test-verified",
    "type-check-only",
    "verifier-blocked",
    "verifier-failed",
];
fn line(s: &str, label: &str) -> Res<String> {
    let v = s.replace(['\r', '\n'], " ").trim().to_string();
    if v.is_empty() {
        Err(fail(format!("{label} must not be empty")))
    } else {
        Ok(v)
    }
}
fn required_option(args: &[String], name: &str) -> Res<String> {
    Ok(option(args, name)?
        .ok_or_else(|| fail(format!("{name} is required")))?
        .to_string())
}
fn pr(s: &str) -> Res<String> {
    if s.starts_with('0')
        || !s.bytes().all(|c| c.is_ascii_digit())
        || s.parse::<u64>()
            .ok()
            .filter(|n| *n > 0 && *n <= 9007199254740991)
            .is_none()
    {
        return Err(fail("PR must be a positive integer"));
    }
    Ok(s.to_string())
}
fn ledger(store: &Path) -> Res<Vec<serde_json::Value>> {
    let text = read_file(store, "ledger.tsv")?.replace('\r', "");
    let mut lines = text.split('\n');
    if lines.next() != Some(LEDGER_HEADER) {
        return Err(fail("ledger.tsv has an invalid header"));
    }
    lines.filter(|x| !x.is_empty()).map(|x| {
        let c: Vec<_> = x.split('\t').collect();
        if c.len()!=6 { return Err(fail("ledger.tsv has a malformed row")); }
        if !VERDICTS.contains(&c[2]) { return Err(fail(format!("ledger.tsv has invalid verdict {}",c[2]))); }
        Ok(serde_json::json!({"pr":c[0],"sha":c[1],"verdict":c[2],"evidence":c[3],"verifier":c[4],"ts":c[5]}))
    }).collect()
}
fn cell(v: &serde_json::Value, k: &str) -> String {
    v[k].as_str().unwrap_or("").to_string()
}
fn save_ledger(store: &Path, rows: &[serde_json::Value]) -> Res<()> {
    let mut out = format!("{LEDGER_HEADER}\n");
    for r in rows {
        out.push_str(
            &["pr", "sha", "verdict", "evidence", "verifier", "ts"]
                .iter()
                .map(|k| clean(&cell(r, k)))
                .collect::<Vec<_>>()
                .join("\t"),
        );
        out.push('\n');
    }
    write(store, "ledger.tsv", &out)
}
fn counts(vals: impl Iterator<Item = String>) -> serde_json::Value {
    let mut m = std::collections::BTreeMap::<String, usize>::new();
    for x in vals {
        *m.entry(x).or_default() += 1;
    }
    serde_json::json!(m)
}
fn count_line(v: &serde_json::Value) -> String {
    let xs: Vec<_> = v
        .as_object()
        .into_iter()
        .flatten()
        .map(|(k, v)| format!("{k}={v}"))
        .collect();
    if xs.is_empty() {
        "none".into()
    } else {
        xs.join(", ")
    }
}
fn pointers(store: &Path) -> Res<Vec<serde_json::Value>> {
    let dir = store.join("inbox");
    let mut files = std::fs::read_dir(&dir)
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                fail(format!(
                    "store is not initialized at {}; run orch init",
                    store.display()
                ))
            } else {
                io(e)
            }
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(io)?;
    files.sort_by_key(|e| e.file_name());
    let mut rows = Vec::new();
    for f in files {
        if !f.file_type().map_err(io)?.is_file()
            || !f.file_name().to_string_lossy().ends_with(".tsv")
        {
            continue;
        }
        let mut raw = std::fs::read_to_string(f.path()).map_err(io)?;
        if raw.ends_with('\n') {
            raw.pop();
            if raw.ends_with('\r') {
                raw.pop();
            }
        }
        let c: Vec<_> = raw.split('\t').collect();
        if c.len() != 5 || raw.contains(['\r', '\n']) {
            return Err(fail(format!(
                "inbox pointer {} is malformed",
                f.file_name().to_string_lossy()
            )));
        }
        rows.push(
            serde_json::json!({"ts":c[0],"agent":c[1],"unit":c[2],"status":c[3],"report":c[4]}),
        );
    }
    Ok(rows)
}
fn gates(store: &Path) -> Res<Vec<serde_json::Value>> {
    let raw = read_file(store, "gates.md")?.replace('\r', "");
    let raw = raw.trim();
    if raw.is_empty() {
        return Ok(vec![]);
    }
    let rest = raw
        .strip_prefix("# Gates\n\n## ")
        .ok_or_else(|| fail("gates.md has an invalid heading"))?;
    let mut rows = Vec::new();
    for block in rest.split("\n\n## ") {
        let mut lines = block.lines().filter(|x| !x.is_empty());
        let id = lines.next().unwrap_or("");
        let mut f = std::collections::HashMap::new();
        for l in lines {
            let (k, v) = l
                .strip_prefix("- ")
                .and_then(|x| x.split_once(": "))
                .ok_or_else(|| fail(format!("gates.md has a malformed gate {id}")))?;
            f.insert(k, v);
        }
        let (Some(q), Some(o), Some(d)) = (f.get("Question"), f.get("Options"), f.get("Default"))
        else {
            return Err(fail(format!("gates.md has a malformed gate {id}")));
        };
        if id.is_empty() {
            return Err(fail(format!("gates.md has a malformed gate {id}")));
        }
        let status = *f.get("Status").unwrap_or(&"");
        let mut row =
            serde_json::json!({"kind":status,"id":id,"question":q,"options":o,"defaultAnswer":d});
        if status == "resolved" && f.contains_key("Answer") {
            row["answer"] = serde_json::json!(f["Answer"])
        } else if status != "open" {
            return Err(fail(format!("gates.md has invalid status {status}")));
        }
        if rows.iter().any(|v| cell(v, "id") == id) {
            return Err(fail("gates.md has duplicate gate ids"));
        }
        rows.push(row);
    }
    Ok(rows)
}
fn save_gates(store: &Path, rows: &[serde_json::Value]) -> Res<()> {
    if rows.is_empty() {
        return write(store, "gates.md", "");
    }
    let blocks: Vec<_> = rows
        .iter()
        .map(|g| {
            let mut x = format!(
                "## {}\n\n- Status: {}\n- Question: {}\n- Options: {}\n- Default: {}",
                cell(g, "id"),
                cell(g, "kind"),
                cell(g, "question"),
                cell(g, "options"),
                cell(g, "defaultAnswer")
            );
            if cell(g, "kind") == "resolved" {
                x.push_str(&format!("\n- Answer: {}", cell(g, "answer")))
            }
            x
        })
        .collect();
    write(
        store,
        "gates.md",
        &format!("# Gates\n\n{}\n", blocks.join("\n\n")),
    )
}
fn standing(store: &Path) -> Res<Vec<serde_json::Value>> {
    let raw = read_file(store, "preferences.md")?.replace('\r', "");
    let mut rows = Vec::new();
    for l in raw.lines().filter(|x| !x.is_empty()) {
        let (n, v) = l
            .split_once(". ")
            .ok_or_else(|| fail("preferences.md has malformed numbering"))?;
        if n != format!("{}", rows.len() + 1) || v.is_empty() {
            return Err(fail("preferences.md has malformed numbering"));
        }
        rows.push(serde_json::json!({"number":rows.len()+1,"line":v}));
    }
    Ok(rows)
}
fn frontier(store: &Path) -> Res<serde_json::Value> {
    let raw = read_file(store, "frontier.json")?;
    let v: serde_json::Value =
        serde_json::from_str(&raw).map_err(|_| fail("frontier.json is not valid JSON"))?;
    let obj = v
        .as_object()
        .ok_or_else(|| fail("frontier.json must contain an object"))?;
    if obj.is_empty() {
        return Ok(serde_json::json!({"generation":0,"prs":[],"lowestUnmerged":null}));
    }
    if v["generation"]
        .as_u64()
        .filter(|x| *x <= 9007199254740991)
        .is_none()
        || !v["prs"].is_array()
        || !(v["lowestUnmerged"].is_null() || v["lowestUnmerged"].as_i64().is_some())
    {
        return Err(fail("frontier.json has an invalid shape"));
    }
    for r in v["prs"].as_array().unwrap() {
        if r["pr"]
            .as_u64()
            .filter(|x| *x > 0 && *x <= 9007199254740991)
            .is_none()
            || r["branches"].as_str().is_none_or(|s| s.is_empty())
            || !r["sha"].is_string()
            || !["OPEN", "MERGED", "CLOSED"].contains(&r["state"].as_str().unwrap_or(""))
        {
            return Err(fail("frontier.json has an invalid PR row"));
        }
    }
    Ok(
        serde_json::json!({"generation":v["generation"],"prs":v["prs"].as_array().unwrap().iter().map(|r|serde_json::json!({"pr":r["pr"],"branches":r["branches"],"sha":r["sha"],"state":r["state"]})).collect::<Vec<_>>(),"lowestUnmerged":v["lowestUnmerged"]}),
    )
}
fn rows_display(
    rows: &[serde_json::Value],
    f: impl Fn(&serde_json::Value) -> String,
    empty: &str,
    limit: Option<usize>,
) -> String {
    if rows.is_empty() {
        return empty.into();
    }
    let n = limit.unwrap_or(rows.len());
    let mut out: Vec<_> = rows.iter().take(n).map(f).collect();
    if rows.len() > n {
        out.push(format!("... {} more; use --json", rows.len() - n))
    }
    out.join("\n")
}

fn table(headers: &[&str], rows: Vec<Vec<String>>) -> String {
    if rows.is_empty() {
        return "(none)".into();
    }
    let mut lines = vec![
        format!("| {} |", headers.join(" | ")),
        format!(
            "| {} |",
            headers
                .iter()
                .map(|_| "---")
                .collect::<Vec<_>>()
                .join(" | ")
        ),
    ];
    for r in rows {
        lines.push(format!(
            "| {} |",
            r.iter()
                .map(|s| s.replace('\\', "\\\\").replace('|', "\\|"))
                .collect::<Vec<_>>()
                .join(" | ")
        ))
    }
    lines.join("\n")
}
fn status(store: &Path) -> Res<(serde_json::Value, String)> {
    let units = read_units(store)?;
    let ledger = ledger(store)?;
    let frontier = frontier(store)?;
    let gates = gates(store)?;
    let states = counts(units.iter().map(|r| r.state.clone()));
    let verdicts = counts(ledger.iter().map(|r| cell(r, "verdict")));
    let mut ids: Vec<_> = gates
        .iter()
        .filter(|r| cell(r, "kind") == "open")
        .map(|r| cell(r, "id"))
        .collect();
    ids.sort();
    let summary = serde_json::json!({"unitStates":states,"ledgerVerdicts":verdicts,"frontierGeneration":frontier["generation"],"openGateIds":ids});
    let previous = std::fs::read_to_string(store.join("status.md"))
        .ok()
        .and_then(|x| {
            x.split("<!-- orch-summary ")
                .nth(1)
                .and_then(|x| x.split(" -->").next())
                .and_then(|x| serde_json::from_str::<serde_json::Value>(x).ok())
        })
        .filter(|v| {
            v["unitStates"]
                .as_object()
                .is_some_and(|m| m.values().all(|v| v.as_u64().is_some()))
                && v["ledgerVerdicts"]
                    .as_object()
                    .is_some_and(|m| m.values().all(|v| v.as_u64().is_some()))
                && v["frontierGeneration"].is_number()
                && v["openGateIds"]
                    .as_array()
                    .is_some_and(|a| a.iter().all(|v| v.is_string()))
        });
    let changed = if let Some(old) = previous {
        let mut changes = Vec::new();
        for (key, label) in [("unitStates", "units"), ("ledgerVerdicts", "ledger")] {
            let a = old[key].as_object().unwrap();
            let b = summary[key].as_object().unwrap();
            let names: std::collections::BTreeSet<_> = a.keys().chain(b.keys()).collect();
            for n in names {
                let x = a.get(n).and_then(|v| v.as_u64()).unwrap_or(0);
                let y = b.get(n).and_then(|v| v.as_u64()).unwrap_or(0);
                if x != y {
                    changes.push(format!("{label} {n} {x}->{y}"))
                }
            }
        }
        if old["frontierGeneration"] != summary["frontierGeneration"] {
            changes.push(format!(
                "frontier generation {}->{}",
                old["frontierGeneration"], summary["frontierGeneration"]
            ))
        }
        if old["openGateIds"] != summary["openGateIds"] {
            changes.push(format!(
                "open gates {}->{}",
                old["openGateIds"].as_array().unwrap().len(),
                ids.len()
            ))
        }
        if changes.is_empty() {
            "no derived changes".into()
        } else {
            changes.join("; ")
        }
    } else {
        "first render".into()
    };
    let unit_table = table(
        &["ID", "Track", "State", "Branch", "PR", "SHA", "Brief"],
        units
            .iter()
            .map(|u| {
                vec![
                    u.id.clone(),
                    u.track.clone(),
                    u.state.clone(),
                    u.branch.clone(),
                    u.pr.clone(),
                    u.sha.clone(),
                    u.brief.clone(),
                ]
            })
            .collect(),
    );
    let ledger_table = table(
        &["PR", "SHA", "Verdict", "Evidence", "Verifier", "Timestamp"],
        ledger
            .iter()
            .map(|r| {
                ["pr", "sha", "verdict", "evidence", "verifier", "ts"]
                    .iter()
                    .map(|k| cell(r, k))
                    .collect()
            })
            .collect(),
    );
    let frontier_table = table(
        &["Branch", "PR", "SHA", "State"],
        frontier["prs"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| {
                vec![
                    cell(r, "branches"),
                    r["pr"].to_string(),
                    cell(r, "sha"),
                    cell(r, "state"),
                ]
            })
            .collect(),
    );
    let gates_table = table(
        &["ID", "Status", "Question", "Options", "Default", "Answer"],
        gates
            .iter()
            .map(|r| {
                [
                    "id",
                    "kind",
                    "question",
                    "options",
                    "defaultAnswer",
                    "answer",
                ]
                .iter()
                .map(|k| cell(r, k))
                .collect()
            })
            .collect(),
    );
    let lowest = if frontier["lowestUnmerged"].is_null() {
        "none".into()
    } else {
        frontier["lowestUnmerged"].to_string()
    };
    let markdown=format!("# Orchestrate status\n\nGenerated: {}\n\n## Units\n\nStates: {}\n\n{}\n\n## Verification ledger\n\nVerdicts: {}\n\n{}\n\n## Frontier\n\nGeneration: {}\nLowest unmerged: {}\n\n{}\n\n## Gates\n\n{}\n\n<!-- orch-summary {} -->\n",base::now_iso(),count_line(&states),unit_table,count_line(&verdicts),ledger_table,frontier["generation"],lowest,frontier_table,gates_table,summary);
    write(store, "status.md", &markdown)?;
    let suffix = if ids.is_empty() {
        String::new()
    } else {
        format!(
            "; ids={}{}",
            ids.iter().take(4).cloned().collect::<Vec<_>>().join(","),
            if ids.len() > 4 {
                format!(",+{} more", ids.len() - 4)
            } else {
                String::new()
            }
        )
    };
    let compact = format!(
        "counts: units={}; states={}; ledger={}\nchanged: {}\ngates open: {}{}",
        units.len(),
        count_line(&states),
        count_line(&verdicts),
        changed,
        ids.len(),
        suffix
    );
    Ok((
        serde_json::json!({"units":units,"ledger":ledger,"frontier":frontier,"gates":gates,"summary":summary,"changed":changed}),
        compact,
    ))
}
pub fn run(store: &Path, args: &[String], json: bool, force: bool) -> Output {
    let mut note = String::new();
    let mut ledger_missing = false;
    let result: Res<String> = (|| {
        let cmd = arg(args, 0)?;
        let sub = args.get(1).map(String::as_str).unwrap_or("");
        match (cmd, sub) {
            ("init", _) => {
                std::fs::create_dir_all(store).map_err(io)?;
                locked(store, force, &mut note, || {
                    std::fs::create_dir_all(store.join("inbox")).map_err(io)?;
                    for (name, content) in [
                        ("units.tsv", format!("{HEADER}\n")),
                        (
                            "ledger.tsv",
                            "pr\tsha\tverdict\tevidence\tverifier\tts\n".into(),
                        ),
                        ("gates.md", String::new()),
                        ("preferences.md", String::new()),
                        ("frontier.json", "{}\n".into()),
                    ] {
                        if !store.join(name).exists() {
                            write(store, name, &content)?;
                        }
                    }
                    Ok(())
                })?;
                let path = store.to_string_lossy().to_string();
                Ok(emit(
                    serde_json::json!({"store":path}),
                    format!("initialized {path}"),
                    json,
                ))
            }
            ("unit", "add") => {
                let id = required(arg(args, 2)?, "unit id")?;
                let track = required(
                    option(args, "--track")?.ok_or_else(|| fail("--track is required"))?,
                    "track",
                )?;
                let brief = option(args, "--brief")?
                    .map(|s| required(s, "brief"))
                    .transpose()?
                    .unwrap_or_default();
                let row = Unit {
                    id,
                    track,
                    state: "pending".into(),
                    branch: String::new(),
                    pr: String::new(),
                    sha: String::new(),
                    brief,
                };
                locked(store, force, &mut note, || {
                    let mut rows = read_units(store)?;
                    if rows.iter().any(|u| u.id == row.id) {
                        return Err(fail(format!("unit {} already exists", row.id)));
                    }
                    rows.push(row.clone());
                    save_units(store, &rows)
                })?;
                Ok(emit(
                    serde_json::to_value(&row).unwrap(),
                    unit_line(&row),
                    json,
                ))
            }
            ("unit", "set") => {
                let id = required(arg(args, 2)?, "unit id")?;
                let state = required(
                    option(args, "--state")?.ok_or_else(|| fail("--state is required"))?,
                    "state",
                )?;
                let row = locked(store, force, &mut note, || {
                    let mut rows = read_units(store)?;
                    let u = rows
                        .iter_mut()
                        .find(|u| u.id == id)
                        .ok_or_else(|| Failure::new("not_found", format!("unit {id} not found")))?;
                    u.state = state;
                    if let Some(v) = option(args, "--branch")? {
                        u.branch = required(v, "branch")?;
                    }
                    if let Some(v) = option(args, "--sha")? {
                        u.sha = required(v, "SHA")?;
                    }
                    if let Some(v) = option(args, "--pr")? {
                        if v.parse::<u64>().ok().filter(|n| *n > 0).is_none() {
                            return Err(fail("PR must be a positive integer"));
                        }
                        u.pr = v.into();
                    }
                    let out = u.clone();
                    save_units(store, &rows)?;
                    Ok(out)
                })?;
                Ok(emit(
                    serde_json::to_value(&row).unwrap(),
                    unit_line(&row),
                    json,
                ))
            }
            ("unit", "get") => {
                let id = required(arg(args, 2)?, "unit id")?;
                let row = read_units(store)?
                    .into_iter()
                    .find(|u| u.id == id)
                    .ok_or_else(|| Failure::new("not_found", format!("unit {id} not found")))?;
                Ok(emit(
                    serde_json::to_value(&row).unwrap(),
                    unit_line(&row),
                    json,
                ))
            }
            ("unit", "list") => {
                let state = option(args, "--state")?;
                let track = option(args, "--track")?;
                let rows: Vec<_> = read_units(store)?
                    .into_iter()
                    .filter(|u| {
                        state.is_none_or(|s| u.state == s) && track.is_none_or(|s| u.track == s)
                    })
                    .collect();
                let mut lines: Vec<_> = rows.iter().take(4).map(unit_line).collect();
                if rows.len() > 4 {
                    lines.push(format!("... {} more; use --json", rows.len() - 4));
                }
                let compact = if lines.is_empty() {
                    "(no units)".into()
                } else {
                    lines.join("\n")
                };
                Ok(emit(serde_json::to_value(rows).unwrap(), compact, json))
            }
            ("unit", "counts") => {
                let mut counts = std::collections::BTreeMap::<String, usize>::new();
                for u in read_units(store)? {
                    *counts.entry(u.state).or_default() += 1;
                }
                let compact = if counts.is_empty() {
                    "none".into()
                } else {
                    counts
                        .iter()
                        .map(|(k, v)| format!("{k}={v}"))
                        .collect::<Vec<_>>()
                        .join(", ")
                };
                Ok(emit(serde_json::to_value(counts).unwrap(), compact, json))
            }
            ("inbox", "push") => {
                let (v, name) = locked(store, force, &mut note, || {
                    push(
                        store,
                        arg(args, 2)?,
                        arg(args, 3)?,
                        arg(args, 4)?,
                        option(args, "--report")?,
                    )
                })?;
                Ok(emit(
                    v.clone(),
                    format!(
                        "{}\t{}\t{name}",
                        v["unit"].as_str().unwrap_or(""),
                        v["status"].as_str().unwrap_or("")
                    ),
                    json,
                ))
            }

            ("ledger", "record") => {
                let pr = pr(arg(args, 2)?)?;
                let sha = required(arg(args, 3)?, "SHA")?;
                let verdict = arg(args, 4)?;
                if !VERDICTS.contains(&verdict) {
                    return Err(fail("verdict must be live-ui-verified, unit-test-verified, type-check-only, verifier-blocked, or verifier-failed"));
                }
                let evidence = required(&required_option(args, "--evidence")?, "evidence")?;
                let verifier = option(args, "--verifier")?
                    .map(|s| required(s, "verifier"))
                    .transpose()?
                    .unwrap_or_default();
                let row = serde_json::json!({"pr":pr,"sha":sha,"verdict":verdict,"evidence":evidence,"verifier":verifier,"ts":base::now_iso()});
                locked(store, force, &mut note, || {
                    let mut rows = ledger(store)?;
                    if let Some(i) = rows
                        .iter()
                        .position(|r| r["pr"] == row["pr"] && r["sha"] == row["sha"])
                    {
                        rows[i] = row.clone()
                    } else {
                        rows.push(row.clone())
                    }
                    save_ledger(store, &rows)
                })?;
                Ok(emit(
                    row.clone(),
                    format!(
                        "{}\t{}\t{}",
                        cell(&row, "pr"),
                        cell(&row, "sha"),
                        cell(&row, "verdict")
                    ),
                    json,
                ))
            }
            ("ledger", "check") => {
                let pr = pr(arg(args, 2)?)?;
                let sha = required(arg(args, 3)?, "SHA")?;
                let row = ledger(store)?
                    .into_iter()
                    .find(|r| cell(r, "pr") == pr && cell(r, "sha") == sha);
                match row {
                    Some(r) => Ok(emit(r.clone(), cell(&r, "verdict"), json)),
                    None => {
                        ledger_missing = true;
                        Ok(emit(
                            serde_json::json!({"pr":pr,"sha":sha,"verdict":"NOT-VERIFIED"}),
                            "NOT-VERIFIED".into(),
                            json,
                        ))
                    }
                }
            }
            ("ledger", "summary") => {
                let v = counts(ledger(store)?.iter().map(|r| cell(r, "verdict")));
                Ok(emit(v.clone(), count_line(&v), json))
            }
            ("inbox", "count") => {
                let n = pointers(store)?.len();
                Ok(emit(serde_json::json!({"count":n}), n.to_string(), json))
            }
            ("inbox", "drain") => {
                let peek = args.iter().any(|a| a == "--peek");
                let read = || -> Res<Vec<serde_json::Value>> {
                    let rows = pointers(store)?;
                    if !peek {
                        let dir = store.join("inbox");
                        let tmp = store.join(format!(
                            ".inbox-drain-{}-{}",
                            std::process::id(),
                            base::unique()
                        ));
                        std::fs::rename(&dir, &tmp).map_err(io)?;
                        if let Err(e) = std::fs::create_dir(&dir) {
                            let _ = std::fs::rename(&tmp, &dir);
                            return Err(io(e));
                        }
                        std::fs::remove_dir_all(tmp).map_err(io)?;
                    }
                    Ok(rows)
                };
                let rows = if peek {
                    read()?
                } else {
                    locked(store, force, &mut note, read)?
                };
                let compact = rows_display(
                    &rows,
                    |r| {
                        ["ts", "agent", "unit", "status", "report"]
                            .iter()
                            .map(|k| cell(r, k))
                            .collect::<Vec<_>>()
                            .join("\t")
                    },
                    "(empty)",
                    None,
                );
                Ok(emit(serde_json::json!(rows), compact, json))
            }
            ("gate", "park") => {
                let id = line(arg(args, 2)?, "gate id")?;
                let q = line(&required_option(args, "--question")?, "question")?;
                let o = line(&required_option(args, "--options")?, "options")?;
                let d = line(&required_option(args, "--default")?, "default")?;
                let row = serde_json::json!({"kind":"open","id":id,"question":q,"options":o,"defaultAnswer":d});
                locked(store, force, &mut note, || {
                    let mut rows = gates(store)?;
                    if let Some(i) = rows.iter().position(|r| r["id"] == row["id"]) {
                        rows[i] = row.clone()
                    } else {
                        rows.push(row.clone())
                    }
                    save_gates(store, &rows)
                })?;
                Ok(emit(
                    row.clone(),
                    format!("{}\topen", cell(&row, "id")),
                    json,
                ))
            }
            ("gate", "resolve") => {
                let id = line(arg(args, 2)?, "gate id")?;
                let answer = line(&required_option(args, "--answer")?, "answer")?;
                let row = locked(store, force, &mut note, || {
                    let mut rows = gates(store)?;
                    let r = rows
                        .iter_mut()
                        .find(|r| cell(r, "id") == id)
                        .ok_or_else(|| Failure::new("not_found", format!("gate {id} not found")))?;
                    r["kind"] = serde_json::json!("resolved");
                    r["answer"] = serde_json::json!(answer);
                    let out = r.clone();
                    save_gates(store, &rows)?;
                    Ok(out)
                })?;
                Ok(emit(
                    row.clone(),
                    format!("{}\tresolved\t{}", id, cell(&row, "answer")),
                    json,
                ))
            }
            ("gate", "list") => {
                let rows: Vec<_> = gates(store)?
                    .into_iter()
                    .filter(|r| cell(r, "kind") == "open")
                    .collect();
                let compact = rows_display(
                    &rows,
                    |r| {
                        ["id", "question", "options", "defaultAnswer"]
                            .iter()
                            .map(|k| cell(r, k))
                            .collect::<Vec<_>>()
                            .join("\t")
                    },
                    "(no open gates)",
                    Some(4),
                );
                Ok(emit(serde_json::json!(rows), compact, json))
            }
            ("standing", "show") => {
                let rows = standing(store)?;
                let compact = rows_display(
                    &rows,
                    |r| format!("{}. {}", r["number"], cell(r, "line")),
                    "(no standing orders)",
                    Some(4),
                );
                Ok(emit(serde_json::json!(rows), compact, json))
            }
            ("standing", "add") => {
                let value = line(arg(args, 2)?, "standing order")?;
                let row = locked(store, force, &mut note, || {
                    let mut rows = standing(store)?;
                    let row = serde_json::json!({"number":rows.len()+1,"line":value});
                    rows.push(row.clone());
                    let text = rows
                        .iter()
                        .map(|r| format!("{}. {}\n", r["number"], cell(r, "line")))
                        .collect::<String>();
                    write(store, "preferences.md", &text)?;
                    Ok(row)
                })?;
                Ok(emit(
                    row.clone(),
                    format!("{}. {}", row["number"], cell(&row, "line")),
                    json,
                ))
            }
            ("frontier", "show") => {
                let v = frontier(store)?;
                let prs = v["prs"].as_array().unwrap();
                let list = if prs.is_empty() {
                    "none".into()
                } else {
                    prs.iter()
                        .map(|r| {
                            format!(
                                "{}#{}@{}:{}",
                                cell(r, "branches"),
                                r["pr"],
                                cell(r, "sha"),
                                cell(r, "state")
                            )
                        })
                        .collect::<Vec<_>>()
                        .join(",")
                };
                let lowest = if v["lowestUnmerged"].is_null() {
                    "none".into()
                } else {
                    v["lowestUnmerged"].to_string()
                };
                Ok(emit(
                    v.clone(),
                    format!(
                        "generation={} prs={} lowest-unmerged={}",
                        v["generation"], list, lowest
                    ),
                    json,
                ))
            }
            ("status", _) => {
                let (v, compact) = locked(store, force, &mut note, || status(store))?;
                Ok(emit(v, compact, json))
            }
            ("frontier", "set") => {
                let repo = option(args, "--repo")?
                    .map(str::to_owned)
                    .or_else(|| std::env::var("ORCH_REPO").ok())
                    .ok_or_else(|| fail("--repo or ORCH_REPO is required"))?;
                let prs = crate::helpers::frontier(
                    Path::new(&repo),
                    option(args, "--forge")?.unwrap_or("gh"),
                    option(args, "--prs")?,
                )?;
                let value = locked(store, force, &mut note, || {
                    let old = frontier(store)?;
                    let lowest = prs
                        .iter()
                        .find(|p| p["state"] == "OPEN")
                        .map(|p| p["pr"].clone())
                        .unwrap_or(serde_json::Value::Null);
                    let v = serde_json::json!({"generation":old["generation"].as_u64().unwrap_or(0)+1,"prs":prs,"lowestUnmerged":lowest});
                    write(
                        store,
                        "frontier.json",
                        &format!("{}\n", serde_json::to_string_pretty(&v).unwrap()),
                    )?;
                    Ok(v)
                })?;
                Ok(emit(value.clone(), value.to_string(), json))
            }
            ("map", _) => {
                let text = std::fs::read_to_string(store.join("modules.md")).map_err(io)?;
                let map = crate::orch::parse_map(&text).map_err(fail)?;
                Ok(emit(
                    serde_json::to_value(&map).unwrap(),
                    map.modules
                        .iter()
                        .map(|m| {
                            format!("{}\t{}\t{}", m.id, m.depends.join(","), m.paths.join(","))
                        })
                        .collect::<Vec<_>>()
                        .join("\n"),
                    json,
                ))
            }
            _ => Err(fail("a valid command is required")),
        }
    })();
    match result {
        Ok(stdout) => Output {
            exit_code: if ledger_missing { 2 } else { 0 },
            stdout,
            stderr: note,
        },
        Err(e) => Output {
            exit_code: if e.kind == "not_found" { 2 } else { 1 },
            stdout: String::new(),
            stderr: format!("{note}error: {}\n", e.message),
        },
    }
}
