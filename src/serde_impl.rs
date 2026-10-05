use crate::{BaseVersion, FullVersion, Version};
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;

struct FromStrVisitor<T> {
    expecting: &'static str,
    marker: PhantomData<T>,
}

impl<T> FromStrVisitor<T> {
    fn new(expecting: &'static str) -> Self {
        Self {
            expecting,
            marker: PhantomData,
        }
    }
}

impl<'de, T> Visitor<'de> for FromStrVisitor<T>
where
    T: FromStr,
    T::Err: fmt::Display,
{
    type Value = T;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.expecting)
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        value.parse().map_err(E::custom)
    }
}

impl Serialize for BaseVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for BaseVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(FromStrVisitor::new(
            "a two-component `major.minor` version number",
        ))
    }
}

impl Serialize for FullVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for FullVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(FromStrVisitor::new(
            "a three-component `major.minor.patch` version number",
        ))
    }
}

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(FromStrVisitor::new(
            "a two- or three-component `major.minor` or `major.minor.patch` version number",
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::{BaseVersion, FullVersion, Version};

    #[yare::parameterized(
        base = { Version::new_base_version(1, 2), "\"1.2\"" },
        full = { Version::new_full_version(1, 2, 3), "\"1.2.3\"" },
    )]
    fn version_round_trip(version: Version, json: &str) {
        assert_eq!(serde_json::to_string(&version).unwrap(), json);
        assert_eq!(serde_json::from_str::<Version>(json).unwrap(), version);
    }

    #[test]
    fn base_version_round_trip() {
        let version = BaseVersion::new(1, 2);

        assert_eq!(serde_json::to_string(&version).unwrap(), "\"1.2\"");
        assert_eq!(
            serde_json::from_str::<BaseVersion>("\"1.2\"").unwrap(),
            version
        );
    }

    #[test]
    fn full_version_round_trip() {
        let version = FullVersion::new(1, 2, 3);

        assert_eq!(serde_json::to_string(&version).unwrap(), "\"1.2.3\"");
        assert_eq!(
            serde_json::from_str::<FullVersion>("\"1.2.3\"").unwrap(),
            version
        );
    }

    #[yare::parameterized(
        empty = { "\"\"" },
        one_component = { "\"1\"" },
        four_components = { "\"1.2.3.4\"" },
        pre_release = { "\"1.2.3-beta\"" },
        leading_zero = { "\"01.2\"" },
        number = { "1" },
    )]
    fn version_invalid(json: &str) {
        assert!(serde_json::from_str::<Version>(json).is_err());
    }

    #[test]
    fn base_version_rejects_full_version() {
        assert!(serde_json::from_str::<BaseVersion>("\"1.2.3\"").is_err());
    }

    #[test]
    fn full_version_rejects_base_version() {
        assert!(serde_json::from_str::<FullVersion>("\"1.2\"").is_err());
    }
}
