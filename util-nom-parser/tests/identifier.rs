mod identifier {
    use util_nom_parser::grammar;
    
    #[grammar{ ( '_'* ['a'-'z'] ["aA0" - "zZ9" | '_'] * ) : String }]
    pub struct Identifier;

    #[test]
    fn keyword() {
        let (input, _) = Identifier::parse("_aB7").unwrap();
        assert_eq!(input, "");
    }
}
