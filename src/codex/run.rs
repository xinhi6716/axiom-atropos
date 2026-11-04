use super::{AppServer, SANDBOXES};
use crate::base::{self, Failure, Res};
use serde_json::{json, Map, Value};
use std::time::{Duration, Instant};

pub struct RunRequest {
    pub brief: String,
    pub cwd: String,
    pub model: String,
    pub sandbox: String,
    pub effort: Option<String>,
    pub output_schema: Option<Value>,
    pub developer_instructions: Option<String>,
    pub timeout_s: u64,
    pub lean: bool,
    pub keep_plugins: Vec<String>,
    pub web_search: bool,
}

impl AppServer {
    pub fn run(&self, req: &RunRequest) -> Res<Value> {
        base::require_directory(&req.cwd)?;
        if !SANDBOXES.contains(&req.sandbox.as_str()) || req.brief.trim().is_empty() {
            return Err(Failure::new(
                "invalid_argument",
                "invalid sandbox or empty brief",
            ));
        }
        let mut thread = json!({"model":req.model,"ephemeral":true,"sandbox":req.sandbox,"approvalPolicy":"never","cwd":req.cwd});
        if req.model.is_empty() {
            thread.as_object_mut().unwrap().remove("model");
        }
        if let Some(s) = req
            .developer_instructions
            .as_ref()
            .filter(|s| !s.is_empty())
        {
            thread["developerInstructions"] = json!(s);
        }
        let disabled = if req.lean {
            let installed = self.request("plugin/installed", json!({"cwds":[req.cwd]}))?;
            let mut plugins: Vec<String> = installed["marketplaces"]
                .as_array()
                .into_iter()
                .flatten()
                .flat_map(|m| m["plugins"].as_array().into_iter().flatten())
                .filter(|p| p["enabled"] == true)
                .filter_map(|p| p["id"].as_str())
                .filter(|id| !req.keep_plugins.iter().any(|k| k == id))
                .map(str::to_owned)
                .collect();
            plugins.sort();
            let config = self.request("config/read", json!({"cwd":req.cwd}))?;
            let mut servers: Vec<String> = config["config"]["mcp_servers"]
                .as_object()
                .into_iter()
                .flat_map(|m| m.keys().cloned())
                .collect();
            servers.sort();
            let plugin_config: Map<String, Value> = plugins
                .iter()
                .map(|p| (p.clone(), json!({"enabled":false})))
                .collect();
            let server_config: Map<String, Value> = servers
                .iter()
                .map(|p| (p.clone(), json!({"enabled":false})))
                .collect();
            let mut config = json!({"features":{"apps":false,"goals":false},"plugins":plugin_config,"mcp_servers":server_config,"skills":{"max_context_tokens":1},"personality":"none"});
            let mut sections = vec!["apps", "goals", "skills_catalog", "personality"];
            if !req.web_search {
                config["web_search"] = json!("disabled");
                sections.push("web_search");
            }
            thread["config"] = config;
            json!({"sections":sections,"plugins":plugins,"mcp_servers":servers})
        } else {
            Value::Null
        };
        let started = self.request("thread/start", thread)?;
        let thread_id = started
            .pointer("/thread/id")
            .and_then(Value::as_str)
            .ok_or_else(|| Failure::new("codex_rpc", "thread/start: missing thread id"))?
            .to_owned();
        let events = self.subscribe(&thread_id);
        let mut turn = json!({"threadId":thread_id,"input":[{"type":"text","text":req.brief}]});
        if let Some(s) = req.effort.as_ref().filter(|s| !s.is_empty()) {
            turn["effort"] = json!(s);
        }
        if let Some(schema) = &req.output_schema {
            turn["outputSchema"] = schema.clone();
        }
        if !disabled.is_null() {
            turn["disabledPluginIds"] = disabled["plugins"].clone();
        }
        let result = self.request("turn/start", turn);
        let started = match result {
            Ok(v) => v,
            Err(e) => {
                self.unsubscribe(&thread_id);
                return if e.kind == "codex_exited" {
                    Ok(report_error(
                        req,
                        &thread_id,
                        "",
                        disabled,
                        "bridge_error",
                        &e.kind,
                        &e.message,
                    ))
                } else {
                    Err(e)
                };
            }
        };
        let Some(turn_id) = started.pointer("/turn/id").and_then(Value::as_str) else {
            self.unsubscribe(&thread_id);
            return Err(Failure::new("codex_rpc", "turn/start: missing turn id"));
        };
        self.register_turn(&thread_id, turn_id);
        let mut report = json!({"status":null,"thread_id":thread_id,"turn_id":turn_id,"model":req.model,"final_message":null,"structured_output":null,"commands":[],"file_changes":[],"refused_requests":[],"usage":null,"disabled":disabled,"started_mcp_servers":[],"error":null});
        let deadline = Instant::now().checked_add(Duration::from_secs(req.timeout_s));
        loop {
            let remaining = deadline
                .map(|d| d.saturating_duration_since(Instant::now()))
                .unwrap_or(Duration::MAX);
            if remaining.is_zero() {
                self.interrupt(&thread_id, turn_id);
                report["status"] = json!("timeout");
                report["error"] = json!({"kind":"timeout","message":format!("turn exceeded {}s and was interrupted", req.timeout_s)});
                break;
            }
            let event = match events.recv_timeout(remaining) {
                Ok(event) => event,
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => continue,
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                    report["status"] = json!("bridge_error");
                    report["error"] =
                        json!({"kind":"codex_exited","message":"codex app-server exited mid-turn"});
                    break;
                }
            };
            let method = event["method"].as_str().unwrap_or("");
            let params = &event["params"];
            if event.get("id").is_some() {
                report["refused_requests"]
                    .as_array_mut()
                    .unwrap()
                    .push(json!(method));
                continue;
            }
            if method == "mcpServer/startupStatus/updated" && params["status"] == "ready" {
                report["started_mcp_servers"]
                    .as_array_mut()
                    .unwrap()
                    .push(params["name"].clone());
                continue;
            }
            if params
                .get("turnId")
                .and_then(Value::as_str)
                .is_some_and(|id| id != turn_id)
            {
                continue;
            }
            if method != "thread/started"
                && method != "mcpServer/startupStatus/updated"
                && (method != "turn/completed" || params["turn"]["id"] == turn_id)
            {
                if let Some(progress) = base::progress::current() {
                    progress.emit(method);
                }
            }
            match method {
                "thread/tokenUsage/updated" => {
                    report["usage"] = params["tokenUsage"]["total"].clone()
                }
                "item/completed" => record_item(&mut report, &params["item"]),
                "error" if params["willRetry"] != true => {
                    report["error"] = turn_error(&params["error"])
                }
                "turn/completed" if params["turn"]["id"] == turn_id => {
                    report["status"] = params["turn"]["status"].clone();
                    if !params["turn"]["error"].is_null() {
                        report["error"] = turn_error(&params["turn"]["error"]);
                    }
                    if report["status"] == "completed" {
                        report["error"] = Value::Null;
                        if let Some(schema) = &req.output_schema {
                            let parsed = report["final_message"]
                                .as_str()
                                .ok_or_else(|| "final message is missing".to_owned())
                                .and_then(|s| {
                                    serde_json::from_str::<Value>(s)
                                        .map_err(|e| format!("final message is not JSON: {e}"))
                                })
                                .and_then(|v| {
                                    base::schema::validate(schema, &v)
                                        .map(|_| v)
                                        .map_err(|e| format!("schema validation failed: {e}"))
                                });
                            match parsed {
                                Ok(v) => {
                                    report["structured_output"] = v;
                                    report["final_message"] = Value::Null;
                                }
                                Err(message) => {
                                    report["status"] = json!("failed");
                                    report["error"] = json!({"kind":"invalid_structured_output","message":message});
                                }
                            }
                        }
                    } else if report["error"].is_null() {
                        report["error"] = json!({"kind":"codex_error","message":format!("turn {}", report["status"].as_str().unwrap_or("unknown"))});
                    }
                    break;
                }
                _ => {}
            }
        }
        self.unregister_turn(&thread_id, turn_id);
        self.unsubscribe(&thread_id);
        Ok(report)
    }
}

