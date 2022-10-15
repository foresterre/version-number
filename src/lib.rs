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
//! In this crate, we call a two component `major.minor` version number a [`CoreVersion`], and
//! we call a three component `major.minor.patch` version number a [`FullVersion`].
//!
//! [`semver`]: https://semver.org/spec/v2.0.0.html
//! [`Version`]: crate::Version
//! [`CoreVersion`]: crate::CoreVersion
//! [`FullVersion`]: crate::FullVersion

use std::fmt;
use std::str::FromStr;

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
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Version {
    /// A two-component `major.minor` version.
    Core(CoreVersion),
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
    pub fn new_core_version(major: u64, minor: u64) -> Self {
        Self::Core(CoreVersion { major, minor })
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
            Self::Core(inner) => inner.major,
            Self::Full(inner) => inner.major,
        }
    }

    /// Returns the `minor` version component.
    ///
    /// Both the two- and three-component version number variants have a minor version.
    /// This is the middle component.
    pub fn minor(&self) -> u64 {
        match self {
            Self::Core(inner) => inner.minor,
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
            Self::Core(_) => None,
            Self::Full(inner) => Some(inner.patch),
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
            Self::Core(inner) => fmt::Display::fmt(&inner, f),
            Self::Full(inner) => fmt::Display::fmt(&inner, f),
        }
    }
}

impl From<(u64, u64)> for Version {
    fn from(tuple: (u64, u64)) -> Self {
        Self::Core(CoreVersion::from(tuple))
    }
}

impl From<(u64, u64, u64)> for Version {
    fn from(tuple: (u64, u64, u64)) -> Self {
        Self::Full(FullVersion::from(tuple))
    }
}

/// A two-component `major.minor` version.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct CoreVersion {
    major: u64,
    minor: u64,
}

impl From<(u64, u64)> for CoreVersion {
    fn from(tuple: (u64, u64)) -> Self {
        CoreVersion {
            major: tuple.0,
            minor: tuple.1,
        }
    }
}

impl fmt::Display for CoreVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}.{}", self.major, self.minor))
    }
}

/// A three-component `major.minor.patch` version.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct FullVersion {
    major: u64,
    minor: u64,
    patch: u64,
}

impl From<(u64, u64, u64)> for FullVersion {
    fn from(tuple: (u64, u64, u64)) -> Self {
        FullVersion {
            major: tuple.0,
            minor: tuple.1,
            patch: tuple.2,
        }
    }
}

impl fmt::Display for FullVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}.{}.{}", self.major, self.minor, self.patch))
    }
}
