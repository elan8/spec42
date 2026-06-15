//! Legacy zip subset extraction for OMG release trees and git-archive bundles.

use std::fs;
use std::io::Cursor;
use std::path::Path;

use zip::read::ZipArchive;

use crate::error::{KparError, Result};

/// Extract files under `{archive_root}/{content_path}/` from a GitHub-style release zip.
pub fn extract_archive_subset(
    archive_bytes: &[u8],
    content_path: &str,
    destination_root: &Path,
) -> Result<()> {
    let cursor = Cursor::new(archive_bytes);
    let mut archive =
        ZipArchive::new(cursor).map_err(|e| KparError::Zip(format!("open archive: {e}")))?;
    if archive.is_empty() {
        return Err(KparError::InvalidArchive("archive is empty".to_string()));
    }

    let root_prefix = {
        let first = archive
            .by_index(0)
            .map_err(|e| KparError::Zip(format!("inspect archive: {e}")))?;
        first
            .name()
            .split('/')
            .next()
            .ok_or_else(|| KparError::InvalidArchive("malformed archive".to_string()))?
            .to_string()
    };

    let normalized = content_path.trim_matches('/').trim_matches('\\');
    let wanted_prefix = if normalized.is_empty() {
        format!("{root_prefix}/")
    } else {
        format!("{root_prefix}/{normalized}/")
    };

    let mut extracted_any = false;
    for index in 0..archive.len() {
        let mut entry = archive
            .by_index(index)
            .map_err(|e| KparError::Zip(format!("read entry {index}: {e}")))?;
        let name = entry.name().to_string();
        if !name.starts_with(&wanted_prefix) || name.ends_with('/') {
            continue;
        }
        let relative = name.trim_start_matches(&wanted_prefix);
        let destination = destination_root.join(relative);
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent).map_err(|source| KparError::Io {
                path: parent.display().to_string(),
                source,
            })?;
        }
        let mut file = fs::File::create(&destination).map_err(|source| KparError::Io {
            path: destination.display().to_string(),
            source,
        })?;
        std::io::copy(&mut entry, &mut file).map_err(|source| KparError::Io {
            path: destination.display().to_string(),
            source,
        })?;
        extracted_any = true;
    }

    if !extracted_any {
        return Err(KparError::InvalidArchive(format!(
            "path '{content_path}' was not found in archive"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;
    use zip::write::{SimpleFileOptions, ZipWriter};

    #[test]
    fn extract_subset_from_release_zip() {
        let temp = tempdir().expect("tempdir");
        let archive_path = temp.path().join("release.zip");
        {
            let file = fs::File::create(&archive_path).expect("create");
            let mut zip = ZipWriter::new(file);
            let options = SimpleFileOptions::default();
            zip.start_file("release-2026-04/sysml.library/A.sysml", options)
                .expect("start");
            zip.write_all(b"package A {}").expect("write");
            zip.start_file("release-2026-04/other/B.sysml", options)
                .expect("start other");
            zip.write_all(b"package B {}").expect("write other");
            zip.finish().expect("finish");
        }
        let bytes = fs::read(&archive_path).expect("read");
        let dest = temp.path().join("out");
        extract_archive_subset(&bytes, "sysml.library", &dest).expect("extract");
        assert!(dest.join("A.sysml").is_file());
        assert!(!dest.join("B.sysml").exists());
    }
}
