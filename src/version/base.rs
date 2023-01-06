use crate::FullVersion;
use std::fmt;

/// A two-component `MAJOR.MINOR` version.
///
/// This version number is a subset of [`semver`]. In particular, it consists of the `MAJOR`
/// and `MINOR` components, and leaves out the `PATCH` and additional labels for pre-release
/// and build metadata.
///
/// If you require a version number which also includes the `PATCH` number,
/// please see the [`FullVersion`] variant. For a [`semver`] compliant parser, you should use
/// the `semver` [`crate`] instead.
///
/// [`semver`]: https://semver.org/spec/v2.0.0.html
/// [`FullVersion`]: crate::FullVersion
/// [`crate`]: https://crates.io/crates/semver
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BaseVersion {
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
}

impl BaseVersion {
    /// Instantiate a two component, version number with `MAJOR` and `MINOR` components.
    ///
    /// See [`BaseVersion`] for more.
    ///
    /// [`BaseVersion`]: crate::BaseVersion
    pub fn new(major: u64, minor: u64) -> Self {
        Self { major, minor }
    }

    /// Convert this base version to a full version.
    ///
    /// This conversion is lossy because the `patch` value is not known to this BaseVersion, and
    /// will initialized as `0`.
    pub fn to_full_version_lossy(self) -> FullVersion {
        FullVersion {
            major: self.major,
            minor: self.minor,
            patch: 0,
        }
    }
}

impl From<(u64, u64)> for BaseVersion {
    fn from(tuple: (u64, u64)) -> Self {
        BaseVersion {
            major: tuple.0,
            minor: tuple.1,
        }
    }
}

impl fmt::Display for BaseVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}.{}", self.major, self.minor))
    }
}

#[cfg(test)]
mod tests {
    use crate::{BaseVersion, FullVersion};

    #[test]
    fn from_tuple() {
        let major = 0;
        let minor = 1;

        assert_eq!(
            BaseVersion { major, minor },
            BaseVersion::from((major, minor))
        );
    }

    #[yare::parameterized(
        zeros = { BaseVersion { major: 0, minor: 0 }, "0.0" },
        non_zero = { BaseVersion { major: 1, minor: 2 }, "1.2" },
    )]
    fn display(base_version: BaseVersion, expected: &str) {
        let displayed = format!("{}", base_version);

        assert_eq!(&displayed, expected);
    }

    #[yare::parameterized(
        instance_0 = { BaseVersion::new(1, 0) },
        instance_1 = { BaseVersion::new(1, 1) },
        instance_m = { BaseVersion::new(1, u64::MAX) },
    )]
    fn to_full_version_lossy(base: BaseVersion) {
        let converted = base.to_full_version_lossy();

        assert_eq!(
            converted,
            FullVersion {
                major: base.major,
                minor: base.minor,
                patch: 0,
            }
        )
    }
}

#[cfg(test)]
mod ord_tests {
    use crate::BaseVersion;
    use std::cmp::Ordering;

    #[yare::parameterized(
        zero = { BaseVersion { major: 0, minor: 0 }, BaseVersion { major: 0, minor: 0 } },
        ones = { BaseVersion { major: 1, minor: 1 }, BaseVersion { major: 1, minor: 1 } },
    )]
    fn equals(lhs: BaseVersion, rhs: BaseVersion) {
        assert_eq!(lhs.cmp(&rhs), Ordering::Equal);
    }

    #[yare::parameterized(
        minor_by_1 = { BaseVersion { major: 0, minor: 0 }, BaseVersion { major: 0, minor: 1 } },
        major_by_1 = { BaseVersion { major: 1, minor: 0 }, BaseVersion { major: 2, minor: 0 } },
    )]
    fn less(lhs: BaseVersion, rhs: BaseVersion) {
        assert_eq!(lhs.cmp(&rhs), Ordering::Less);
    }

    #[yare::parameterized(
        minor_by_1 = { BaseVersion { major: 0, minor: 1 }, BaseVersion { major: 0, minor: 0 } },
        major_by_1 = { BaseVersion { major: 1, minor: 0 }, BaseVersion { major: 0, minor: 0 } },
    )]
    fn greater(lhs: BaseVersion, rhs: BaseVersion) {
        assert_eq!(lhs.cmp(&rhs), Ordering::Greater);
    }
}

#[cfg(test)]
mod partial_ord_tests {
    use crate::BaseVersion;
    use std::cmp::Ordering;

    #[yare::parameterized(
        zero = { BaseVersion { major: 0, minor: 0 }, BaseVersion { major: 0, minor: 0 } },
        ones = { BaseVersion { major: 1, minor: 1 }, BaseVersion { major: 1, minor: 1 } },
    )]
    fn equals(lhs: BaseVersion, rhs: BaseVersion) {
        assert_eq!(lhs.partial_cmp(&rhs), Some(Ordering::Equal));
    }

    #[yare::parameterized(
        minor_by_1 = { BaseVersion { major: 0, minor: 0 }, BaseVersion { major: 0, minor: 1 } },
        major_by_1 = { BaseVersion { major: 1, minor: 0 }, BaseVersion { major: 2, minor: 0 } },
    )]
    fn less(lhs: BaseVersion, rhs: BaseVersion) {
        assert_eq!(lhs.partial_cmp(&rhs), Some(Ordering::Less));
    }

    #[yare::parameterized(
        minor_by_1 = { BaseVersion { major: 0, minor: 1 }, BaseVersion { major: 0, minor: 0 } },
        major_by_1 = { BaseVersion { major: 1, minor: 0 }, BaseVersion { major: 0, minor: 0 } },
    )]
    fn greater(lhs: BaseVersion, rhs: BaseVersion) {
        assert_eq!(lhs.partial_cmp(&rhs), Some(Ordering::Greater));
    }
}
