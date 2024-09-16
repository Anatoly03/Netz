
pub trait NetworkWriter {
    /// The write function [curries](https://en.wikipedia.org/wiki/Currying)
    /// a series of parameters. The first parameter takes the language
    /// identifier, the second the template identifier. Together, the two
    /// generate the file contents.
    /// 
    /// ```
    /// write("cs")("struct_name")
    /// ```
    /// 
    /// ### Language Identifiers
    /// - `cpp`
    /// - `cs`
    /// - `dart`
    /// - `rs`
    /// - `ts`
    fn write(language: &str) -> dyn Fn(&str) -> Option<String>;
}
