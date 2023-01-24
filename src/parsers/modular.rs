//! The _modular parser_ module.
//!
//! Please refer to to the [`parsers`] module for a more detailed description of
//! this type of parser.
//!
//! [`crate::parsers`]

use crate::parsers::{ParseBase, ParseFull, ParseVersion};
use crate::{BaseVersion, FullVersion, Version};

pub use error::ParseError;
pub use parser::{ParsedBase, ParsedFull, ParsedState, Parser, Unparsed};

mod component;
mod error;
mod parser;
mod take_while_peekable;

/// A convenience interface to the modular parser.
///
/// If you want low-lever access to the modular parser, and use it as intended, you should use the
/// [`modular::Parser`] struct directly. This interface primarily exists to have an implementation
/// of the _modular parser_ for the [`ParseVersion`] trait, which allows for interchangeability of
/// implementations.
///
/// [`modular::Parser`]: Parser
pub struct ModularParser;

impl ParseVersion for ModularParser {
    type Error = ParseError;

    fn parse_version<B: AsRef<[u8]>>(input: B) -> Result<Version, Self::Error> {
        let parser = Parser::from_slice(input.as_ref());

        parser.parse()
    }
}

impl ParseBase for ModularParser {
    type Error = ParseError;

    fn parse_base<B: AsRef<[u8]>>(input: B) -> Result<BaseVersion, Self::Error> {
        let parser = Parser::from_slice(input.as_ref());

        parser
            .parse_base()
            .and_then(|parser| parser.finish_base_version())
    }
}

impl ParseFull for ModularParser {
    type Error = ParseError;

    fn parse_full<B: AsRef<[u8]>>(input: B) -> Result<FullVersion, Self::Error> {
        let parser = Parser::from_slice(input.as_ref());

        parser
            .parse_full()
            .and_then(|parser| parser.finish_full_version())
    }
}
