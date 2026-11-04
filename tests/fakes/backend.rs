use serde_json::{json, Value};
use std::{
    collections::HashMap,
    io::{BufRead, Write},
};
fn send(v: Value) {
    println!("{v}");
    std::io::stdout().flush().unwrap();
}
fn event(method: &str, p: Value) {
    send(json!({"method":method,"params":p}));
}
fn complete(t: &str, status: &str, error: Value) {
    event(
        "turn/completed",
        json!({"threadId":t,"turn":{"id":format!("{t}-turn"),"status":status,"error":error}}),
    );
}
fn item(t: &str, value: Value) {
    event(
        "item/completed",
        json!({"threadId":t,"turnId":format!("{t}-turn"),"item":value}),
    );
}
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--version") {
        println!("native-test-backend 1");
        return;
    }
    if args.first().is_some_and(|s| s == "host") {
        let path = std::env::var_os("FAKE_HOST_LOG").unwrap();
        writeln!(
            std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .unwrap(),
            "{}",
            json!(args[1..])
        )
        .unwrap();
        return;
    }
    if args.first().is_some_and(|s| s == "agy") {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let v: Value = serde_json::from_str(&line).unwrap();
        let prompt = v["message"]["content"].as_str().unwrap();
        assert_eq!(args.last().unwrap(), "-p=");
        if prompt == "SILENT" {
            std::process::exit(3);
        }
        let response = match prompt {
            "EMPTY" => String::new(),
            "CWD" => json!(std::fs::read_dir(".").unwrap().count()).to_string(),
            _ => format!("echo: {prompt}"),
        };
        send(
            json!({"event":"result","result":{"status":if prompt=="ERROR"{"ERROR"}else{"SUCCESS"},"response":response,"error":"refused","conversation_id":"c1","usage":{"total_tokens":1}}}),
        );
        return;
    }
    let mut threads: HashMap<String, Value> = HashMap::new();
    let mut approval = String::new();
    for line in std::io::stdin().lock().lines() {
        let m: Value = serde_json::from_str(&line.unwrap()).unwrap();
        let p = &m["params"];
        let id = &m["id"];
        if m.get("method").is_none() {
            if m.get("error").is_some() && !approval.is_empty() {
                item(
                    &approval,
                    json!({"type":"agentMessage","text":"approval refused"}),
                );
                complete(&approval, "completed", Value::Null);
            }
            continue;
        }
        let result = match m["method"].as_str().unwrap() {
            "initialize" => json!({"userAgent":"native fake"}),
            "model/list" => {
                json!({"data":[{"id":"gpt-6-luna","isDefault":true},{"id":"gpt-6.1-terra"},{"id":"gpt-6-sol"}]})
            }
            "plugin/installed" => {
                json!({"marketplaces":[{"plugins":[{"id":"panel@test","enabled":true},{"id":"keep@test","enabled":true}]}]})
            }
            "config/read" => json!({"config":{"mcp_servers":{"node_repl":{}}}}),
            "thread/start" => {
                assert_eq!(p["approvalPolicy"], "never");
                assert_eq!(p["ephemeral"], true);
                let t = format!("thread-{}", threads.len() + 1);
                threads.insert(t.clone(), p.clone());
                json!({"thread":{"id":t}})
            }
            "turn/start" => {
                let t = p["threadId"].as_str().unwrap();
                let brief = p["input"][0]["text"].as_str().unwrap();
                send(
                    json!({"id":id,"result":{"turn":{"id":format!("{t}-turn"),"status":"inProgress"}}}),
                );
                match brief {
                    "CRASH" => std::process::exit(7),
                    "HANG" => continue,
                    "APPROVAL" => {
                        approval = t.into();
                        send(
                            json!({"id":"approval","method":"item/commandExecution/requestApproval","params":{"threadId":t,"turnId":format!("{t}-turn")}}),
                        );
                        continue;
                    }
                    "LIMIT" => {
                        complete(
                            t,
                            "failed",
                            json!({"message":"limit","codexErrorInfo":"usageLimitExceeded"}),
                        );
                        continue;
                    }
                    "BADMODEL" => {
                        complete(
                            t,
                            "failed",
                            json!({"message":json!({"error":{"code":"model_not_found"}}).to_string(),"codexErrorInfo":"other"}),
                        );
                        continue;
                    }
                    _ => {}
                }
                item(
                    t,
                    json!({"type":"commandExecution","command":"test-command","status":"completed","exitCode":0}),
                );
                item(
                    t,
                    json!({"type":"fileChange","status":"completed","changes":[{"path":"src/lib.rs","kind":{"type":"update"}}]}),
                );
                event(
                    "thread/tokenUsage/updated",
                    json!({"threadId":t,"turnId":format!("{t}-turn"),"tokenUsage":{"total":{"totalTokens":12}}}),
                );
                let text = if p.get("outputSchema").is_some() {
                    json!({"answer":"ok"})
                } else {
                    json!({"brief":brief,"thread":threads[t],"turn":p})
                };
                item(t, json!({"type":"agentMessage","text":text.to_string()}));
                complete(t, "completed", Value::Null);
                continue;
            }
            "turn/interrupt" => {
                let t = p["threadId"].as_str().unwrap();
                complete(t, "interrupted", Value::Null);
                json!({})
            }
            _ => continue,
        };
        send(json!({"id":id,"result":result}));
    }
}
