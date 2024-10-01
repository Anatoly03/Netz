use util_parser::grammar;

#[grammar(~)]
pub struct Whitespace;

#[test]
fn whitespace() {
    let (input, _) = Whitespace::parse("\t \r\n").unwrap();
    assert_eq!(input, "");
}

#[test]
fn no_whitespace() {
    assert!(Whitespace::parse("").is_err());
}
