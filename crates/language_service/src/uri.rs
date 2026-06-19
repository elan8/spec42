use url::Url;

/// Normalizes a document URI for consistent lookup (file URIs on Windows, etc.).
pub fn normalize_uri(uri: &Url) -> Url {
    if uri.scheme() != "file" {
        return uri.clone();
    }
    if let Ok(path) = uri.to_file_path() {
        if let Ok(mut normalized) = Url::from_file_path(path) {
            let p = normalized.path();
            if p.len() >= 3 {
                let mut chars: Vec<char> = p.chars().collect();
                if chars[0] == '/'
                    && chars[1].is_ascii_alphabetic()
                    && chars.get(2) == Some(&':')
                {
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

/// Returns true when `candidate` is under any of the library root URIs.
pub fn uri_under_any_library(candidate: &Url, library_paths: &[Url]) -> bool {
    library_paths
        .iter()
        .any(|root| candidate.as_str().starts_with(root.as_str()))
}
