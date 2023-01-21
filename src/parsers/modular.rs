//! The _modular parser_ module.
//!
//! Please refer to to the [`parsers`] module for a more detailed description of
//! this type of parser.
//!
//! [`crate::parsers`]

pub use builder::{ParsedBase, ParsedFull, ParsedState, Parser, Unparsed};

mod builder;
mod component;
mod error;
mod take_while_peekable;
