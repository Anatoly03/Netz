
mod ws {
    use util_parser::grammar;

    #[grammar(~)]
    pub struct Whitespace;

    #[test]
    fn parse_whitespace() {
        let (input, _) = Whitespace::parse("\t \r\n").unwrap();
        assert_eq!(input, "");
    }
}
