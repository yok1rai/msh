use msh::*;
use nix::sys::signal::{self, SaFlags, SigAction, SigHandler, SigSet, Signal};

fn main() {
    let signals = signals::SignalHandler::default();

    let action = SigAction::new(
        SigHandler::Handler(process::sigchld_handler),
        SaFlags::SA_RESTART,
        SigSet::empty(),
    );

    unsafe {
        signal::sigaction(Signal::SIGCHLD, &action).expect("failed to install SIGCHLD handler");
    }

    loop {
        let command: String = match utils::input("> ") {
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
            Ok(bltn) => bltn.execute().unwrap(),

            Err(e) => {
                if e.downcast_ref::<builtin::NotBuiltInError>().is_some() {
                    match process::run(parsed) {
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

