use std::fs;

use kpar::pack::{build_kpar, PackOptions};
use kpar::schema::Project;
use workspace::{
    library::bundle::discover_library_roots, resolve_explicit_library_path, EngineBuilder,
    LibraryInstallRoot, LibraryPackageRoots,
};

fn minimal_domain_kpar(work: &std::path::Path) -> std::path::PathBuf {
    let lib = work.join("lib");
    fs::create_dir_all(&lib).expect("create lib dir");
    fs::write(
        lib.join("Domain.sysml"),
        b"package Domain { part def Widget; }",
    )
    .expect("write model");
    let kpar_path = work.join("domain.kpar");
    let project = Project {
        name: "TestDomain".to_string(),
        version: "1.0.0".to_string(),
        description: None,
        license: None,
        publisher: None,
        maintainer: vec![],
        website: None,
        topic: vec![],
        usage: vec![],
    };
    let options = PackOptions {
        project,
        source_roots: vec![lib],
        excludes: vec![],
    };
    build_kpar(&options, &kpar_path).expect("pack kpar");
    kpar_path
}

#[test]
fn archive_path_materializes_to_package_roots() {
    let temp = tempfile::tempdir().expect("tempdir");
    let cache_dir = temp.path().join("cache");
    let kpar_path = minimal_domain_kpar(temp.path());
    let resolved =
        resolve_explicit_library_path(&kpar_path, &cache_dir, "domain-libraries").expect("resolve");
    assert_eq!(resolved.source, "archive-materialized");
    assert!(resolved.install_path.is_dir());
    assert!(!resolved.package_roots.roots.is_empty());
    assert!(resolved
        .package_roots
        .roots
        .iter()
        .all(|root| root.is_dir()));
}

#[test]
fn install_root_uses_discovered_package_roots() {
    let temp = tempfile::tempdir().expect("tempdir");
    let root = temp.path().join("materialized-root");
    let lib = root.join("lib");
    fs::create_dir_all(&lib).expect("create lib");
    fs::write(lib.join("Widget.sysml"), b"package Widget { part def A; }").expect("write");

    let package_roots = LibraryPackageRoots::from_install_root(&LibraryInstallRoot(root.clone()));
    assert!(package_roots
        .roots
        .iter()
        .any(|path| discover_library_roots(path)
            .iter()
            .any(|r| r.ends_with("lib"))
            || path.ends_with("lib")));

    let engine = EngineBuilder::default()
        .cache_dir(temp.path().join("cache"))
        .domain_libraries_path(root)
        .no_stdlib(true)
        .build()
        .expect("build engine");
    assert!(engine.package_roots().iter().any(|path| path.is_dir()));
}
