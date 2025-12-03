use std::{fs, path::PathBuf};

/// This test tries to write profile.sb file that describes the rules in the sandbox
/// to make sure potential attacker can not override those rules
fn main() {
    let profile_path = PathBuf::from("./target/cargo-safe/profile.sb");

    assert!(
        fs::exists(&profile_path).unwrap(),
        "File {} doesn't exists. Make sure profile location is consistent with the cargo-safe rules",
        profile_path.display()
    );

    fs::write(profile_path, "Malicious content").unwrap();
}
