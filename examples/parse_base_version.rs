use version_number::BaseVersion;

fn main() {
    let full_version = BaseVersion::parse("1.2").expect("Unable to parse!");

    assert_eq!(full_version, BaseVersion::new(1, 2));
}
