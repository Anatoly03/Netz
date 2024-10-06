mod identifier {
    use util_nom_parser::grammar;

    #[grammar{ ['a'-'z'] }]
    pub struct Lowercase;

    /// Expects an alphanumeric identifier starting with a ascii
    /// lowercase letter, optionally prefixed by underscores.
    #[grammar{ '_'* ['a'-'z'] ["aA0" - "zZ9" | "_"]* }]
    pub struct RawIdentifier;

    /// Expects an alphanumeric identifier starting with a ascii
    /// lowercase letter, optionally prefixed by underscores.
    #[grammar{ [ALPHA "_"] [ALPHA NUM "_"] * }]
    pub struct RawVariable;
    
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
    fn raw_id() {
        assert!(RawIdentifier::parse("a").is_ok());
        assert!(RawIdentifier::parse("_abc").is_ok());
        assert!(RawIdentifier::parse("_abc00_").is_ok());
        assert!(RawIdentifier::parse("ident").is_ok());
        
        assert!(RawIdentifier::parse("Ident").is_err()); // Our grammar expects identifiers to start lowercase
        assert!(RawIdentifier::parse("!").is_err());
        assert!(RawIdentifier::parse("_").is_err());
        assert!(RawVariable::parse("0123").is_err());
    }

    #[test]
    fn raw_var() {
        assert!(RawVariable::parse("a").is_ok());
        assert!(RawVariable::parse("_abc").is_ok());
        assert!(RawVariable::parse("_abc00_").is_ok());
        assert!(RawVariable::parse("ident").is_ok());
        assert!(RawVariable::parse("Ident").is_ok()); // Our grammar expects identifiers to start lowercase
        assert!(RawVariable::parse("_").is_ok());
        
        assert!(RawVariable::parse("!").is_err());
        assert!(RawVariable::parse("0123").is_err());
    }

    // #[test]
    // fn keyword() {
    //     let (input, _) = Identifier::parse("_aB7").unwrap();
    //     assert_eq!(input, "");
    // }
}
