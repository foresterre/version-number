# Changelog

## Unreleased

## [1.0.0]

### Added

* Added `Version::to_full_version_lossy` method
* Added `Version::is_compatible_with` method, which compares only the components present in the version
* Added `Version::matches` method
* Added `From<BaseVersion>` and `From<FullVersion>` implementations for `Version`
* Added `FromStr` implementations for `BaseVersion` and `FullVersion`
* Added `TryFrom<&semver::Version>` implementation for `FullVersion`, behind the `semver` feature
* Added `serde` feature, which (de)serializes `Version`, `BaseVersion` and `FullVersion` as strings

### Changed

* `BaseVersion::new`, `FullVersion::new`, `Version::new_base_version` and `Version::new_full_version` are now `const`
* Updated `thiserror` to version 2
* MSRV is now 1.61

[1.0.0]: https://github.com/foresterre/version-number/releases/tag/v1.0.0

## [0.4.0]

### Added

* Added `FullVersion::parse` convenience method
* Added `BaseVersion::parse` convenience method
* Added `Version::map` method
* Added `BaseVersion::map` method
* Added `FullVersion::map` method
* Added `Version::map_major` method
* Added `Version::map_minor` method
* Added `Version::map_patch` method

[0.4.0]: https://github.com/foresterre/version-number/releases/tag/v0.4.0

## [0.3.0]

### Added

* Added modular parser, which can parse both the BaseVersion and FullVersions in incrementally.

[0.3.0]: https://github.com/foresterre/version-number/releases/tag/v0.3.0

## [0.2.2]

### Fixed

* Updated docs which still referred to accepting leading zeros, while they're now rejected

[0.2.2]: https://github.com/foresterre/version-number/releases/tag/v0.2.2

## [0.2.1]

(Re-release of [0.2.0])

[0.2.1]: https://github.com/foresterre/version-number/releases/tag/v0.2.1

## [0.2.0]

### Changed

* Leading zeros are no longer allowed

[0.2.0]: https://github.com/foresterre/version-number/releases/tag/v0.2.0

## [0.1.0]

### Added

* Added definitions for two- and three component versions
* Added parser for two- and three component versions

[0.1.0]: https://github.com/foresterre/version-number/releases/tag/v0.1.0
