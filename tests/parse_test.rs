use msh::utils::parse;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty_input() {
        assert_eq!(parse(""), Ok(vec![]));
    }

    #[test]
    fn parse_single_command() {
        assert_eq!(parse("echo"), Ok(vec!["echo".to_string()]));
    }

    #[test]
    fn parse_multiple_arguments() {
        assert_eq!(
            parse("echo hello world"),
            Ok(vec![
                "echo".to_string(),
                "hello".to_string(),
                "world".to_string(),
            ])
        );
    }

    #[test]
    fn parse_multiple_spaces() {
        assert_eq!(
            parse("echo   hello    world"),
            Ok(vec![
                "echo".to_string(),
                "hello".to_string(),
                "world".to_string(),
            ])
        );
    }

    #[test]
    fn parse_tabs() {
        assert_eq!(
            parse("echo\thello\tworld"),
            Ok(vec![
                "echo".to_string(),
                "hello".to_string(),
                "world".to_string(),
            ])
        );
    }

    #[test]
    fn parse_quoted_argument() {
        assert_eq!(
            parse(r#"echo "hello world""#),
            Ok(vec!["echo".to_string(), "hello world".to_string(),])
        );
    }

    #[test]
    fn parse_empty_quoted_argument() {
        assert_eq!(
            parse(r#"echo "" test"#),
            Ok(vec!["echo".to_string(), "test".to_string(),])
        );
    }

    #[test]
    fn parse_unterminated_quote() {
        assert_eq!(
            parse(r#"echo "hello"#),
            Err("undetermined quote".to_string())
        );
    }

    #[test]
    fn parse_quotes_inside_argument() {
        assert_eq!(
            parse(r#"echo foo"bar"baz"#),
            Ok(vec!["echo".to_string(), "foobarbaz".to_string(),])
        );
    }

    #[test]
    fn parse_background_command() {
        assert_eq!(
            parse("sleep 10 &"),
            Ok(vec!["sleep".to_string(), "10".to_string(), "&".to_string(),])
        );
    }

    #[test]
    fn parse_full_line_comment() {
        assert_eq!(parse("# this is a comment"), Ok(vec![]));
    }

    #[test]
    fn parse_inline_comment() {
        assert_eq!(
            parse("echo hello # this is a comment"),
            Ok(vec!["echo".to_string(), "hello".to_string(),])
        );
    }

    #[test]
    fn parse_hash_inside_quotes() {
        assert_eq!(
            parse(r##"echo "hello # world""##),
            Ok(vec!["echo".to_string(), "hello # world".to_string(),])
        );
    }

    #[test]
    fn parse_comment_on_multiple_lines() {
        assert_eq!(
            parse(
                r#"
                # first comment
                echo hello
                # second comment
                echo world
                "#
            ),
            Ok(vec![
                "echo".to_string(),
                "hello".to_string(),
                "echo".to_string(),
                "world".to_string(),
            ])
        );
    }
}
