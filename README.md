# version-number

_Parsing the "version core" of semver numbers and their shorthands_

### Introduction

A crate to parse two- and three component version numbers. The three component version numbers are a subset of
semver, namely, just the "version core" of a semver version (that is, pre-release and/or build modifiers are not supported).
Two component versions are a shorthand of the three component version number, where the patch number is not specified.

Examples of two- respectively three component version numbers are `1.51` and `1.7.0`.

An example where this version type is found, is the `package.rust-version` field in the Cargo manifest (which crate authors use
to set the MSRV).

We call a two component `major.minor` version number, such as `1.51`, a **Core Version**, and a three component
`major.minor.patch` version number, such as `1.7.0`, a **Full Version**.

### Add as a dependency

To add `rust-version` as a dependency to your Rust project, you may run  `cargo add version-number`.

Alternatively, you may add the `version-number` crate manually to your Cargo manifest (i.e. `Cargo.toml`) as a dependency:

```toml
[dependencies]
version-number = "0.2"
```

### Usage

```rust
use version_number::Version;

fn main() {
    let core = Version::parse("1.27").unwrap();
    println!("Two component version: {}", core);
  
    let full = Version::parse("1.27.0").unwrap();
    println!("Three component version: {}", full);
}
```

Please refer to the [docs](https://docs.rs/version-number/latest/version_number/) to review all functionality.

### License
 
Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

#### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
