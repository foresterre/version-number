#[derive(Debug, Eq, PartialEq)]
pub enum ParseError {
    NoInput,
    Overflow,
    NoSeparator,
    ExpectedEndOfInput,
    NoLeadingZeroAllowed,
}
