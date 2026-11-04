use axiom_atropos::{
    codex::{AppServer, RunRequest},
    distribution, install,
    memory::{Memory, Record},
    orch,
};
use serde_json::{json, Value};
use std::{
    fs,
    io::{BufRead, BufReader, Write},
    path::Path,
    process::{Command, Stdio},
    sync::mpsc,
    time::Duration,
};
const EXE: &str = env!("CARGO_BIN_EXE_atropos");
const FAKE: &str = env!("CARGO_BIN_EXE_fake-backend");
fn write(path: impl AsRef<Path>, text: &str) {
    let p = path.as_ref();
    fs::create_dir_all(p.parent().unwrap()).unwrap();
    fs::write(p, text).unwrap();
}
fn request(dir: &Path, brief: &str) -> RunRequest {
    RunRequest {
        brief: brief.into(),
        cwd: dir.to_string_lossy().into(),
        model: "gpt-6.1-terra".into(),
        sandbox: "workspace-write".into(),
        effort: Some("high".into()),
        output_schema: None,
        developer_instructions: None,
        timeout_s: 2,
        lean: true,
        keep_plugins: vec!["keep@test".into()],
        web_search: false,
    }
}
fn server() -> AppServer {
    AppServer::with_request_timeout(vec![FAKE.into()], Some(Duration::from_secs(5)))
}
#[test]
fn codex_reports_events_and_isolates_child_configuration() {
    let d = tempfile::tempdir().unwrap();
    let s = server();
    assert_eq!(s.resolve_model("terra").unwrap(), "gpt-6.1-terra");
    assert!(s.resolve_model("absent").is_err());
    let r = s.run(&request(d.path(), "hello 資料")).unwrap();
    assert_eq!(r["status"], "completed");
    assert_eq!(r["usage"]["totalTokens"], 12);
    assert_eq!(r["commands"][0]["exit_code"], 0);
    assert_eq!(r["file_changes"][0]["path"], "src/lib.rs");
    let echo: Value = serde_json::from_str(r["final_message"].as_str().unwrap()).unwrap();
    assert_eq!(
        echo["thread"]["config"]["plugins"]["panel@test"]["enabled"],
        false
    );
    assert!(echo["thread"]["config"]["plugins"]
        .get("keep@test")
        .is_none());
    assert_eq!(
        echo["thread"]["config"]["mcp_servers"]["node_repl"]["enabled"],
        false
    );
    let mut req = request(d.path(), "default");
    req.model.clear();
    req.lean = false;
    let r = s.run(&req).unwrap();
    let echo: Value = serde_json::from_str(r["final_message"].as_str().unwrap()).unwrap();
    assert!(echo["thread"].get("model").is_none());
    assert!(echo["thread"].get("config").is_none());
}
#[test]
fn codex_schema_errors_approvals_limits_and_reconnection() {
    let d = tempfile::tempdir().unwrap();
    let s = server();
    let mut req = request(d.path(), "structured");
    req.output_schema = Some(
        json!({"type":"object","required":["answer"],"properties":{"answer":{"type":"string"}}}),
    );
    assert_eq!(s.run(&req).unwrap()["structured_output"]["answer"], "ok");
    req.output_schema = Some(json!({"required":["missing"]}));
    assert_eq!(
        s.run(&req).unwrap()["error"]["kind"],
        "invalid_structured_output"
    );
    for (brief, kind) in [
        ("BADMODEL", "model_not_found"),
        ("LIMIT", "usageLimitExceeded"),
        ("CRASH", "codex_exited"),
    ] {
        let r = s.run(&request(d.path(), brief)).unwrap();
        assert_eq!(r["error"]["kind"], kind, "{r}");
        assert_ne!(r["status"], "completed");
    }
    let r = s.run(&request(d.path(), "APPROVAL")).unwrap();
    assert_eq!(r["status"], "completed");
    assert_eq!(
        r["refused_requests"][0],
        "item/commandExecution/requestApproval"
    );
    assert_eq!(
        s.run(&request(d.path(), "after crash")).unwrap()["status"],
        "completed"
    );
}
#[test]
fn codex_timeout_interrupts_then_accepts_next_turn() {
    let d = tempfile::tempdir().unwrap();
    let s = server();
    let r = s.run(&request(d.path(), "HANG")).unwrap();
    assert_eq!(r["status"], "timeout");
    assert_eq!(
        s.run(&request(d.path(), "next")).unwrap()["status"],
        "completed"
    );
}

