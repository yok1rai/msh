use nix::{
    sys::wait::{WaitPidFlag, WaitStatus, waitpid},
    unistd::{ForkResult, execvp, fork},
};
use std::error::Error;
use std::ffi::CString;

pub extern "C" fn sigchld_handler(_: i32) {
    loop {
        match waitpid(nix::unistd::Pid::from_raw(-1), Some(WaitPidFlag::WNOHANG)) {
            Ok(WaitStatus::StillAlive) | Err(_) => break,
            Ok(_) => {}
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

