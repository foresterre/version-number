use super::component::{is_done, parse_component, parse_dot, peek_is_dot};
use super::error::ModularParserError;
use crate::{BaseVersion, FullVersion, Version};
use std::iter::Peekable;
use std::slice::Iter;

// States

/// A parser state of a _modular parser_.
///
/// This is the initial state.  
#[derive(Debug)]
pub struct Unparsed;

/// A parser state of a _modular parser_.
///
/// When the parser has reached this state, a two component [`BaseVersion`]
/// has been parsed, although no end-of-input check has taken place.
///
/// For example, given the hypothetical input `a.bc`, we are only aware that
/// `a.b` is a valid two component version number, while the tokens `c` may be
/// anything. In practice, the `c` tokens cannot be the complete set of available
/// tokens because the component parsing implementation has to distinguish
/// between the tokens in the component (i.e. the number) and those outside of
/// it. This though, is only an implementation detail, and not a concern of the
/// API.
#[derive(Debug)]
pub struct ParsedBase {
    version: BaseVersion,
}

/// A parser state of a _modular parser_.
///
/// When the parser has reached this state, a three component [`FullVersion`]
/// has been parsed, although no end-of-input check has taken place.
///
/// For example, given the hypothetical input `a.b.cd`, we are only aware that
/// `a.b.c` is a valid three component version number, while the tokens `d` may
/// be anything. In practice, the `d` tokens cannot be the complete set of
/// available tokens because the component parsing implementation has to
/// distinguish between the tokens in the component (i.e. the number)
/// and those outside of it. This though, is only an implementation detail,
/// and not a concern of the API.
#[derive(Debug)]
pub struct ParsedFull {
    version: FullVersion,
}

/// A trait to restrict the state of the [`Parser`] to valid state instances.
///
/// Since this trait is public it can technically be implemented outside of this
/// crate. Usually, this is not what you want though, although it may be useful
/// for testing, which is why it's not sealed.
pub trait ParsedState {}
impl ParsedState for Unparsed {}
impl ParsedState for ParsedBase {}
impl ParsedState for ParsedFull {}

// Parser

/// A parser which may be used to parse a [`Version`] or its discriminants ([`BaseVersion`] and
/// [`FullVersion`]), incrementally.
#[derive(Debug)]
pub struct Parser<'p, S: ParsedState> {
    state: S,
    iter: Peekable<Iter<'p, u8>>,
}

impl<'p> Parser<'p, Unparsed> {
    /// Construct a parser from a byte slice.
    ///
    /// # Example
    ///
    /// ```
    /// use version_number::parsers::modular::Parser;
    /// let parser = Parser::from_slice("1.0.0".as_bytes());
    /// ```
    pub fn from_slice(bytes: &'p [u8]) -> Parser<'p, Unparsed> {
        let iter = bytes.iter();

        Parser {
            state: Unparsed,
            iter: iter.peekable(),
        }
    }
}

impl<'p> Parser<'p, Unparsed> {
    /// Parse the base of a [`Version`]. The `base` are the `major` and `minor` components
    /// of a version. An example of a `base` version which will parse, would be `1.2`.
    ///
    /// This method returns another [`Parser`] instance. To get the parsed version
    /// after parsing the `base`, you may use [`Parser::finish`].
    ///
    /// In case you want to either parse a two or three component version, and you
    /// don't care which one you have, you should use [`Parser::parse`] instead.
    ///
    /// # Example
    ///
    /// ```
    /// use version_number::BaseVersion;
    /// use version_number::parsers::modular::{Parser};
    ///
    /// let parser = Parser::from_slice("1.2".as_bytes());
    ///
    /// let base = parser.parse_base().unwrap();
    ///
    /// assert_eq!(base.inner_version(), &BaseVersion::new(1, 2));
    /// ```
    pub fn parse_base(self) -> Result<Parser<'p, ParsedBase>, ModularParserError> {
        let Self { mut iter, .. } = self;

        let major = parse_component(iter.by_ref())?;
        parse_dot(iter.by_ref())?;
        let minor = parse_component(iter.by_ref())?;

        let version = BaseVersion::new(major, minor);

        Ok(Parser {
            state: ParsedBase { version },
            iter,
        })
    }

