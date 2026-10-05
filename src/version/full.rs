use crate::parsers::modular;
use crate::{BaseVersion, FullVersionParser, ParserError};
#[cfg(feature = "semver")]
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

/// A three-component `MAJOR.MINOR.PATCH` version.
///
/// This version number is a subset of [`semver`]. In particular, it consists of the `MAJOR`.
/// `MINOR` and `PATCH` components, and leaves out the additional labels for pre-release and build
/// metadata.
///
/// If you require a version number which also discards the `PATCH` number,
/// please see the [`BaseVersion`] variant.
///
/// For a [`semver`] compliant parser, you should use the `semver` [`crate`] instead.
///
/// # Converting to a semver::Version
///
/// This version type may be converted to a [`semver::Version`] using the [`From`] trait, assuming
/// the `semver` feature is enabled.
///
///
/// [`semver`]: https://semver.org/spec/v2.0.0.html
/// [`BaseVersion`]: crate::BaseVersion
/// [`crate`]: https://crates.io/crates/semver
/// [`semver::Version`]: https://docs.rs/semver/1/semver/struct.Version.html
/// [`From`]: https://doc.rust-lang.org/std/convert/trait.From.html
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FullVersion {
    /// A `major` version is incremented when backwards incompatible changes are made to a public
    /// API.
    ///
    /// When this number equals `0`, the version is considered an *unstable initial development
    /// version*.
    pub major: u64,
    /// The `minor` version is incremented when backwards compatibles changes are made to a public
    /// API.
    ///
    /// When the version number is considered an *unstable initial development version*, it may also
    /// be incremented for backwards incompatible changes.
    pub minor: u64,
    /// The `patch` version is incremented when backwards compatibles bug fixes are made.
    pub patch: u64,
}

impl FullVersion {
    /// Instantiate a three component, version number with `MAJOR`, `MINOR` and `PATCH` components.
    ///
    /// See [`FullVersion`] for more.
    ///
    /// [`FullVersion`]: crate::FullVersion
    pub const fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Parse a three component, `major.minor.patch` version number from a given input.
    ///
    /// Returns a [`ParserError`] if it fails to parse.
    pub fn parse(input: &str) -> Result<Self, ParserError> {
        modular::ModularParser.parse_full(input)
    }

    /// Convert this full version to a base version.
    ///
    /// This conversion is lossy because the `patch` value is lost upon conversion.
    pub fn to_base_version_lossy(self) -> BaseVersion {
        BaseVersion {
            major: self.major,
            minor: self.minor,
        }
    }

    /// Map a [`FullVersion`] to `U`.
    ///
    /// # Example
    ///
    /// ```
    /// use version_number::FullVersion;
    ///
    /// // 🧑‍🔬
    /// fn invert_version(v: FullVersion) -> FullVersion {
    ///     FullVersion::new(v.patch, v.minor, v.major)
    /// }
    ///
    /// let example = FullVersion::new(1, 2, 3);
    ///
    /// assert_eq!(example.map(invert_version), FullVersion::new(3, 2, 1));
    /// ```
    pub fn map<U, F>(self, fun: F) -> U
    where
        F: FnOnce(Self) -> U,
    {
        fun(self)
    }
}

#[cfg(feature = "semver")]
impl From<FullVersion> for semver::Version {
    /// Convert the given [`FullVersion`] to a [`semver::Version`].
    ///
    /// Requires the `semver` feature to be enabled.
    ///
    /// # Example
    ///
    /// ```
    /// # use version_number::FullVersion;
    ///
    /// let version = FullVersion::new(1, 2, 3);
    /// let converted: semver::Version = version.into();
    ///
    /// assert_eq!(converted, semver::Version::new(1, 2, 3));
    /// ```
    ///
    /// [`FullVersion`]: crate::FullVersion
    /// [`semver::Version`]: https://docs.rs/semver/1/semver/struct.Version.html
    fn from(version: FullVersion) -> Self {
        semver::Version::new(version.major, version.minor, version.patch)
    }
}

#[cfg(feature = "semver")]
impl TryFrom<&semver::Version> for FullVersion {
    type Error = FromSemverError;

