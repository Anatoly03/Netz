//! This crate implements a utility trait that converts any string to a
//! particular case style. The case names are majorly named after the answer
//! to the StackOverflow question [What are the different kinds of cases?]
//! (https://stackoverflow.com/questions/17326185/what-are-the-different-kinds-of-cases).
//! An identifier is a string consisting of alphanumeric characters and a few
//! separation markers, like the dash and the underscore, with a `word` being
//! defined as a lexicographical atomic component of the identifier.

/// Contains the characters that case insensitive separate identifier words.
const SEPARATION_CHARACTERS: &str = &"-_~,. ";

/// Defines methods to convert an identifier into various case styles
/// based on the implemented word splitter method.
pub trait CaseStyles {
    /// Splits an identifier into atomic words. A word can be either uppercase (`ABC`)
    /// capitalized (`Abc`), or lowercase (`abc`). This method will panic if it has to
    /// work with case insensitive characters other than a few separation markers.
    fn to_split_case(&self) -> Vec<String>;

    /// Converts the identifier to flatcase (`flatcase`).
    fn to_flat_case(&self) -> String {
        self.to_split_case().join("").to_lowercase()
    }

    /// Converts the identifier to kebab case (`dash-case`).
    fn to_kebab_case(&self) -> String {
        self.to_split_case().join("-").to_lowercase()
    }

    /// Converts the identifier to camel case (`camelCase`).
    fn to_camel_case(&self) -> String {
        let mut s = self.to_pascal_case();
        s[0..1].make_ascii_lowercase();
        s
    }

    /// Converts the identifier to pascal case (`PascalCase`, `CapitalCamelCase`).
    fn to_pascal_case(&self) -> String {
        let capitalize = |s: &String| {
            let mut out = s.to_lowercase().clone();
            out[0..1].make_ascii_uppercase();
            out
        };

        (&self.to_split_case())
            .into_iter()
            .map(capitalize)
            .collect::<Vec<String>>()
            .join("")
    }

    /// Converts the identifier to snake case (`snake_case`).
    fn to_snake_case(&self) -> String {
        self.to_split_case().join("_").to_lowercase()
    }

    /// Converts the identifier to constant case (`UPPER_CASE`).
    fn to_constant_case(&self) -> String {
        self.to_split_case().join("_").to_uppercase()
    }
}

/// Implements for `CaseStyles` for the [common trait](https://www.reddit.com/r/rust/comments/zfgo1f/common_trait_for_str_string_string_arcstring/)
/// which all string types share.
impl<T: AsRef<str>> CaseStyles for T {
    fn to_split_case(&self) -> Vec<String> {
        let identifier = self.as_ref().to_string();
        let separation = identifier
            .split(|c| SEPARATION_CHARACTERS.contains(c))
            .map(ToString::to_string)
            .collect::<Vec<String>>();
        let mut vec = vec![];

        for element in separation {
            let mut buffer = String::from("");

            // The separation characters split the word by "obvious" splits, this
            // will split any input in `dash-case` and `snake_case` into two words,
            // but the individual word components have to be split again based on
            // their case. The following needs to be considered:
            // 
            // - `Dashed-SnakeCase` equives `dashed-snake-case`
            // - `HTTPRequest` equives `http-request`
            // - `ECMAScript` equives `ecma-script`
            // - `camelsLoveOCaml` equives `camels-love-o-caml`,
            //   ambigious with semantic `..-ocaml`
            // 
            // From these observations, here are the rules of the algorithm
            // ranked by priority:
            // 1. All lower case letters following one lower case letter must be part
            //    of the same word.
            // 2. A capital letter following lower case letters belongs to the following
            //    word.

            for (idx, c) in element.char_indices() {
                const DEFAULT: char = '?';

                let previous_letter = buffer.chars().last().unwrap_or(DEFAULT);
                let next_letter = element.chars().skip(idx + 1).next().unwrap_or(DEFAULT);

                match c {
                    // If we're an uppercase letter and the next letter is lowercase, we start
                    // a new word from this letter.
                    // This covers `...Aa` and `aA...`
                    c if c.is_ascii_uppercase() && (previous_letter.is_ascii_lowercase() || next_letter.is_ascii_lowercase()) => {
                        vec.push(buffer);
                        buffer = c.to_string();
                    }
                    // If we're an uppercase letter and the next letter keeps case, continue
                    // writing to buffer.
                    // This covers `...AA`
                    c if c.is_ascii_uppercase() /*&& next_letter.is_ascii_uppercase()*/ => {
                        buffer += c.to_string().as_str();
                    }
                    // We land here if we are in a series of lowercase letters. In this case,
                    // we keep writing into the buffer
                    // This covers `...aa`
                    c if c.is_ascii_lowercase() => {
                        buffer += c.to_string().as_str();
                    }
                    // NOTE: The case `...aA` does not need coverage.
                    // If the letter has no case, panic.
                    _ => panic!(
                        "Identifier expected to consist of upper or lowercase letters, got `...{}{}{}...`",
                        previous_letter, c, next_letter
                    ),
                }
            }

            if buffer.len() != 0 {
                vec.push(buffer);
            }
        }

        vec.into_iter().filter(|s| s.len() != 0).collect()
    }
}

// TODO: implement (split case should return self): impl<'a, T: AsRef<&'a str>, U: Iter<T>> CaseStyles for U

#[cfg(test)]
mod tests {
    use super::CaseStyles;

    #[test]
    fn split_case() {
        assert_eq!(
            "HelloWorld".to_split_case(),
            vec!["Hello".to_string(), "World".to_string()]
        );
        assert_eq!(
            "helloWorld".to_split_case(),
            vec!["hello".to_string(), "World".to_string()]
        );
        assert_eq!(
            "__helloWorld".to_split_case(),
            vec!["hello".to_string(), "World".to_string()]
        );
        assert_eq!(
            "AbcABC".to_split_case(),
            vec!["Abc".to_string(), "ABC".to_string()]
        );
        assert_eq!(
            "ABCAbc".to_split_case(),
            vec!["ABC".to_string(), "Abc".to_string()]
        );
    }

    #[test]
    fn camel_case() {
        assert_eq!("HelloWorld".to_camel_case(), "helloWorld");
        assert_eq!("HTTPRequest".to_camel_case(), "httpRequest");
    }

    #[test]
    fn constant_case() {
        assert_eq!("HelloWorld".to_constant_case(), "HELLO_WORLD");
    }

    #[test]
    fn flat_case() {
        assert_eq!("HelloWorld".to_flat_case(), "helloworld");
    }

    #[test]
    fn kebab_case() {
        assert_eq!("HelloWorld".to_kebab_case(), "hello-world");
    }

    #[test]
    fn pascal_case() {
        assert_eq!("HelloWorld".to_pascal_case(), "HelloWorld");
        assert_eq!("HTTPRequest".to_pascal_case(), "HttpRequest");
        assert_eq!("HTTP-Request".to_pascal_case(), "HttpRequest");
    }

    #[test]
    fn snake_case() {
        assert_eq!("HelloWorld".to_snake_case(), "hello_world");
    }
}
