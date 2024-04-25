use version_number::Version;

fn main() {
    let major_minor = Version::parse("1.27").unwrap();
    println!("Version: {major_minor}");

    let major_minor_patch = Version::parse("1.27.0").unwrap();
    println!("Version: {major_minor_patch}");
}
