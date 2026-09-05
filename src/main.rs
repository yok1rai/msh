use msh::*;
use nix::sys::signal::{self, SaFlags, SigAction, SigHandler, SigSet, Signal};
use std::sync::atomic::Ordering;

fn main() {
    let signals = signals::SignalHandler::default();
    let mut job_table = process::JobTable::new();

    let action = SigAction::new(
        SigHandler::Handler(process::sigchld_handler),
        SaFlags::SA_RESTART,
        SigSet::empty(),
    );

    unsafe {
        signal::sigaction(Signal::SIGCHLD, &action).expect("failed to install SIGCHLD handler");
    }

    loop {
        if process::CHILD_EXITED.swap(false, Ordering::Relaxed) {
            process::reap_children(&mut job_table);
        }
        let command: String = match utils::input(config::get_prompt().as_str()) {
            Ok(command) => command,
            Err(e) => {
                if signals.was_interrupted() {
                    println!();
                    continue;
                }

                eprintln!("{e}");
                continue;
            }
        };

        let parsed = match utils::parse(command.trim()) {
            Ok(parsed) => parsed,
            Err(e) => {
                eprintln!("parsing err: {e}");
                continue;
            }
        };

        match builtin::BuiltIn::new(&parsed) {
            Ok(bltn) => bltn.execute(&job_table).unwrap(),

            Err(e) => {
                if e.downcast_ref::<builtin::NotBuiltInError>().is_some() {
                    match process::run(parsed, &mut job_table) {
                        Ok(()) => (),
                        Err(e) => {
                            eprintln!("{e}");
                            continue;
                        }
                    }
                }
            }
        }
    }
}
