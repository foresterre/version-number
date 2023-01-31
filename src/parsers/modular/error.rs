use crate::parsers::error::ExpectedError;
use crate::parsers::NumericError;
use crate::ParserError;

/// Errors which may be returned during parsing, by the _modular parser_.
#[derive(Clone, Debug, thiserror::Error, Eq, PartialEq)]
pub enum ModularParserError {
    // /// Expected input token(s) for currently being parsed token, but got nothing.
    // NoInputForComponent,
    // /// Expected end of input, but got more tokens.
    // ExpectedEOI,
    /// When this error variant is returned, the parser expected that no more
    /// tokens should be present, but instead 1 or more additional tokens
    /// were not parsed yet.
    ///
    #[error("Expected end of input after parsing third version number component, but got: '{}'", char::from(*.got))]
    ExpectedEndOfInput {
        /// An additional token still present when the parser was expected to have
        /// reached the end-of-input for the given input.
        got: u8,
    },

    /// When this error variant is returned, the '.' token was expected, but
    /// a different token was present, or the end-of-input reached.
    ///
    /// The `got` field shows the token read.
    #[error(
        "Expected the dot-separator '.', but got '{}'",
        .got.map(|c| String::from(char::from(c))).unwrap_or_else(|| "EOI".to_string()),
    )]
    ExpectedSeparator {
        /// Token read, or `None` if we unexpectedly got the end-of-input.
        got: Option<u8>,
    },

    /// When this error variant is returned, a numeric token was expected, but
    /// a different token was present, or the end-of-input reached.
    #[error(
        "Expected 0-9, but got '{}'",
        .got.map(|c| String::from(char::from(c))).unwrap_or_else(|| "EOI".to_string()),
    )]
    ExpectedNumericToken {
        /// Token read, or `None` if we unexpectedly got the end-of-input.
        got: Option<u8>,
    },

    /// An error variant for faults when parsing and constructing a number.
    #[error(transparent)]
    NumberError(#[from] NumberError),
}

/// An error type for faults relating to parsing and constructing numbers.
#[derive(Clone, Debug, thiserror::Error, Eq, PartialEq)]
pub enum NumberError {
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

impl From<ModularParserError> for ParserError {
    fn from(value: ModularParserError) -> Self {
        match value {
            ModularParserError::ExpectedEndOfInput { got } => {
                ParserError::Expected(ExpectedError::EndOfInput {
                    at: None,
                    got: char::from(got),
                })
            }
            ModularParserError::ExpectedNumericToken { got } => {
                ParserError::Expected(ExpectedError::Numeric {
                    at: None,
                    got: got.map(char::from),
                })
            }
            ModularParserError::ExpectedSeparator { got } => {
                ParserError::Expected(ExpectedError::Separator {
                    at: None,
                    got: got.map(char::from),
                })
            }
            ModularParserError::NumberError(e) => match e {
                NumberError::LeadingZero => ParserError::Numeric(NumericError::LeadingZero),
                NumberError::Overflow => ParserError::Numeric(NumericError::Overflow),
            },
        }
    }
}
