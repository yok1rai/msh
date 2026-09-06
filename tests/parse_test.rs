use mshell::utils::parse;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty_input() {
        assert_eq!(parse(""), Ok(vec![]));
    }

    #[test]
    fn parse_single_command() {
        assert_eq!(parse("echo"), Ok(vec![vec!["echo".to_string()]]));
    }

    #[test]
    fn parse_multiple_arguments() {
        assert_eq!(
            parse("echo hello world"),
            Ok(vec![vec![
                "echo".to_string(),
                "hello".to_string(),
                "world".to_string(),
            ]])
        );
    }

    #[test]
    fn parse_multiple_spaces() {
        assert_eq!(
            parse("echo   hello    world"),
            Ok(vec![vec![
                "echo".to_string(),
                "hello".to_string(),
                "world".to_string(),
            ]])
        );
    }

    #[test]
    fn parse_tabs() {
        assert_eq!(
            parse("echo\thello\tworld"),
            Ok(vec![vec![
                "echo".to_string(),
                "hello".to_string(),
                "world".to_string(),
            ]])
        );
    }

    #[test]
    fn parse_mixed_spaces_and_tabs() {
        assert_eq!(
            parse("echo \t hello\tworld"),
            Ok(vec![vec![
                "echo".to_string(),
                "hello".to_string(),
                "world".to_string(),
            ]])
        );
    }

    #[test]
    fn parse_quoted_argument() {
        assert_eq!(
            parse(r#"echo "hello world""#),
            Ok(vec![vec!["echo".to_string(), "hello world".to_string(),]])
        );
    }

    #[test]
    fn parse_empty_quoted_argument() {
        assert_eq!(
            parse(r#"echo "" test"#),
            Ok(vec![vec!["echo".to_string(), "test".to_string(),]])
        );
    }

    #[test]
    fn parse_quotes_inside_argument() {
        assert_eq!(
            parse(r#"echo foo"bar"baz"#),
            Ok(vec![vec!["echo".to_string(), "foobarbaz".to_string(),]])
        );
    }

    #[test]
    fn parse_multiple_quoted_arguments() {
        assert_eq!(
            parse(r#"echo "hello world" "foo bar""#),
            Ok(vec![vec![
                "echo".to_string(),
                "hello world".to_string(),
                "foo bar".to_string(),
            ]])
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
    fn parse_unterminated_quote_on_second_line() {
        assert_eq!(
            parse(
                r#"echo hello
echo "world"#
            ),
            Err("undetermined quote".to_string())
        );
    }

    #[test]
    fn parse_background_command() {
        assert_eq!(
            parse("sleep 10 &"),
            Ok(vec![vec![
                "sleep".to_string(),
                "10".to_string(),
                "&".to_string(),
            ]])
        );
    }

    #[test]
    fn parse_full_line_comment() {
        assert_eq!(parse("# this is a comment"), Ok(vec![vec![]]));
    }

    #[test]
    fn parse_inline_comment() {
        assert_eq!(
            parse("echo hello # this is a comment"),
            Ok(vec![vec!["echo".to_string(), "hello".to_string(),]])
        );
    }

    #[test]
    fn parse_hash_inside_quotes() {
        assert_eq!(
            parse(r##"echo "hello # world""##),
            Ok(vec![vec!["echo".to_string(), "hello # world".to_string(),]])
        );
    }

    #[test]
    fn parse_hash_at_start_inside_quotes() {
        assert_eq!(
            parse(r##"echo "#hello""##),
            Ok(vec![vec!["echo".to_string(), "#hello".to_string(),]])
        );
    }

    #[test]
    fn parse_hash_after_quoted_argument() {
        assert_eq!(
            parse(r##"echo "hello" # comment"##),
            Ok(vec![vec!["echo".to_string(), "hello".to_string(),]])
        );
    }

    #[test]
    fn parse_comment_only_after_whitespace() {
        assert_eq!(parse("   # comment"), Ok(vec![vec![]]));
    }

    #[test]
    fn parse_multiple_lines() {
        assert_eq!(
            parse(
                r#"echo hello
echo world
sleep 10"#
            ),
            Ok(vec![
                vec!["echo".to_string(), "hello".to_string(),],
                vec!["echo".to_string(), "world".to_string(),],
                vec!["sleep".to_string(), "10".to_string(),],
            ])
        );
    }

    #[test]
    fn parse_multiple_lines_with_comments() {
        assert_eq!(
            parse(
                r#"# first comment
echo hello
# second comment
echo world"#
            ),
            Ok(vec![
                vec![],
                vec!["echo".to_string(), "hello".to_string(),],
                vec![],
                vec!["echo".to_string(), "world".to_string(),],
            ])
        );
    }

    #[test]
    fn parse_multiple_lines_with_inline_comments() {
        assert_eq!(
            parse(
                r#"echo hello # first comment
echo world # second comment"#
            ),
            Ok(vec![
                vec!["echo".to_string(), "hello".to_string(),],
                vec!["echo".to_string(), "world".to_string(),],
            ])
        );
    }

    #[test]
    fn parse_empty_lines() {
        assert_eq!(
            parse(
                r#"echo hello

echo world"#
            ),
            Ok(vec![
                vec!["echo".to_string(), "hello".to_string(),],
                vec![],
                vec!["echo".to_string(), "world".to_string(),],
            ])
        );
    }

    #[test]
    fn parse_whitespace_only_line() {
        assert_eq!(
            parse("echo hello\n   \t"),
            Ok(vec![vec!["echo".to_string(), "hello".to_string(),], vec![],])
        );
    }

    #[test]
    fn parse_multiple_whitespace_lines() {
        assert_eq!(
            parse("echo hello\n   \t\n\techo world"),
            Ok(vec![
                vec!["echo".to_string(), "hello".to_string(),],
                vec![],
                vec!["echo".to_string(), "world".to_string(),],
            ])
        );
    }

    #[test]
    fn parse_config_style_input() {
        assert_eq!(
            parse(
                r#"# msh configuration
prompt = "> "
color = true
name = "msh""#
            ),
            Ok(vec![
                vec![],
                vec!["prompt".to_string(), "=".to_string(), "> ".to_string(),],
                vec!["color".to_string(), "=".to_string(), "true".to_string(),],
                vec!["name".to_string(), "=".to_string(), "msh".to_string(),],
            ])
        );
    }

    #[test]
    fn parse_config_with_inline_comment() {
        assert_eq!(
            parse(r#"prompt = "> " # default shell prompt"#),
            Ok(vec![vec![
                "prompt".to_string(),
                "=".to_string(),
                "> ".to_string(),
            ]])
        );
    }

    #[test]
    fn parse_special_characters_inside_quotes() {
        assert_eq!(
            parse(r#"echo "hello # $HOME = foo & bar""#),
            Ok(vec![vec![
                "echo".to_string(),
                "hello # $HOME = foo & bar".to_string(),
            ]])
        );
    }
}
