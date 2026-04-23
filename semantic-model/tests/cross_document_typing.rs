//! Integration: merged workspace graph resolves typing across documents.

use semantic_model::{
    add_cross_document_edges_for_uri, build_graph_from_doc, RelationshipKind, SemanticGraph,
};
use sysml_v2_parser::parse;
use tower_lsp::lsp_types::Url;

#[test]
fn cross_document_requirement_usage_typing_after_merge() {
    let defs = r#"
        package R {
            requirement def EnduranceReq;
        }
    "#;
    let usage = r#"
        package R {
            requirement enduranceCheck : EnduranceReq;
        }
    "#;
    let root_defs = parse(defs).expect("parse defs");
    let root_usage = parse(usage).expect("parse usage");
    let uri_defs = Url::parse("file:///requirements_defs.sysml").expect("uri defs");
    let uri_usage = Url::parse("file:///requirements_usage.sysml").expect("uri usage");

    let mut g = SemanticGraph::new();
    g.merge(build_graph_from_doc(&root_defs, &uri_defs));
    g.merge(build_graph_from_doc(&root_usage, &uri_usage));

    add_cross_document_edges_for_uri(&mut g, &uri_usage);

    let edges = g.edges_for_uri_as_strings(&uri_usage);
    let has_typing = edges.iter().any(|(src, tgt, kind, _)| {
        *kind == RelationshipKind::Typing
            && src.ends_with("enduranceCheck")
            && tgt.ends_with("EnduranceReq")
    });
    assert!(
        has_typing,
        "expected cross-document typing edge enduranceCheck -> EnduranceReq; edges: {:?}",
        edges
    );
}

#[test]
fn cross_document_attribute_typing_resolves_via_package_import() {
    let lib = r#"
        package ScalarValues {
            attribute def Real;
        }
    "#;
    let main = r#"
        package Demo {
            private import ScalarValues::Real;
            part def P {
                attribute x : Real;
            }
        }
    "#;
    let root_lib = parse(lib).expect("parse lib");
    let root_main = parse(main).expect("parse main");
    let uri_lib = Url::parse("file:///scalar_values.sysml").expect("uri lib");
    let uri_main = Url::parse("file:///main.sysml").expect("uri main");

    let mut g = SemanticGraph::new();
    g.merge(build_graph_from_doc(&root_lib, &uri_lib));
    g.merge(build_graph_from_doc(&root_main, &uri_main));

    add_cross_document_edges_for_uri(&mut g, &uri_main);

    let edges = g.edges_for_uri_as_strings(&uri_main);
    let has_typing = edges.iter().any(|(src, tgt, kind, _)| {
        *kind == RelationshipKind::Typing && src.ends_with("x") && tgt.ends_with("Real")
    });
    assert!(
        has_typing,
        "expected typing edge from attribute x to ScalarValues::Real via import; edges: {:?}",
        edges
    );
}

#[test]
fn cross_document_attribute_typing_resolves_via_wildcard_package_import() {
    let lib = r#"
        package ScalarValues {
            attribute def Real;
        }
    "#;
    let main = r#"
        package Demo {
            import ScalarValues::*;
            part def P {
                attribute x : Real;
            }
        }
    "#;
    let root_lib = parse(lib).expect("parse lib");
    let root_main = parse(main).expect("parse main");
    let uri_lib = Url::parse("file:///scalar_values.sysml").expect("uri lib");
    let uri_main = Url::parse("file:///main.sysml").expect("uri main");

    let mut g = SemanticGraph::new();
    g.merge(build_graph_from_doc(&root_lib, &uri_lib));
    g.merge(build_graph_from_doc(&root_main, &uri_main));

    add_cross_document_edges_for_uri(&mut g, &uri_main);

    let edges = g.edges_for_uri_as_strings(&uri_main);
    let has_typing = edges.iter().any(|(src, tgt, kind, _)| {
        *kind == RelationshipKind::Typing && src.ends_with("x") && tgt.ends_with("Real")
    });
    assert!(
        has_typing,
        "expected typing edge from attribute x to ScalarValues::Real via wildcard import; edges: {:?}",
        edges
    );
}

