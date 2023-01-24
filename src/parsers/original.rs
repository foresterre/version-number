//! The _original parser_ module.
//!
//! Please refer to to the [`parsers`] module for a more detailed description of
//! this type of parser.
//!
//! [`crate::parsers`]

pub use error::{Error, ErrorReason, NumberError};
pub use parser::Parser;

mod error;
mod parser;

#[cfg(test)]
mod tests;