    /// Convert the given [`semver::Version`] to a [`FullVersion`].
    ///
    /// Requires the `semver` feature to be enabled.
    ///
    /// Fails if the version has a pre-release or build metadata label, since a [`FullVersion`]
    /// can't hold those.
    ///
    /// # Example
    ///
    /// ```
    /// # use version_number::FullVersion;
    /// # use std::convert::TryFrom;
    ///
    /// let version = semver::Version::new(1, 2, 3);
    /// let converted = FullVersion::try_from(&version).unwrap();
    ///
    /// assert_eq!(converted, FullVersion::new(1, 2, 3));
    ///
    /// let pre_release = semver::Version::parse("1.2.3-beta.1").unwrap();
    /// assert!(FullVersion::try_from(&pre_release).is_err());
    /// ```
    ///
    /// [`FullVersion`]: crate::FullVersion
    /// [`semver::Version`]: https://docs.rs/semver/1/semver/struct.Version.html
    fn try_from(version: &semver::Version) -> Result<Self, Self::Error> {
        if !version.pre.is_empty() {
            return Err(FromSemverError::PreRelease(version.pre.clone()));
        }

        if !version.build.is_empty() {
            return Err(FromSemverError::BuildMetadata(version.build.clone()));
        }

        Ok(FullVersion::new(
            version.major,
            version.minor,
            version.patch,
        ))
    }
}

/// The error returned when a [`semver::Version`] can't be converted to a [`FullVersion`].
///
/// [`FullVersion`]: crate::FullVersion
/// [`semver::Version`]: https://docs.rs/semver/1/semver/struct.Version.html
#[cfg(feature = "semver")]
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum FromSemverError {
    /// The version has a pre-release label, like the `beta.1` in `1.2.3-beta.1`.
    #[error("Pre-release labels are not supported, but got '{0}'")]
    PreRelease(semver::Prerelease),

    /// The version has a build metadata label, like the `abc` in `1.2.3+abc`.
    #[error("Build metadata labels are not supported, but got '{0}'")]
    BuildMetadata(semver::BuildMetadata),
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

impl FromStr for FullVersion {
    type Err = ParserError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::parse(input)
    }
}

#[cfg(all(test, feature = "semver"))]
mod semver_tests {
    use crate::{FromSemverError, FullVersion};
    use std::convert::TryFrom;

    #[yare::parameterized(
        zeros = { "0.0.0", FullVersion::new(0, 0, 0) },
        regular = { "1.2.3", FullVersion::new(1, 2, 3) },
        large = { "18446744073709551615.0.1", FullVersion::new(u64::MAX, 0, 1) },
    )]
    fn try_from_semver_ok(input: &str, expected: FullVersion) {
        let version = semver::Version::parse(input).unwrap();

        assert_eq!(FullVersion::try_from(&version).unwrap(), expected);
    }

    #[yare::parameterized(
        nightly = { "1.2.3-nightly", "nightly" },
        beta = { "1.2.3-beta.1", "beta.1" },
        pre_release_and_build = { "1.2.3-beta.1+abc", "beta.1" },
    )]
    fn try_from_semver_pre_release(input: &str, expected: &str) {
        let version = semver::Version::parse(input).unwrap();

        assert_eq!(
            FullVersion::try_from(&version).unwrap_err(),
            FromSemverError::PreRelease(semver::Prerelease::new(expected).unwrap())
        );
    }

    #[test]
    fn try_from_semver_build_metadata() {
        let version = semver::Version::parse("1.2.3+abc").unwrap();

        assert_eq!(
            FullVersion::try_from(&version).unwrap_err(),
            FromSemverError::BuildMetadata(semver::BuildMetadata::new("abc").unwrap())
        );
    }

    #[test]
    fn round_trip() {
        let version = FullVersion::new(1, 2, 3);
        let converted = semver::Version::from(version);

        assert_eq!(FullVersion::try_from(&converted).unwrap(), version);
    }
}

#[cfg(test)]
mod tests {
    use crate::{BaseVersion, FullVersion};

    #[test]
    fn from_tuple() {
        let major = 0;
        let minor = 1;
        let patch = 2;

        assert_eq!(
            FullVersion {
                major,
                minor,
                patch
            },
            FullVersion::from((major, minor, patch))
        );
    }

    #[yare::parameterized(
        zeros = { FullVersion { major: 0, minor: 0, patch: 0 }, "0.0.0" },
        non_zero = { FullVersion { major: 1, minor: 2, patch: 3 }, "1.2.3" },
    )]
    fn display(base_version: FullVersion, expected: &str) {
        let displayed = format!("{}", base_version);

        assert_eq!(&displayed, expected);
    }

    #[test]
    fn to_base_version_lossy() {
        let full = FullVersion {
            major: 1,
            minor: 2,
            patch: 3,
        };
        let converted = full.to_base_version_lossy();

        assert_eq!(BaseVersion { major: 1, minor: 2 }, converted)
    }

    #[test]
    fn map() {
        let version = BaseVersion::new(1, 2);
        let mapped = version.map(|v| ("Everything is awesome", v.major, v.minor));

        assert_eq!(mapped.1, 1);
        assert_eq!(mapped.2, 2);
    }
}

#[cfg(test)]
mod ord_tests {
    use crate::FullVersion;
    use std::cmp::Ordering;