    /// Parse a full, three component major, minor, patch [`Version`]. A two
    /// component input, consisting of only the major and minor components, will be rejected.
    ///
    /// # Example
    ///
    /// ```
    /// use version_number::{BaseVersion, FullVersion};
    /// use version_number::parsers::modular::{Parser};
    /// let parser = Parser::from_slice("1.2.3".as_bytes());
    ///
    /// let base = parser.parse_full().unwrap();
    ///
    /// assert_eq!(base.inner_version(), &FullVersion::new(1, 2, 3));
    /// ```
    pub fn parse_full(self) -> Result<Parser<'p, ParsedFull>, ModularParserError> {
        let parser = self.parse_base()?;
        parser.parse_patch()
    }

    /// Parse a `base`, two component `major.minor` [`Version`], or a `full`, three component `major.minor.patch`,
    /// depending on the input.
    ///
    /// # Example 1
    ///
    /// ```
    /// use version_number::{BaseVersion, FullVersion, Version};
    /// use version_number::parsers::modular::Parser;
    ///
    /// let parser = Parser::from_slice("1.2".as_bytes());
    ///
    /// let version = parser.parse();
    ///
    /// assert_eq!(version.unwrap(), Version::Base(BaseVersion::new(1, 2)));
    /// ```    
    ///
    /// # Example 2
    ///
    /// ```
    /// use version_number::{FullVersion, Version};
    /// use version_number::parsers::modular::Parser;
    ///
    /// let parser = Parser::from_slice("1.2.3".as_bytes());
    ///
    /// let version = parser.parse();
    ///
    /// assert_eq!(version.unwrap(), Version::Full(FullVersion::new(1, 2, 3)));
    /// ```    
    ///
    /// # Example 3
    ///
    /// ```
    /// use version_number::{FullVersion, Version};
    /// use version_number::parsers::modular::Parser;
    ///
    /// let parser = Parser::from_slice("1.2.".as_bytes());
    ///
    /// let version = parser.parse();
    ///
    /// assert!(version.is_err());
    /// ```
    pub fn parse(self) -> Result<Version, ModularParserError> {
        let mut parser = self.parse_base()?;

        if peek_is_dot(parser.iter.by_ref()) {
            parser.parse_patch()?.finish()
        } else {
            parser.finish()
        }
    }
}

impl<'p> Parser<'p, ParsedBase> {
    /// Parse the patch component, to produce a [`FullVersion`].
    ///
    /// # Example
    ///
    /// ```
    /// use version_number::{FullVersion};
    /// use version_number::parsers::modular::Parser;
    ///
    /// let input = "1.2.3";
    ///
    /// let parser = Parser::from_slice(input.as_bytes());
    /// let full = parser
    ///     .parse_base()
    ///     .unwrap()
    ///     .parse_patch()
    ///     .unwrap();
    ///
    /// assert_eq!(full.inner_version(), &FullVersion::new(1, 2, 3));
    /// ```
    pub fn parse_patch(self) -> Result<Parser<'p, ParsedFull>, ModularParserError> {
        let Self {
            mut iter,
            state: ParsedBase {
                version: BaseVersion { major, minor },
            },
        } = self;

        parse_dot(iter.by_ref())?;
        let patch = parse_component(iter.by_ref())?;

        let version = FullVersion::new(major, minor, patch);

        Ok(Parser {
            state: ParsedFull { version },
            iter,
        })
    }

    /// Parses a `patch` component if it exists and returns a [`Version::Full`],
    /// or if the input does not have a third component, returns the two component [`Version::Base`]
    /// variant instead.
    ///
    /// Prefer [`Parser::parse`] over this method when possible, as this method clones the underlying
    /// iterator to determine whether we do have additional content.
    pub fn parse_patch_or_finish(self) -> Result<Version, ModularParserError> {
        if peek_is_dot(self.iter.clone().by_ref()) {
            self.finish()
        } else {
            self.parse_patch()?.finish()
        }
    }

    /// Checks that there is no remaining input, and returns a [`Version`], which
    /// wraps the parsed base version.
    ///
    /// When there is remaining input, this method will return a [`ModularParserError::ExpectedEOI`]
    /// instead.
    pub fn finish(self) -> Result<Version, ModularParserError> {
        self.finish_base_version().map(Version::Base)
    }

    /// Checks that there is no remaining input, and returns a [`BaseVersion`].
    ///
    /// When there is remaining input, this method will return a [`ModularParserError::ExpectedEOI`]
    /// instead.
    pub fn finish_base_version(self) -> Result<BaseVersion, ModularParserError> {
        let Self { mut iter, state } = self;

        is_done(iter.by_ref())?;

        Ok(state.version)
    }

    /// Returns the so far successfully parsed version state.
    ///
    /// **NB:** Unless the end of input has been reached, this version may not be valid.
    pub fn inner_version(&self) -> &BaseVersion {
        &self.state.version
    }
}

impl<'p> Parser<'p, ParsedFull> {
    /// Checks that there is no remaining input, and returns a [`Version`], which
    /// wraps the parsed base version.
    ///
    /// When there is remaining input, this method will return a [`ModularParserError::ExpectedEOI`]
    pub fn finish(self) -> Result<Version, ModularParserError> {
        let Self { mut iter, state } = self;

        is_done(iter.by_ref())?;

        Ok(Version::Full(state.version))
    }

