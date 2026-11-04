use crate::{
    base::{self, Failure, Res},
    codex::{AppServer, RunRequest},
    memory::{Memory, Record},
};
use serde_json::{json, Value};
use std::{
    io::{BufRead, Write},
    sync::{Arc, Mutex},
    time::Duration,
};

#[derive(Default)]
pub struct Bridge {
    codex: Mutex<Option<Arc<AppServer>>>,
}
fn s<'a>(v: &'a Value, k: &str) -> &'a str {
    v[k].as_str().unwrap_or("")
}
fn opt(v: &Value, k: &str) -> Option<String> {
    v[k].as_str().map(str::to_owned)
}
fn list(v: &Value, k: &str) -> Vec<String> {
    v[k].as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_owned)
        .collect()
}
pub fn tools() -> Value {
    serde_json::from_str(include_str!("tools.json")).expect("tool descriptors")
}
impl Bridge {
    fn codex(&self) -> Res<Arc<AppServer>> {
        let mut lock = self.codex.lock().unwrap_or_else(|e| e.into_inner());
        if lock.is_none() {
            *lock = Some(Arc::new(AppServer::new(base::resolve_command(
                "AXIOM_CODEX_CMD",
                "codex",
            )?)));
        }
        Ok(lock.as_ref().unwrap().clone())
    }
    pub fn call(&self, name: &str, a: &Value) -> Res<Value> {
        let descriptors = tools();
        let descriptor = descriptors
            .as_array()
            .unwrap()
            .iter()
            .find(|t| t["name"] == name)
            .ok_or_else(|| Failure::new("unknown_tool", format!("unknown tool {name}")))?;
        base::schema::validate(&descriptor["inputSchema"], a)
            .map_err(|e| Failure::new("invalid_argument", e))?;
        match name {
            "codex_run" => {
                let server = self.codex()?;
                let model = match opt(a, "model") {
                    Some(m) => m,
                    None => match a["role"].as_str() {
                        Some(role) => server.resolve_model(role)?,
                        None => String::new(),
                    },
                };
                server.run(&RunRequest {
                    brief: s(a, "brief").into(),
                    cwd: s(a, "cwd").into(),
                    model,
                    sandbox: a["sandbox"].as_str().unwrap_or("workspace-write").into(),
                    effort: opt(a, "effort"),
                    output_schema: a.get("output_schema").cloned().filter(|v| !v.is_null()),
                    developer_instructions: opt(a, "developer_instructions"),
                    timeout_s: a["timeout_s"].as_u64().unwrap_or(1800),
                    lean: a["lean"] != false,
                    keep_plugins: list(a, "keep_plugins"),
                    web_search: a["web_search"] == true,
                })
            }
            "gemini_preview" => preview(a),
            "orch" => {
                let store = crate::orch::store_for(&base::data_home()?, s(a, "project"))?;
                let mut args = list(a, "args");
                let json = args.iter().any(|a| a == "--json");
                let force = args.iter().any(|a| a == "--force");
                args.retain(|a| a != "--json" && a != "--force");
                let r = crate::orch::run(&store, &args, json, force);
                Ok(
                    json!({"status":if r.exit_code==0{"completed"}else{"failed"},"store":store,"exit_code":r.exit_code,"stdout":r.stdout,"stderr":r.stderr,"error":if r.exit_code==0{Value::Null}else{json!({"kind":"orch_failed","message":r.stderr})}}),
                )
            }
            "memory_record" => Memory::open(&base::data_home()?)?.record(&Record {
                project: s(a, "project").into(),
                project_dir: s(a, "project_dir").into(),
                unit: s(a, "unit").into(),
                title: s(a, "title").into(),
                summary: s(a, "summary").into(),
                body: s(a, "body").into(),
                wish: s(a, "wish").into(),
                symbols: list(a, "symbols"),
                files: list(a, "files"),
            }),
            "memory_search" => Memory::open(&base::data_home()?)?.search(
                s(a, "query"),
                a["project"].as_str(),
                a["limit"].as_i64().unwrap_or(5),
            ),
            "memory_get" => Memory::open(&base::data_home()?)?.get(
                a["id"]
                    .as_i64()
                    .ok_or_else(|| Failure::new("invalid_argument", "invalid id"))?,
            ),
            "memory_digest" => {
                Memory::open(&base::data_home()?)?.digest(s(a, "project"), a["add"].as_str())
            }
            "bridge_status" => {
                let mut result = json!({"status":"completed","runtime":"rust","version":env!("CARGO_PKG_VERSION"),"codex_app_server_running":self.codex.lock().unwrap().as_ref().is_some_and(|c|c.alive()),"orch":{"runtime":"native-rust"}});
                for (key, var) in [("codex", "AXIOM_CODEX_CMD"), ("agy", "AXIOM_AGY_CMD")] {
                    result[key]=match base::resolve_command(var,key).and_then(|cmd| {
                        let mut c=base::proc::command(&cmd); c.arg("--version");
                        crate::process::capture(&mut c,None,Duration::from_secs(30)).map(|o|json!({"command":cmd,"version":o.stdout.trim().lines().last().map(|s|vec![s]),"exit_code":o.exit_code}))
                    }) {Ok(v)=>v,Err(e)=>json!({"error":e.to_json()})};
                }
                result["statechart_tlc"] = json!({"JAVA":std::env::var("JAVA").ok(),"TLA2TOOLS":std::env::var("TLA2TOOLS").ok()});
                Ok(result)
            }
            _ => unreachable!(),
        }
    }
}
impl Drop for Bridge {
    fn drop(&mut self) {
        if let Some(c) = self.codex.get_mut().unwrap().as_ref() {
            c.close();
        }
    }
}
fn preview(a: &Value) -> Res<Value> {
    if s(a, "prompt").trim().is_empty() {
        return Err(Failure::new("invalid_argument", "prompt is empty"));
    }
    let temp = tempfile::tempdir().map_err(|e| Failure::new("io", e.to_string()))?;
    let cwd = match a["cwd"].as_str() {
        Some(p) => base::require_directory(p)?,
        None => temp.path().into(),
    };
    let mut c = base::proc::command(&base::resolve_command("AXIOM_AGY_CMD", "agy")?);
    c.current_dir(cwd).args([
        "--input-format",
        "stream-json",
        "--output-format",
        "stream-json",
        "--model",
        a["model"].as_str().unwrap_or("gemini-4-argon-medium"),
    ]);
    if let Some(schema) = a.get("json_schema").filter(|s| !s.is_null()) {
        let path = temp.path().join("schema.json");
        std::fs::write(&path, schema.to_string()).map_err(|e| Failure::new("io", e.to_string()))?;
        c.arg("--json-schema").arg(path);
    }
    c.arg("-p=");
    let input = format!(
        "{}\n",
        json!({"event":"user","message":{"role":"user","content":s(a,"prompt")}})
    );
    let output = match crate::process::capture(
        &mut c,
        Some(input.as_bytes()),
        Duration::from_secs(a["timeout_s"].as_u64().unwrap_or(600)),
    ) {
        Ok(o) => o,
        Err(e) if e.kind == "timeout" => {
            return Ok(json!({"status":"timeout","error":e.to_json()}))
        }
        Err(e) => return Err(e),
    };
    let result = output
        .stdout
        .lines()
        .filter_map(|l| serde_json::from_str::<Value>(l).ok())
        .rfind(|v| v["event"] == "result")
        .ok_or_else(|| {
            Failure::new(
                "agy_no_result",
                format!(
                    "agy exited {} without result: {}",
                    output.exit_code, output.stderr
                ),
            )
        })?["result"]
        .clone();
    let error = if result["status"] != "SUCCESS" {
        json!({"kind":format!("agy_{}",result["status"].as_str().unwrap_or("unknown").to_lowercase()),"message":result["error"].as_str().unwrap_or("")})
    } else if result["response"].as_str().unwrap_or("").trim().is_empty()
        && result["structured_output"].is_null()
    {
        json!({"kind":"empty_response","message":"agy returned no text"})
    } else {
        Value::Null
    };
    Ok(
        json!({"status":if error.is_null(){"completed"}else{"failed"},"response":result["response"],"structured_output":result["structured_output"],"conversation_id":result["conversation_id"],"usage":result["usage"],"error":error}),
    )
}
pub fn response(bridge: &Bridge, m: Value) -> Option<Value> {
    if !m.is_object() || m["jsonrpc"] != "2.0" || !m["method"].is_string() {
        return Some(
            json!({"jsonrpc":"2.0","id":null,"error":{"code":-32600,"message":"invalid request"}}),
        );
    }
    let id = m.get("id")?.clone();
    let p = m.get("params").cloned().unwrap_or_else(|| json!({}));
    if !p.is_object() {
        return Some(
            json!({"jsonrpc":"2.0","id":id,"error":{"code":-32602,"message":"params must be an object"}}),
        );
    }
    let result = match m["method"].as_str().unwrap() {
        "initialize" => {
            json!({"protocolVersion":p["protocolVersion"].as_str().unwrap_or("2025-06-18"),"capabilities":{"tools":{}},"serverInfo":{"name":"axiom-bridge","version":env!("CARGO_PKG_VERSION")}})
        }
        "ping" => json!({}),
        "tools/list" => json!({"tools":tools()}),
        "tools/call" => {
            let (value, error) =
                match bridge.call(s(&p, "name"), p.get("arguments").unwrap_or(&json!({}))) {
                    Ok(v) => {
                        let e = v.get("status").is_some_and(|s| s != "completed");
                        (v, e)
                    }
                    Err(e) => (json!({"status":"bridge_error","error":e.to_json()}), true),
                };
            json!({"content":[{"type":"text","text":value.to_string()}],"structuredContent":value,"isError":error})
        }
        _ => {
            return Some(
                json!({"jsonrpc":"2.0","id":id,"error":{"code":-32601,"message":"method not found"}}),
            )
        }
    };
    Some(json!({"jsonrpc":"2.0","id":id,"result":result}))
}
pub fn serve() -> Res<()> {
    let bridge = Arc::new(Bridge::default());
    let output = Arc::new(Mutex::new(std::io::stdout()));
    let mut tasks = Vec::new();
    for line in std::io::stdin().lock().lines() {
        let line = line.map_err(|e| Failure::new("io", e.to_string()))?;
        if line.trim().is_empty() {
            continue;
        }
        let parsed = serde_json::from_str::<Value>(&line);
        let b = bridge.clone();
        let o = output.clone();
        let work = move || {
            let r = match parsed {
                Ok(v) => response(&b, v),
                Err(e) => Some(
                    json!({"jsonrpc":"2.0","id":null,"error":{"code":-32700,"message":e.to_string()}}),
                ),
            };
            if let Some(r) = r {
                let mut w = o.lock().unwrap();
                let _ = writeln!(w, "{r}");
                let _ = w.flush();
            }
        };
        if line.contains("tools/call") {
            tasks.push(std::thread::spawn(work));
        } else {
            work();
        }
        let mut i = 0;
        while i < tasks.len() {
            if tasks[i].is_finished() {
                let _ = tasks.swap_remove(i).join();
            } else {
                i += 1;
            }
        }
    }
    for task in tasks {
        let _ = task.join();
    }
    Ok(())
}
