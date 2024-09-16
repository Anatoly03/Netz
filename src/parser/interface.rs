use nom::IResult;

/// This trait describes a common trait that all network
/// file parser elements have.
pub trait NetworkParser : Sized {
    /// Parse the given element using a `&str` input and
    /// `Self` output.
    fn parse(input: &str) -> IResult<&str, Self>;
}
