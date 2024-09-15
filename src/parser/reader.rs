use nom::{
    bytes::complete::{is_not, tag, take_while}, character::is_space, combinator::map_res, error::{Error, ErrorKind}, sequence::tuple, Err, IResult};

/**
 * 
 */
#[derive(Debug,PartialEq)]
pub struct NetworkStruct {

}

/**
 * 
 */
#[derive(Debug,PartialEq)]
pub struct NetworkFile {

}

fn error<I, O>(input: I) -> IResult<I, O> {
    IResult::Err(Err::Failure(Error { input, code: ErrorKind::Fail }))
}

/**
 Trim all currently leading whitespace, if any. Under the hood, it trims all characters that return positive to `char.is_whitespace()`
 */
fn trim(input: &str, expect_once: bool) -> IResult<&str, ()> {
    let (input, spaces) = take_while(|chr: char| chr.is_whitespace())(input)?;
    if expect_once && spaces.len() == 0 {
        return error("expected whitespace separator")
    }
    Ok((input, ()))
}

/**
 Trim all currently leading whitespace, if any. Under the hood, it trims all characters that return positive to `char.is_whitespace()`
 */
fn identifier(input: &str) -> IResult<&str, &str> {
    let (input, identifier) = take_while(|chr: char| chr.is_ascii_lowercase() || chr.is_ascii_uppercase() || chr == '_')(input)?;
    if let Some(c) = identifier.chars().next() {
        if c.is_numeric() {
            return error("first character of identifier expected to be latin letter or '_'");
        }
        return Ok((input, identifier));
    }
    return error("expected identifier, one of a-z, A-Z or `_`");
}

/**
 A C-style comment can start with `/*` and end with `*/`, or start with `//` and end with a newline.
 */
// fn parse_comment_opt(input: &str) -> IResult<&str, ()> {
//     trim(input);
//     opt(alt("*", "//"))
// }

/**
 * 
 */
fn parse_struct(input: &str) -> IResult<&str, NetworkStruct> {
    let (input, _) = tag("struct")(input)?;
    let (input, _) = trim(input, true)?;
    let (input, struct_name) = identifier(input)?;
    let (input, _) = trim(input, false)?;
    let (input, _) = tag("{")(input)?;
    let (input, _) = trim(input, false)?;
    let (input, _) = tag("}")(input)?;
    Ok((input, NetworkStruct {}))
}

/**
 * 
 */
pub fn parse(input: &str) -> IResult<&str, NetworkFile> {
    let (input, struc) = parse_struct(input)?;
    Ok((input, NetworkFile {}))
}
