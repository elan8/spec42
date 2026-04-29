use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

fn collect_sysml_files(root: &Path, out: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_sysml_files(&path, out);
            } else if path
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.eq_ignore_ascii_case("sysml"))
                .unwrap_or(false)
            {
                out.push(path);
            }
        }
    }
}

fn parse_package_name(content: &str) -> Option<String> {
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("package ") {
            let name = rest
                .split_whitespace()
                .next()
                .unwrap_or("")
                .trim_end_matches('{')
                .trim();
            if !name.is_empty() {
                return Some(name.to_string());
            }
        }
    }
    None
}

fn parse_import_targets(content: &str) -> Vec<String> {
    let mut imports = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("import ") {
            let target = rest
                .trim_end_matches(';')
                .trim_end_matches("::*")
                .trim()
                .to_string();
            if !target.is_empty() {
                imports.push(target);
            }
        }
    }
    imports
}

fn package_set_for(root: &Path) -> BTreeSet<String> {
    let mut files = Vec::new();
    collect_sysml_files(root, &mut files);
    let mut packages = BTreeSet::new();
    for file in files {
        if let Ok(content) = fs::read_to_string(&file) {
            if let Some(pkg) = parse_package_name(&content) {
                packages.insert(pkg);
            }
        }
    }
    packages
}

fn collect_import_violations(root: &Path, forbidden_packages: &BTreeSet<String>) -> Vec<String> {
    let mut files = Vec::new();
    collect_sysml_files(root, &mut files);
    let mut violations = Vec::new();
    for file in files {
        let Ok(content) = fs::read_to_string(&file) else {
            continue;
        };
        for import_target in parse_import_targets(&content) {
            if forbidden_packages.contains(&import_target) {
                violations.push(format!(
                    "{} imports forbidden package {}",
                    file.display(),
                    import_target
                ));
            }
        }
    }
    violations
}

#[test]
fn technical_and_cross_cutting_do_not_import_business_packages() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let domain_root = repo_root.join("domain-libraries");
    let technical_root = domain_root.join("technical");
    let cross_cutting_root = domain_root.join("cross-cutting");
    let business_root = domain_root.join("business");

    let business_packages = package_set_for(&business_root);
    assert!(
        !business_packages.is_empty(),
        "expected at least one business package under {}",
        business_root.display()
    );

    let mut violating_imports = Vec::new();
    for layer_root in [&technical_root, &cross_cutting_root] {
        violating_imports.extend(collect_import_violations(layer_root, &business_packages));
    }

    assert!(
        violating_imports.is_empty(),
        "Layering violations found:\n{}",
        violating_imports.join("\n")
    );
}

#[test]
fn technical_electronics_does_not_import_business_packages() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let domain_root = repo_root.join("domain-libraries");
    let electronics_root = domain_root.join("technical").join("electronics");
    let business_root = domain_root.join("business");

    let business_packages = package_set_for(&business_root);
    assert!(
        !business_packages.is_empty(),
        "expected at least one business package under {}",
        business_root.display()
    );
    assert!(
        electronics_root.is_dir(),
        "expected electronics domain directory at {}",
        electronics_root.display()
    );

    let violations = collect_import_violations(&electronics_root, &business_packages);
    assert!(
        violations.is_empty(),
        "Electronics layering violations found:\n{}",
        violations.join("\n")
    );
}
