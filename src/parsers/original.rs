//! The _original parser_ module.
//!
//! Please refer to to the [`parsers`] module for a more detailed description of
//! this type of parser.
//!
//! [`crate::parsers`]

pub use error::{Error, ErrorReason, NumberError};

mod error;

#[cfg(test)]
mod tests;

macro_rules! to_number {
    ($initial:expr) => {
        Ok(u64::from($initial - b'0'))
    };
    ($current:expr, $next:expr) => {{
        $current
            .checked_mul(10)
            .and_then(|c| c.checked_add(u64::from($next - b'0')))
            .ok_or_else(|| NumberError::Overflow)
    }};
}

type Number = u64;

#[derive(Copy, Clone)]
struct NumberConstructor(Number);

impl NumberConstructor {
    fn try_new(digit: u8) -> Result<Self, NumberError> {
        to_number!(digit).map(NumberConstructor)
    }

    fn append_digit(&mut self, digit: u8) -> Result<(), NumberError> {
        if self.0 == 0 {
            return Err(NumberError::LeadingZero);
        }

        self.0 = to_number!(self.0, digit)?;

        Ok(())
    }

    fn as_value(&self) -> Number {
        self.0
    }
}

struct NumberComponent(Option<NumberConstructor>);

impl NumberComponent {
    fn new() -> Self {
        Self(None)
    }

    fn insert_digit(&mut self, token: u8) -> Result<(), NumberError> {
        if let Some(num) = &mut self.0 {
            // A digit was already pushed
            num.append_digit(token)
        } else {
            let number = NumberConstructor::try_new(token)?;
            self.0 = Some(number);
            Ok(())
        }
    }

    fn get(&self) -> Option<NumberConstructor> {
        self.0
    }
}

/// The _orignal parser_ Parser 😊.
///
/// # Example
///
/// ```
/// use version_number::parsers::original::Parser;
/// use version_number::Version;
///
/// let parser = Parser::from_slice("1.2.3".as_bytes());
/// let version = parser.parse().unwrap();
///
/// assert_eq!(version, Version::new_full_version(1, 2,3));
/// ```
pub struct Parser<'slice> {
    slice: &'slice [u8],
}

impl<'slice> Parser<'slice> {
    /// Construct a new [`Parser`] from a slice of bytes.
    ///
    /// # Example
    ///
    /// ```
    /// use version_number::parsers::original::Parser;
    ///
    /// let _parser = Parser::from_slice("1.2".as_bytes());
    /// ```
    pub fn from_slice(slice: &'slice [u8]) -> Self {
        Parser { slice }
    }

    /// Parse a two- or three component version number from the given input.
    ///
    /// # Example
    ///
    /// ```
    /// use version_number::parsers::original::Parser;
    /// use version_number::Version;
    ///
    /// let parser = Parser::from_slice("1.2".as_bytes());
    /// let version = parser.parse().unwrap();
    ///
    /// assert_eq!(version, Version::new_base_version(1, 2));
    pub fn parse(&self) -> Result<crate::Version, Error> {
        let mut cursor = 0;

        let first = self.parse_number(&mut cursor)?;
        self.parse_dot(&mut cursor)?;
        let second = self.parse_number(&mut cursor)?;

        if self.is_done(cursor) {
            // is_done = true
            return Ok(crate::Version::Base(crate::BaseVersion {
                major: first.as_value(),
                minor: second.as_value(),
            }));
        }

        // is_done = false
        self.parse_dot(&mut cursor)?;
        let third = self.parse_number(&mut cursor)?;

        if self.is_done(cursor) {
            // is_done = true
            return Ok(crate::Version::Full(crate::FullVersion {
                major: first.as_value(),
                minor: second.as_value(),
                patch: third.as_value(),
            }));
        }

        Err(Error::from_parser_with_cursor(
            self,
            cursor,
            ErrorReason::ExpectedEndOfInput {
                extra_input: self.slice[cursor..].to_vec(),
            },
        ))
    }

    fn parse_number(&self, cursor: &mut usize) -> Result<NumberConstructor, Error> {
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
