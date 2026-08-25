//! Embedded, validated starter workspace for `spec42 init`.
//!
//! The template is a small, tool-neutral multi-file model: a root system definition,
//! baseline configuration, requirements, domain types, and a README with the matching
//! Spec42 validation command. A project manifest is generated from the target directory name.

use std::fs;
use std::path::{Path, PathBuf};

use kpar::Project;

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

/// Initialize a project in a new, empty, or existing model directory.
///
/// New and empty directories receive the starter workspace. An existing non-empty directory is
/// promoted by adding only `.project.json`. Existing files, including an existing manifest, are
/// never overwritten.
pub fn scaffold(root: &Path, usage: Vec<kpar::ProjectUsage>) -> Result<ScaffoldResult, String> {
    let populate_starter = match fs::symlink_metadata(root) {
        Ok(metadata) if !metadata.file_type().is_dir() => {
            return Err(format!(
                "cannot initialize {}: target exists and is not a directory",
                root.display()
            ));
        }
        Ok(_) => {
            let mut entries = fs::read_dir(root)
                .map_err(|error| format!("cannot inspect {}: {error}", root.display()))?;
            entries.next().is_none()
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            fs::create_dir_all(root)
                .map_err(|error| format!("cannot create {}: {error}", root.display()))?;
            true
        }
        Err(error) => return Err(format!("cannot inspect {}: {error}", root.display())),
    };

    let project_path = root.join(kpar::PROJECT_FILE);
    if project_path.exists() {
        return Ok(ScaffoldResult {
            root: root.to_path_buf(),
            files_written: 0,
        });
    }

    for template in FILES.iter().filter(|_| populate_starter) {
        let destination = root.join(template.relative_path);
        if destination.exists() {
            return Err(format!(
                "cannot initialize {}: {} already exists; existing files are never overwritten",
                root.display(),
                destination.display()
            ));
        }
    }

    for template in FILES.iter().filter(|_| populate_starter) {
        let destination = root.join(template.relative_path);
        let parent = destination
            .parent()
            .expect("template paths always have a parent directory");
        fs::create_dir_all(parent)
            .map_err(|error| format!("cannot create {}: {error}", parent.display()))?;
        fs::write(&destination, template.contents)
            .map_err(|error| format!("cannot write {}: {error}", destination.display()))?;
    }

    let mut project = Project {
        name: project_name(root),
        version: "0.1.0".into(),
        description: None,
        license: None,
        publisher: None,
        maintainer: Vec::new(),
        website: None,
        topic: Vec::new(),
        usage,
    };
    if project.validate_identity().is_err() {
        project.name = "model".into();
    }
    let mut project_json = serde_json::to_string_pretty(&project)
        .map_err(|error| format!("cannot serialize {}: {error}", project_path.display()))?;
    project_json.push('\n');
    fs::write(&project_path, project_json)
        .map_err(|error| format!("cannot write {}: {error}", project_path.display()))?;

    Ok(ScaffoldResult {
        root: root.to_path_buf(),
        files_written: 1 + if populate_starter { FILES.len() } else { 0 },
    })
}

fn project_name(root: &Path) -> String {
    let candidate = root
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("model");
    let normalized = candidate
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.') {
                character
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches(['.', '-', ' '])
        .to_string();
    if normalized.is_empty() {
        "model".into()
    } else {
        normalized
    }
}
