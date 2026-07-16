use language_service::search_workspace_symbols;

use super::support::multi_doc;

#[test]
fn search_empty_query_returns_all_symbols() {
    let workspace = multi_doc(&[
        ("a.sysml", "package A { part def Alpha; }"),
        ("b.sysml", "package B { part def Beta; }"),
    ]);
    let matches = search_workspace_symbols(&workspace, "");
    let names: Vec<_> = matches.iter().map(|m| m.name.as_str()).collect();
    assert!(names.contains(&"Alpha"));
    assert!(names.contains(&"Beta"));
}

#[test]
fn search_filters_by_substring() {
    let workspace = multi_doc(&[
        ("a.sysml", "package A { part def Alpha; part def Alpaca; }"),
        ("b.sysml", "package B { part def Beta; }"),
    ]);
    let matches = search_workspace_symbols(&workspace, "alp");
    let names: Vec<_> = matches.iter().map(|m| m.name.as_str()).collect();
    assert!(names.contains(&"Alpha"));
    assert!(names.contains(&"Alpaca"));
    assert!(!names.contains(&"Beta"));
}

#[test]
fn search_includes_path_and_container_metadata() {
    let workspace = multi_doc(&[("pkg/vehicle.sysml", "package VehiclePkg { part def Car; }")]);
    let matches = search_workspace_symbols(&workspace, "Car");
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].name, "Car");
    assert!(
        matches[0].path.contains("vehicle.sysml"),
        "path should identify source file: {}",
        matches[0].path
    );
    assert!(
        matches[0].container.is_some(),
        "expected container name for nested symbol"
    );
}
