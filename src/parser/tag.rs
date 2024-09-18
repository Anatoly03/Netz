use super::{
    comment::Comment,
    identifier::NetworkIdentifier,
    interface::NetworkParser,
};
use nom::{bytes::complete::tag, IResult};

/// A tag is a marker that can extend a functionality of a
/// network definition file.
///
/// #### Example
///
/// ```net
/// @ExampleTag
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct Tag {
    tag_name: String,
}

impl Tag {
    pub fn new(tag_name: String) -> Self {
        Self {
            tag_name,
        }
    }

    pub fn name(&self) -> &str {
        &self.tag_name.as_str()
    }
}

impl NetworkParser for Tag {
    /// A tag consists of two required elements, the '@' symbol and
    /// a custom identifier afterwards.
    fn parse(input: &str) -> IResult<&str, Self> {
        // expect the '@' tag annotation
        let (input, _) = Comment::parse(input)?;
        let (input, _) = tag("@")(input)?;

        // read the tag name
        let (input, tag_name) = NetworkIdentifier::parse(input)?;

        IResult::Ok((input, Tag::new(tag_name.identity)))
    }
}

#[cfg(test)]
mod tag_test {
    use super::*;

    #[test]
    fn simple_tag() {
        let (_, tag) = Tag::parse("@HelloWorld").unwrap();
        assert_eq!(tag.tag_name, "HelloWorld");
    }

    #[test]
    fn documented_tag() {
        let (_, tag) = Tag::parse("/** A tag. */ @Doc").unwrap();
        assert_eq!(tag.tag_name, "Doc");
    }
}
