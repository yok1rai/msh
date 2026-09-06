use nix::unistd::{geteuid, gethostname};
use rustyline::{DefaultEditor, error::ReadlineError};
use std::{env, error::Error, fs::OpenOptions, io::Read, str::FromStr};

pub fn read_file(path: &str) -> std::io::Result<String> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}

pub fn input<T: Default>(prompt: &str) -> Result<T, Box<dyn Error>>
where
    T: FromStr,
    T::Err: Error + 'static,
{
    let mut rl = DefaultEditor::new()?;
    let buffer = match rl.readline(prompt) {
        Ok(line) => line,
        Err(ReadlineError::Interrupted) => {
            eprintln!("^C");
            return Ok(T::default());
        }
        Err(ReadlineError::Eof) => {
            eprintln!("^D");

            return Ok(T::default());
        }
        Err(e) => return Err(Box::new(e)),
    };
    Ok(buffer.trim().parse::<T>()?)
}

pub fn parse(src: &str) -> Result<Vec<Vec<String>>, String> {
    let mut result = Vec::new();

    for line in src.lines() {
        let mut args = Vec::new();
        let mut current = String::new();
        let mut quoted = false;

        for c in line.chars() {
            match c {
                '#' if !quoted => break,

                '"' => {
                    quoted = !quoted;
                }

                ' ' | '\t' if !quoted => {
                    if !current.is_empty() {
                        args.push(std::mem::take(&mut current));
                    }
                }

                _ => {
                    current.push(c);
                }
            }
        }

        if quoted {
            return Err("undetermined quote".into());
        }

        if !current.is_empty() {
            args.push(current);
        }

        result.push(args);
    }

    Ok(result)
}

pub fn expand_builtin_vars() {
    let prompt_symbol = { if geteuid().is_root() { "#" } else { "$" } };
    let hostname = gethostname().unwrap_or("anon".into());
    let pwd = env::current_dir()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();
    let prompt_pwd = if let Ok(home) = env::var("HOME") {
        if pwd == home {
            "~".to_string()
        } else if let Some(rest) = pwd.strip_prefix(&(home + "/")) {
            format!("~/{rest}")
        } else {
            pwd.clone()
        }
    } else {
        pwd.clone()
    };
    unsafe {
        env::set_var("PROMPT_SYMBOL", prompt_symbol);
        env::set_var("HOSTNAME", hostname);
        env::set_var("PROMPT_PWD", prompt_pwd);
    }
}

pub fn expand(args: Vec<String>) -> Vec<String> {
    args.into_iter()
        .map(|arg| {
            let mut result = String::new();
            let mut chars = arg.chars().peekable();

            while let Some(c) = chars.next() {
                if c == '$' {
                    let mut name = String::new();

                    while let Some(&c) = chars.peek() {
                        if c.is_alphanumeric() || c == '_' {
                            name.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    if name.is_empty() {
                        result.push('$');
                    } else {
                        match env::var(&name) {
                            Ok(value) => result.push_str(&value),
                            Err(_) => {
                                result.push('$');
                                result.push_str(&name);
                            }
                        }
                    }
                } else {
                    result.push(c);
                }
            }

            result
        })
        .collect()
}
