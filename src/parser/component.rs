use crate::parser::error::ParseError;
use crate::parser::take_while_peekable::TakeWhilePeekable;
use std::iter::Peekable;

pub fn parse_component(input: &mut Peekable<impl Iterator<Item = u8>>) -> Result<u64, ParseError> {
    input
        .take_while_peekable(
            |&tok| (b'0'..=b'9').contains(&tok), /* todo: check: manually unroll or optimized? */
        )
        .fold(
            Err(ParseError::NoInput),
            |state: Result<u64, ParseError>, next| {
                let next = u64::from(next - b'0');

                match state {
                    Ok(0) => Err(ParseError::NoLeadingZeroAllowed),
                    Ok(value) => value
                        .checked_mul(10)
                        .and_then(|lhs| lhs.checked_add(next))
                        .ok_or(ParseError::Overflow),
                    Err(ParseError::NoInput) => Ok(next),
                    Err(err) => Err(err),
                }
            },
        )
}

pub fn parse_dot(input: &mut impl Iterator<Item = u8>) -> Result<(), ParseError> {
    input
        .next()
        .filter(|&token| token == b'.')
        .map(|_| ())
        .ok_or(ParseError::NoSeparator)
}

pub fn is_done(input: &mut impl Iterator<Item = u8>) -> Result<(), ParseError> {
    if let None = input.next() {
        Ok(())
    } else {
        Err(ParseError::ExpectedEndOfInput)
    }
}
