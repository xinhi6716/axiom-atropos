use super::Res;
use std::{
    path::Path,
    process::{Child, Command},
    time::{Duration, Instant},
};
pub fn command(argv: &[String]) -> Command {
    let mut c = Command::new(argv.first().map(String::as_str).unwrap_or(""));
    c.args(argv.iter().skip(1));
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        c.process_group(0);
    }
    c
}
pub fn kill_tree(child: &mut Child) {
    #[cfg(unix)]
    unsafe {
        libc::kill(-(child.id() as i32), libc::SIGKILL);
    }
    #[cfg(windows)]
    {
        kill_windows_descendants(child);
    }
    let _ = child.kill();
    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(10) {
        if !matches!(child.try_wait(), Ok(None)) {
            break;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}
#[cfg(windows)]
fn kill_windows_descendants(child: &Child) {
    use std::ffi::c_void;
    use std::os::windows::io::AsRawHandle;
    #[derive(Clone, Copy, Default)]
    #[repr(C)]
    struct FileTime {
        low: u32,
        high: u32,
    }
    impl FileTime {
        fn value(self) -> u64 {
            (u64::from(self.high) << 32) | u64::from(self.low)
        }
    }
    #[repr(C)]
    struct Entry {
        size: u32,
        usage: u32,
        pid: u32,
        heap: usize,
        module: u32,
        threads: u32,
        parent: u32,
        priority: i32,
        flags: u32,
        exe: [u16; 260],
    }
    #[link(name = "kernel32")]
    extern "system" {
        fn CreateToolhelp32Snapshot(flags: u32, pid: u32) -> *mut c_void;
        fn Process32FirstW(snapshot: *mut c_void, entry: *mut Entry) -> i32;
        fn Process32NextW(snapshot: *mut c_void, entry: *mut Entry) -> i32;
        fn OpenProcess(access: u32, inherit: i32, pid: u32) -> *mut c_void;
        fn TerminateProcess(process: *mut c_void, code: u32) -> i32;
        fn CloseHandle(handle: *mut c_void) -> i32;
        fn GetProcessTimes(
            process: *mut c_void,
            created: *mut FileTime,
            exited: *mut FileTime,
            kernel: *mut FileTime,
            user: *mut FileTime,
        ) -> i32;
        fn GetSystemTimeAsFileTime(time: *mut FileTime);
    }
    unsafe fn created(process: *mut c_void) -> Option<u64> {
        let mut times = [FileTime::default(); 4];
        (GetProcessTimes(
            process,
            &mut times[0],
            &mut times[1],
            &mut times[2],
            &mut times[3],
        ) != 0)
            .then(|| times[0].value())
    }
    unsafe {
        let Some(root_created) = created(child.as_raw_handle()) else {
            return;
        };
        let mut cutoff = FileTime::default();
        GetSystemTimeAsFileTime(&mut cutoff);
        let snapshot = CreateToolhelp32Snapshot(2, 0);
        if snapshot as isize == -1 {
            return;
        }
        let mut entry: Entry = std::mem::zeroed();
        entry.size = std::mem::size_of::<Entry>() as u32;
        let mut edges = Vec::new();
        let mut ok = Process32FirstW(snapshot, &mut entry);
        while ok != 0 {
            edges.push((entry.parent, entry.pid));
            ok = Process32NextW(snapshot, &mut entry);
        }
        CloseHandle(snapshot);
        let mut descendants = vec![(child.id(), root_created)];
        let mut handles = Vec::new();
        let mut i = 0;
        while i < descendants.len() {
            let (parent, parent_created) = descendants[i];
            for &(_, pid) in edges.iter().filter(|(p, _)| *p == parent) {
                if pid == std::process::id() || descendants.iter().any(|(p, _)| *p == pid) {
                    continue;
                }
                // Pin identity while walking and terminating. Parent PID alone
                // can describe a dead, unrelated process whose PID was reused.
                let process = OpenProcess(1 | 0x1000, 0, pid);
                if process.is_null() {
                    continue;
                }
                if let Some(time) = created(process)
                    .filter(|time| *time >= parent_created && *time <= cutoff.value())
                {
                    descendants.push((pid, time));
                    handles.push(process);
                } else {
                    CloseHandle(process);
                }
            }
            i += 1;
        }
        for process in handles.into_iter().rev() {
            TerminateProcess(process, 1);
            CloseHandle(process);
        }
    }
}
pub fn pid_alive(pid: u32) -> bool {
    #[cfg(unix)]
    {
        let r = unsafe { libc::kill(pid as i32, 0) };
        r == 0 || std::io::Error::last_os_error().raw_os_error() != Some(libc::ESRCH)
    }
    #[cfg(windows)]
    unsafe {
        use windows_sys::Win32::{
            Foundation::{CloseHandle, STILL_ACTIVE},
            System::Threading::{
                GetExitCodeProcess, OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION,
            },
        };
        let h = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid);
        if h.is_null() {
            return false;
        }
        let mut code = 0;
        let ok = GetExitCodeProcess(h, &mut code);
        CloseHandle(h);
        ok != 0 && code == STILL_ACTIVE as u32
    }
}
pub struct ShellRun {
    pub exit_code: i32,
    pub output: String,
    pub timed_out: bool,
}
pub fn run_shell(cmd: &str, cwd: &Path, timeout: Duration) -> Res<ShellRun> {
    #[cfg(unix)]
    let args = vec!["sh".into(), "-c".into(), cmd.into()];
    #[cfg(windows)]
    let args = vec!["cmd".into(), "/D".into(), "/S".into(), "/C".into()];
    let mut shell = command(&args);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        shell.raw_arg(format!("\"{cmd}\""));
    }
    shell.current_dir(cwd);
    match crate::process::capture(&mut shell, None, timeout) {
        Ok(o) => Ok(ShellRun {
            exit_code: o.exit_code,
            output: format!("{}{}", o.stdout, o.stderr),
            timed_out: false,
        }),
        Err(e) if e.kind == "timeout" => Ok(ShellRun {
            exit_code: -1,
            output: e.message,
            timed_out: true,
        }),
        Err(e) => Err(e),
    }
}
