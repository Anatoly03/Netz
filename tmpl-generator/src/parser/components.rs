//! This module contains struct and enum definitions of various
//! parsed components of a template file.

use super::util;
use nom::{combinator::all_consuming, IResult, Parser};

#[derive(Clone, Debug, PartialEq)]
pub struct ForeachScope {
    pub value: String,
    pub variable: Vec<String>,
    pub scope: Vec<TemplateElement>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TemplateElement {
    /// Any ignored template element like comments and whitespace that have
    /// to return a template element type.
    Ignored,

    /// An (environment) variable is a simple concatenation of identifiers
    /// with dots (e.g. `root.child.attribute`) and has two definition states:
    /// When undefined, it can be used with `requires` to reject a scope, and
    /// when defined, it will write the variables' string contents. All
    /// variables are of type string.
    ///
    /// The variable enum will also hold any internal function, like `indent`,
    /// `outdent` and `require_newline`
    Variable(Vec<String>),

    /// A variable with the `requires` keyword will act as a boolean value and
    /// read its' defined state. If undefined, the entire scope (marked by the
    /// round parentheses `(` and `)`) will be rejected and not printed.
    Requires(Vec<String>),

    /// A string literal is the most simple element of a template file
    /// and in execution it simply writes its' content.
    StringLiteral(String),

    /// A scope is a concatenation of several template elements and in
    /// execution it simply iterates over the elements for writing.
    Scope(Vec<TemplateElement>),

    /// Foreach is an iterative scanner over an environment variable and
    /// will continue writing until the end of the "array variable".
    ///
    /// This is done by defining a macro variable `value` from an "array
    /// variable" `variable`. Iteration is done, while `variable.N`,
    /// starting with `variable.0` is defined. `value` is simply a macro
    /// for `variable.N`.
    ///
    /// ```tmpl
    /// // root.0 = "1"
    /// // root.0.Hello = "A"
    /// // root.1 = "1"
    /// // root.1.Hello = "B"
    /// // root.2 = "1"
    /// // root.2.Hello = "C"
    ///
    /// foreach v : root {
    ///     v.Hello " "
    /// }
    /// ```
    ///
    /// will produce the following text
    ///
    /// ```txt
    /// A B C
    /// ```
    Foreach(ForeachScope),
}

impl TemplateElement {
    pub fn from_str(input: &str) -> Option<TemplateElement> {
        let (input, v) = Self::scope_from_str.parse(input).ok()?;
        Some(TemplateElement::Scope(v))
    }

    fn scope_from_str(input: &str) -> IResult<&str, Vec<TemplateElement>> {
        all_consuming(util::scope).parse(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_template() {
        TemplateElement::from_str("").unwrap();
    }
}
