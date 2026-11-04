use crate::{
    base::{Failure, Res},
    AppError, CommandOutput,
};
use std::{
    io::{Read, Write},
    process::{Command, Stdio},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

pub fn run(program: &str, args: &[String], timeout: Duration) -> Result<CommandOutput, AppError> {
    let mut argv = vec![program.to_owned()];
    argv.extend_from_slice(args);
    capture(&mut crate::base::proc::command(&argv), None, timeout)
        .map_err(|e| AppError::new(e.kind, e.message))
}
pub fn capture(
    command: &mut Command,
    input: Option<&[u8]>,
    timeout: Duration,
) -> Res<CommandOutput> {
    command
        .stdin(if input.is_some() {
            Stdio::piped()
        } else {
            Stdio::null()
        })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = command
        .spawn()
        .map_err(|e| Failure::new("spawn", e.to_string()))?;
    let read = |mut r: Box<dyn Read + Send>| {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let mut b = Vec::new();
            let _ = tx.send(r.read_to_end(&mut b).map(|_| b));
        });
        rx
    };
    let out = read(Box::new(child.stdout.take().unwrap()));
    let err = read(Box::new(child.stderr.take().unwrap()));
    let start = Instant::now();
    if let Some(input) = input {
        let bytes = input.to_vec();
        let mut stdin = child.stdin.take().unwrap();
        thread::spawn(move || {
            let _ = stdin.write_all(&bytes);
        });
    }
    let status = loop {
        match child.try_wait() {
            Ok(Some(s)) => break s,
            Ok(None) if start.elapsed() < timeout => thread::sleep(Duration::from_millis(10)),
            Ok(None) => {
                crate::base::proc::kill_tree(&mut child);
                return Err(Failure::new("timeout", "process deadline exceeded"));
            }
            Err(e) => {
                crate::base::proc::kill_tree(&mut child);
                return Err(Failure::new("process", e.to_string()));
            }
        }
    };
    let mut receive = |rx: mpsc::Receiver<std::io::Result<Vec<u8>>>| -> Res<String> {
        match rx.recv_timeout(timeout.saturating_sub(start.elapsed())) {
            Ok(Ok(b)) => Ok(String::from_utf8_lossy(&b).into_owned()),
            Ok(Err(e)) => Err(Failure::new("process", e.to_string())),
            Err(_) => {
                crate::base::proc::kill_tree(&mut child);
                Err(Failure::new("timeout", "output pipe deadline exceeded"))
            }
        }
    };
    Ok(CommandOutput {
        exit_code: status.code().unwrap_or(-1),
        stdout: receive(out)?,
        stderr: receive(err)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timeout_does_not_wait_for_inherited_pipes() {
        #[cfg(windows)]
        let (program, args) = (
            "cmd",
            vec!["/C", "start /B cmd /C ping -n 6 127.0.0.1 >NUL"],
        );
        #[cfg(unix)]
        let (program, args) = ("sh", vec!["-c", "sleep 5 &"]);
        let args = args.into_iter().map(String::from).collect::<Vec<_>>();
        let started = Instant::now();
        let error = run(program, &args, Duration::from_millis(250)).unwrap_err();
        assert_eq!(error.kind, "timeout");
        assert!(started.elapsed() < Duration::from_secs(4));
    }

    #[test]
    fn timeout_terminates_running_process() {
        #[cfg(windows)]
        let (program, args) = ("cmd", vec!["/C", "ping -n 6 127.0.0.1 >NUL"]);
        #[cfg(unix)]
        let (program, args) = ("sh", vec!["-c", "sleep 5"]);
        let args = args.into_iter().map(String::from).collect::<Vec<_>>();
        let error = run(program, &args, Duration::from_millis(250)).unwrap_err();
        assert_eq!(error.kind, "timeout", "{error}");
    }
}
