// Unlicense — cochranblock.org
//! provenance-docs: AI-piloted development documentation framework.
//! Hot reload via PID lockfile + SIGTERM/SIGKILL handoff.

use std::fs;
use std::io::Read as _;
use std::path::PathBuf;
use std::process;
use std::thread;
use std::time::Duration;

use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;

fn lockfile_path() -> PathBuf {
    let base = dirs_or_home();
    let dir = base.join("provenance-docs");
    let _ = fs::create_dir_all(&dir);
    dir.join("pid")
}

fn dirs_or_home() -> PathBuf {
    if let Some(d) = std::env::var_os("XDG_DATA_HOME") {
        PathBuf::from(d)
    } else if let Some(h) = std::env::var_os("HOME") {
        PathBuf::from(h).join(".local").join("share")
    } else {
        PathBuf::from("/tmp")
    }
}

fn read_old_pid(path: &PathBuf) -> Option<i32> {
    let mut buf = String::new();
    fs::File::open(path).ok()?.read_to_string(&mut buf).ok()?;
    buf.trim().parse().ok()
}

fn retire_old(pid: i32) {
    let nix_pid = Pid::from_raw(pid);

    // SIGTERM — graceful shutdown
    if kill(nix_pid, Signal::SIGTERM).is_err() {
        // Process already gone
        return;
    }
    println!("sent SIGTERM to old pid {pid}, waiting 5s");

    for _ in 0..50 {
        thread::sleep(Duration::from_millis(100));
        if kill(nix_pid, None).is_err() {
            println!("old pid {pid} exited");
            return;
        }
    }

    // SIGKILL — force
    println!("old pid {pid} still alive, sending SIGKILL");
    let _ = kill(nix_pid, Signal::SIGKILL);
    thread::sleep(Duration::from_millis(200));
}

fn write_pid(path: &PathBuf) {
    let pid = process::id();
    if let Err(e) = fs::write(path, pid.to_string()) {
        eprintln!("failed to write pidfile: {e}");
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let subcommand = args.get(1).map(|s| s.as_str());

    if subcommand == Some("generate-toi") {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        process::exit(provenance_docs::generate_toi(root));
    }

    let lock = lockfile_path();

    // Hot reload: retire old instance
    if let Some(old_pid) = read_old_pid(&lock) {
        let my_pid = process::id() as i32;
        if old_pid != my_pid {
            retire_old(old_pid);
        }
    }

    // Claim the lockfile
    write_pid(&lock);

    // Run doc validation
    let failures = provenance_docs::f30();

    // Clean up pidfile on exit
    let _ = fs::remove_file(&lock);

    process::exit(if failures == 0 { 0 } else { 1 });
}
