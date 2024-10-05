mod whitespace_opt {
    use util_parser::grammar;

    #[grammar(~?)]
    pub struct OptionalWhitespace;
    
    #[test]
    fn whitespace() {
        let (input, _) = OptionalWhitespace::parse("\t \r\n").unwrap();
        assert_eq!(input, "");
    }

    #[test]
    fn no_whitespace() {
        let (input, _) = OptionalWhitespace::parse("").unwrap();
        assert_eq!(input, "");
    }
}
