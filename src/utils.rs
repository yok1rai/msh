use std::{
    error::Error,
    fs::OpenOptions,
    io::{self, Read, Write},
    str::FromStr,
};

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
    print!("{prompt}");
    io::stdout().flush()?;

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

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