#[test]
fn cross_document_attribute_typing_resolves_via_public_reexport_chain() {
    let lib = r#"
        package Core {
            attribute def Name;
        }
    "#;
    let intermediate = r#"
        package Domain {
            public import Core::*;
        }
    "#;
    let main = r#"
        package Demo {
            import Domain::*;
            part def Consumer {
                attribute groupName : Name;
            }
        }
    "#;
    let root_lib = parse(lib).expect("parse lib");
    let root_intermediate = parse(intermediate).expect("parse intermediate");
    let root_main = parse(main).expect("parse main");
    let uri_lib = Url::parse("file:///core.sysml").expect("uri lib");
    let uri_intermediate = Url::parse("file:///domain.sysml").expect("uri intermediate");
    let uri_main = Url::parse("file:///main.sysml").expect("uri main");

    let mut g = SemanticGraph::new();
    g.merge(build_graph_from_doc(&root_lib, &uri_lib));
    g.merge(build_graph_from_doc(&root_intermediate, &uri_intermediate));
    g.merge(build_graph_from_doc(&root_main, &uri_main));

    add_cross_document_edges_for_uri(&mut g, &uri_main);

    let edges = g.edges_for_uri_as_strings(&uri_main);
    assert!(
        edges.iter().any(|(src, tgt, kind, _)| {
            *kind == RelationshipKind::Typing && src.ends_with("groupName") && tgt.ends_with("Name")
        }),
        "expected typing edge via public import re-export; edges: {:?}",
        edges
    );
}

#[test]
fn cross_document_attribute_typing_does_not_resolve_via_private_reexport_chain() {
    let lib = r#"
        package Core {
            attribute def Name;
        }
    "#;
    let intermediate = r#"
        package Domain {
            private import Core::*;
        }
    "#;
    let main = r#"
        package Demo {
            import Domain::*;
            part def Consumer {
                attribute groupName : Name;
            }
        }
    "#;
    let root_lib = parse(lib).expect("parse lib");
    let root_intermediate = parse(intermediate).expect("parse intermediate");
    let root_main = parse(main).expect("parse main");
    let uri_lib = Url::parse("file:///core.sysml").expect("uri lib");
    let uri_intermediate = Url::parse("file:///domain.sysml").expect("uri intermediate");
    let uri_main = Url::parse("file:///main.sysml").expect("uri main");

    let mut g = SemanticGraph::new();
    g.merge(build_graph_from_doc(&root_lib, &uri_lib));
    g.merge(build_graph_from_doc(&root_intermediate, &uri_intermediate));
    g.merge(build_graph_from_doc(&root_main, &uri_main));

    add_cross_document_edges_for_uri(&mut g, &uri_main);

    let edges = g.edges_for_uri_as_strings(&uri_main);
    assert!(
        !edges.iter().any(|(src, tgt, kind, _)| {
            *kind == RelationshipKind::Typing && src.ends_with("groupName") && tgt.ends_with("Name")
        }),
        "did not expect typing edge through private-only import chain; edges: {:?}",
        edges
    );
}

#[test]
fn cross_document_attribute_typing_resolves_via_recursive_namespace_import() {
    let lib = r#"
        package Core {
            package Nested {
                attribute def Name;
            }
        }
    "#;
    let main = r#"
        package Demo {
            import Core::**;
            part def Consumer {
                attribute groupName : Name;
            }
        }
    "#;
    let root_lib = parse(lib).expect("parse lib");
    let root_main = parse(main).expect("parse main");
    let uri_lib = Url::parse("file:///core.sysml").expect("uri lib");
    let uri_main = Url::parse("file:///main.sysml").expect("uri main");

    let mut g = SemanticGraph::new();
    g.merge(build_graph_from_doc(&root_lib, &uri_lib));
    g.merge(build_graph_from_doc(&root_main, &uri_main));

    add_cross_document_edges_for_uri(&mut g, &uri_main);

    let edges = g.edges_for_uri_as_strings(&uri_main);
    assert!(
        edges.iter().any(|(src, tgt, kind, _)| {
            *kind == RelationshipKind::Typing && src.ends_with("groupName") && tgt.ends_with("Name")
        }),
        "expected typing edge via recursive import; edges: {:?}",
        edges
    );
}

