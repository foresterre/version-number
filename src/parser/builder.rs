use super::take_while_peekable::TakeWhilePeekable;
use crate::BaseVersion;
use std::iter::Peekable;

pub fn parse_base_version<I>(input: I) -> Result<BaseVersion, Error>
where
    I: IntoIterator<Item = u8>,
{
    let mut input = input.into_iter().peekable();

    let major = parse_component(input.by_ref())?;
    parse_dot(input.by_ref())?;
    let minor = parse_component(input.by_ref())?;
    is_done(input.by_ref())?;

    Ok(BaseVersion { major, minor })
}

// pub fn parse_full_version<I: AsRef<str>>(_input: I) -> crate::FullVersion {
//     todo!()
// }

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    NoInput,
    Overflow,
    NoSeparator,
    ExpectedEndOfInput,
    NoLeadingZeroAllowed,
}

pub fn parse_component(input: &mut Peekable<impl Iterator<Item = u8>>) -> Result<u64, Error> {
    input
        .take_while_peekable(
            |&tok| (b'0'..=b'9').contains(&tok), /* todo: check: manually unroll or optimized? */
        )
        .fold(Err(Error::NoInput), |state: Result<u64, Error>, next| {
            let next = u64::from(next - b'0');

            match state {
                Ok(0) => Err(Error::NoLeadingZeroAllowed),
                Ok(value) => value
                    .checked_mul(10)
                    .and_then(|lhs| lhs.checked_add(next))
                    .ok_or(Error::Overflow),
                Err(Error::NoInput) => Ok(next),
                Err(err) => Err(err),
            }
        })
}

pub fn parse_dot(input: &mut impl Iterator<Item = u8>) -> Result<(), Error> {
    input
        .next()
        .filter(|&token| token == b'.')
        .map(|_| ())
        .ok_or(Error::NoSeparator)
}

pub fn is_done(input: &mut impl Iterator<Item = u8>) -> Result<(), Error> {
    if let None = input.next() {
        Ok(())
    } else {
        Err(Error::ExpectedEndOfInput)
    }
}

// -- todo: build builder again

#[cfg(test)]
mod tests {
    use super::parse_base_version;
    use super::Error;
    use crate::BaseVersion;
    use yare::parameterized;

    #[test]
    fn test() {}

    #[parameterized(
        zeroes = { "0.0", 0, 0 },
        one = { "1.0", 1, 0 },
        ones = { "1.1", 1, 1 },
    )]
    fn accepted(input: &str, major: u64, minor: u64) {
        let input = input.as_bytes();

        // todo: accept both &u8 and u8
        let parsed = parse_base_version(input.iter().cloned()).unwrap();

        assert_eq!(BaseVersion::new(major, minor), parsed);
    }

    #[parameterized(
        no_leading_zero_component_0 = { "00.0", Error::NoLeadingZeroAllowed },
        no_leading_zero_component_1 = { "01.0", Error::NoLeadingZeroAllowed },
        no_leading_zero_component_2 = { "1.01", Error::NoLeadingZeroAllowed },
    )]
    fn rejected(input: &str, expected_err: Error) {
        let input = input.as_bytes();

        let err = parse_base_version(input.iter().cloned()).unwrap_err();

        assert_eq!(err, expected_err);
    }
}
