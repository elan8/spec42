//! URI identity policy, owned by the source authority and re-exported here for hosts that
//! reach it through the language service.

pub use sysml_query::source::normalize_uri;

/// Returns true when `candidate` is under any of the library root URIs.
pub fn uri_under_any_library(candidate: &url::Url, library_paths: &[url::Url]) -> bool {
    sysml_query::source::uri_under_any(candidate, library_paths)
}
