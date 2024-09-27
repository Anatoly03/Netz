//! This module contains the decorator combinator. It will try to
//! parse a keyword in the input or cancel with an error.
//! 
//! ```rs
//! decorator!(input, name);
//! ```

#[macro_export]
macro_rules! decorator {
    ($i:ident, $x:ident) => {
        keyword!($i, "@");
        identifier!($i, $x);
    };
}
