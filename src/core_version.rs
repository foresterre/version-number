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
pub struct CoreVersion {
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

impl CoreVersion {
    /// Instantiate a two component, version number with `MAJOR` and `MINOR` components.
    ///
    /// See [`CoreVersion`] for more.
    ///
    /// [`CoreVersion`]: crate::CoreVersion
    pub fn new(major: u64, minor: u64) -> Self {
        Self { major, minor }
    }

    /// Convert this core version to a full version.
    ///
    /// This conversion is lossy because the `patch` value is not known to this CoreVersion, and
    /// will initialized as `0`.
    pub fn to_full_version_lossy(self) -> FullVersion {
        FullVersion {
            major: self.major,
            minor: self.minor,
            patch: 0,
        }
    }
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

#[cfg(test)]
mod tests {
    use crate::{CoreVersion, FullVersion};

    #[test]
    fn from_tuple() {
        let major = 0;
        let minor = 1;

        assert_eq!(
            CoreVersion { major, minor },
            CoreVersion::from((major, minor))
        );
    }

    #[yare::parameterized(
        zeros = { CoreVersion { major: 0, minor: 0 }, "0.0" },
        non_zero = { CoreVersion { major: 1, minor: 2 }, "1.2" },
    )]
    fn display(core_version: CoreVersion, expected: &str) {
        let displayed = format!("{}", core_version);

        assert_eq!(&displayed, expected);
    }

    #[yare::parameterized(
        instance_0 = { CoreVersion::new(1, 0) },
        instance_1 = { CoreVersion::new(1, 1) },
        instance_m = { CoreVersion::new(1, u64::MAX) },
    )]
    fn to_full_version_lossy(core: CoreVersion) {
        let converted = core.to_full_version_lossy();

        assert_eq!(
            converted,
            FullVersion {
                major: core.major,
                minor: core.minor,
                patch: 0,
            }
        )
    }
}

#[cfg(test)]
mod ord_tests {
    use crate::CoreVersion;
    use std::cmp::Ordering;

    #[yare::parameterized(
        zero = { CoreVersion { major: 0, minor: 0 }, CoreVersion { major: 0, minor: 0 } },
        ones = { CoreVersion { major: 1, minor: 1 }, CoreVersion { major: 1, minor: 1 } },
    )]
    fn equals(lhs: CoreVersion, rhs: CoreVersion) {
        assert_eq!(lhs.cmp(&rhs), Ordering::Equal);
    }

    #[yare::parameterized(
        minor_by_1 = { CoreVersion { major: 0, minor: 0 }, CoreVersion { major: 0, minor: 1 } },
        major_by_1 = { CoreVersion { major: 1, minor: 0 }, CoreVersion { major: 2, minor: 0 } },
    )]
    fn less(lhs: CoreVersion, rhs: CoreVersion) {
        assert_eq!(lhs.cmp(&rhs), Ordering::Less);
    }

    #[yare::parameterized(
        minor_by_1 = { CoreVersion { major: 0, minor: 1 }, CoreVersion { major: 0, minor: 0 } },
        major_by_1 = { CoreVersion { major: 1, minor: 0 }, CoreVersion { major: 0, minor: 0 } },
    )]
    fn greater(lhs: CoreVersion, rhs: CoreVersion) {
        assert_eq!(lhs.cmp(&rhs), Ordering::Greater);
    }
}

#[cfg(test)]
mod partial_ord_tests {
    use crate::CoreVersion;
    use std::cmp::Ordering;

    #[yare::parameterized(
        zero = { CoreVersion { major: 0, minor: 0 }, CoreVersion { major: 0, minor: 0 } },
        ones = { CoreVersion { major: 1, minor: 1 }, CoreVersion { major: 1, minor: 1 } },
    )]
    fn equals(lhs: CoreVersion, rhs: CoreVersion) {
        assert_eq!(lhs.partial_cmp(&rhs), Some(Ordering::Equal));
    }

    #[yare::parameterized(
        minor_by_1 = { CoreVersion { major: 0, minor: 0 }, CoreVersion { major: 0, minor: 1 } },
        major_by_1 = { CoreVersion { major: 1, minor: 0 }, CoreVersion { major: 2, minor: 0 } },
    )]
    fn less(lhs: CoreVersion, rhs: CoreVersion) {
        assert_eq!(lhs.partial_cmp(&rhs), Some(Ordering::Less));
    }

    #[yare::parameterized(
        minor_by_1 = { CoreVersion { major: 0, minor: 1 }, CoreVersion { major: 0, minor: 0 } },
        major_by_1 = { CoreVersion { major: 1, minor: 0 }, CoreVersion { major: 0, minor: 0 } },
    )]
    fn greater(lhs: CoreVersion, rhs: CoreVersion) {
        assert_eq!(lhs.partial_cmp(&rhs), Some(Ordering::Greater));
    }
}
