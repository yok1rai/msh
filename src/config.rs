use crate::*;

pub fn get_prompt() -> String {
    let home = match std::env::var("HOME") {
        Ok(home) => home,
        Err(_) => return "> ".to_string(),
    };
    let path = format!("{}/.config/msh/main.conf", home);
    let raw = utils::read_file(&path).unwrap_or_else(|_| "prompt = \"> \"".to_string());

    let parsed = utils::parse(&raw).unwrap_or_default();

    for i in 0..parsed.len().saturating_sub(2) {
        if parsed[i] == "prompt" && parsed[i + 1] == "=" {
            return parsed[i + 2].clone();
        }
    }

    "> ".to_string()
}
