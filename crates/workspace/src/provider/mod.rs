//! Document provider adapters for host embedding.

pub mod changeset;
pub mod filesystem;

pub use changeset::ChangesetDocumentProvider;
pub use filesystem::{FileSystemDocumentProvider, HostFilesystemProvider};
pub use sysml_source::{
    InMemoryDocumentProvider, SysmlDocument, SysmlDocumentProvider, SysmlDocumentSourceKind,
};
