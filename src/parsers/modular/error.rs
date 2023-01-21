#[derive(Debug, Eq, PartialEq)]
pub enum ParseError {
    /// Expected input token(s) for currently being parsed token, but got nothing.
    NoInputForComponent,
    /// Each component has a max value of [`u64::MAX`].
    Overflow,
    /// Expected the dot (`.`) separator, but another token was found.
    ExpectedSeparator,
    /// Expected end of input, but got more tokens.
    ExpectedEOI,
    /// A number component should be 0 or start with 1-9.
    LeadingZeroNotAllowed,
}