struct Client {
    child: std::process::Child,
    input: Option<std::process::ChildStdin>,
    output: mpsc::Receiver<Value>,
}
impl Client {
    fn new(home: &Path) -> Self {
        let mut c = Command::new(EXE)
            .arg("bridge")
            .env("PATH", "")
            .env("AXIOM_DATA_HOME", home)
            .env("AXIOM_CODEX_CMD", json!([FAKE]).to_string())
            .env("AXIOM_AGY_CMD", json!([FAKE, "agy"]).to_string())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap();
        let input = c.stdin.take();
        let out = c.stdout.take().unwrap();
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            for l in BufReader::new(out).lines() {
                if tx.send(serde_json::from_str(&l.unwrap()).unwrap()).is_err() {
                    break;
                }
            }
        });
        Self {
            child: c,
            input,
            output: rx,
        }
    }
    fn send(&mut self, v: Value) {
        writeln!(self.input.as_mut().unwrap(), "{v}").unwrap();
    }
    fn recv(&self) -> Value {
        self.output.recv_timeout(Duration::from_secs(15)).unwrap()
    }
    fn call(&mut self, name: &str, args: Value) -> Value {
        self.send(json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":name,"arguments":args}}));
        self.recv()["result"].clone()
    }
}
impl Drop for Client {
    fn drop(&mut self) {
        self.input.take();
        let start = std::time::Instant::now();
        while matches!(self.child.try_wait(), Ok(None)) {
            if start.elapsed() > Duration::from_secs(2) {
                let _ = self.child.kill();
                break;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        let _ = self.child.wait();
    }
}
#[test]
fn mcp_recovers_from_bad_frames_and_validates_arguments_without_interpreters() {
    let d = tempfile::tempdir().unwrap();
    let mut c = Client::new(d.path());
    writeln!(c.input.as_mut().unwrap(), "{{broken").unwrap();
    assert_eq!(c.recv()["error"]["code"], -32700);
    c.send(Value::Null);
    assert_eq!(c.recv()["error"]["code"], -32600);
    c.send(json!({"jsonrpc":"2.0","method":"notifications/initialized"}));
    c.send(json!({"jsonrpc":"2.0","id":2,"method":"tools/list"}));
    assert_eq!(c.recv()["result"]["tools"].as_array().unwrap().len(), 8);
    let r = c.call("memory_search", json!({"query":"abc","limit":0}));
    assert_eq!(r["isError"], true);
    assert_eq!(r["structuredContent"]["error"]["kind"], "invalid_argument");
    let r = c.call(
        "codex_run",
        json!({"brief":"native","cwd":d.path(),"role":"terra"}),
    );
    assert_eq!(r["isError"], false);
    assert_eq!(r["structuredContent"]["status"], "completed");
    let r = c.call("bridge_status", json!({}));
    assert_eq!(r["structuredContent"]["codex"]["exit_code"], 0);
}
#[test]
fn mcp_ping_is_responsive_while_tool_runs() {
    let d = tempfile::tempdir().unwrap();
    let mut c = Client::new(d.path());
    c.send(json!({"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"codex_run","arguments":{"brief":"HANG","cwd":d.path(),"timeout_s":10}}}));
    c.send(json!({"jsonrpc":"2.0","id":2,"method":"ping"}));
    assert_eq!(c.recv()["id"], 2);
    let r = c.recv();
    assert_eq!(r["id"], 1);
    assert_eq!(r["result"]["structuredContent"]["status"], "timeout");
}
#[test]
fn gemini_default_directory_and_failure_contract() {
    let d = tempfile::tempdir().unwrap();
    let mut c = Client::new(d.path());
    for (prompt, error) in [
        ("CWD", false),
        ("hello", false),
        ("EMPTY", true),
        ("ERROR", true),
        ("SILENT", true),
    ] {
        let r = c.call("gemini_preview", json!({"prompt":prompt}));
        assert_eq!(r["isError"], error, "{r}");
        if prompt == "CWD" {
            assert_eq!(r["structuredContent"]["response"], "0");
        }
    }
}
fn record(root: &Path) -> Record {
    Record {
        project: "demo".into(),
        project_dir: root.to_string_lossy().into(),
        unit: "unit-a".into(),
        title: "資料 storage".into(),
        summary: "儲存相容性".into(),
        body: "durable fact".into(),
        symbols: vec!["save".into()],
        files: vec!["file.txt".into()],
        wish: "keep data".into(),
    }
}
#[test]
fn memory_roundtrip_ranking_append_and_staleness() {
    let d = tempfile::tempdir().unwrap();
    let project = d.path().join("repo 資料");
    write(project.join("file.txt"), "old");
    let m = Memory::open(d.path()).unwrap();
    let a = m.record(&record(&project)).unwrap();
    let b = m.record(&record(&project)).unwrap();
    assert_ne!(a["id"], b["id"]);
    let id = a["id"].as_i64().unwrap();
    assert_eq!(m.get(id).unwrap()["entry"]["stale"], false);
    assert_eq!(
        m.search("資料 storage", Some("demo"), 5).unwrap()["entries"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    write(project.join("file.txt"), "changed");
    assert_eq!(m.get(id).unwrap()["entry"]["stale"], true);
    fs::remove_dir_all(&project).unwrap();
    assert!(m.get(id).unwrap()["entry"]["stale"].is_null());
    assert!(m.digest("demo", Some("hello\nworld")).unwrap()["digest"]
        .as_str()
        .unwrap()
        .contains("hello world"));
    assert!(m.digest("../escape", Some("bad")).is_err());
}
#[test]
fn memory_reads_preexisting_python_schema() {
    let d = tempfile::tempdir().unwrap();
    fs::create_dir_all(d.path().join("memory")).unwrap();
    let db = rusqlite::Connection::open(d.path().join("memory/memory.db")).unwrap();
    db.execute_batch("CREATE TABLE entries(id INTEGER PRIMARY KEY, project TEXT NOT NULL, unit TEXT NOT NULL, title TEXT NOT NULL, summary TEXT NOT NULL, body TEXT NOT NULL, symbols TEXT NOT NULL, wish TEXT NOT NULL, files TEXT NOT NULL, project_dir TEXT NOT NULL, created REAL NOT NULL); INSERT INTO entries VALUES(42,'legacy','u','old title','old summary','old body','save load','old wish','[]','missing',1.0);").unwrap();
    drop(db);
    let m = Memory::open(d.path()).unwrap();
    let e = m.get(42).unwrap();
    assert_eq!(e["entry"]["title"], "old title");
    assert_eq!(e["entry"]["symbols"], json!(["save", "load"]));
}
fn orch_ok(store: &Path, args: &[&str]) -> Value {
    let r = orch::run(
        store,
        &args.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
        true,
        false,
    );
    assert_eq!(r.exit_code, 0, "{}", r.stderr);
    serde_json::from_str(&r.stdout).unwrap()
}
#[test]
fn orchestrator_persists_units_inbox_ledger_and_lock() {
    let d = tempfile::tempdir().unwrap();
    let p = d.path();
    orch_ok(p, &["init"]);
    orch_ok(
        p,
        &[
            "unit",
            "add",
            "core",
            "--track",
            "rust",
            "--branch",
            "feature/core",
        ],
    );
    orch_ok(p, &["unit", "set", "core", "--state", "verified"]);
    assert_eq!(orch_ok(p, &["unit", "get", "core"])["state"], "verified");
    orch_ok(p, &["inbox", "push", "builder", "core", "built"]);
    assert_eq!(orch_ok(p, &["inbox", "count"])["count"], 1);
    orch_ok(p, &["inbox", "drain"]);
    assert_eq!(orch_ok(p, &["inbox", "count"])["count"], 0);
    orch_ok(
        p,
        &[
            "ledger",
            "record",
            "7",
            "abc",
            "unit-test-verified",
            "--evidence",
            "test.log",
        ],
    );
    assert_eq!(
        orch_ok(p, &["ledger", "check", "7", "abc"])["verdict"],
        "unit-test-verified"
    );
    orch_ok(p, &["standing", "add", "keep user files"]);
    orch_ok(p, &["status"]);
    write(p.join(".orch.lock"), &std::process::id().to_string());
    let o = orch::run(
        p,
        &[
            "unit".into(),
            "set".into(),
            "core".into(),
            "--state".into(),
            "bad".into(),
        ],
        true,
        false,
    );
    assert_ne!(o.exit_code, 0);
    assert_eq!(orch_ok(p, &["unit", "get", "core"])["state"], "verified");
}
fn fixture(root: &Path) {
    write(root.join("skills/demo/SKILL.md"), "old skill");
    write(root.join("pstack-resources/README.md"), "old resources");
    write(root.join("plugin.json"), "{\"version\":\"1\"}");
}
#[test]
fn installer_updates_only_owned_unchanged_bundles() {
    let d = tempfile::tempdir().unwrap();
    let source = d.path().join("source");
    let home = d.path().join("home");
    fixture(&source);
    write(home.join("skills/personal/SKILL.md"), "mine");
    install::skills(&source, &home).unwrap();
    write(source.join("skills/demo/SKILL.md"), "new skill");
    install::skills(&source, &home).unwrap();
    assert_eq!(
        fs::read_to_string(home.join("skills/demo/SKILL.md")).unwrap(),
        "new skill"
    );
    assert_eq!(
        fs::read_to_string(home.join("skills/personal/SKILL.md")).unwrap(),
        "mine"
    );
    write(home.join("skills/demo/SKILL.md"), "user edit");
    assert!(install::skills(&source, &home).is_err());
    assert_eq!(
        fs::read_to_string(home.join("skills/demo/SKILL.md")).unwrap(),
        "user edit"
    );
}
#[test]
fn installer_refuses_unowned_bundles_and_rolls_back_partial_moves() {
    let d = tempfile::tempdir().unwrap();
    let source = d.path().join("source");
    let home = d.path().join("home");
    fixture(&source);
    write(home.join("skills/demo/SKILL.md"), "mine");
    assert!(install::skills(&source, &home).is_err());
    fs::remove_dir_all(home.join("skills")).unwrap();
    install::skills(&source, &home).unwrap();
    fs::remove_dir_all(home.join("skills")).unwrap();
    write(home.join("skills"), "blocked parent");
    write(source.join("pstack-resources/README.md"), "new resources");
    assert!(install::skills(&source, &home).is_err());
    assert_eq!(
        fs::read_to_string(home.join("pstack-resources/README.md")).unwrap(),
        "old resources"
    );
}
#[test]
fn native_host_registration_uses_embedded_versioned_marketplace() {
    let d = tempfile::tempdir().unwrap();
    let log = d.path().join("calls.jsonl");
    let out = Command::new(EXE)
        .args(["install", "--host", "codex"])
        .env("PATH", "")
        .env("HOME", d.path())
        .env("USERPROFILE", d.path())
        .env("AXIOM_CODEX_CMD", json!([FAKE, "host"]).to_string())
        .env("FAKE_HOST_LOG", &log)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let calls: Vec<Value> = fs::read_to_string(log)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    assert_eq!(calls[0][0], "plugin");
    assert_eq!(calls[0][2], "add");
    let root = Path::new(calls[0][3].as_str().unwrap());
    assert!(root.join("plugin.json").is_file());
    assert!(root
        .join(if cfg!(windows) {
            "bin/atropos.exe"
        } else {
            "bin/atropos"
        })
        .is_file());
    assert_eq!(
        calls[1],
        json!(["plugin", "add", "axiom-forge@axiom-forge"])
    );
}
#[test]
fn embedded_assets_are_complete_and_do_not_overwrite() {
    let d = tempfile::tempdir().unwrap();
    distribution::materialize(d.path(), None).unwrap();
    assert_eq!(distribution::validate(d.path()).unwrap()["skills"], 66);
    assert!(distribution::materialize(d.path(), None).is_err());
    let names: Vec<_> = axiom_atropos::assets::FILES.iter().map(|a| a.0).collect();
    assert!(names.iter().all(|p| !p.starts_with("tests/")
        && !p.split('/').any(|n| n.eq_ignore_ascii_case("AGENTS.md"))
        && !p.ends_with(".env")));
}
#[test]
fn archive_hashes_and_extracted_runtime_work_without_interpreters() {
    let d = tempfile::tempdir().unwrap();
    let archive = d.path().join("candidate.zip");
    let out = Command::new(EXE)
        .args([
            "package",
            "--candidate",
            "--source",
            env!("CARGO_MANIFEST_DIR"),
            "--output",
        ])
        .arg(&archive)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let mut zip = zip::ZipArchive::new(fs::File::open(archive).unwrap()).unwrap();
    let extracted = d.path().join("unpacked");
    zip.extract(&extracted).unwrap();
    let manifest: Value =
        serde_json::from_slice(&fs::read(extracted.join("MANIFEST.json")).unwrap()).unwrap();
    for (name, hash) in manifest["files"].as_object().unwrap() {
        assert_eq!(
            distribution::hash(&fs::read(extracted.join(name)).unwrap()),
            hash.as_str().unwrap()
        );
    }
    let exe = extracted.join(if cfg!(windows) {
        "bin/atropos.exe"
    } else {
        "bin/atropos"
    });
    let out = Command::new(exe)
        .args(["validate", "--source"])
        .arg(&extracted)
        .env("PATH", "")
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
}
#[test]
fn helper_logs_escape_tsv_and_plan_checker_rejects_missing_evidence() {
    let d = tempfile::tempdir().unwrap();
    let log = d.path().join("decisions.tsv");
    let args = vec![
        log.to_string_lossy().into(),
        "phase".into(),
        "=formula".into(),
        "why\twhy".into(),
        "proof".into(),
        "done".into(),
    ];
    axiom_atropos::helpers::run("work-log", &args).unwrap();
    let text = fs::read_to_string(log).unwrap();
    assert!(text.contains("'=formula\twhy why"));
    let plan = d.path().join("plan.md");
    write(&plan, "# Plan\n\n## Work\n- [ ] do something\n");
    assert!(axiom_atropos::helpers::check_plan(&plan).is_err());
}

#[test]
fn installer_typo_does_not_fall_back_to_default_host() {
    let error = install::run(&["--hast".into(), "pi".into()]).unwrap_err();
    assert_eq!(error.kind, "invalid_argument");
}

#[test]
fn installer_lock_prevents_competing_writer() {
    let d = tempfile::tempdir().unwrap();
    let source = d.path().join("source");
    let home = d.path().join("home");
    fixture(&source);
    fs::create_dir_all(&home).unwrap();
    let lock = fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(home.join(".axiom-forge-install.lock"))
        .unwrap();
    fs2::FileExt::lock_exclusive(&lock).unwrap();
    assert!(install::skills(&source, &home)
        .unwrap_err()
        .message
        .contains("another installation"));
    assert!(!home.join("skills").exists());
}

#[cfg(unix)]
#[test]
fn installer_refuses_linked_destination() {
    let d = tempfile::tempdir().unwrap();
    let source = d.path().join("source");
    let home = d.path().join("home");
    let elsewhere = d.path().join("elsewhere");
    fixture(&source);
    fs::create_dir_all(&elsewhere).unwrap();
    std::os::unix::fs::symlink(&elsewhere, &home).unwrap();
    assert!(install::skills(&source, &home).is_err());
    assert!(fs::read_dir(elsewhere).unwrap().next().is_none());
}

#[cfg(windows)]
#[test]
fn installer_refuses_junction_destination() {
    let d = tempfile::tempdir().unwrap();
    let source = d.path().join("source");
    let home = d.path().join("home");
    let elsewhere = d.path().join("elsewhere");
    fixture(&source);
    fs::create_dir_all(&elsewhere).unwrap();
    assert!(Command::new("cmd")
        .args(["/C", "mklink", "/J"])
        .arg(&home)
        .arg(&elsewhere)
        .output()
        .unwrap()
        .status
        .success());
    assert!(install::skills(&source, &home).is_err());
    assert!(fs::read_dir(elsewhere).unwrap().next().is_none());
}

#[test]
fn plan_checker_accepts_upstream_fixture_and_markdown_fences() {
    let d = tempfile::tempdir().unwrap();
    let path = d.path().join("plan.md");
    let base = include_str!("fixtures/valid-plan.md");
    for suffix in [
        "",
        "````markdown\n```markdown\n## Example only\n```\n````\n",
        "Use ``echo `x`: “quoted” — example``.\n",
    ] {
        write(&path, &format!("{base}{suffix}"));
        assert!(
            axiom_atropos::helpers::check_plan(&path).is_ok(),
            "{suffix}"
        );
    }
    for suffix in [
        "```markdown `invalid`\n## Example only\n```\n",
        "Use ``unclosed: bad` here.\n",
        "## Unexpected section\n",
    ] {
        write(&path, &format!("{base}{suffix}"));
        assert!(
            axiom_atropos::helpers::check_plan(&path).is_err(),
            "{suffix}"
        );
    }
}

#[test]
fn host_update_restores_previous_marketplace_when_new_registration_fails() {
    let d = tempfile::tempdir().unwrap();
    let old = d.path().join("old-source");
    fs::create_dir_all(&old).unwrap();
    let log = d.path().join("calls.jsonl");
    let out = Command::new(EXE)
        .args(["install", "--host", "codex", "--update"])
        .env("HOME", d.path())
        .env("USERPROFILE", d.path())
        .env("AXIOM_CODEX_CMD", json!([FAKE, "host"]).to_string())
        .env("FAKE_HOST_LOG", &log)
        .env("FAKE_PREVIOUS_ROOT", &old)
        .env("FAKE_FAIL_NEW_SOURCE", "1")
        .output()
        .unwrap();
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("restored as a local snapshot"));
    let calls: Vec<Value> = fs::read_to_string(log)
        .unwrap()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    assert_eq!(
        calls[1],
        json!(["plugin", "marketplace", "remove", "axiom-forge"])
    );
    assert_eq!(
        calls.last().unwrap(),
        &json!(["plugin", "marketplace", "add", old])
    );
}

#[test]
fn explicit_store_does_not_require_data_home() {
    let d = tempfile::tempdir().unwrap();
    let out = Command::new(EXE)
        .args(["orch", "--store"])
        .arg(d.path())
        .arg("init")
        .env_remove("AXIOM_DATA_HOME")
        .env_remove("CLAUDE_PLUGIN_DATA")
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(d.path().join("units.tsv").is_file());
    let missing = Command::new(EXE)
        .args(["orch", "--store"])
        .output()
        .unwrap();
    assert_eq!(missing.status.code(), Some(2));
}
