//! Source admission contracts shared by hosts and semantic construction.

use source_identity::ContentDigest;
use url::Url;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SysmlDocumentSourceKind {
    Workspace,
    StandardLibrary,
    Library,
    External,
}

#[derive(Debug, Clone)]
pub struct SysmlDocument {
    pub uri: Url,
    pub content: String,
    pub path_hint: Option<String>,
    pub source_kind: SysmlDocumentSourceKind,
    pub content_digest: Option<ContentDigest>,
    pub byte_size: Option<i64>,
}

pub trait SysmlDocumentProvider {
    fn load_documents(&self) -> Result<Vec<SysmlDocument>, String>;
}

#[derive(Debug, Default, Clone)]
pub struct InMemoryDocumentProvider {
    pub documents: Vec<SysmlDocument>,
}

impl InMemoryDocumentProvider {
    pub fn new(documents: Vec<SysmlDocument>) -> Self {
        Self { documents }
    }
}

impl SysmlDocumentProvider for InMemoryDocumentProvider {
    fn load_documents(&self) -> Result<Vec<SysmlDocument>, String> {
        Ok(self.documents.clone())
    }
}

impl SysmlDocument {
    pub fn from_memory_path(
        scope: &str,
        path: &str,
        content: String,
        source_kind: SysmlDocumentSourceKind,
        content_digest: Option<ContentDigest>,
        byte_size: Option<i64>,
    ) -> Result<Self, String> {
        let normalized_path = path.trim_start_matches('/').replace('\\', "/");
        let uri = Url::parse(&format!("memory://{scope}/{normalized_path}"))
            .map_err(|error| format!("failed to build source URI for {path}: {error}"))?;
        Ok(Self {
            uri,
            content,
            path_hint: Some(path.to_string()),
            source_kind,
            content_digest,
            byte_size,
        })
    }

    pub fn from_uri(
        uri: &str,
        content: String,
        path_hint: Option<String>,
        source_kind: SysmlDocumentSourceKind,
        content_digest: Option<ContentDigest>,
        byte_size: Option<i64>,
    ) -> Result<Self, String> {
        let uri = Url::parse(uri)
            .map_err(|error| format!("failed to parse source URI '{uri}': {error}"))?;
        Ok(Self {
            uri,
            content,
            path_hint,
            source_kind,
            content_digest,
            byte_size,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_uri_schemes_are_preserved() {
        let document = SysmlDocument::from_uri(
            "surreal://org/project/document/Architecture.sysml",
            "package Architecture {}".to_string(),
            Some("Architecture.sysml".to_string()),
            SysmlDocumentSourceKind::External,
            None,
            None,
        )
        .expect("custom URI");
        assert_eq!(document.uri.scheme(), "surreal");
    }
}
