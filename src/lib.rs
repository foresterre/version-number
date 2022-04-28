#![deny(missing_docs)]

//! # version-number
//!
//! ## Synopsis
//! A crate to represent and parse two- and three-component version numbers of the form `major.minor`,
//! and `major.minor.patch` respectively. These version numbers are often seen within the Rust
//! project manifests.
//!
//! ## Semver compatibility
//!
//! The version numbers accepted by this crate are mostly a subset of semver version numbers,
//! with the exception of also allowing two component (shorthand) `major.minor` versions.
//!
//! For example, `1.0` and `1.0.0` are both accepted by this library, while the former is
//! rejected by [`semver`].
//!
//! In addition [`crate::Version`] does not accept extra labels such as build parameters, which are
//! an extension of the [`semver`] version number itself.
//!
//!
//! ### Exception: leading zeros (To be removed)
//!
//! Another exception where this library differs from semver is by accepting leading zeros,
//! i.e. version number components starting with a zero ('0').
//!
//! These are not allowed in semver (e.g. `01.0.0` is disallowed, but `1.0.0` is allowed).
//! This library currently however does allow leading zeroes (and the library does not
//! disambiguate between `01.0.0` and `1.0.0`, so both are seen as equivalent).
//!
//! **This is planned to change in a future version**
//!
//! [`semver`]: https://semver.org/spec/v2.0.0.html

use std::fmt;
use std::str::FromStr;

mod parser;
mod version;

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
    MajorMinor(MajorMinor),
    /// A three-component `major.minor.patch` version.
    MajorMinorPatch(MajorMinorPatch),
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
    pub fn new_major_minor(major: u64, minor: u64) -> Self {
        Self::MajorMinor(MajorMinor { major, minor })
    }

    /// Create a new three-component `major.minor.patch` version number.
    pub fn new_major_minor_patch(major: u64, minor: u64, patch: u64) -> Self {
        Self::MajorMinorPatch(MajorMinorPatch {
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
            Self::MajorMinor(inner) => inner.major,
            Self::MajorMinorPatch(inner) => inner.major,
        }
    }

    /// Returns the `minor` version component.
    ///
    /// Both the two- and three-component version number variants have a minor version.
    /// This is the middle component.
    pub fn minor(&self) -> u64 {
        match self {
            Self::MajorMinor(inner) => inner.minor,
            Self::MajorMinorPatch(inner) => inner.minor,
        }
    }

    /// Returns the `patch` version component, if any.
    ///
    /// A three component `major.minor.patch` version will return a `Some(<version>)`,
    /// while a two component `major.minor` version will return None instead.
    ///
    /// If it exists, it is the last component.
    pub fn patch(&self) -> Option<u64> {
        match self {
            Self::MajorMinor(_) => None,
            Self::MajorMinorPatch(inner) => Some(inner.patch),
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
            Self::MajorMinor(inner) => fmt::Display::fmt(&inner, f),
            Self::MajorMinorPatch(inner) => fmt::Display::fmt(&inner, f),
        }
    }
}

impl From<(u64, u64)> for Version {
    fn from(tuple: (u64, u64)) -> Self {
        Self::MajorMinor(MajorMinor::from(tuple))
    }
}

impl From<(u64, u64, u64)> for Version {
    fn from(tuple: (u64, u64, u64)) -> Self {
        Self::MajorMinorPatch(MajorMinorPatch::from(tuple))
    }
}

/// A two-component `major.minor` version.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct MajorMinor {
    major: u64,
    minor: u64,
}

impl From<(u64, u64)> for MajorMinor {
    fn from(tuple: (u64, u64)) -> Self {
        MajorMinor {
            major: tuple.0,
            minor: tuple.1,
        }
    }
}

impl fmt::Display for MajorMinor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}.{}", self.major, self.minor))
    }
}

/// A three-component `major.minor.patch` version.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct MajorMinorPatch {
    major: u64,
    minor: u64,
    patch: u64,
}

impl From<(u64, u64, u64)> for MajorMinorPatch {
    fn from(tuple: (u64, u64, u64)) -> Self {
        MajorMinorPatch {
            major: tuple.0,
            minor: tuple.1,
            patch: tuple.2,
        }
    }
}

impl fmt::Display for MajorMinorPatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}.{}.{}", self.major, self.minor, self.patch))
    }
}
