use nix::{
    sys::wait::{WaitPidFlag, WaitStatus, waitpid},
    unistd::{ForkResult, Pid, execvp, fork},
};
use std::error::Error;
use std::ffi::CString;
use std::sync::atomic::{AtomicBool, Ordering};

pub static CHILD_EXITED: AtomicBool = AtomicBool::new(false);

pub extern "C" fn sigchld_handler(_: i32) {
    CHILD_EXITED.store(true, Ordering::Relaxed);
}

pub fn reap_children() {
    loop {
        match waitpid(Pid::from_raw(-1), Some(WaitPidFlag::WNOHANG)) {
            Ok(WaitStatus::Exited(pid, status)) => {
                println!("[BG PID: {pid}] exited with status {status}");
            }
            Ok(WaitStatus::Signaled(pid, signal, _)) => {
                println!("[BG PID: {pid}] exited by signal {signal}");
            }
            Ok(WaitStatus::StillAlive) => break,
            Ok(_) => {}
            Err(nix::errno::Errno::ECHILD) => break,
            Err(e) => {
                eprintln!("wait pid: {e}");
                break;
            }
        }
    }
}

pub fn run(mut command: Vec<String>) -> Result<(), Box<dyn Error>> {
    let bg = command.last().map(|s| s.trim() == "&").unwrap_or(false);

    if bg {
        command.pop();
    }

    if command.is_empty() {
        return Ok(());
    }

    let args: Vec<CString> = command
        .into_iter()
        .map(CString::new)
        .collect::<Result<_, _>>()?;

    match unsafe { fork()? } {
        ForkResult::Child => {
            execvp(&args[0], &args)?;
            unreachable!();
        }

        ForkResult::Parent { child } => {
            if bg {
                println!("[BG PID: {child}]");
            } else {
                waitpid(child, None)?;
            }
        }
    }

    Ok(())
}

