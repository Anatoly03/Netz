mod keyword {
    use util_parser::grammar;

    #[grammar("keyword")]
    pub struct Keyword ();

    #[test]
    fn keyword() {
        let (input, _) = Keyword::parse("keyword").unwrap();
        assert_eq!(input, "");
    }

    #[test]
    fn not_keyword() {
        assert!(Keyword::parse("").is_err());
        assert!(Keyword::parse("\t").is_err());
    }
}