    #[yare::parameterized(
        zero = { FullVersion { major: 0, minor: 0, patch: 0 }, FullVersion { major: 0, minor: 0, patch: 0 } },
        ones = { FullVersion { major: 1, minor: 1, patch: 1 }, FullVersion { major: 1, minor: 1, patch: 1 } },
    )]
    fn equals(lhs: FullVersion, rhs: FullVersion) {
        assert_eq!(lhs.cmp(&rhs), Ordering::Equal);
    }

    #[yare::parameterized(
        major_by_1 = { FullVersion { major: 0, minor: 0, patch: 0 }, FullVersion { major: 1, minor: 0, patch: 0 } },
        minor_by_1 = { FullVersion { major: 0, minor: 0, patch: 0 }, FullVersion { major: 0, minor: 1, patch: 0 } },
        patch_by_1 = { FullVersion { major: 0, minor: 0, patch: 0 }, FullVersion { major: 0, minor: 0, patch: 1 } },
    )]
    fn less(lhs: FullVersion, rhs: FullVersion) {
        assert_eq!(lhs.cmp(&rhs), Ordering::Less);
    }

    #[yare::parameterized(
        major_by_1 = { FullVersion { major: 1, minor: 0, patch: 0 }, FullVersion { major: 0, minor: 0, patch: 0 } },
        minor_by_1 = { FullVersion { major: 0, minor: 1, patch: 0 }, FullVersion { major: 0, minor: 0, patch: 0 } },
        patch_by_1 = { FullVersion { major: 0, minor: 0, patch: 1 }, FullVersion { major: 0, minor: 0, patch: 0 } },
    )]
    fn greater(lhs: FullVersion, rhs: FullVersion) {
        assert_eq!(lhs.cmp(&rhs), Ordering::Greater);
    }
}

#[cfg(test)]
mod partial_ord_tests {
    use crate::FullVersion;
    use std::cmp::Ordering;

    #[yare::parameterized(
        zero = { FullVersion { major: 0, minor: 0, patch: 0 }, FullVersion { major: 0, minor: 0, patch: 0 } },
        ones = { FullVersion { major: 1, minor: 1, patch: 1 }, FullVersion { major: 1, minor: 1, patch: 1 } },
    )]
    fn equals(lhs: FullVersion, rhs: FullVersion) {
        assert_eq!(lhs.partial_cmp(&rhs), Some(Ordering::Equal));
    }

    #[yare::parameterized(
        major_by_1 = { FullVersion { major: 0, minor: 0, patch: 0 }, FullVersion { major: 1, minor: 0, patch: 0 } },
        minor_by_1 = { FullVersion { major: 0, minor: 0, patch: 0 }, FullVersion { major: 0, minor: 1, patch: 0 } },
        patch_by_1 = { FullVersion { major: 0, minor: 0, patch: 0 }, FullVersion { major: 0, minor: 0, patch: 1 } },
    )]
    fn less(lhs: FullVersion, rhs: FullVersion) {
        assert_eq!(lhs.partial_cmp(&rhs), Some(Ordering::Less));
    }

    #[yare::parameterized(
        major_by_1 = { FullVersion { major: 1, minor: 0, patch: 0 }, FullVersion { major: 0, minor: 0, patch: 0 } },
        minor_by_1 = { FullVersion { major: 0, minor: 1, patch: 0 }, FullVersion { major: 0, minor: 0, patch: 0 } },
        patch_by_1 = { FullVersion { major: 0, minor: 0, patch: 1 }, FullVersion { major: 0, minor: 0, patch: 0 } },
    )]
    fn greater(lhs: FullVersion, rhs: FullVersion) {
        assert_eq!(lhs.partial_cmp(&rhs), Some(Ordering::Greater));
    }
}

#[cfg(test)]
mod parse_full {
    use crate::parsers::error::ExpectedError;
    use crate::parsers::NumericError;
    use crate::{FullVersion, ParserError};

    #[test]
    fn ok() {
        let version = FullVersion::parse("1.2.3").unwrap();

        assert_eq!(version, FullVersion::new(1, 2, 3));
    }

    #[test]
    fn err_on_base_only() {
        let result = FullVersion::parse("1.2");

        assert!(matches!(
            result.unwrap_err(),
            ParserError::Expected(ExpectedError::Separator { .. })
        ));
    }

    #[test]
    fn err_on_not_finished() {
        let result = FullVersion::parse("1.2.3.");

        assert!(matches!(
            result.unwrap_err(),
            ParserError::Expected(ExpectedError::EndOfInput { .. })
        ));
    }

    #[test]
    fn err_on_starts_with_0() {
        let result = FullVersion::parse("1.2.03");

        assert!(matches!(
            result.unwrap_err(),
            ParserError::Numeric(NumericError::LeadingZero)
        ));
    }
}
