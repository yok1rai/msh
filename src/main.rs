use msh::*;

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

        let command_name = match command.split_whitespace().next() {
            Some(command) => command,
            None => continue,
        };

        if builtin::BUILTNS.contains(&command_name) {
            if let Some(builtin) = builtin::BuiltIn::new(&command) {
                if let Err(e) = builtin.execute() {
                    eprintln!("{e}");
                }
            }
            continue;
        }

        if let Err(e) = process::run(&command) {
            eprintln!("{e}");
        }
    }
}

