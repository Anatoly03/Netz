mod identifier {
    use util_nom_parser::grammar;

    #[grammar{ ['a'-'z'] }]
    pub struct Lowercase;

    #[grammar{ '_'* ['a'-'z'] ["aA0" - "zZ9" | "_"]* }]
    pub struct RawIdentifier;
    
    // #[grammar{ ( '_'* ['a'-'z'] ["aA0" - "zZ9" | '_'] * ) : String }]
    // pub struct Identifier;

    #[test]
    fn lowercase() {
        assert!(Lowercase::parse("a").is_ok());
        assert!(Lowercase::parse("b").is_ok());
        assert!(Lowercase::parse("z").is_ok());
        assert!(Lowercase::parse("A").is_err());
        assert!(Lowercase::parse("!").is_err());
    }

    #[test]
    fn raw() {
        assert!(RawIdentifier::parse("_abc").is_ok());
        assert!(RawIdentifier::parse("_abc00_").is_ok());
        assert!(RawIdentifier::parse("ident").is_ok());
        assert!(RawIdentifier::parse("Ident").is_err());
        assert!(RawIdentifier::parse("!").is_err());
    }

    // #[test]
    // fn keyword() {
    //     let (input, _) = Identifier::parse("_aB7").unwrap();
    //     assert_eq!(input, "");
    // }
}
