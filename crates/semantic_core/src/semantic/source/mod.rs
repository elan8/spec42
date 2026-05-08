pub mod providers;

use url::Url;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SysmlDocumentSourceKind {
    Workspace,
    Library,
    External,
}

#[derive(Debug, Clone)]
pub struct SysmlDocument {
    pub uri: Url,
    pub content: String,
    pub path_hint: Option<String>,
    pub source_kind: SysmlDocumentSourceKind,
    pub sha256: Option<String>,
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
        sha256: Option<String>,
        byte_size: Option<i64>,
    ) -> Result<Self, String> {
        let normalized_path = path.trim_start_matches('/').replace('\\', "/");
        let uri = Url::parse(&format!("memory://{scope}/{normalized_path}"))
            .map_err(|err| format!("failed to build source URI for {path}: {err}"))?;
        Ok(Self {
            uri,
            content,
            path_hint: Some(path.to_string()),
            source_kind,
            sha256,
            byte_size,
        })
    }
}
