use super::*;
use crate::parsers::error::ExpectedError;
use crate::parsers::NumericError;

/// The top-level error type for an _orignal parser_.
#[derive(Clone, Debug, thiserror::Error)]
#[error(
    "Unable to parse '{input}' to a version number: {reason}{}",
    self.fmt()
)]
pub struct OriginalParserError {
    input: String,
    cursor: Option<usize>,
    reason: ErrorReason,
}

impl OriginalParserError {
    /// The reason why the given input could not be parsed to a [`crate::Version`].
    pub fn reason(&self) -> &ErrorReason {
        &self.reason
    }
}

impl OriginalParserError {
    pub(crate) fn from_parser(parser: &Parser<'_>, reason: ErrorReason) -> Self {
        Self {
            input: String::from_utf8_lossy(parser.slice).to_string(),
            cursor: None,
            reason,
        }
    }

    pub(crate) fn from_parser_with_cursor(
        slice: &Parser<'_>,
        cursor: usize,
        reason: ErrorReason,
    ) -> Self {
        Self {
            input: String::from_utf8_lossy(slice.slice).to_string(),
            cursor: Some(cursor),
            reason,
        }
    }

    fn fmt(&self) -> String {
        if let Some(c) = self.cursor {
            Self::squiggle(&self.input, c).unwrap_or_default()
        } else {
            String::default()
        }
    }

    fn squiggle(input: &str, cursor: usize) -> Option<String> {
        let lead = "Unable to parse '".len();
        let err_from = lead + cursor;
        let err_end = input.len().checked_sub(cursor + 1)?; // this may fail to look as expected if the input contains multiple lines

        let spaces = std::iter::repeat_with(|| " ").take(err_from);
        let marker = std::iter::once_with(|| "^");
        let squiggle = std::iter::repeat_with(|| "~").take(err_end);
        let newline = std::iter::once_with(|| "\n");

        Some(
            newline
                .clone()
                .chain(spaces)
                .chain(marker)
                .chain(squiggle)
                .chain(newline)
                .collect(),
        )
    }
}

/// Reasons for why a given input cannot be parsed to a [`crate::Version`].
#[derive(Clone, Debug, thiserror::Error, Eq, PartialEq)]
pub enum ErrorReason {
    /// When this error variant is returned, the parser expected that no more
    /// tokens should be present, but instead 1 or more additional tokens
    /// were not parsed yet. The `extra_input` field contains the remaining
    /// tokens.
    ///
    /// The error display implementation tries to print these remaining tokens
    /// as a [`String`].
    #[error("Expected end of input after parsing third version number component, but got: '{}'", String::from_utf8_lossy(.extra_input.as_slice()))]
    ExpectedEndOfInput {
        /// A `Vec` of unexpected tokens, which were still present while the parser
        /// expected to have reached the end-of-input for the given input.
        extra_input: Vec<u8>,
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
    #[error("{0}")]
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

impl From<OriginalParserError> for ParserError {
    fn from(value: OriginalParserError) -> Self {
        match value.reason {
            ErrorReason::NumberError(e) => match e {
                NumberError::LeadingZero => ParserError::Numeric(NumericError::LeadingZero),
                NumberError::Overflow => ParserError::Numeric(NumericError::Overflow),
            },
            ErrorReason::ExpectedEndOfInput { extra_input } => {
                ParserError::Expected(ExpectedError::EndOfInput {
                    at: value.cursor,
                    got: char::from(extra_input[0]),
                })
            }
            ErrorReason::ExpectedSeparator { got } => {
                ParserError::Expected(ExpectedError::Separator {
                    at: value.cursor,
                    got: got.map(char::from),
                })
            }
            ErrorReason::ExpectedNumericToken { got } => {
                ParserError::Expected(ExpectedError::Numeric {
                    at: value.cursor,
                    got: got.map(char::from),
                })
            }
        }
    }
}
