use url::Url;

/// Normalizes a document URI for consistent lookup.
///
/// Existing file paths are canonicalized so alternate filesystem spellings (for example,
/// macOS's `/var` symlink to `/private/var`) cannot create distinct document identities.
/// Nonexistent paths retain their lexical identity, which is required for editor buffers that
/// have not been saved yet. Windows drive letters are normalized to lowercase in either case.
pub fn normalize_uri(uri: &Url) -> Url {
    if uri.scheme() != "file" {
        return uri.clone();
    }
    if let Ok(path) = uri.to_file_path() {
        let path = std::fs::canonicalize(&path).unwrap_or(path);
        if let Ok(mut normalized) = Url::from_file_path(path) {
            let p = normalized.path();
            if p.len() >= 3 {
                let mut chars: Vec<char> = p.chars().collect();
                if chars[0] == '/' && chars[1].is_ascii_alphabetic() && chars.get(2) == Some(&':') {
                    chars[1] = chars[1].to_ascii_lowercase();
                    let new_path: String = chars.into_iter().collect();
                    if let Ok(u) = Url::parse(&format!("file://{new_path}")) {
                        normalized = u;
                    }
                }
            }
            return normalized;
        }
    }
    uri.clone()
}

#[cfg(test)]
mod tests {
    use super::normalize_uri;
    use url::Url;

    #[cfg(unix)]
    #[test]
    fn existing_file_uris_use_the_canonical_filesystem_identity() {
        use std::os::unix::fs::symlink;

        let temp = tempfile::tempdir().expect("tempdir");
        let real_dir = temp.path().join("real");
        let alias_dir = temp.path().join("alias");
        std::fs::create_dir(&real_dir).expect("real directory");
        symlink(&real_dir, &alias_dir).expect("directory symlink");
        let real_file = real_dir.join("Model.sysml");
        std::fs::write(&real_file, "package Model;").expect("model");

        let aliased = Url::from_file_path(alias_dir.join("Model.sysml")).expect("aliased URI");
        let canonical = Url::from_file_path(&real_file).expect("canonical URI");

        assert_eq!(normalize_uri(&aliased), normalize_uri(&canonical));
    }

    #[test]
    fn nonexistent_file_uri_keeps_its_lexical_identity() {
        let temp = tempfile::tempdir().expect("tempdir");
        let path = temp.path().join("Unsaved.sysml");
        let uri = Url::from_file_path(path).expect("unsaved URI");

        assert_eq!(normalize_uri(&uri), uri);
    }
}

/// Returns true when `candidate` is under any of the library root URIs.
pub fn uri_under_any_library(candidate: &Url, library_paths: &[Url]) -> bool {
    library_paths
        .iter()
        .any(|root| candidate.as_str().starts_with(root.as_str()))
}
