//! The _original parser_ module.
//!
//! Please refer to to the [`parsers`] module for a more detailed description of
//! this type of parser.
//!
//! [`crate::parsers`]

use crate::parsers::error::ExpectedError;
use crate::parsers::{BaseVersionParser, FullVersionParser, VersionParser};
use crate::{BaseVersion, FullVersion, ParserError, Version};
pub use error::{ErrorReason, NumberError, OriginalParserError};
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
#[derive(Debug)]
pub struct OriginalParser;

impl VersionParser for OriginalParser {
    fn parse_version<B: AsRef<[u8]>>(&self, input: B) -> Result<Version, ParserError> {
        let parser = Parser::from_slice(input.as_ref());

        parser.parse().map_err(ParserError::from)
    }
}

impl BaseVersionParser for OriginalParser {
    fn parse_base<B: AsRef<[u8]>>(&self, input: B) -> Result<BaseVersion, ParserError> {
        let parser = Parser::from_slice(input.as_ref());

        parser
            .parse()
            .and_then(|v| match v {
                Version::Base(b) => Ok(b),
                Version::Full(f) => Err(OriginalParserError::from_parser(
                    &parser,
                    ErrorReason::ExpectedEndOfInput {
                        extra_input: format!(".{}", f.patch).into_bytes(),
                    },
                )),
            })
            .map_err(ParserError::from)
    }
}

impl FullVersionParser for OriginalParser {
    fn parse_full<B: AsRef<[u8]>>(&self, input: B) -> Result<FullVersion, ParserError> {
        let parser = Parser::from_slice(input.as_ref());

        parser.parse().map_err(From::from).and_then(|v| match v {
            Version::Base(_) => Err(ParserError::Expected(ExpectedError::Separator {
                at: None,
                got: None,
            })),
            Version::Full(f) => Ok(f),
        })
    }
}
