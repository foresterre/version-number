use std::fmt;

/// A two-component `major.minor` version.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
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
    use crate::CoreVersion;

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
        zero_prefix = { CoreVersion { major: 01, minor: 02 }, "1.2" },
        non_zero = { CoreVersion { major: 1, minor: 2 }, "1.2" },
    )]
    fn display(core_version: CoreVersion, expected: &str) {
        let displayed = format!("{}", core_version);

        assert_eq!(&displayed, expected);
    }
}
