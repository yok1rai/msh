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
            return Ok(T::default());
        }
        Err(ReadlineError::Eof) => {
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
