use crate::parsers::modular::error::ParseError;
use crate::parsers::modular::take_while_peekable::TakeWhilePeekable;
use std::iter::Peekable;

/// Parse a single component of a version. A component is the number value which is separated by the
/// dot values. For example, the version `1.22` consists of two components; the major component with
/// value `1` and the minor component with value `22`. This particular function is not aware which
/// component it is parsing, and also does not account for the separator(s).
///
/// A component value must be `0`, or start with a token with value `1` up to and including `9`.
/// For example, the values `0`, `1`, `39`, `90` are all valid, while `00`, `01`, `09273` are not.
pub fn parse_component<'b>(
    input: &mut Peekable<impl Iterator<Item = &'b u8>>,
) -> Result<u64, ParseError> {
    input
        .take_while_peekable(|&tok| (b'0'..=b'9').contains(&tok))
        .fold(
            Err(ParseError::NoInputForComponent),
            |state: Result<u64, ParseError>, next| {
                let next = u64::from(next - b'0');

                match state {
                    Ok(0) => Err(ParseError::LeadingZeroNotAllowed),
                    Ok(value) => value
                        .checked_mul(10)
                        .and_then(|lhs| lhs.checked_add(next))
                        .ok_or(ParseError::Overflow),
                    Err(ParseError::NoInputForComponent) => Ok(next),
                    Err(err) => Err(err),
                }
            },
        )
}

/// Peeks at the next token in the iterator and checks whether the token is the `.` character.
/// If this holds, returns `true`. If there's no more element to consume, or the character is not the
/// `.` character, `false` is returned instead.
pub fn peek_is_dot<'b>(input: &mut Peekable<impl Iterator<Item = &'b u8>>) -> bool {
    input.peek().map(|&&token| token == b'.').unwrap_or(false)
}

/// Consumes the next element of the iterator and checks whether the value is the character `.`.
/// If this holds, then the value `Ok(())` will be returned.
/// If there is no next character, i.e. the iterator returns `None`, or the token returned is not   
/// the character `.`, a `Err(ParseError::ExpectedSeparator)` will be returned.
pub fn parse_dot<'b>(input: &mut impl Iterator<Item = &'b u8>) -> Result<(), ParseError> {
    input
        .next()
        .filter(|&&token| token == b'.')
        .map(|_| ())
        .ok_or(ParseError::ExpectedSeparator)
}

/// Consumes the next element of the iterator, and returns `Ok(())` if there isn't any next value,
/// or `Err(ParseError::ExpectedEOI)` if there is.
pub fn is_done<'b>(input: &mut impl Iterator<Item = &'b u8>) -> Result<(), ParseError> {
    if let None = input.next() {
        Ok(())
    } else {
        Err(ParseError::ExpectedEOI)
    }
}
