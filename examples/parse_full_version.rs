use version_number::FullVersion;

fn main() {
    let full_version = FullVersion::parse("1.2.3").expect("Unable to parse!");

    assert_eq!(full_version, FullVersion::new(1, 2, 3));
}
