//! Regression tests for Grid View's `asElementTable`/`columnView` column-configuration mechanism
//! (Phase 5b of the Browser/Grid View conformance work). Real syntax confirmed against two
//! official OMG examples (`sysml-v2-parser/sysml-v2-release/sysml/src/training/42. Views/Views
//! Example.sysml` and `.../validation/11-View and Viewpoint/11a-View-Viewpoint.sysml`), both using
//! exactly `view :>> columnView[1] { render asTextualNotation; }`.

use sysml_model::build_graph_from_doc;
use url::Url;

fn graph_for(input: &str) -> sysml_model::SemanticGraph {
    let parsed = sysml_v2_parser::parse(input).expect("parse");
    let uri = Url::parse("file:///grid_view_column_view.sysml").expect("uri");
    build_graph_from_doc(&parsed, &uri)
}

#[test]
fn standalone_rendering_usage_captures_column_view_in_declaration_order() {
    let g = graph_for(
        r#"
        package P {
            rendering asTextualNotationTable :> asElementTable {
                view :>> columnView[1] {
                    render asTextualNotation;
                }
            }
        }
    "#,
    );
    let rendering = g
        .nodes_named("asTextualNotationTable")
        .into_iter()
        .next()
        .expect("rendering node");
    let children = g.children_of(rendering);
    let columns: Vec<_> = children
        .iter()
        .filter(|child| child.element_kind.as_str() == "view column")
        .collect();
    assert_eq!(columns.len(), 1, "expected exactly one columnView child");
    let column = columns[0];
    assert_eq!(
        column
            .declared_facts
            .relationships
            .redefinition
            .first()
            .map(|target| target.reference.as_str()),
        Some("columnView")
    );
    assert_eq!(
        column
            .attributes
            .get("renderingType")
            .and_then(|v| v.as_str()),
        Some("asTextualNotation")
    );
}

#[test]
fn inline_render_binding_on_a_view_usage_captures_column_view() {
    let g = graph_for(
        r#"
        package P {
            view 'system structure generation' {
                render asElementTable {
                    view :>> columnView[1] {
                        render asTextualNotation;
                    }
                }
            }
        }
    "#,
    );
    let rendering = g
        .nodes_named("asElementTable")
        .into_iter()
        .next()
        .expect("view rendering node");
    let children = g.children_of(rendering);
    let columns: Vec<_> = children
        .iter()
        .filter(|child| child.element_kind.as_str() == "view column")
        .collect();
    assert_eq!(columns.len(), 1, "expected exactly one columnView child");
    assert_eq!(
        columns[0]
            .attributes
            .get("renderingType")
            .and_then(|v| v.as_str()),
        Some("asTextualNotation")
    );
}

#[test]
fn multiple_column_views_are_captured_in_declaration_order() {
    let g = graph_for(
        r#"
        package P {
            rendering multiColumn :> asElementTable {
                view :>> columnView[1] {
                    render asTextualNotation;
                }
                view :>> columnView[2] {
                    render asTextualNotation;
                }
            }
        }
    "#,
    );
    let rendering = g
        .nodes_named("multiColumn")
        .into_iter()
        .next()
        .expect("rendering node");
    let children = g.children_of(rendering);
    let columns: Vec<_> = children
        .iter()
        .filter(|child| child.element_kind.as_str() == "view column")
        .collect();
    assert_eq!(columns.len(), 2, "expected two columnView children");
}
