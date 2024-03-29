use crate::BaseVersion;
use std::fmt;

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
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
        }
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
