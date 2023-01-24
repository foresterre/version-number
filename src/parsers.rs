//! This module contains multiple parsers. In the next paragraphs, each parser is described.
//!
//! # Parsers
//!
//! ## Original
//!
//! The [`original::Parser`] parses a version, byte by byte (`u8`). It stores a
//! slice of bytes in the Parser struct and cursor is created at the start of the
//! [`original::Parser::parse`] method, which acts as an index pointing to the
//! next token to be parsed.
//!
//! ## Modular
//!
//! The [`modular::Parser`] has an API which is based on the _type state_ pattern.
//! Internally it uses a `Peekable` iterator over bytes (`u8`).
//! The modular parser can parse a version incrementally.
//!
//! # Example
//!
//! In this example we show a basic example of how the original and modular parsers
//! can be used to parse a [`Version`]. For more detailed examples, see their
//! respective modules.
//!
//! ```
//! // Normally you would only use one of these, not both!
//! use version_number::parsers::original;
//! use version_number::parsers::modular;
//!
//! // As an alternative valid input, we could have used a three component version like `1.64.1`.
//! let two_component_version = "1.64";
//!
//! let original_parser = original::Parser::from_slice(two_component_version.as_bytes());
//! let modular_parser = modular::Parser::from_slice(two_component_version.as_bytes());
//!
//! let original_parsed = original_parser.parse().unwrap();
//! let modular_parsed = modular_parser.parse().unwrap();
//!
//! assert_eq!(original_parsed, modular_parsed );
//! ```
//!
// TODO: list use-cases, advantages and disadvantages

use crate::{BaseVersion, FullVersion, Version};

pub mod modular;
pub mod original;

/// Parse a UTF-8 slice of bytes to a [`Version`].
pub trait ParseVersion {
    /// Specific error type of the implementing parser.
    type Error;

    /// Parse a UTF-8 formatted input buffer, `input` to a [`Version`].
    /// This version may be a [`BaseVersion`], or a [`FullVersion`].
    ///
    /// If the version type is known in advance, or you require a specifically
    /// a base- or full version, you may instead use [`ParseBase::parse_base`]
    /// or [`ParseFull::parse_full`].
    fn parse_version<B: AsRef<[u8]>>(input: B) -> Result<Version, Self::Error>;
}

/// Parse a UTF-8 slice of bytes to a [`BaseVersion`].
pub trait ParseBase {
    /// Specific error type of the implementing parser.
    type Error;

    /// Parse a UTF-8 formatted input buffer, `input` to a [`BaseVersion`].
    ///
    /// If you don't know, or care, whether the version consists of two or three components,
    /// you may instead use [`ParseVersion::parse_version`].
    fn parse_base<B: AsRef<[u8]>>(input: B) -> Result<BaseVersion, Self::Error>;
}

/// Parse a UTF-8 slice of bytes to a [`FullVersion`].
pub trait ParseFull {
    /// Specific error type of the implementing parser.
    type Error;

    /// Parse a UTF-8 formatted input buffer, `input` to a [`FullVersion`].
    ///
    /// If you don't know, or care, whether the version consists of two or three components,
    /// you may instead use [`ParseVersion::parse_version`].
    fn parse_full<B: AsRef<[u8]>>(input: B) -> Result<FullVersion, Self::Error>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dynamic() {
        let input = "1.22.33";
        let expected = Version::new_full_version(1, 22, 33);

        let original = <original::OriginalParser as ParseVersion>::parse_version(input).unwrap();
        assert_eq!(original, expected.clone());

        let modular = <modular::ModularParser as ParseVersion>::parse_version(input).unwrap();
        assert_eq!(modular, expected);
    }
}
