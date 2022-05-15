#[cfg(test)]
mod tests;

macro_rules! to_number {
    ($initial:expr) => {
        Ok(Some(u64::from($initial - b'0')))
    };
    ($current:expr, $next:expr) => {{
        $current
            .checked_mul(10)
            .and_then(|c| c.checked_add(u64::from($next - b'0')))
            .ok_or_else(|| NumberError::Overflow)
            .map(|v| Some(v))
    }};
}

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
    fn from_parser(slice: &Parser<'_>, reason: ErrorReason) -> Self {
        Self {
            input: String::from_utf8_lossy(slice.slice).to_string(),
            cursor: None,
            reason,
        }
    }

    fn from_parser_with_cursor(slice: &Parser<'_>, cursor: usize, reason: ErrorReason) -> Self {
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
    /// Returned
    #[error("Expected end of input after parsing third version number component, but got: '{}'", String::from_utf8_lossy(.extra_input.as_slice()))]
    ExpectedEndOfInput { extra_input: Vec<u8> },

    #[error(
        "Expected '{}', but got '{}'",
        char::from(*.expected),
        char::from(*.got)
    )]
    ExpectedToken { expected: u8, got: u8 },

    #[error("{0}")]
    NumberError(#[from] NumberError),

    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,
}

#[derive(Clone, Debug, thiserror::Error, Eq, PartialEq)]
pub enum NumberError {
    #[error("Overflow: Found number component which would be larger than the maximum supported number (max={})", Number::MAX)]
    Overflow,
}

type Number = u64;
struct NumberComponent(Option<Number>);

impl NumberComponent {
    fn new() -> Self {
        Self(None)
    }

    fn insert_digit(&mut self, token: u8) -> Result<(), NumberError> {
        if let Some(v) = self.0 {
            self.0 = to_number!(v, token)?;
            Ok(())
        } else {
            self.0 = to_number!(token)?;
            Ok(())
        }
    }

    fn get(&self) -> Option<Number> {
        self.0
    }
}

pub struct Parser<'slice> {
    slice: &'slice [u8],
}

impl<'slice> Parser<'slice> {
    pub fn from_slice(slice: &'slice [u8]) -> Self {
        Parser { slice }
    }

    pub fn parse(&self) -> Result<crate::Version, Error> {
        let mut cursor = 0;

        let first = self.parse_number(&mut cursor)?;
        self.parse_dot(&mut cursor)?;
        let second = self.parse_number(&mut cursor)?;

        if self.is_done(cursor) {
            // is_done = true
            return Ok(crate::Version::MajorMinor(crate::MajorMinor {
                major: first,
                minor: second,
            }));
        }

        // is_done = false
        self.parse_dot(&mut cursor)?;
        let third = self.parse_number(&mut cursor)?;

        if self.is_done(cursor) {
            // is_done = true
            return Ok(crate::Version::MajorMinorPatch(crate::MajorMinorPatch {
                major: first,
                minor: second,
                patch: third,
            }));
        }

        return Err(Error::from_parser_with_cursor(
            self,
            cursor,
            ErrorReason::ExpectedEndOfInput {
                extra_input: self.slice[cursor..].to_vec(),
            },
        ));
    }

    fn parse_number(&self, cursor: &mut usize) -> Result<Number, Error> {
        let mut value = NumberComponent::new();

        while let Some(&b) = self.slice.get(*cursor) {
            if !b.is_ascii_digit() {
                break;
            }

            value
                .insert_digit(b)
                .map_err(|error| Error::from_parser(self, error.into()))?;

            *cursor += 1;
        }

        value.get().ok_or_else(|| {
            Error::from_parser_with_cursor(self, *cursor, ErrorReason::UnexpectedEndOfInput)
        })
    }

    fn parse_dot(&self, cursor: &mut usize) -> Result<(), Error> {
        match self.slice.get(*cursor) {
            Some(&b) if b == b'.' => {
                *cursor += 1;
                Ok(())
            }
            Some(&b) => Err(Error::from_parser_with_cursor(
                self,
                *cursor,
                ErrorReason::ExpectedToken {
                    expected: b'.',
                    got: b,
                },
            )),
            None => Err(Error::from_parser_with_cursor(
                self,
                *cursor,
                ErrorReason::UnexpectedEndOfInput,
            )),
        }
    }

    fn is_done(&self, cursor: usize) -> bool {
        cursor >= self.slice.len()
    }
}

impl<'b, T> From<T> for Parser<'b>
where
    T: Into<&'b [u8]>,
{
    fn from(item: T) -> Self {
        Parser { slice: item.into() }
    }
}
