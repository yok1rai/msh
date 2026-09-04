use std::{
    error::Error,
    io::{self, Write},
    str::FromStr,
};

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

pub fn parse(src: &str) -> Result<Vec<String>, String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut quoted = false;

    for c in src.chars() {
        match c {
            '"' => quoted = !quoted,
            ' ' | '\t' if !quoted => {
                if !current.is_empty() {
                    args.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(c),
        }
    }

    if quoted {
        return Err("undetermined quote".into());
    }

    if !current.is_empty() {
        args.push(current);
    }

    Ok(args)
}