fn report_error(
    req: &RunRequest,
    thread: &str,
    turn: &str,
    disabled: Value,
    status: &str,
    kind: &str,
    message: &str,
) -> Value {
    json!({"status":status,"thread_id":thread,"turn_id":turn,"model":req.model,"final_message":null,"structured_output":null,"commands":[],"file_changes":[],"refused_requests":[],"usage":null,"disabled":disabled,"started_mcp_servers":[],"error":{"kind":kind,"message":message}})
}

fn turn_error(error: &Value) -> Value {
    let info = &error["codexErrorInfo"];
    let mut kind = info
        .as_str()
        .or_else(|| {
            info.as_object()
                .and_then(|o| o.keys().next().map(String::as_str))
        })
        .unwrap_or("codex_error")
        .to_owned();
    let message = error["message"].as_str().unwrap_or("");
    if kind == "other" {
        if let Ok(v) = serde_json::from_str::<Value>(message) {
            if let Some(code) = v
                .pointer("/error/code")
                .and_then(Value::as_str)
                .filter(|s| !s.is_empty())
            {
                kind = code.to_owned();
            }
        }
    }
    let lower = kind.to_lowercase();
    let mut result = json!({"kind":kind,"message":message});
    if lower.contains("usagelimit") || lower.contains("auth") {
        result["user_action"] = json!(true);
    }
    result
}

fn record_item(report: &mut Value, item: &Value) {
    match item["type"].as_str() {
        Some("agentMessage") => report["final_message"] = item["text"].clone(),
        Some("commandExecution") => report["commands"].as_array_mut().unwrap().push(
            json!({"command":item["command"],"status":item["status"],"exit_code":item["exitCode"]}),
        ),
        Some("fileChange") => {
            for change in item["changes"].as_array().into_iter().flatten() {
                report["file_changes"].as_array_mut().unwrap().push(json!({"path":change["path"],"kind":change["kind"]["type"],"status":item["status"]}));
            }
        }
        _ => {}
    }
}
