use version_number::parsers::modular::ModularParser;
use version_number::{
    BaseVersion, BaseVersionParser, FullVersion, FullVersionParser, Version, VersionParser,
};

fn main() {
    let input_base = "1.2";

    // Parse to BaseVersion
    let only_base = ModularParser
        .parse_base(input_base)
        .expect("Unable to parse two component MAJOR.MINOR version with 'parse_base'");

    assert_eq!(only_base, BaseVersion::new(1, 2));
    println!("Version (two components, with: 'parse_base'): {only_base}");

    let input_full = "1.2.3";

    // Parse to FullVersion
    let only_full = ModularParser
        .parse_full(input_full)
        .expect("Unable to parse three component MAJOR.MINOR.PATCH version with 'parse_full'");

    assert_eq!(only_full, FullVersion::new(1, 2, 3));
    println!("Version (three components, with: 'parse_full'): {only_full}");

    // Parse to Version::Base(BaseVersion)
    let either_base = ModularParser
        .parse_version(input_base)
        .expect("Unable to parse two component MAJOR.MINOR version with 'parse_version'");

    assert_eq!(either_base, Version::Base(only_base));
    println!("Version (two components, with: 'parse_version'): {either_base}");

    // Parse to Version::Full(FullVersion)
    let either_full = ModularParser
        .parse_version(input_full)
        .expect("Unable to parse two component MAJOR.MINOR.PATCH version with 'parse_version'");

    assert_eq!(either_full, Version::Full(only_full));
    println!("Version (three components, with: 'parse_version'): {either_full}");
}
