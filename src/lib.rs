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

use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

use crate::parsers::original;

pub use parsers::{BaseVersionParser, FullVersionParser, ParserError, VersionParser};
#[cfg(feature = "semver")]
pub use version::FromSemverError;
pub use version::{BaseVersion, FullVersion};

/// This crate contains multiple parsers.
///
/// In general, it's easiest to use the well tested [`parsers::original::Parser`], which is also used
/// (currently) by [`Version::parse`].
pub mod parsers;

#[cfg(feature = "serde")]
mod serde_impl;
mod version;

/// Top level errors for version-numbers.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An error which specifies failure to parse a version number.
    #[error(transparent)]
    ParserError(#[from] ParserError),
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
    /// Returns a [`Error::ParserError`] if it fails to parse.
    pub fn parse(input: &str) -> Result<Self, Error> {
        original::Parser::from(input.as_bytes())
            .parse()
            .map_err(|e| Error::from(Into::<ParserError>::into(e)))
    }

    /// Create a new two-component `major.minor` version number.
    pub const fn new_base_version(major: u64, minor: u64) -> Self {
        Self::Base(BaseVersion { major, minor })
    }

    /// Create a new three-component `major.minor.patch` version number.
    pub const fn new_full_version(major: u64, minor: u64, patch: u64) -> Self {
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

    /// Convert this version to a three-component [`FullVersion`].
    ///
    /// A two-component version gets a `patch` of `0`, so `1.2` becomes `1.2.0`.
    pub fn to_full_version_lossy(&self) -> FullVersion {
        match self {
            Self::Base(inner) => inner.to_full_version_lossy(),
            Self::Full(inner) => *inner,
        }
    }

    /// Compare `self` with a [`FullVersion`], using only the components which `self` has.
    ///
    /// A two-component version ignores the `patch` of `other`, so `1.2` is equal to both `1.2.0`
    /// and `1.2.9`, while `1.2.0` is only equal to `1.2.0`.
    ///
    /// # Example
    ///
    /// ```
    /// use std::cmp::Ordering;
    /// use version_number::{FullVersion, Version};
    ///
    /// let base = Version::new_base_version(1, 2);
    /// let full = Version::new_full_version(1, 2, 0);
    ///
    /// assert_eq!(base.cmp_components(&FullVersion::new(1, 2, 9)), Ordering::Equal);
    /// assert_eq!(full.cmp_components(&FullVersion::new(1, 2, 9)), Ordering::Less);
    /// assert_eq!(base.cmp_components(&FullVersion::new(1, 1, 9)), Ordering::Greater);
    /// ```
    // This is a method instead of a `PartialOrd` implementation, because the matching
    // `PartialEq` would not be transitive: `1.2.0 == 1.2` and `1.2 == 1.2.9`, but `1.2.0 != 1.2.9`.
    pub fn cmp_components(&self, other: &FullVersion) -> Ordering {
        match self {
            Self::Base(inner) => inner.cmp(&other.to_base_version_lossy()),
            Self::Full(inner) => inner.cmp(other),
        }
    }

    /// Check whether `other` matches `self`, using only the components which `self` has.
    ///
    /// See [`Version::cmp_components`] for how the versions are compared.
    ///
    /// # Example
    ///
    /// ```
    /// use version_number::{FullVersion, Version};
    ///
    /// let base = Version::new_base_version(1, 2);
    ///
    /// assert!(base.matches(&FullVersion::new(1, 2, 0)));
    /// assert!(base.matches(&FullVersion::new(1, 2, 9)));
    /// assert!(!base.matches(&FullVersion::new(1, 3, 0)));
    /// ```
    pub fn matches(&self, other: &FullVersion) -> bool {
        self.cmp_components(other) == Ordering::Equal
    }

    /// Check of which variant `self` is.
    pub fn is(&self, variant: Variant) -> bool {
        match self {
            Version::Base(_) => matches!(variant, Variant::Base),
            Version::Full(_) => matches!(variant, Variant::Full),
        }
    }

    /// Map a [`Version`] to `U`.
    ///
    /// # Example
    ///
    /// ```
    /// use version_number::{BaseVersion, FullVersion, Version};
    ///
    /// // 🧑‍🔬
    /// fn invert_version(v: Version) -> Version {
    ///     match v {
    ///         Version::Base(base) => Version::Base(BaseVersion::new(base.minor, base.major)),
    ///         Version::Full(full) => Version::Full(FullVersion::new(full.patch, full.minor, full.major))
    ///     }
    /// }
    ///
    /// let base_example = Version::Base(BaseVersion::new(1, 2));
    /// let full_example = Version::Full(FullVersion::new(1, 2, 3));
    ///
    /// assert_eq!(base_example.map(invert_version), Version::Base(BaseVersion::new(2, 1)));
    /// assert_eq!(full_example.map(invert_version), Version::Full(FullVersion::new(3, 2, 1)));
    /// ```
    #[inline]
    pub fn map<U, F>(self, fun: F) -> U
    where
        F: FnOnce(Self) -> U,
    {
        fun(self)
    }

    /// Map over the `major` version component of the [`Version`].
    ///
    /// # Example
    ///
    /// ```
    /// use version_number::{BaseVersion, FullVersion, Version};
    ///
    /// let base_example = Version::Base(BaseVersion::new(0, 0));
    /// let full_example = Version::Full(FullVersion::new(0, 0, 0));
    ///
    /// assert_eq!(base_example.map_major(|a| a + 1), Version::Base(BaseVersion::new(1, 0)));
    /// assert_eq!(full_example.map_major(|a| a + 1), Version::Full(FullVersion::new(1, 0, 0)));
    /// ```
    #[inline]
    pub fn map_major<F>(self, fun: F) -> Self
    where
        F: FnOnce(u64) -> u64,
    {
        self.map(|v| match v {
            Self::Base(base) => Version::Base(BaseVersion::new(fun(base.major), base.minor)),
            Self::Full(full) => {
                Version::Full(FullVersion::new(fun(full.major), full.minor, full.patch))
            }
        })
    }

    /// Map over the `minor` version component of the [`Version`].
    ///
    /// # Example
    ///
    /// ```
    /// use version_number::{BaseVersion, FullVersion, Version};
    ///
    /// let base_example = Version::Base(BaseVersion::new(0, 0));
    /// let full_example = Version::Full(FullVersion::new(0, 0, 0));
    ///
    /// assert_eq!(base_example.map_minor(|a| a + 1), Version::Base(BaseVersion::new(0, 1)));
    /// assert_eq!(full_example.map_minor(|a| a + 1), Version::Full(FullVersion::new(0, 1, 0)));
    /// ```
    #[inline]
    pub fn map_minor<F>(self, fun: F) -> Self
    where
        F: FnOnce(u64) -> u64,
    {
        self.map(|v| match v {
            Self::Base(base) => Version::Base(BaseVersion::new(base.major, fun(base.minor))),
            Self::Full(full) => {
                Version::Full(FullVersion::new(full.major, fun(full.minor), full.patch))
            }
        })
    }

    /// Map over the `patch` version component of the [`Version`].
    /// If no `patch` version exists (in case the [`Version`] consists of two components),
    /// then the original version is returned.
    ///
    /// # Example
    ///
    /// ```
    /// use version_number::{BaseVersion, FullVersion, Version};
    ///
    /// let base_example = Version::Base(BaseVersion::new(0, 0));
    /// let full_example = Version::Full(FullVersion::new(0, 0, 0));
    ///
    /// assert_eq!(base_example.map_patch(|a| a + 1), Version::Base(BaseVersion::new(0, 0)));
    /// assert_eq!(full_example.map_patch(|a| a + 1), Version::Full(FullVersion::new(0, 0, 1)));
    /// ```
    #[inline]
    pub fn map_patch<F>(self, fun: F) -> Self
    where
        F: FnOnce(u64) -> u64,
    {
        self.map(|v| match v {
            Self::Base(base) => Version::Base(BaseVersion::new(base.major, base.minor)),
            Self::Full(full) => {
                Version::Full(FullVersion::new(full.major, full.minor, fun(full.patch)))
            }
        })
    }
}

impl FromStr for Version {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Error> {
        original::Parser::from_slice(input.as_bytes())
            .parse()
            .map_err(|e| Error::from(Into::<ParserError>::into(e)))
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

impl From<BaseVersion> for Version {
    fn from(version: BaseVersion) -> Self {
        Self::Base(version)
    }
}

impl From<FullVersion> for Version {
    fn from(version: FullVersion) -> Self {
        Self::Full(version)
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
    use std::cmp::Ordering;

    #[yare::parameterized(
        base = { Version::new_base_version(1, 2), FullVersion::new(1, 2, 0) },
        full = { Version::new_full_version(1, 2, 3), FullVersion::new(1, 2, 3) },
    )]
    fn to_full_version_lossy(version: Version, expected: FullVersion) {
        assert_eq!(version.to_full_version_lossy(), expected);
    }

    #[yare::parameterized(
        base_eq_patch_zero = { Version::new_base_version(1, 56), FullVersion::new(1, 56, 0), Ordering::Equal },
        base_eq_patch_nonzero = { Version::new_base_version(1, 56), FullVersion::new(1, 56, 99), Ordering::Equal },
        base_lt_minor = { Version::new_base_version(1, 56), FullVersion::new(1, 57, 0), Ordering::Less },
        base_lt_major = { Version::new_base_version(1, 56), FullVersion::new(2, 0, 0), Ordering::Less },
        base_gt_minor = { Version::new_base_version(1, 56), FullVersion::new(1, 55, 99), Ordering::Greater },
        base_gt_major = { Version::new_base_version(2, 0), FullVersion::new(1, 99, 99), Ordering::Greater },
        full_eq = { Version::new_full_version(1, 56, 1), FullVersion::new(1, 56, 1), Ordering::Equal },
        full_lt_patch = { Version::new_full_version(1, 56, 0), FullVersion::new(1, 56, 1), Ordering::Less },
        full_lt_minor = { Version::new_full_version(1, 56, 9), FullVersion::new(1, 57, 0), Ordering::Less },
        full_lt_major = { Version::new_full_version(1, 99, 9), FullVersion::new(2, 0, 0), Ordering::Less },
        full_gt_patch = { Version::new_full_version(1, 56, 1), FullVersion::new(1, 56, 0), Ordering::Greater },
        full_gt_minor = { Version::new_full_version(1, 57, 0), FullVersion::new(1, 56, 9), Ordering::Greater },
        full_gt_major = { Version::new_full_version(2, 0, 0), FullVersion::new(1, 99, 9), Ordering::Greater },
    )]
    fn cmp_components(version: Version, other: FullVersion, expected: Ordering) {
        assert_eq!(version.cmp_components(&other), expected);
    }

    #[yare::parameterized(
        base_patch_zero = { Version::new_base_version(1, 56), FullVersion::new(1, 56, 0), true },
        base_patch_nonzero = { Version::new_base_version(1, 56), FullVersion::new(1, 56, 3), true },
        base_other_minor = { Version::new_base_version(1, 56), FullVersion::new(1, 55, 0), false },
        base_other_major = { Version::new_base_version(1, 56), FullVersion::new(2, 56, 0), false },
        full_same = { Version::new_full_version(1, 56, 3), FullVersion::new(1, 56, 3), true },
        full_other_patch = { Version::new_full_version(1, 56, 0), FullVersion::new(1, 56, 3), false },
    )]
    fn matches(version: Version, other: FullVersion, expected: bool) {
        assert_eq!(version.matches(&other), expected);
    }

    #[test]
    fn from_base_version() {
        let version = Version::from(BaseVersion::new(1, 2));

        assert_eq!(version, Version::Base(BaseVersion::new(1, 2)));
    }

    #[test]
    fn from_full_version() {
        let version = Version::from(FullVersion::new(1, 2, 3));

        assert_eq!(version, Version::Full(FullVersion::new(1, 2, 3)));
    }

    #[test]
    fn const_constructors() {
        const BASE: Version = Version::new_base_version(1, 2);
        const FULL: Version = Version::new_full_version(1, 2, 3);

        assert_eq!(BASE, Version::Base(BaseVersion::new(1, 2)));
        assert_eq!(FULL, Version::Full(FullVersion::new(1, 2, 3)));
    }

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

    #[test]
    fn map() {
        let version = Version::Base(BaseVersion::new(0, 0));

        let mapped = version.map(|v| match v {
            Version::Base(base) => Version::Full(FullVersion::new(base.major, base.minor, 999)),
            v => v,
        });

        assert_eq!(mapped, Version::Full(FullVersion::new(0, 0, 999)));
    }

    #[yare::parameterized(
        base = { Version::Base(BaseVersion::new(0, 0)) },
        full = { Version::Full(FullVersion::new(0, 0, 0)) },
    )]
    fn map_major(version: Version) {
        let mapped = version.map_major(|_v| 999);

        assert_eq!(mapped.major(), 999);
    }

    #[yare::parameterized(
        base = { Version::Base(BaseVersion::new(0, 0)) },
        full = { Version::Full(FullVersion::new(0, 0, 0)) },
    )]
    fn map_minor(version: Version) {
        let mapped = version.map_minor(|_v| 999);

        assert_eq!(mapped.minor(), 999);
    }

    #[test]
    fn map_patch_base() {
        let version = Version::Base(BaseVersion::new(0, 0));
        let mapped = version.map_patch(|_v| 999);

        assert!(mapped.patch().is_none());
    }

    #[test]
    fn map_patch_full() {
        let version = Version::Full(FullVersion::new(0, 0, 0));
        let mapped = version.map_patch(|_v| 999);

        assert_eq!(mapped.patch().unwrap(), 999);
    }
}
