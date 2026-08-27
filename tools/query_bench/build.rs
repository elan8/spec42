fn main() {
    let profile = std::env::var("PROFILE").expect("Cargo sets PROFILE for build scripts");
    println!("cargo:rustc-env=SPEC42_BENCH_BUILD_PROFILE={profile}");
}
