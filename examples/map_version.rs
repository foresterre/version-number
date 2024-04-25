use version_number::{Variant, Version};

fn main() {
    let version = Version::parse("9.8.7").unwrap();

    let opinions = version.map(|v| {
        if v.is(Variant::Base) {
            "Wowies"
        } else {
            "Nowsies"
        }
    });

    println!("{opinions}");
}
