use nom::{
    bytes::complete::{take_while, take_while_m_n},
    // character::complete::{alphanumeric1, anychar},
    // combinator::fail,
    IResult,
};

/// Reusable code for error generation.
// pub fn fail<O>(input: &'static impl AsRef<str>) -> IResult<&'static str, O> {
//     IResult::Err(nom::Err::Failure(nom::error::Error {
//         input: input.as_ref(),
//         code: nom::error::ErrorKind::Fail,
//     }))
// }

/// Trim all currently leading whitespace, if any. Under the hood,
/// it trims all characters that return positive to `char.is_whitespace()`
pub fn trim(input: &str) -> IResult<&str, &str> {
    take_while(|chr: char| chr.is_whitespace())(input)
}

/// Trim all currently leading whitespace, expecting at least one.
pub fn expect_space(input: &str) -> IResult<&str, &str> {
    let (input, _) = take_while_m_n(1, 1, |chr: char| chr.is_whitespace())(input)?;
    trim(input)
}

// pub fn identifier(input: &str) -> IResult<&str, &str> {
//     recognize(all_consuming(pair(
//         verify(nom::character::complete::char, |&c: &char| {
//             c.is_ascii_alphabetic()
//         }),
//         many0_count(alphanumeric1),
//     )))(input)

    // let (input, identity) =
    //     take_while(|chr: char| chr.is_ascii_alphanumeric() || chr == '_')(input)?;

    // if identity.len() == 0 {
    //     return fail();

    //     return IResult::Err(nom::Err::Failure(nom::error::VerboseError {
    //         errors: vec![(input, )],
    //     }));
    // }

    // if let Some(chr) = identity.chars().next() {
    //     if chr.is_ascii_digit() {
    //         fail(format!(
    //             "first character of an identifier `{}` expected to be ascii letter or `_`, got {}",
    //             identity, chr
    //         ).as_str())
    //     } else {
    //         Ok((input, identity))
    //     }
    // } else {
    //     fail("expected identifier: one of a-z, A-Z and `_`")
    // }
// }
