use serde::{Deserialize, Serialize};
use std::{fmt, path::PathBuf};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct AppError {
    pub kind: String,
    pub message: String,
}
impl AppError {
    pub fn new(kind: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            kind: kind.into(),
            message: message.into(),
        }
    }
}
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.message)
    }
}
impl std::error::Error for AppError {}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CommandOutput {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}
impl CommandOutput {
    pub fn success(stdout: impl Into<String>) -> Self {
        Self {
            exit_code: 0,
            stdout: stdout.into(),
            stderr: String::new(),
        }
    }
    pub fn error(exit_code: i32, message: impl Into<String>) -> Self {
        Self {
            exit_code,
            stdout: String::new(),
            stderr: message.into(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Context {
    pub asset_root: Option<PathBuf>,
}
impl Context {
    pub fn new(asset_root: Option<PathBuf>) -> Self {
        Self { asset_root }
    }
    pub fn data_root(&self) -> Result<PathBuf, AppError> {
        std::env::var_os("AXIOM_DATA_HOME").filter(|s| !s.is_empty())
            .or_else(|| std::env::var_os("CLAUDE_PLUGIN_DATA").filter(|s| !s.is_empty()))
            .map(PathBuf::from)
            .ok_or_else(|| AppError::new("config", "neither AXIOM_DATA_HOME nor CLAUDE_PLUGIN_DATA is set, so there is nowhere to keep state"))
    }
}
