//! A module for a common error API.
//! Individual parsers may implement their own, more detailed error type.
//! This error type may be used with the convenience parse traits, [`ParseVersion`],
//! [`ParseBase`] and [`ParseFull`].
//!
//! [`ParseVersion`]: crate::VersionParser
//! [`ParseBase`]: crate::BaseVersionParser
//! [`ParseFull`]: crate::FullVersionParser

#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error("todo!")]
    Todo,
}
