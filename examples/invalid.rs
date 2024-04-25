use version_number::Version;

fn main() {
    // Additional labels such as build flags are not supported!
    let err = Version::parse("1.0.0-alpha").unwrap_err();

    eprintln!("{err}"); // Expected end of input, but got '-' at 5.
}
