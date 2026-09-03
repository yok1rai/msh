use nix::unistd::{ForkResult, execvp, fork};
use std::error::Error;
use std::ffi::CString;

pub fn run(command: &str) -> Result<(), Box<dyn Error>> {
    let parts: Vec<&str> = command.split_whitespace().collect();

    if parts.is_empty() {
        return Ok(());
    }

    let args: Vec<CString> = parts
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
