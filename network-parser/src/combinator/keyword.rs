//! This module contains the keyword combinator. It will try to
//! parse a keyword in the input or cancel with an error.
//! 
//! ```rs
//! keyword!(input, "struct");
//! ```

#[macro_export]
macro_rules! keyword {
    ($i:ident, $key:literal) => {
        whitespace!($i);
        let ($i, _) = tag($key).parse($i)?;
    };
}
