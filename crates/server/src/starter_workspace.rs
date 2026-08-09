//! Embedded, validated starter workspace for `spec42 init`.
//!
//! The template is a small, tool-neutral multi-file model: a root system definition,
//! baseline configuration, requirements, domain types, and a README with the matching
//! Spec42 validation command. It intentionally contains no project manifest so the
//! workspace can be used with ordinary files and folders.

use std::fs;
use std::path::{Path, PathBuf};

struct TemplateFile {
    relative_path: &'static str,
    contents: &'static str,
}

const FILES: &[TemplateFile] = &[
    TemplateFile {
        relative_path: "README.md",
        contents: include_str!("../templates/init/general/README.md"),
    },
    TemplateFile {
        relative_path: "model/definitions/system.sysml",
        contents: include_str!("../templates/init/general/model/definitions/system.sysml"),
    },
    TemplateFile {
        relative_path: "model/configurations/baseline.sysml",
        contents: include_str!("../templates/init/general/model/configurations/baseline.sysml"),
    },
    TemplateFile {
        relative_path: "model/library/domain_types.sysml",
        contents: include_str!("../templates/init/general/model/library/domain_types.sysml"),
    },
    TemplateFile {
        relative_path: "model/requirements/system_requirements.sysml",
        contents: include_str!(
            "../templates/init/general/model/requirements/system_requirements.sysml"
        ),
    },
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScaffoldResult {
    pub root: PathBuf,
    pub files_written: usize,
}

/// Create the starter workspace in a new or empty directory.
///
/// The directory is checked before any file is written, so this operation never
/// overwrites an existing workspace file. A non-empty directory is rejected to
/// prevent a partial scaffold from being mixed into an unrelated project.
pub fn scaffold(root: &Path) -> Result<ScaffoldResult, String> {
    match fs::symlink_metadata(root) {
        Ok(metadata) if !metadata.file_type().is_dir() => {
            return Err(format!(
                "cannot initialize {}: target exists and is not a directory",
                root.display()
            ));
        }
        Ok(_) => {
            let mut entries = fs::read_dir(root)
                .map_err(|error| format!("cannot inspect {}: {error}", root.display()))?;
            if entries.next().is_some() {
                return Err(format!(
                    "cannot initialize {}: target directory is not empty; existing files are never overwritten",
                    root.display()
                ));
            }
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            fs::create_dir_all(root)
                .map_err(|error| format!("cannot create {}: {error}", root.display()))?;
        }
        Err(error) => return Err(format!("cannot inspect {}: {error}", root.display())),
    }

    for template in FILES {
        let destination = root.join(template.relative_path);
        if destination.exists() {
            return Err(format!(
                "cannot initialize {}: {} already exists; existing files are never overwritten",
                root.display(),
                destination.display()
            ));
        }
    }

    for template in FILES {
        let destination = root.join(template.relative_path);
        let parent = destination
            .parent()
            .expect("template paths always have a parent directory");
        fs::create_dir_all(parent)
            .map_err(|error| format!("cannot create {}: {error}", parent.display()))?;
        fs::write(&destination, template.contents)
            .map_err(|error| format!("cannot write {}: {error}", destination.display()))?;
    }

    Ok(ScaffoldResult {
        root: root.to_path_buf(),
        files_written: FILES.len(),
    })
}