#[test]
fn cross_document_attribute_typing_resolves_via_membership_import_from_public_reexported_library_package(
) {
    let lib_base = r#"
        standard library package ISQBase {
            attribute def DurationValue;
        }
    "#;
    let lib = r#"
        standard library package ISQ {
            public import ISQBase::*;
        }
    "#;
    let main = r#"
        package Demo {
            private import ISQ::DurationValue;
            part def Timer {
                attribute duration : DurationValue;
            }
        }
    "#;
    let root_lib_base = parse(lib_base).expect("parse lib base");
    let root_lib = parse(lib).expect("parse lib");
    let root_main = parse(main).expect("parse main");
    let uri_lib_base = Url::parse("file:///isq_base.sysml").expect("uri lib base");
    let uri_lib = Url::parse("file:///isq.sysml").expect("uri lib");
    let uri_main = Url::parse("file:///main.sysml").expect("uri main");

    let mut g = SemanticGraph::new();
    g.merge(build_graph_from_doc(&root_lib_base, &uri_lib_base));
    g.merge(build_graph_from_doc(&root_lib, &uri_lib));
    g.merge(build_graph_from_doc(&root_main, &uri_main));

    add_cross_document_edges_for_uri(&mut g, &uri_main);

    let edges = g.edges_for_uri_as_strings(&uri_main);
    let has_typing = edges.iter().any(|(src, tgt, kind, _)| {
        *kind == RelationshipKind::Typing
            && src.ends_with("duration")
            && tgt.ends_with("DurationValue")
    });
    assert!(
        has_typing,
        "expected typing edge from attribute duration to ISQBase::DurationValue via ISQ::DurationValue import; edges: {:?}",
        edges
    );
}

#[test]
fn cross_document_attribute_typing_resolves_with_multiple_wildcard_import_siblings() {
    let lib_base = r#"
        standard library package ISQBase {
            attribute def DurationValue;
        }
    "#;
    let lib_other = r#"
        standard library package ISQInformation {
            attribute def BitRateValue;
        }
    "#;
    let lib = r#"
        standard library package ISQ {
            public import ISQBase::*;
            public import ISQInformation::*;
        }
    "#;
    let main = r#"
        package Demo {
            private import ISQ::DurationValue;
            part def Timer {
                attribute duration : DurationValue;
            }
        }
    "#;
    let root_lib_base = parse(lib_base).expect("parse lib base");
    let root_lib_other = parse(lib_other).expect("parse other lib");
    let root_lib = parse(lib).expect("parse lib");
    let root_main = parse(main).expect("parse main");
    let uri_lib_base = Url::parse("file:///isq_base.sysml").expect("uri lib base");
    let uri_lib_other = Url::parse("file:///isq_information.sysml").expect("uri other lib");
    let uri_lib = Url::parse("file:///isq.sysml").expect("uri lib");
    let uri_main = Url::parse("file:///main.sysml").expect("uri main");

    let mut g = SemanticGraph::new();
    g.merge(build_graph_from_doc(&root_lib_base, &uri_lib_base));
    g.merge(build_graph_from_doc(&root_lib_other, &uri_lib_other));
    g.merge(build_graph_from_doc(&root_lib, &uri_lib));
    g.merge(build_graph_from_doc(&root_main, &uri_main));

    add_cross_document_edges_for_uri(&mut g, &uri_main);

    let edges = g.edges_for_uri_as_strings(&uri_main);
    let has_typing = edges.iter().any(|(src, tgt, kind, _)| {
        *kind == RelationshipKind::Typing
            && src.ends_with("duration")
            && tgt.ends_with("DurationValue")
    });
    assert!(
        has_typing,
        "expected typing edge from attribute duration to ISQBase::DurationValue even with multiple wildcard imports; edges: {:?}",
        edges
    );
}
