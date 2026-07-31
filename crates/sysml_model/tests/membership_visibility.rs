//! Regression tests for `attach_membership_visibility` (Phase 4a of the Browser/Grid View
//! conformance work): the `visibility` node attribute must be populated from an explicit
//! `private`/`protected`/`public` member prefix across representative element kinds, not just
//! the handful of builder call sites that had it before this pass (imports, ref decls,
//! requirement-body members).

use sysml_model::build_graph_from_doc;
use url::Url;

fn graph_for(input: &str) -> sysml_model::SemanticGraph {
    let parsed = sysml_v2_parser::parse(input).expect("parse");
    let uri = Url::parse("file:///membership_visibility.sysml").expect("uri");
    build_graph_from_doc(&parsed, &uri)
}

fn visibility_of(g: &sysml_model::SemanticGraph, name: &str) -> Option<String> {
    g.nodes_named(name)
        .first()
        .and_then(|node| node.attributes.get("visibility"))
        .and_then(|value| value.as_str())
        .map(str::to_string)
}

#[test]
fn part_def_and_usage_carry_explicit_visibility() {
    let g = graph_for(
        r#"
        package P {
            private part def PrivatePartDef;
            protected part def ProtectedPartDef;
            part def PublicPartDef;
            part owner {
                private part privatePart : PrivatePartDef;
                protected part protectedPart : ProtectedPartDef;
                part publicPart : PublicPartDef;
            }
        }
    "#,
    );
    assert_eq!(
        visibility_of(&g, "PrivatePartDef"),
        Some("Private".to_string())
    );
    assert_eq!(
        visibility_of(&g, "ProtectedPartDef"),
        Some("Protected".to_string())
    );
    assert_eq!(visibility_of(&g, "PublicPartDef"), None);
    assert_eq!(
        visibility_of(&g, "privatePart"),
        Some("Private".to_string())
    );
    assert_eq!(
        visibility_of(&g, "protectedPart"),
        Some("Protected".to_string())
    );
    assert_eq!(visibility_of(&g, "publicPart"), None);
}

#[test]
fn attribute_def_and_usage_carry_explicit_visibility() {
    let g = graph_for(
        r#"
        package P {
            private attribute def PrivateAttrDef;
            part owner {
                private attribute privateAttr : PrivateAttrDef;
                attribute publicAttr : PrivateAttrDef;
            }
        }
    "#,
    );
    assert_eq!(
        visibility_of(&g, "PrivateAttrDef"),
        Some("Private".to_string())
    );
    assert_eq!(
        visibility_of(&g, "privateAttr"),
        Some("Private".to_string())
    );
    assert_eq!(visibility_of(&g, "publicAttr"), None);
}

#[test]
fn action_def_and_state_def_carry_explicit_visibility() {
    let g = graph_for(
        r#"
        package P {
            private action def PrivateActionDef;
            protected state def ProtectedStateDef;
        }
    "#,
    );
    assert_eq!(
        visibility_of(&g, "PrivateActionDef"),
        Some("Private".to_string())
    );
    assert_eq!(
        visibility_of(&g, "ProtectedStateDef"),
        Some("Protected".to_string())
    );
}

#[test]
fn requirement_and_view_usages_carry_explicit_visibility() {
    let g = graph_for(
        r#"
        package P {
            private requirement def PrivateRequirementDef;
            private view privateView {
                render asTreeDiagram;
            }
        }
    "#,
    );
    assert_eq!(
        visibility_of(&g, "PrivateRequirementDef"),
        Some("Private".to_string())
    );
    assert_eq!(
        visibility_of(&g, "privateView"),
        Some("Private".to_string())
    );
}
