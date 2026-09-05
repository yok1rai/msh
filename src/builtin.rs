use std::{
    env,
    error::Error,
    fmt::Display,
    io::{self, Write},
    path::PathBuf,
    process,
};

#[derive(Debug)]
pub struct BuiltIn {
    command: String,
    args: Vec<String>,
}

#[derive(Debug)]
pub struct BuiltInError {
    msg: String,
}

impl Display for BuiltInError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for BuiltInError {}

#[derive(Debug)]
pub struct NotBuiltInError {}

impl Display for NotBuiltInError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "not a builtin")
    }
}

impl Error for NotBuiltInError {}

const BUILTINS: &[&str] = &[
    "cd", "echo", "exit", "pwd", "type", "shrug", "jobs", "eval", "let",
];

impl BuiltIn {
    pub fn new(args: &[String]) -> Result<Self, Box<dyn Error>> {
        if args.is_empty() {
            let err = BuiltInError {
                msg: "No argument given".to_string(),
            };
            Err(Box::new(err))
        } else if !BUILTINS.contains(&args[0].trim()) {
            let err = NotBuiltInError {};
            Err(Box::new(err))
        } else {
            Ok(Self {
                command: args[0].clone(),
                args: args.to_vec(),
            })
        }
    }
    pub fn execute(&self, job_table: &crate::process::JobTable) -> Result<(), Box<dyn Error>> {
        match self.command.as_str() {
            "cd" => self.cd(),
            "echo" => self.echo(),
            "exit" => self.exit(),
            "pwd" => self.pwd(),
            "type" => self._type(),
            "shrug" => self.shrug(),
            "jobs" => Self::jobs(job_table),
            "let" => self._let(),
            "eval" => self.eval(),
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
            if BUILTINS.contains(&arg.as_str()) {
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

    fn jobs(jobs: &crate::process::JobTable) -> Result<(), Box<dyn Error>> {
        if jobs.len() < 1 {
            eprintln!("there is no active jobs");
            return Ok(());
        }
        for job in jobs.iter() {
            let state = match job.state {
                crate::process::JobState::Running => "Running",
                crate::process::JobState::Stopped => "Stopped",
                crate::process::JobState::Done => "Done",
            };
            println!("[{}] {} {}", job.id, state, job.command);
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
    fn _let(&self) -> Result<(), Box<dyn Error>> {
        let Some(name) = self.args.get(1) else {
            eprintln!("you must enter variable name");
            return Ok(());
        };
        let Some(val) = self.args.get(2) else {
            eprintln!("you must enter the value");
            return Ok(());
        };
        unsafe {
            env::set_var(name, val);
        }
        println!("{name} variable set to {val}");
        Ok(())
    }
    fn eval(&self) -> Result<(), Box<dyn Error>> {
        if let Some(op) = self.args.get(2) {
            let num1: f64 = match self.args[1].trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("you must enter a number");
                    return Ok(());
                }
            };
            let num2: f64 = if let Some(num) = self.args.get(3) {
                match num.trim().parse::<f64>() {
                    Ok(num) => num,
                    Err(_) => {
                        eprintln!("you must enter a number");
                        return Ok(());
                    }
                }
            } else {
                eprintln!("you must enter second number");
                return Ok(());
            };
            let op_result = match op.trim() {
                "+" => num1 + num2,
                "-" => num1 - num2,
                "*" => num1 * num2,
                "/" => {
                    if num2 == 0.0 {
                        eprintln!("you cannot divide by zero");
                        return Ok(());
                    }
                    num1 / num2
                }
                "%" => {
                    if num2 == 0.0 {
                        eprintln!("you cannot module by zero");
                        return Ok(());
                    }
                    num1 % num2
                }
                "@" => num1.powf(1.0 / num2),
                _ => {
                    eprintln!("invalid operator");
                    return Ok(());
                }
            };
            println!("{op_result}");
        } else {
            eprintln!("You must specify an operator");
        }
        Ok(())
    }

    fn shrug(&self) -> Result<(), Box<dyn Error>> {
        println!("_        _");
        println!(" \\_(ツ)_/");
        Ok(())
    }
}
