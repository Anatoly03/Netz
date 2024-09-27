//! This crate implements a utility trait that converts any string to a
//! particular case style. The case names are majorly named after the answer
//! to the StackOverflow question [What are the different kinds of cases?]
//! (https://stackoverflow.com/questions/17326185/what-are-the-different-kinds-of-cases). An identifier is a string consisting of alphanumeric characters and a few separation markers, like the dash and the underscore, with a `word` being defined as a lexicographical atomic component of the identifier.

/// Contains the characters that case insensitive separate identifier words.
const SEPARATION_CHARACTERS: &str = &"-_~,.";

/// Defines methods to convert an identifier into various case styles
/// based on the implemented word splitter method.
pub trait CaseStyles {
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
        let mut vec = identifier
            .split(|c| SEPARATION_CHARACTERS.contains(c))
            .map(ToString::to_string)
            .collect();

        vec
    }
}

// TODO: implement (split case should return self): impl<'a, T: AsRef<&'a str>, U: Iter<T>> CaseStyles for U

#[cfg(test)]
mod tests {
    use super::CaseStyles;

    #[test]
    fn camel_case() {
        assert_eq!("HelloWorld".to_camel_case(), "helloWorld");
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
    }

    #[test]
    fn snake_case() {
        assert_eq!("HelloWorld".to_snake_case(), "hello_world");
    }
}
