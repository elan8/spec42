//! Compact display of source identities in editor presentation.
//!
//! The semantic publication retains the complete normalized identity. These helpers produce only
//! display labels: navigation continues to use the original URI, and no semantic provenance is
//! inferred from path spelling.

const DEFAULT_MAX_CHARS: usize = 72;

pub(crate) fn source_identity_label(identity: &str) -> String {
    source_identity_label_with_limit(identity, DEFAULT_MAX_CHARS)
}

fn source_identity_label_with_limit(identity: &str, limit: usize) -> String {
    if identity.chars().count() <= limit {
        return identity.to_string();
    }

    if let Ok(uri) = url::Url::parse(identity) {
        if uri.scheme() == "file" {
            return file_uri_label(uri.path(), limit);
        }
        if let Some(authority) = uri.host_str() {
            let mut prefix = format!("{}://{}", uri.scheme(), authority);
            if let Some(port) = uri.port() {
                prefix.push_str(&format!(":{port}"));
            }
            let suffix = significant_path_suffix(uri.path(), 2);
            let mut label = format!("{prefix}/…/{suffix}");
            if let Some(query) = uri.query() {
                label.push('?');
                label.push_str(query);
            }
            if let Some(fragment) = uri.fragment() {
                label.push('#');
                label.push_str(fragment);
            }
            return middle_elide(&label, limit);
        }
    }

    path_label(identity, limit)
}

fn file_uri_label(path: &str, limit: usize) -> String {
    if path.chars().count() <= limit {
        return path.to_string();
    }
    let prefix = path
        .split('/')
        .find(|segment| !segment.is_empty())
        .map(|segment| format!("/{segment}"))
        .unwrap_or_default();
    middle_elide(
        &format!("{prefix}/…/{}", significant_path_suffix(path, 4)),
        limit,
    )
}

fn path_label(path: &str, limit: usize) -> String {
    if path.chars().count() <= limit {
        return path.to_string();
    }
    let normalized = path.replace('\\', "/");
    let suffix = significant_path_suffix(&normalized, 3);
    let prefix = if normalized.starts_with('/') {
        normalized
            .split('/')
            .find(|segment| !segment.is_empty())
            .map(|segment| format!("/{segment}"))
            .unwrap_or_default()
    } else if normalized.as_bytes().get(1) == Some(&b':') {
        normalized[..2].to_string()
    } else {
        normalized
            .split('/')
            .find(|segment| !segment.is_empty())
            .unwrap_or_default()
            .to_string()
    };
    middle_elide(&format!("{prefix}/…/{suffix}"), limit)
}

fn significant_path_suffix(path: &str, count: usize) -> String {
    let segments = path
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();
    segments[segments.len().saturating_sub(count)..].join("/")
}

fn middle_elide(value: &str, limit: usize) -> String {
    let length = value.chars().count();
    if length <= limit {
        return value.to_string();
    }
    if limit <= 1 {
        return "…".chars().take(limit).collect();
    }
    let head = (limit - 1) / 2;
    let tail = limit - 1 - head;
    let start = value.chars().take(head).collect::<String>();
    let end = value
        .chars()
        .skip(length.saturating_sub(tail))
        .collect::<String>();
    format!("{start}…{end}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_identities_are_unchanged() {
        assert_eq!(
            source_identity_label_with_limit("models/domain.sysml", 40),
            "models/domain.sysml"
        );
    }

    #[test]
    fn file_uris_keep_the_distinguishing_tail() {
        let identity = "file:///Users/luke/Library/Application%20Support/io.Elan8.spec42/standard-library/versions/2026-04/kpar/Kernel_Data_Type_Library-1.0.0/ScalarValues.kerml";
        let label = source_identity_label_with_limit(identity, 72);
        assert!(label.contains('…'));
        assert!(label.ends_with("2026-04/kpar/Kernel_Data_Type_Library-1.0.0/ScalarValues.kerml"));
        assert!(!label.starts_with("file://"));
    }

    #[test]
    fn web_uris_keep_authority_filename_and_fragment() {
        let identity = "https://www.omg.org/spec/SysML/20250201/resource/with/a/very/long/path/Systems-Library.kpar#Systems";
        let label = source_identity_label_with_limit(identity, 72);
        assert!(label.starts_with("https://www.omg.org/"));
        assert!(label.contains('…'));
        assert!(label.ends_with("Systems-Library.kpar#Systems"));
    }

    #[test]
    fn windows_paths_keep_drive_and_filename() {
        let path =
            r"C:\Users\modeler\repositories\large-monorepo\models\vehicle\domain\Vehicle.sysml";
        let label = source_identity_label_with_limit(path, 48);
        assert!(label.starts_with("C:/…/"));
        assert!(label.ends_with("vehicle/domain/Vehicle.sysml"));
    }

    #[test]
    fn middle_elision_counts_unicode_characters() {
        let label = middle_elide("αβγδεζηθικ", 7);
        assert_eq!(label, "αβγ…θικ");
        assert_eq!(label.chars().count(), 7);
    }
}
