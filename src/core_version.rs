use crate::FullVersion;
use std::fmt;

/// A two-component `major.minor` version.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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

impl From<FullVersion> for CoreVersion {
    /// A lossy conversion, which discards the `patch` component.
    fn from(v: FullVersion) -> Self {
        Self {
            major: v.major,
            minor: v.minor,
        }
    }
}

#[cfg(test)]
mod tests {
    mod conversion {
        use crate::{CoreVersion, FullVersion};

        #[test]
        fn lossy_conversion() {
            let full = FullVersion::from((1, 2, 3));
            let core = Into::<CoreVersion>::into(full.clone());

            assert_eq!(full.major, core.major);
            assert_eq!(full.minor, core.minor);
        }
    }
}
