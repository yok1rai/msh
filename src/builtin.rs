use std::{
    env,
    error::Error,
    io::{self, Write},
    path::PathBuf,
    process,
};

pub struct BuiltIn<'a> {
    command: &'a str,
    args: Vec<&'a str>,
}

pub const BUILTNS: [&str; 6] = ["cd", "echo", "exit", "pwd", "type", "shrug"];

impl<'a> BuiltIn<'a> {
    pub fn new(input: &'a str) -> Option<Self> {
        let args: Vec<&'a str> = input.split_whitespace().collect();

        if args.is_empty() {
            return None;
        }

        Some(Self {
            command: args[0],
            args,
        })
    }
    pub fn execute(&self) -> Result<(), Box<dyn Error>> {
        match self.command {
            "cd" => self.cd(),
            "echo" => self.echo(),
            "exit" => self.exit(),
            "pwd" => self.pwd(),
            "type" => self._type(),
            "shrug" => self.shrug(),
            _ => Ok(()),
        }
    }

    fn cd(&self) -> Result<(), Box<dyn Error>> {
        if let Some(arg) = self.args.get(1) {
            if std::env::current_dir()?.to_string_lossy() == "/" && arg == &".." {
                eprintln!("you cannot go below the root");
                return Ok(());
            } else if arg == &"~" {
                env::set_current_dir(env::var("HOME")?)?;
                println!("you switched to {}", env::current_dir()?.to_string_lossy());
                return Ok(());
            }
            env::set_current_dir(arg)?;
            println!("you switched to {}", env::current_dir()?.to_string_lossy())
        } else {
            eprintln!("you should enter a target");
        }
        Ok(())
    }
    fn pwd(&self) -> Result<(), Box<dyn Error>> {
        println!("{}", std::env::current_dir()?.to_string_lossy());
        Ok(())
    }
    fn echo(&self) -> Result<(), Box<dyn Error>> {
        for (i, arg) in self.args[1..].iter().enumerate() {
            if i > 0 {
                print!(" ");
            }
            print!("{arg}",);
        }
        println!();
        io::stdout().flush()?;
        Ok(())
    }
    fn exit(&self) -> Result<(), Box<dyn Error>> {
        process::exit(0);
    }
    fn _type(&self) -> Result<(), Box<dyn Error>> {
        if let Some(arg) = self.args.get(1) {
            if BUILTNS.contains(arg) {
                println!("{} is a shell-builtin", arg);
            } else if self.find_in_path().is_some() {
                println!("{} is a binary", arg);
            } else {
                eprintln!("{} not found", arg);
            }
        } else {
            eprintln!("you must enter a target");
        }
        Ok(())
    }
    fn find_in_path(&self) -> Option<PathBuf> {
        let command = self.args.get(1)?;
        let path = env::var_os("PATH")?;

        for directory in env::split_paths(&path) {
            let candidate = directory.join(command);

            if candidate.is_file() {
                return Some(candidate);
            }
        }

        None
    }
    fn shrug(&self) -> Result<(), Box<dyn Error>> {
        println!("_        _");
        println!(" \\_(ツ)_/");
        Ok(())
    }
}
