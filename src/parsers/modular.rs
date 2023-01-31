//! The _modular parser_ module.
//!
//! Please refer to to the [`parsers`] module for a more detailed description of
//! this type of parser.
//!
//! [`crate::parsers`]

use crate::parsers::{BaseVersionParser, FullVersionParser, VersionParser};
use crate::{BaseVersion, FullVersion, ParserError, Version};

pub use error::ModularParserError;
pub use parser::{ParsedBase, ParsedFull, ParsedState, Parser, Unparsed};

mod component;
mod error;
mod parser;
mod take_while_peekable;

/// A convenience interface to the modular parser.
///
/// If you want low-lever access to the modular parser, and use it as intended, you should use the
/// [`modular::Parser`] struct directly. This interface primarily exists to have an implementation
/// of the _modular parser_ for the [`VersionParser`] trait, which allows for interchangeability of
/// implementations.
///
/// [`modular::Parser`]: Parser
#[derive(Debug)]
pub struct ModularParser;

impl VersionParser for ModularParser {
    fn parse_version<B: AsRef<[u8]>>(&self, input: B) -> Result<Version, ParserError> {
        let parser = Parser::from_slice(input.as_ref());

        parser.parse().map_err(ParserError::from)
    }
}

impl BaseVersionParser for ModularParser {
    fn parse_base<B: AsRef<[u8]>>(&self, input: B) -> Result<BaseVersion, ParserError> {
        let parser = Parser::from_slice(input.as_ref());

        parser
            .parse_base()
            .and_then(|parser| parser.finish_base_version())
            .map_err(ParserError::from)
    }
}

impl FullVersionParser for ModularParser {
    fn parse_full<B: AsRef<[u8]>>(&self, input: B) -> Result<FullVersion, ParserError> {
        let parser = Parser::from_slice(input.as_ref());

        parser
            .parse_full()
            .and_then(|parser| parser.finish_full_version())
            .map_err(ParserError::from)
    }
}
