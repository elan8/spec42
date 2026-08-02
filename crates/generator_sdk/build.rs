use std::{env, fs, path::PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(
        env::var_os("CARGO_MANIFEST_DIR").expect("Cargo provides CARGO_MANIFEST_DIR"),
    );
    let sdk_wit = manifest_dir.join("wit/generator.wit");
    let host_wit = manifest_dir.join("../generator_host/wit/generator.wit");

    println!("cargo:rerun-if-changed={}", sdk_wit.display());

    // The host crate is absent when this SDK is packaged or copied into an
    // external generator. Inside this workspace, fail the build if the two
    // public contract copies drift apart.
    if host_wit.exists() {
        println!("cargo:rerun-if-changed={}", host_wit.display());
        let sdk = fs::read(&sdk_wit).expect("read SDK WIT contract");
        let host = fs::read(&host_wit).expect("read host WIT contract");
        assert_eq!(
            sdk, host,
            "generator SDK WIT differs from crates/generator_host/wit/generator.wit"
        );
    }
}
