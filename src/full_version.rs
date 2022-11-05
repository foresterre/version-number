use crate::CoreVersion;
use std::fmt;

/// A three-component `major.minor.patch` version.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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
    /// Convert this full version to a core version.
    ///
    /// This conversion is lossy because the `patch` value is lost upon conversion.
    pub fn to_core_version_lossy(self) -> CoreVersion {
        CoreVersion {
            major: self.major,
            minor: self.minor,
        }
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
    use crate::{CoreVersion, FullVersion};

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
        zero_prefix = { FullVersion { major: 01, minor: 02, patch: 03 }, "1.2.3" },
        non_zero = { FullVersion { major: 1, minor: 2, patch: 3 }, "1.2.3" },
    )]
    fn display(core_version: FullVersion, expected: &str) {
        let displayed = format!("{}", core_version);

        assert_eq!(&displayed, expected);
    }

    #[test]
    fn convert_lossy() {
        let full = FullVersion {
            major: 1,
            minor: 2,
            patch: 3,
        };
        let converted = full.to_core_version_lossy();

        assert_eq!(CoreVersion { major: 1, minor: 2 }, converted)
    }
}