    /// Checks that there is no remaining input, and returns a [`FullVersion`].
    ///
    /// When there is remaining input, this method will return a [`ModularParserError::ExpectedEOI`]
    /// instead.
    pub fn finish_full_version(self) -> Result<FullVersion, ModularParserError> {
        let Self { mut iter, state } = self;

        is_done(iter.by_ref())?;

        Ok(state.version)
    }

    /// Returns the so far successfully parsed version.
    ///
    /// **NB:** Unless the end of input has been reached, this version may not be valid.
    pub fn inner_version(&self) -> &FullVersion {
        &self.state.version
    }
}

#[cfg(test)]
mod tests_leading_zeros {
    use super::*;
    use crate::parsers::modular::NumberError;
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
        let parsed = Parser::from_slice(input)
            .parse_base()
            .and_then(|parser| parser.finish_base_version())
            .unwrap();

        assert_eq!(BaseVersion::new(major, minor), parsed);
    }

    #[parameterized(
        no_leading_zero_component_0 = { "00.0", ModularParserError::NumberError(NumberError::LeadingZero) },
        no_leading_zero_component_1 = { "01.0", ModularParserError::NumberError(NumberError::LeadingZero) },
        no_leading_zero_component_2 = { "1.01", ModularParserError::NumberError(NumberError::LeadingZero) },
    )]
    fn rejected(input: &str, expected_err: ModularParserError) {
        let input = input.as_bytes();
        let err = Parser::from_slice(input)
            .parse_base()
            .and_then(|parser| parser.finish_base_version())
            .unwrap_err();

        assert_eq!(err, expected_err);
    }
}

#[cfg(test)]
mod tests_parser_base {
    use super::*;
    use crate::parsers::modular::NumberError;
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
        let parser = Parser::from_slice(input.as_bytes());

        let base = parser.parse_base().unwrap();
        let version = base.inner_version();

        assert_eq!(&BaseVersion::new(major, minor), version);

        let version = base.finish().unwrap();
        assert_eq!(Version::new_base_version(major, minor), version);
    }

    #[test]
    fn rejected_on_no_input() {
        let input = "";
        let parser = Parser::from_slice(input.as_bytes());
        let err = parser.parse_base().unwrap_err();

        assert_eq!(err, ModularParserError::ExpectedNumericToken { got: None });
    }

    #[test]
    fn rejected_on_no_input2() {
        let input = "1.";
        let parser = Parser::from_slice(input.as_bytes());
        let err = parser.parse_base().unwrap_err();

        assert_eq!(err, ModularParserError::ExpectedNumericToken { got: None });
    }

    #[test]
    fn rejected_on_overflow() {
        // u64::MAX is accepted
        let input = format!("{}.{}5", u64::MAX, 1844674407370955161_u64);
        let parser = Parser::from_slice(input.as_bytes());
        assert!(parser.parse_base().is_ok());

        // but u64::MAX+1 overflows
        let input = format!("{}6.0", 1844674407370955161_u64);
        let parser = Parser::from_slice(input.as_bytes());
        let err = parser.parse_base().unwrap_err();

        assert_eq!(err, ModularParserError::NumberError(NumberError::Overflow));
    }

    #[test]
    fn rejected_on_separator_expected() {
        let input = "1";
        let parser = Parser::from_slice(input.as_bytes());
        let err = parser.parse_base().unwrap_err();

        assert_eq!(err, ModularParserError::ExpectedSeparator { got: None });
    }

    #[test]
    fn rejected_on_eoi_expected() {
        let input = "1.0.0";
        let parser = Parser::from_slice(input.as_bytes());
        let err = parser.parse_base().unwrap().finish().unwrap_err();

        assert_eq!(err, ModularParserError::ExpectedEndOfInput { got: b'.' });
    }

    #[test]
    fn rejected_on_leading_zero_not_allowed() {
        let input = "1.01";
        let parser = Parser::from_slice(input.as_bytes());
        let err = parser.parse_base().unwrap_err();

        assert_eq!(
            err,
            ModularParserError::NumberError(NumberError::LeadingZero)
        );
    }

    #[parameterized(
        in_first_component_1 = { "01.9" },
        in_first_component_2 = { "00.9" },
        in_second_component_1 = { "9.01" },
        in_second_component_2 = { "9.00" },
    )]
    fn rejected_on_leading_zero_not_allowed(input: &str) {
        let parser = Parser::from_slice(input.as_bytes());
        let err = parser.parse_base().unwrap_err();

        assert_eq!(
            err,
            ModularParserError::NumberError(NumberError::LeadingZero)
        );
    }
}
