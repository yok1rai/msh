use mshell::builtin;

#[cfg(test)]
mod tests {
    use super::*;

    fn args(args: &[&str]) -> Vec<String> {
        args.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn builtin_rejects_empty_args() {
        let result = builtin::BuiltIn::new(&[]);

        assert!(result.is_err());

        let err = result.unwrap_err();

        assert!(err.downcast_ref::<builtin::BuiltInError>().is_some());
    }

    #[test]
    fn builtin_rejects_unknown_command() {
        let result = builtin::BuiltIn::new(&args(&["foobar"]));

        assert!(result.is_err());

        let err = result.unwrap_err();

        assert!(err.downcast_ref::<builtin::NotBuiltInError>().is_some());
    }

    #[test]
    fn builtin_accepts_cd() {
        assert!(builtin::BuiltIn::new(&args(&["cd"])).is_ok());
    }

    #[test]
    fn builtin_accepts_echo() {
        assert!(builtin::BuiltIn::new(&args(&["echo"])).is_ok());
    }

    #[test]
    fn builtin_accepts_exit() {
        assert!(builtin::BuiltIn::new(&args(&["exit"])).is_ok());
    }

    #[test]
    fn builtin_accepts_pwd() {
        assert!(builtin::BuiltIn::new(&args(&["pwd"])).is_ok());
    }

    #[test]
    fn builtin_accepts_shrug() {
        assert!(builtin::BuiltIn::new(&args(&["shrug"])).is_ok());
    }
}
