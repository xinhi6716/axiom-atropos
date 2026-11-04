use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Brief {
    pub goal: String,
    pub scope: Vec<String>,
    pub verify: String,
    pub context: Option<String>,
    pub text: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub id: String,
    pub purpose: String,
    pub paths: Vec<String>,
    pub depends: Vec<String>,
    pub contract: String,
    pub accept_file: String,
    pub accept_command: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Map {
    pub preamble: String,
    pub modules: Vec<Module>,
    pub full_test: Option<String>,
}
fn fields(text: &str, names: &[&str]) -> HashMap<String, String> {
    let mut result = HashMap::<String, String>::new();
    let mut active: Option<&str> = None;
    for line in text.lines() {
        let field = names.iter().find(|name| {
            line.strip_prefix(**name)
                .is_some_and(|s| s.is_empty() || s.starts_with(char::is_whitespace))
        });
        if let Some(name) = field {
            active = Some(name);
            result
                .entry(name.to_string())
                .or_default()
                .push_str(line[name.len()..].trim());
        } else if line.starts_with(char::is_whitespace) {
            if let Some(name) = active {
                let value = result.get_mut(name).unwrap();
                if !value.is_empty() {
                    value.push('\n');
                }
                value.push_str(line.trim());
            }
        } else {
            active = None;
        }
    }
    result
}
fn words(s: &str) -> Vec<String> {
    s.split(|c: char| c.is_whitespace() || c == ',')
        .filter(|v| !v.is_empty())
        .map(str::to_string)
        .collect()
}
pub fn parse_brief(text: &str) -> Result<Brief, String> {
    let start = text.lines().position(|line| {
        line.strip_prefix("GOAL")
            .is_some_and(|rest| rest.is_empty() || rest.starts_with(char::is_whitespace))
    });
    let body = start
        .map(|n| text.lines().skip(n).collect::<Vec<_>>().join("\n"))
        .unwrap_or_default();
    let f = fields(&body, &["GOAL", "SCOPE", "VERIFY", "CONTEXT"]);
    for n in ["GOAL", "SCOPE", "VERIFY"] {
        if f.get(n).is_none_or(|s| s.trim().is_empty()) {
            return Err(n.into());
        }
    }
    if words(&f["SCOPE"]).is_empty() {
        return Err("SCOPE".into());
    }
    Ok(Brief {
        goal: f["GOAL"].clone(),
        scope: words(&f["SCOPE"]),
        verify: f["VERIFY"].clone(),
        context: f.get("CONTEXT").cloned(),
        text: text.into(),
    })
}
fn normalized(s: &str) -> String {
    let s = s.replace('\\', "/");
    s.trim_start_matches("./").to_string()
}
pub fn covers(patterns: &[String], path: &str) -> bool {
    let p = normalized(path);
    patterns.iter().any(|x| {
        let x = normalized(x);
        x == p || (x.ends_with('/') && p.starts_with(&x))
    })
}
pub fn overlaps(a: &[String], b: &[String]) -> bool {
    a.iter().any(|x| covers(b, x)) || b.iter().any(|x| covers(a, x))
}
fn backtick(s: &str) -> Option<String> {
    let (_, tail) = s.split_once('`')?;
    Some(tail.split_once('`')?.0.to_string())
}
pub fn parse_map(text: &str) -> Result<Map, String> {
    let mut preamble = String::new();
    let mut sections: Vec<(String, String)> = Vec::new();
    let mut full_test = None;
    for line in text.lines() {
        if let Some(v) = line
            .strip_prefix("FULL TEST")
            .filter(|v| v.is_empty() || v.starts_with(char::is_whitespace))
        {
            full_test = Some(backtick(v).unwrap_or_else(|| v.trim().into()));
            break;
        }
        if let Some(id) = line.strip_prefix("## ") {
            sections.push((id.trim().into(), String::new()));
        } else if let Some((_, body)) = sections.last_mut() {
            body.push_str(line);
            body.push('\n');
        } else {
            preamble.push_str(line);
            preamble.push('\n');
        }
    }
    let mut modules: Vec<Module> = Vec::new();
    for (id, body) in sections {
        let f = fields(
            &body,
            &["PURPOSE", "PATHS", "DEPENDS", "CONTRACT", "ACCEPT"],
        );
        if modules.iter().any(|m| m.id == id) {
            return Err(format!("module {id}: duplicate id"));
        }
        for n in ["PURPOSE", "PATHS", "DEPENDS", "CONTRACT", "ACCEPT"] {
            if f.get(n).is_none_or(|s| s.trim().is_empty()) {
                return Err(format!("module {id}: missing {n}"));
            }
        }
        let depends = if f["DEPENDS"].trim() == "none" {
            vec![]
        } else {
            words(&f["DEPENDS"])
        };
        if f["DEPENDS"].trim() != "none" && depends.is_empty() {
            return Err(format!("module {id}: missing DEPENDS"));
        }
        for dep in &depends {
            if !modules.iter().any(|m| m.id == *dep) {
                return Err(format!("module {id}: unknown or later dependency {dep}"));
            }
        }
        let paths = words(&f["PATHS"]);
        if paths.is_empty() {
            return Err(format!("module {id}: missing PATHS"));
        }
        for m in &modules {
            if overlaps(&paths, &m.paths) {
                return Err(format!("module {id}: PATHS overlap module {}", m.id));
            }
        }
        let accept = &f["ACCEPT"];
        let first = accept
            .split_whitespace()
            .next()
            .unwrap_or("")
            .trim_end_matches(',');
        let command = backtick(accept).unwrap_or_else(|| {
            accept
                .split_once(char::is_whitespace)
                .map(|(_, rest)| rest.trim().to_string())
                .unwrap_or_default()
        });
        if first.is_empty() || command.is_empty() {
            return Err(format!("module {id}: missing ACCEPT"));
        }
        modules.push(Module {
            id,
            purpose: f["PURPOSE"].clone(),
            paths,
            depends,
            contract: f["CONTRACT"].clone(),
            accept_file: first.into(),
            accept_command: command,
        });
    }
    Ok(Map {
        preamble,
        modules,
        full_test,
    })
}
