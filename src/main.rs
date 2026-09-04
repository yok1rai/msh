use msh::{builtin::BuiltIn, *};

fn main() {
    let signals = signals::SignalHandler::default();

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
        match BuiltIn::new(&parsed) {
            Ok(bltn) => bltn.execute().unwrap(),
            Err(e) => {
                if let Some(_) = e.downcast_ref::<builtin::NotBuiltInError>() {
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
