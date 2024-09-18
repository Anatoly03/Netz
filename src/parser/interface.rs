use nom::{error::{Error, ErrorKind}, Err, IResult};

/// This trait describes a common trait that all network
/// file parser elements have.
pub trait NetworkParser : Sized {
    /// Parse the given element using a `&str` input and
    /// `Self` output.
    fn parse(input: &str) -> IResult<&str, Self>;

    /// Parse the given element using a `&str` input and
    /// `Self` output, and expect to advance at least one
    /// symbol.
    fn parse1(input: &str) -> IResult<&str, Self> {
        let (advanced_input, output) = Self::parse(input)?;
        if input == advanced_input {
            IResult::Err(Err::Error(Error::new(input, ErrorKind::Many1)))
        } else {
            IResult::Ok((advanced_input, output))
        }
    }
}
