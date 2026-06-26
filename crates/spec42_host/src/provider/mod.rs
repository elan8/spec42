//! Document provider adapters for host embedding.

pub mod changeset;
pub mod filesystem;

pub use changeset::ChangesetDocumentProvider;
pub use filesystem::HostFilesystemProvider;

pub use sysml_model::{
    FileSystemDocumentProvider, InMemoryDocumentProvider, SysmlDocument, SysmlDocumentProvider,
    SysmlDocumentSourceKind,
};
