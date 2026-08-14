//! Parser-authored membership visibility remains a typed graph fact across representative
//! builder surfaces; presentation attributes are not a semantic authority.

use sysml_model::build_graph_from_doc;
use url::Url;

fn graph_for(input: &str) -> sysml_model::SemanticGraph {
    let parsed = sysml_v2_parser::parse(input).expect("parse");
    let uri = Url::parse("file:///membership_visibility.sysml").expect("uri");
    build_graph_from_doc(&parsed, &uri)
}

fn visibility_of(
    g: &sysml_model::SemanticGraph,
    name: &str,
) -> Option<sysml_model::VisibilityKind> {
    g.nodes_named(name)
        .first()
        .and_then(|node| node.declared_facts.membership.as_ref())
        .and_then(|membership| membership.visibility)
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
        Some(sysml_model::VisibilityKind::Private)
    );
    assert_eq!(
        visibility_of(&g, "ProtectedPartDef"),
        Some(sysml_model::VisibilityKind::Protected)
    );
    assert_eq!(visibility_of(&g, "PublicPartDef"), None);
    assert_eq!(
        visibility_of(&g, "privatePart"),
        Some(sysml_model::VisibilityKind::Private)
    );
    assert_eq!(
        visibility_of(&g, "protectedPart"),
        Some(sysml_model::VisibilityKind::Protected)
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
        Some(sysml_model::VisibilityKind::Private)
    );
    assert_eq!(
        visibility_of(&g, "privateAttr"),
        Some(sysml_model::VisibilityKind::Private)
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
        Some(sysml_model::VisibilityKind::Private)
    );
    assert_eq!(
        visibility_of(&g, "ProtectedStateDef"),
        Some(sysml_model::VisibilityKind::Protected)
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
        Some(sysml_model::VisibilityKind::Private)
    );
    assert_eq!(
        visibility_of(&g, "privateView"),
        Some(sysml_model::VisibilityKind::Private)
    );
}
