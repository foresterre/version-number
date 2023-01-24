//! The _original parser_ module.
//!
//! Please refer to to the [`parsers`] module for a more detailed description of
//! this type of parser.
//!
//! [`crate::parsers`]

use crate::parsers::{ParseBase, ParseFull, ParseVersion};
use crate::{BaseVersion, FullVersion, Version};
pub use error::{Error, ErrorReason, NumberError};
pub use parser::Parser;

mod error;
mod parser;

#[cfg(test)]
mod tests;

/// A convenience interface to the original parser.
///
/// You can also use the parser directly using [`original::Parser`].
///
/// [`original::Parser`]: Parser
pub struct OriginalParser;

impl ParseVersion for OriginalParser {
    type Error = Error;

    fn parse_version<B: AsRef<[u8]>>(&self, input: B) -> Result<Version, Self::Error> {
        let parser = Parser::from_slice(input.as_ref());

        parser.parse()
    }
}

impl ParseBase for OriginalParser {
    type Error = Error;

    fn parse_base<B: AsRef<[u8]>>(&self, input: B) -> Result<BaseVersion, Self::Error> {
        let parser = Parser::from_slice(input.as_ref());

        parser.parse().and_then(|v| match v {
            Version::Base(b) => Ok(b),
            Version::Full(f) => Err(Error::from_parser(
                &parser,
                ErrorReason::ExpectedEndOfInput {
                    extra_input: format!(".{}", f.patch).into_bytes(),
                },
            )),
        })
    }
}

impl ParseFull for OriginalParser {
    type Error = Error;

    fn parse_full<B: AsRef<[u8]>>(&self, input: B) -> Result<FullVersion, Self::Error> {
        let parser = Parser::from_slice(input.as_ref());

        parser.parse().and_then(|v| match v {
            Version::Base(_) => Err(Error::from_parser(
                &parser,
                ErrorReason::UnexpectedEndOfInput,
            )),
            Version::Full(f) => Ok(f),
        })
    }
}
