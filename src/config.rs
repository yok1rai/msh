use crate::*;

pub fn get_prompt() -> String {
    let home = match std::env::var("HOME") {
        Ok(home) => home,
        Err(_) => return "> ".to_string(),
    };
    let path = format!("{home}/.config/msh/main.conf");
    let raw = utils::read_file(&path).unwrap_or_else(|_| "prompt = \"> \"".to_string());
    let parsed = utils::parse(&raw).unwrap_or_default();
    for line in parsed {
        if line.len() >= 3 && line[0] == "prompt" && line[1] == "=" {
            return line[2].clone();
        }
    }
    "> ".to_string()
}
