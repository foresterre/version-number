#![allow(missing_docs)]

use crate::{CoreVersion, Version};
use std::borrow::Borrow;
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;

#[derive(Debug, thiserror::Error, Eq, PartialEq)]
#[error("The range must not be empty")]
pub struct EmptyRangeError;

/// An unidirectional range from smaller to larger core version.
/// We encode the versions as 128-bit integers.
#[derive(Debug)]
pub struct CoreRange {
    begin: EncodedVersion,
    end: EncodedVersion,
}

impl CoreRange {
    pub fn try_new<L: Into<CoreVersion>, R: Into<CoreVersion>>(
        begin_inclusive: L,
        end_exclusive: R,
    ) -> Result<Self, EmptyRangeError> {
        let lhs = begin_inclusive.into();
        let rhs = end_exclusive.into();

        if lhs.major < rhs.major || lhs.minor < rhs.minor {
            Ok(Self {
                begin: EncodedVersion::from(lhs),
                end: EncodedVersion::from(rhs),
            })
        } else {
            Err(EmptyRangeError)
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct EncodedVersion {
    encoded: u128,
}

impl From<CoreVersion> for EncodedVersion {
    fn from(version: CoreVersion) -> Self {
        let encoded = (version.major as u128) << 64 | (version.minor as u128);

        Self { encoded }
    }
}

impl From<EncodedVersion> for CoreVersion {
    fn from(version: EncodedVersion) -> Self {
        let inner = version.encoded;

        Self {
            major: (inner >> 64) as u64,
            minor: inner as u64,
        }
    }
}

pub struct RangeMap<V> {
    inner: BTreeMap<CoreRange, V>,
}

impl<V> RangeMap<V> {
    pub fn empty() -> Self {
        Self {
            inner: BTreeMap::new(),
        }
    }

    /// Checks whether the given range is available to the map as a key.
    pub fn contains(&self, range: &impl Into<CoreRange>) -> bool {
        todo!()
    }

    /// Returns the version range of which the given `version` is part,
    /// assuming it exists in the map.
    pub fn range(&self, version: CoreVersion) -> Option<&CoreRange> {
        todo!()
    }

    /// Returns the value which matches the version range of which the given `version` is part,
    /// assuming it exists in the map.
    pub fn value(&self, version: CoreVersion) -> Option<&V> {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    mod instantiations {
        use crate::range::{CoreRange, EmptyRangeError};
        use crate::CoreVersion;

        #[yare::parameterized(
            one_major_diff = { CoreVersion::from((1, 0)), CoreVersion::from((2, 0)) },
            larger_major_with_smaller_minor = { CoreVersion::from((1, 1)), CoreVersion::from((2, 1)) },
            one_minor_diff = { CoreVersion::from((1, 0)), CoreVersion::from((1, 2)) },
        )]
        fn accept(lhs: CoreVersion, rhs: CoreVersion) {
            assert!(CoreRange::try_new(lhs, rhs).is_ok())
        }

        #[yare::parameterized(
            identity = { CoreVersion::from((0, 0)), CoreVersion::from((0, 0)) },
            eq_major = { CoreVersion::from((1, 0)), CoreVersion::from((1, 0)) },
            eq_minor = { CoreVersion::from((0, 1)), CoreVersion::from((0, 1)) },
            empty_set_on_major = { CoreVersion::from((1, 0)), CoreVersion::from((0, 0)) },
            empty_set_on_minor = { CoreVersion::from((1, 1)), CoreVersion::from((1, 0)) },
        )]
        fn reject(lhs: CoreVersion, rhs: CoreVersion) {
            assert_eq!(CoreRange::try_new(lhs, rhs).unwrap_err(), EmptyRangeError);
        }
    }

    mod encoding {
        use crate::range::EncodedVersion;
        use crate::CoreVersion;

        #[yare::parameterized(
            major = { CoreVersion::from((1, 0)), CoreVersion::from((0, 0)) },
            minor = { CoreVersion::from((0, 1)), CoreVersion::from((0, 0)) },
            large_major = { CoreVersion::from((u64::MAX, 0)), CoreVersion::from((u64::MAX-1, 0)) },
            large_minor = { CoreVersion::from((0, u64::MAX)), CoreVersion::from((0, u64::MAX-1)) },
            large_components = { CoreVersion::from((u64::MAX/2, u64::MAX/2)), CoreVersion::from((u64::MAX/3, u64::MAX/3)) },
        )]
        fn inequalities(lhs: CoreVersion, rhs: CoreVersion) {
            let lhs = EncodedVersion::from(lhs);
            let rhs = EncodedVersion::from(rhs);

            assert!(lhs.encoded > rhs.encoded)
        }

        #[test]
        fn unpack() {
            let unpacked1 = CoreVersion::from((1, 2));
            let packed = EncodedVersion::from(unpacked1);
            let unpacked2 = CoreVersion::from(packed);

            assert_eq!(unpacked1, unpacked2);
        }
    }

    mod use_case {
        use crate::range::CoreRange;
        use crate::CoreVersion;
        use std::collections::BTreeMap;

        #[test]
        fn test() {
            // A mapping from a version range to a command
            let mapping = BTreeMap::<CoreRange, String>::new();

            // for this to be true, a given version, must be comparable to a range
            // since we have `K: Borrow<Q> + Ord,`with Q := given version, we have to impl
            // Borrow<Version> for CoreRange and Ord for CoreRange.

            let given_version = CoreVersion::from((1, 2));
        }
    }
}
