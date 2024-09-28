//! This module implements stdlib traits for StringBuilder.
//! 
//! - `From<T>` for strings and string slices
//! - `Into<String>` conversion of a StringBuilder into a String
//! - `ops::Add<&str>` and the assign variant. Concatenation with
//!   string slices

use crate::StringBuilder;

// A StringBuilder can be created from any string
// or string slice.
//
// ```
// StringBuilder::from("");
// ```
impl<T: AsRef<str>> From<T> for StringBuilder {
    fn from(value: T) -> Self {
        Self {
            buffer: value.as_ref().to_string(),
            indent: 0,
        }
    }
}

// A string builder converted to string is simply
// the buffer of the string builder.
//
// ```
// TODO add example
// ```
impl Into<String> for StringBuilder {
    fn into(self) -> String {
        self.buffer
    }
}

// String Builders accept concatenation with string slices.
//
// ```
// builder = builder + "export class";
// ```
impl std::ops::Add<&str> for StringBuilder {
    type Output = StringBuilder;

    fn add(mut self, rhs: &str) -> Self::Output {
        self.write(rhs);
        self
    }
}

// String Builders accept concatenation with string slices.
//
// ```
// builder += "export class";
// ```
impl std::ops::AddAssign<&str> for StringBuilder {
    fn add_assign(&mut self, rhs: &str) {
        self.write(rhs);
    }
}
