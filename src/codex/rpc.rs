use crate::base::{proc, Failure, Res};
use serde_json::{json, Value};
use std::{
    collections::{HashMap, HashSet},
    io::{BufRead, BufReader, Write},
    process::{Child, ChildStdin, Stdio},
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

pub const SANDBOXES: [&str; 2] = ["read-only", "workspace-write"];
type EventChannel = (Sender<Value>, Option<Receiver<Value>>);
struct Connection {
    child: Mutex<Child>,
    input: Mutex<ChildStdin>,
    alive: AtomicBool,
    pending: Mutex<HashMap<u64, Sender<Value>>>,
    events: Mutex<HashMap<String, EventChannel>>,
}
pub struct AppServer {
    command: Vec<String>,
    connection: Mutex<Option<Arc<Connection>>>,
    start: Mutex<()>,
    ids: AtomicU64,
    turns: Mutex<HashSet<(String, String)>>,
    request_timeout: Option<Duration>,
}
impl AppServer {
    pub fn new(command: Vec<String>) -> Self {
        Self::with_request_timeout(command, None)
    }
    #[doc(hidden)]
    pub fn with_request_timeout(command: Vec<String>, timeout: Option<Duration>) -> Self {
        Self {
            command,
            connection: Mutex::new(None),
            start: Mutex::new(()),
            ids: AtomicU64::new(1),
            turns: Mutex::new(HashSet::new()),
            request_timeout: timeout,
        }
    }
    pub fn alive(&self) -> bool {
        self.connection.lock().unwrap().as_ref().is_some_and(|c| {
            c.alive.load(Ordering::Acquire)
                && matches!(c.child.lock().unwrap().try_wait(), Ok(None))
        })
    }
    pub fn close(&self) {
        if let Some(c) = self.connection.lock().unwrap().take() {
            c.alive.store(false, Ordering::Release);
            proc::kill_tree(&mut c.child.lock().unwrap());
            c.pending.lock().unwrap().clear();
            c.events.lock().unwrap().clear();
        }
        self.turns.lock().unwrap().clear();
    }
    fn ensure_started(&self) -> Res<Arc<Connection>> {
        let _guard = self.start.lock().unwrap();
        if self.alive() {
            return self
                .connection
                .lock()
                .unwrap()
                .clone()
                .ok_or_else(|| Failure::new("codex_exited", "codex app-server closed"));
        }
        self.close();
        if self.command.is_empty() {
            return Err(Failure::new("not_installed", "empty codex command"));
        }
        let mut cmd = proc::command(&self.command);
        let mut child = cmd
            .args(["app-server", "--listen", "stdio://"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| Failure::new("codex_exited", e.to_string()))?;
        let input = child.stdin.take().unwrap();
        let output = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();
        let c = Arc::new(Connection {
            child: Mutex::new(child),
            input: Mutex::new(input),
            alive: AtomicBool::new(true),
            pending: Mutex::new(HashMap::new()),
            events: Mutex::new(HashMap::new()),
        });
        *self.connection.lock().unwrap() = Some(c.clone());
        let reader = c.clone();
        thread::spawn(move || {
            let mut output = BufReader::new(output);
            let mut line = Vec::new();
            loop {
                line.clear();
                if !matches!(output.read_until(b'\n', &mut line), Ok(n) if n > 0) {
                    break;
                }
                let text = String::from_utf8_lossy(&line);
                let Ok(msg) = serde_json::from_str::<Value>(&text) else {
                    continue;
                };
                if let Some(method) = msg.get("method").and_then(Value::as_str) {
                    if let Some(id) = msg.get("id") {
                        let _ = Self::send(
                            &reader,
                            json!({"id":id,"error":{"code":-32601,"message":"axiom bridge grants no interactive approvals"}}),
                        );
                    }
                    if let Some(id) = msg
                        .pointer("/params/threadId")
                        .and_then(Value::as_str)
                        .or_else(|| msg.pointer("/params/thread/id").and_then(Value::as_str))
                    {
                        if let Some(entry) = reader.events.lock().unwrap().get(id) {
                            let _ = entry.0.send(msg.clone());
                        }
                    }
                    let _ = method;
                } else if let Some(id) = msg.get("id").and_then(Value::as_u64) {
                    if let Some(thread_id) =
                        msg.pointer("/result/thread/id").and_then(Value::as_str)
                    {
                        reader
                            .events
                            .lock()
                            .unwrap()
                            .entry(thread_id.to_owned())
                            .or_insert_with(|| {
                                let (tx, rx) = mpsc::channel();
                                (tx, Some(rx))
                            });
                    }
                    if let Some(tx) = reader.pending.lock().unwrap().remove(&id) {
                        let _ = tx.send(msg);
                    }
                }
            }
            reader.alive.store(false, Ordering::Release);
            reader.pending.lock().unwrap().clear();
            reader.events.lock().unwrap().clear();
        });
        thread::spawn(move || {
            for line in BufReader::new(stderr).lines() {
                if line.is_err() {
                    break;
                }
            }
        });
        if let Err(e) = self.request_on(&c, "initialize", json!({"clientInfo":{"name":"axiom-chronos","title":"Axiom Chronos","version":"0.1.0"}})).and_then(|_| Self::send(&c, json!({"method":"initialized"}))) { self.close(); return Err(e); }
        Ok(c)
    }
    fn send(c: &Connection, message: Value) -> Res<()> {
        let mut input = c.input.lock().unwrap();
        writeln!(input, "{message}")
            .and_then(|_| input.flush())
            .map_err(|e| Failure::new("codex_exited", e.to_string()))
    }
    fn request_on(&self, c: &Connection, method: &str, params: Value) -> Res<Value> {
        let id = self.ids.fetch_add(1, Ordering::Relaxed);
        let (tx, rx) = mpsc::channel();
        {
            let mut pending = c.pending.lock().unwrap();
            if !c.alive.load(Ordering::Acquire) {
                return Err(Failure::new("codex_exited", "codex app-server exited"));
            }
            pending.insert(id, tx);
        }
        if let Err(e) = Self::send(c, json!({"id":id,"method":method,"params":params})) {
            c.pending.lock().unwrap().remove(&id);
            return Err(e);
        }
        let timeout = self.request_timeout.unwrap_or_else(|| {
            Duration::from_secs(match method {
                "initialize" => 60,
                "turn/interrupt" => 30,
                _ => 120,
            })
        });
        let msg = match rx.recv_timeout(timeout) {
            Ok(msg) => msg,
            Err(mpsc::RecvTimeoutError::Timeout) => {
                c.pending.lock().unwrap().remove(&id);
                return Err(Failure::new("timeout", format!("{method} timed out")));
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                return Err(Failure::new(
                    "codex_exited",
                    format!("codex app-server exited during {method}"),
                ))
            }
        };
        if let Some(error) = msg.get("error") {
            return Err(Failure::new(
                "codex_rpc",
                format!(
                    "{method}: {}: {}",
                    error.get("code").unwrap_or(&Value::Null),
                    error
                        .get("message")
                        .and_then(Value::as_str)
                        .unwrap_or("unknown error")
                ),
            ));
        }
        msg.get("result")
            .cloned()
            .ok_or_else(|| Failure::new("codex_rpc", format!("{method}: missing result")))
    }
    pub(crate) fn request(&self, method: &str, params: Value) -> Res<Value> {
        let c = self.ensure_started()?;
        self.request_on(&c, method, params)
    }
    pub(crate) fn subscribe(&self, thread_id: &str) -> Receiver<Value> {
        let Some(c) = self.connection.lock().unwrap().clone() else {
            let (tx, rx) = mpsc::channel();
            drop(tx);
            return rx;
        };
        let mut events = c.events.lock().unwrap();
        let entry = events.entry(thread_id.to_owned()).or_insert_with(|| {
            let (tx, rx) = mpsc::channel();
            (tx, Some(rx))
        });
        if !c.alive.load(Ordering::Acquire) {
            events.remove(thread_id);
            let (tx, rx) = mpsc::channel();
            drop(tx);
            return rx;
        }
        entry.1.take().unwrap_or_else(|| {
            let (tx, rx) = mpsc::channel();
            drop(tx);
            rx
        })
    }
    pub(crate) fn unsubscribe(&self, thread_id: &str) {
        if let Some(c) = self.connection.lock().unwrap().as_ref() {
            c.events.lock().unwrap().remove(thread_id);
        }
    }
    pub(crate) fn interrupt(&self, thread_id: &str, turn_id: &str) {
        if let Some(c) = self.connection.lock().unwrap().clone() {
            let _ = self.request_on(
                &c,
                "turn/interrupt",
                json!({"threadId":thread_id,"turnId":turn_id}),
            );
        }
    }
    pub(crate) fn register_turn(&self, thread_id: &str, turn_id: &str) {
        self.turns
            .lock()
            .unwrap()
            .insert((thread_id.into(), turn_id.into()));
    }
    pub(crate) fn unregister_turn(&self, thread_id: &str, turn_id: &str) {
        self.turns
            .lock()
            .unwrap()
            .remove(&(thread_id.into(), turn_id.into()));
    }
    pub fn cancel_all(&self) {
        let turns: Vec<_> = self.turns.lock().unwrap().iter().cloned().collect();
        let Some(c) = self.connection.lock().unwrap().clone() else {
            return;
        };
        for (thread_id, turn_id) in turns {
            let _ = self.request_on(
                &c,
                "turn/interrupt",
                json!({"threadId":thread_id,"turnId":turn_id}),
            );
        }
    }
    pub fn list_models(&self) -> Res<Vec<String>> {
        let mut result = Vec::new();
        let mut cursor = None;
        let mut seen = HashSet::new();
        loop {
            let params = cursor
                .as_ref()
                .map_or_else(|| json!({}), |c| json!({"cursor":c}));
            let page = self.request("model/list", params)?;
            let data = page
                .get("data")
                .and_then(Value::as_array)
                .ok_or_else(|| Failure::new("codex_rpc", "model/list: missing data"))?;
            for model in data {
                if let Some(id) = model
                    .get("id")
                    .and_then(Value::as_str)
                    .or_else(|| model.get("model").and_then(Value::as_str))
                {
                    result.push(id.to_owned());
                }
            }
            cursor = page
                .get("nextCursor")
                .and_then(Value::as_str)
                .filter(|s| !s.is_empty())
                .map(str::to_owned);
            match &cursor {
                Some(c) if seen.insert(c.clone()) => {}
                Some(_) => return Err(Failure::new("codex_rpc", "model/list: repeated cursor")),
                None => break,
            }
        }
        Ok(result)
    }
    pub fn resolve_model(&self, role: &str) -> Res<String> {
        for _ in 0..2 {
            if let Some(model) = self
                .list_models()?
                .into_iter()
                .filter(|m| role_matches(role, m))
                .max_by(|a, b| compare_versions(a, b, role))
            {
                return Ok(model);
            }
        }
        Err(Failure::new(
            "model_not_found",
            format!("no model for role {role:?} in model/list"),
        ))
    }
}
impl Drop for AppServer {
    fn drop(&mut self) {
        self.close();
    }
}
fn version<'a>(role: &str, model: &'a str) -> Option<&'a str> {
    let v = model
        .strip_prefix("gpt-")?
        .strip_suffix(&format!("-{role}"))?;
    if !role.is_empty()
        && v.split('.')
            .all(|s| !s.is_empty() && s.bytes().all(|c| c.is_ascii_digit()))
    {
        Some(v)
    } else {
        None
    }
}
pub fn role_matches(role: &str, model: &str) -> bool {
    version(role, model).is_some()
}
fn compare_versions(a: &str, b: &str, role: &str) -> std::cmp::Ordering {
    let a: Vec<_> = version(role, a).unwrap().split('.').collect();
    let b: Vec<_> = version(role, b).unwrap().split('.').collect();
    for (x, y) in a.iter().zip(&b) {
        let x = x.trim_start_matches('0');
        let y = y.trim_start_matches('0');
        let cmp = x.len().cmp(&y.len()).then_with(|| x.cmp(y));
        if cmp != std::cmp::Ordering::Equal {
            return cmp;
        }
    }
    a.len().cmp(&b.len())
}
