use crate::parser::component::{is_done, parse_component, parse_dot};
use crate::parser::error::ParseError;
use crate::{BaseVersion, FullVersion, Version};
use std::iter::Peekable;
use std::slice::Iter;

struct Builder<S: ParsedState, I> {
    state: S,
    iter: I,
}

struct Unparsed;
struct ParsedBase {
    version: BaseVersion,
}
struct ParsedFull {
    version: FullVersion,
}

pub trait ParsedState {}
impl ParsedState for Unparsed {}
impl ParsedState for ParsedBase {}
impl ParsedState for ParsedFull {}

impl<S: ParsedState, I: Iterator<Item = u8>> Builder<S, I> {
    fn from_bytes(bytes: &[u8]) -> Builder<Unparsed, Peekable<I>> {
        todo!()
    }
}

impl<I: Iterator> Builder<Unparsed, I> {
    fn parse_base(self) -> Builder<ParsedBase, I> {
        todo!()
    }

    fn parse_full(self) -> Builder<ParsedFull, I> {
        todo!()
    }
}

impl<I: Iterator> Builder<ParsedBase, I> {
    fn parse_full(self) -> Builder<ParsedFull, I> {
        todo!()
    }

    fn try_build(self) -> Version {
        todo!()
    }
}

impl<I: Iterator> Builder<ParsedFull, I> {
    fn try_build(self) -> Version {
        todo!()
    }
}

// TODO: typing experiment only
fn parse_base() {
    let builder = Builder::from_bytes("hello".as_bytes());
    let base = builder.parse_base();
    let output = base.try_build();
}

// TODO: typing experiment only
fn parse_full() {
    let builder = Builder::from_bytes("hello".as_bytes());
    let base = builder.parse_base();
    let full = base.parse_full();
    let output = full.try_build();
}

pub fn parse_base_version<I>(input: I) -> Result<BaseVersion, ParseError>
where
    I: IntoIterator<Item = u8>,
{
    let mut input = input.into_iter().peekable();

    let major = parse_component(input.by_ref())?;
    parse_dot(input.by_ref())?;
    let minor = parse_component(input.by_ref())?;
    is_done(input.by_ref())?;

    Ok(BaseVersion { major, minor })
}

// pub fn parse_full_version<I: AsRef<str>>(_input: I) -> crate::FullVersion {
//     todo!()
// }

// -- todo: build builder again

#[cfg(test)]
mod tests {
    use super::parse_base_version;
    use super::ParseError;
    use crate::BaseVersion;
    use yare::parameterized;

    #[test]
    fn test() {}

    #[parameterized(
        zeroes = { "0.0", 0, 0 },
        one = { "1.0", 1, 0 },
        ones = { "1.1", 1, 1 },
    )]
    fn accepted(input: &str, major: u64, minor: u64) {
        let input = input.as_bytes();

        // todo: accept both &u8 and u8
        let parsed = parse_base_version(input.iter().cloned()).unwrap();

        assert_eq!(BaseVersion::new(major, minor), parsed);
    }

    #[parameterized(
        no_leading_zero_component_0 = { "00.0", ParseError::NoLeadingZeroAllowed },
        no_leading_zero_component_1 = { "01.0", ParseError::NoLeadingZeroAllowed },
        no_leading_zero_component_2 = { "1.01", ParseError::NoLeadingZeroAllowed },
    )]
    fn rejected(input: &str, expected_err: ParseError) {
        let input = input.as_bytes();

        let err = parse_base_version(input.iter().cloned()).unwrap_err();

        assert_eq!(err, expected_err);
    }
}
