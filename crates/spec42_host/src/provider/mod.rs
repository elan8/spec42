//! Document provider adapters for host embedding.

pub mod changeset;
pub mod filesystem;

pub use changeset::ChangesetDocumentProvider;
pub use filesystem::HostFilesystemProvider;

pub use semantic_core::{
    FileSystemDocumentProvider, InMemoryDocumentProvider, SysmlDocument, SysmlDocumentProvider,
    SysmlDocumentSourceKind,
};
