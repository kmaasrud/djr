use cargo_lock::Lockfile;
use std::path::PathBuf;

fn main() {
    let lockfile = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Cargo.lock");
    let lock = Lockfile::load(lockfile).unwrap();

    for pkg in lock.packages.iter() {
        // Populate the build env with the version of the djr dependency
        if pkg.name.as_str() == "djr" {
            println!("cargo:rustc-env=DJR_VERSION={}", pkg.version);
        }
    }
}
