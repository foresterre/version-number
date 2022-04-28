use version_number::Version;

fn main() {
    // Additional labels such as build flags are not supported!
    let err = Version::parse("1.0.0-alpha").unwrap_err();

    // prints:
    // Unable to parse '1.0.0-alpha' to a version number: Expected end of input after parsing third version number component, but got: '-alpha'
    //                       ^~~~~~
    eprintln!("{}", err);
}
