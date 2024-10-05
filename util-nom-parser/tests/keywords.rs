mod keyword {
    use util_nom_parser::grammar;

    #[grammar("keyword")]
    pub struct Keyword ();

    #[grammar("struct" ~ "{" ~ "}")]
    pub struct Construct ();

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

    #[test]
    fn construct() {
        let (input, _) = Construct::parse("struct \t\n { \t\n\n } \t\n").unwrap();
        assert_eq!(input, " \t\n");
    }

    #[test]
    fn not_construct() {
        assert!(Construct::parse("").is_err());
        assert!(Construct::parse("struct { \t").is_err());
    }
}
