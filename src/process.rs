use nix::unistd::{ForkResult, execvp, fork};
use std::error::Error;
use std::ffi::CString;

pub fn run(command: Vec<String>) -> Result<(), Box<dyn Error>> {
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
            nix::sys::wait::waitpid(child, None)?;
        }
    }

    Ok(())
}

