use version_number::{BaseVersion, FullVersion};

fn main() {
    let original_base = BaseVersion::new(1, 2);

    let converted_full = original_base.to_full_version_lossy();
    assert_eq!(converted_full, FullVersion::new(1, 2, 0));

    let converted_base = converted_full.to_base_version_lossy();
    assert_eq!(converted_base, BaseVersion::new(1, 2));
}
