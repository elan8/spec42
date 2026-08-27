use std::fs;
use std::io::{Cursor, Read};
use std::process::Command;

use tempfile::tempdir;

#[test]
fn bundle_and_unbundle_use_project_metadata_defaults_end_to_end() {
    let temp = tempdir().expect("tempdir");
    let source = temp.path().join("source");
    fs::create_dir_all(&source).expect("create source");
    fs::write(
        source.join(".project.json"),
        br#"{"name":"example-library","version":"1.2.3"}"#,
    )
    .expect("write project metadata");
    fs::write(source.join("Example.sysml"), "package Example {}").expect("write source");

    let binary = env!("CARGO_BIN_EXE_spec42");
    let bundled = Command::new(binary)
        .current_dir(temp.path())
        .args([
            "bundle",
            source.to_str().expect("utf-8 source"),
            "--no-compress",
        ])
        .output()
        .expect("run bundle");
    assert!(
        bundled.status.success(),
        "bundle failed: {}",
        String::from_utf8_lossy(&bundled.stderr)
    );
    let archive = temp.path().join("example-library-1.2.3.kpar");
    assert!(
        archive.is_file(),
        "bundle default output must use project metadata"
    );
    let mut zip =
        zip::read::ZipArchive::new(Cursor::new(fs::read(&archive).expect("read archive")))
            .expect("open archive");
    assert_eq!(
        zip.by_name("Example.sysml")
            .expect("source archive entry")
            .compression(),
        zip::CompressionMethod::Stored,
        "--no-compress must store source entries without ZIP compression"
    );
    let mut archived_project = Vec::new();
    zip.by_name(".project.json")
        .expect("project archive entry")
        .read_to_end(&mut archived_project)
        .expect("read project archive entry");
    let archive_bytes = fs::read(&archive).expect("read initial archive");
    let duplicate_bundle = Command::new(binary)
        .current_dir(temp.path())
        .args(["bundle", source.to_str().expect("utf-8 source")])
        .output()
        .expect("run duplicate bundle");
    assert!(!duplicate_bundle.status.success());
    assert_eq!(fs::read(&archive).expect("archive remains"), archive_bytes);

    let unpacked = Command::new(binary)
        .current_dir(temp.path())
        .args(["unbundle", archive.to_str().expect("utf-8 archive")])
        .output()
        .expect("run unbundle");
    assert!(
        unpacked.status.success(),
        "unbundle failed: {}",
        String::from_utf8_lossy(&unpacked.stderr)
    );
    assert!(
        temp.path()
            .join("example-library")
            .join("Example.sysml")
            .is_file(),
        "unbundle default destination must use the archive project name"
    );
    let unpacked_project = temp.path().join("example-library").join(".project.json");
    assert_eq!(
        fs::read(&unpacked_project).expect("unbundled project metadata"),
        archived_project,
        "unbundle must restore the archive's project metadata"
    );
    assert!(
        !temp
            .path()
            .join("example-library")
            .join(".meta.json")
            .exists(),
        "archive-specific metadata must not be restored as workspace configuration"
    );

    let rebundled_archive = temp.path().join("rebundled.kpar");
    let rebundled_directory = temp.path().join("example-library");
    let rebundled = Command::new(binary)
        .current_dir(temp.path())
        .args([
            "bundle",
            rebundled_directory
                .to_str()
                .expect("utf-8 unbundled directory"),
            "-o",
            rebundled_archive.to_str().expect("utf-8 archive path"),
        ])
        .output()
        .expect("rebundle unbundled project");
    assert!(
        rebundled.status.success(),
        "rebundling the unbundled project failed: {}",
        String::from_utf8_lossy(&rebundled.stderr)
    );
    assert!(rebundled_archive.is_file());

    let duplicate_unbundle = Command::new(binary)
        .current_dir(temp.path())
        .args(["unbundle", archive.to_str().expect("utf-8 archive")])
        .output()
        .expect("run duplicate unbundle");
    assert!(!duplicate_unbundle.status.success());
    assert!(
        temp.path()
            .join("example-library")
            .join("Example.sysml")
            .is_file(),
        "a refused unbundle must preserve the existing destination"
    );
}

#[test]
fn bundle_rejects_a_directory_without_project_metadata() {
    let temp = tempdir().expect("tempdir");
    let source = temp.path().join("source");
    fs::create_dir_all(&source).expect("create source");
    fs::write(source.join("Example.sysml"), "package Example {}").expect("write source");

    let result = Command::new(env!("CARGO_BIN_EXE_spec42"))
        .current_dir(temp.path())
        .args(["bundle", source.to_str().expect("utf-8 source")])
        .output()
        .expect("run bundle");

    assert!(!result.status.success());
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(stderr.contains(".project.json"), "{stderr}");
    assert!(stderr.contains("spec42 init"), "{stderr}");
}
