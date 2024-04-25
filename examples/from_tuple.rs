use version_number::Version;

fn main() {
    let major_minor_from_tuple = Version::from((1, 27));
    println!("{major_minor_from_tuple}");

    let major_minor_patch_from_tuple = Version::from((1, 27, 0));
    println!("{major_minor_patch_from_tuple}");
}
