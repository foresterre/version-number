//! A module for a common error API.
//! Individual parsers may implement their own, more detailed error type.
//! This error type may be used with the convenience parse traits, [`ParseVersion`],
//! [`ParseBase`] and [`ParseFull`].
//!
//! [`ParseVersion`]: crate::VersionParser
//! [`ParseBase`]: crate::BaseVersionParser
//! [`ParseFull`]: crate::FullVersionParser

type Index = usize;

///
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum ParserError {
    /// An error variant for fault when a some type of input, or none at all,
    /// was expected next.
    #[error(transparent)]
    Expected(#[from] ExpectedError),

    /// An error variant for faults when parsing and constructing a number.
    #[error(transparent)]
    Numeric(#[from] NumericError),
}

/// An error type for faults relating to parsing and expecting a certain type of
/// token.
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum ExpectedError {
    /// When this error variant is returned, the `.` token, i.e. the separator, was expected, but
    /// a different token was present. The `got` field shows the token read.
    #[error("Expected dot token '.', but got '{}'{}.", 
        .got.map(String::from).unwrap_or_else(|| "EOI".to_string()),
        .at.map(|i| format!(" at {}", i)).unwrap_or_default())
    ]
    Separator {
        /// Place where the token was expected.
        ///
        /// May be `None` if the respective parser does not store the index.
        at: Option<Index>,
        /// Token found instead, or `None` if we unexpectedly got the end-of-input.
        got: Option<char>,
    },

    /// When this error variant is returned, the parser expected that no more
    /// tokens should be present, but instead 1 or more additional tokens
    /// were not parsed yet.
    ///
    /// The `got` field contains the next token received, where
    /// it expected none to be remaining.
    #[error("Expected end of input, but got '{got}'{}.", .at.map(|i| format!(" at {}", i)).unwrap_or_default())]
    EndOfInput {
        /// Place where the end-of-input was expected.
        ///
        /// May be `None` if the respective parser does not store the index.
        at: Option<Index>,
        /// Character found at the expected end-of-input.
        got: char,
    },

    /// When this error variant is returned, a numeric token was expected, but
    /// a different token was present.
    ///
    /// The `got` field shows the token read.
    #[error("Expected numeric token (0-9), but got '{}'{}.",
        .got.map(String::from).unwrap_or_else(|| "EOI".to_string()),
        .at.map(|i| format!(" at {}", i)).unwrap_or_default())
    ]
    Numeric {
        /// Place where the token was expected.
        ///
        /// May be `None` if the respective parser does not store the index.
        at: Option<Index>,
        /// Token found instead, or `None` if we unexpectedly got the end-of-input.
        got: Option<char>,
    },
}

/// An error type for faults relating to parsing and constructing numbers.
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum NumericError {
    /// When this error variant is returned, the parser detected that the number started with a leading
    /// zero, which is not allowed for number components.
    #[error("Number may not start with a leading zero, unless the complete component is '0'")]
    LeadingZero,

    /// This error variant is returned if the number would overflow.
    ///
    /// Each number component consists of a 64 bits unsigned integer.
    #[error("Overflow: Found number component which would be larger than the maximum supported number (max={})", u64::MAX)]
    Overflow,
}
