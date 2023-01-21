use super::*;

/// The top-level error type for an _orignal parser_.
#[derive(Clone, Debug, thiserror::Error)]
#[error(
    "Unable to parse '{input}' to a version number: {reason}{}",
    self.fmt()
)]
pub struct Error {
    input: String,
    cursor: Option<usize>,
    reason: ErrorReason,
}

impl Error {
    /// The reason why the given input could not be parsed to a [`crate::Version`].
    pub fn reason(&self) -> &ErrorReason {
        &self.reason
    }
}

impl Error {
    pub(crate) fn from_parser(slice: &Parser<'_>, reason: ErrorReason) -> Self {
        Self {
            input: String::from_utf8_lossy(slice.slice).to_string(),
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

    /// When this error variant is returned, a specific token was expected, but
    /// a different token was present. The `expected` field shows which token
    /// was expected, while the `got` field shows the token read.
    #[error(
        "Expected '{}', but got '{}'",
        char::from(*.expected),
        char::from(*.got)
    )]
    ExpectedToken {
        /// The expected token.
        expected: u8,
        /// The actually present token.
        got: u8,
    },

    /// An error variant for faults when parsing and constructing a number.
    #[error("{0}")]
    NumberError(#[from] NumberError),

    /// When this error variant is returned, the parser still expected some next
    /// token, but no such token could be read by the parser anymore, because
    /// there were no remaining tokens to be parsed.
    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,
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
