//! KerML Project Archive (KPAR) read, pack, validate, and legacy zip helpers.

pub mod error;
pub mod legacy;
pub mod pack;
pub mod read;
pub mod schema;

pub use error::{KparError, Result};
pub use legacy::extract_archive_subset;
pub use pack::{build_kpar, default_domain_excludes, PackOptions};
pub use read::{
    is_kpar_archive, materialize, materialize_kpar_directory, open_kpar_bytes, open_kpar_path,
    verify_checksums, KparArchive, MaterializedProject,
};
pub use schema::{Meta, Project, ProjectUsage, META_FILE, PROJECT_FILE};
