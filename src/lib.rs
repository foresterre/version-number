#![deny(missing_docs)]

//! # version-number
//!
//! ## Synopsis
//!
//! A crate to represent and parse two- and three-component version numbers of the form `major.minor`,
//! and `major.minor.patch` respectively. These version numbers are often seen within the Rust
//! project manifests.
//!
//! ## Semver compatibility
//!
//! The version numbers accepted by this crate are a subset of semver version numbers,
//! with the exception of also allowing two component (shorthand) `major.minor` versions.
//!
//! For example, `1.0` and `1.0.0` are both accepted by this library, while the former is
//! rejected by [`semver`].
//!
//! In addition [`Version`] does not accept extra labels such as build parameters, which are
//! an extension of the [`semver`] version number itself.
//!
//! In this crate, we call a two component `major.minor` version number a [`BaseVersion`], and
//! we call a three component `major.minor.patch` version number a [`FullVersion`].
//!
//! [`semver`]: https://semver.org/spec/v2.0.0.html
//! [`Version`]: crate::Version
//! [`BaseVersion`]: crate::BaseVersion
//! [`FullVersion`]: crate::FullVersion

use std::fmt;
use std::str::FromStr;

pub use base_version::BaseVersion;
pub use full_version::FullVersion;

mod base_version;
mod full_version;
mod parser;

/// Top level errors for version-numbers.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An error which specifies failure to parse a version number.
    #[error("{0}")]
    ParseError(#[from] parser::Error),
}

/// A numbered version which is a two-component `major.minor` version number,
/// or a three-component `major.minor.patch` version number.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum Version {
    /// A two-component `major.minor` version.
    Base(BaseVersion),
    /// A three-component `major.minor.patch` version.
    Full(FullVersion),
}

impl Version {
    /// Parse a two- or three-component, `major.minor` or `major.minor.patch` respectively,
    /// version number from a given input.
    ///
    /// Returns a [`crate::Error::ParseError`] if it fails to parse.
    pub fn parse(input: &str) -> Result<Self, Error> {
        parser::Parser::from(input.as_bytes())
            .parse()
            .map_err(From::from)
    }

    /// Create a new two-component `major.minor` version number.
    pub fn new_base_version(major: u64, minor: u64) -> Self {
        Self::Base(BaseVersion { major, minor })
    }

    /// Create a new three-component `major.minor.patch` version number.
    pub fn new_full_version(major: u64, minor: u64, patch: u64) -> Self {
        Self::Full(FullVersion {
            major,
            minor,
            patch,
        })
    }

    /// Returns the `major` version component.
    ///
    /// Both the two- and three-component version number variants have a major version.
    /// This is the leading component.
    pub fn major(&self) -> u64 {
        match self {
            Self::Base(inner) => inner.major,
            Self::Full(inner) => inner.major,
        }
    }

    /// Returns the `minor` version component.
    ///
    /// Both the two- and three-component version number variants have a minor version.
    /// This is the middle component.
    pub fn minor(&self) -> u64 {
        match self {
            Self::Base(inner) => inner.minor,
            Self::Full(inner) => inner.minor,
        }
    }

    /// Returns the `patch` version component, if any.
    ///
    /// A three component `major.minor.patch` version will return a `Some(<version>)`,
    /// while a two component `major.minor` version will return `None` instead.
    ///
    /// If it exists, it is the last component.
    pub fn patch(&self) -> Option<u64> {
        match self {
            Self::Base(_) => None,
            Self::Full(inner) => Some(inner.patch),
        }
    }

    /// Check of which variant `self` is.
    pub fn is(&self, variant: Variant) -> bool {
        match self {
            Version::Base(_) => matches!(variant, Variant::Base),
            Version::Full(_) => matches!(variant, Variant::Full),
        }
    }
}

impl FromStr for Version {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Error> {
        parser::Parser::from_slice(input.as_bytes())
            .parse()
            .map_err(From::from)
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Base(inner) => fmt::Display::fmt(&inner, f),
            Self::Full(inner) => fmt::Display::fmt(&inner, f),
        }
    }
}

impl From<(u64, u64)> for Version {
    fn from(tuple: (u64, u64)) -> Self {
        Self::Base(BaseVersion::from(tuple))
    }
}

impl From<(u64, u64, u64)> for Version {
    fn from(tuple: (u64, u64, u64)) -> Self {
        Self::Full(FullVersion::from(tuple))
    }
}

/// Type used to indicate which variant of a [`Version`] is used.
/// The options are [`Base`] for [`Version::Base`], and [`Full`] for [`Version::Full`].
///
/// [`Version`]: crate::Version
/// [`Base`]: crate::Variant::Base
/// [`Version::Base`]: crate::Version::Base
/// [`Full`]: crate::Variant::Full
/// [`Version::Full`]: crate::Version::Full
#[derive(Copy, Clone, Debug)]
pub enum Variant {
    /// Indicates a [`Version::Base`] is used.
    ///
    /// [`Version::Base`]: crate::Version::Base
    Base,
    /// Indicates a [`Version::Full`] is used.
    ///
    /// [`Version::Full`]: crate::Version::Full
    Full,
}

#[cfg(test)]
mod tests {
    use crate::{BaseVersion, FullVersion, Variant, Version};

    #[test]
    fn is_base_variant() {
        let version = Version::Base(BaseVersion::new(0, 0));

        assert!(version.is(Variant::Base));
        assert!(!version.is(Variant::Full));
    }

    #[test]
    fn is_full_variant() {
        let version = Version::Full(FullVersion::new(0, 0, 0));

        assert!(version.is(Variant::Full));
        assert!(!version.is(Variant::Base));
    }
}
