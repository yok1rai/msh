use msh::utils::parse;

#[test]
fn parse_test() {
    println!("test 1, conjoined word test: helloworld");
    assert_eq!(parse("helloworld").unwrap(), ["helloworld"]);
    println!("passed");

    println!("task 2, multiple words test: hello world test");
    assert_eq!(
        parse("hello world test").unwrap(),
        ["hello", "world", "test"]
    );
    println!("passed");

    println!("task 3, multiple words as one element test: hello \"world bro\"");
    assert_eq!(
        parse("hello \"world bro\"").unwrap(),
        ["hello", "world bro"]
    );
    println!("passed");

    println!("task 4, odd number of quotes should error: hello \"world \"bro\"");
    let result = parse("hello \"world \"bro\"");
    assert!(result.is_err(), "expected an error, got {:?}", result);
    assert_eq!(result.unwrap_err(), "undetermined quote");
    println!("passed");
}
