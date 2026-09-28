//! Requirement derivation (the `RequirementDerivation` domain library) against the real standard
//! library: `#derivation`, `#original` and `#derive` are prefix metadata whose `SemanticMetadata`
//! `baseType` specializes the connection and its ends, and `derivation_relationships` classifies
//! the ends by what they specialize.

use spec42::cli::Cli;
use spec42::host_snapshot::load_snapshot_for_paths;
use sysml_query::resolved_slice::{DerivationEndpoint, ElementKind, QueryAnswer};

use crate::common::with_isolated_data_dir;

fn stdlib_cli() -> Cli {
    Cli {
        config_path: None,
        library_paths: vec![],
        stdlib_path: None,
        kpar_library_paths: Vec::new(),
        project_libraries: Vec::new(),
        disabled_kpar_libraries: Vec::new(),
        no_stdlib: false,
        stdio: false,
        command: None,
    }
}

/// The OMG `RequirementDerivationExample` (SysML v2 release), plus a package-level def-less
/// `#derivation connection` with anonymous `#original`/`#derive` ends and a feature-chain end.
const MODEL: &str = r#"package Derivations {
    private import RequirementDerivation::*;

    requirement def Req1;
    requirement def Req1_1;
    requirement def Req1_2;

    #derivation connection def Req1_Derivation {
        end #original r1 : Req1;
        end #derive r1_1 : Req1_1;
        end #derive r1_2 : Req1_2;
    }

    part def System;
    part def Subsystem1;
    part def Subsystem2;
    part system : System {
        part sub1 : Subsystem1;
        part sub2 : Subsystem2;
    }

    part satisfactionContext {
        ref :>> system;
        satisfy requirement req1 : Req1 by system;
        satisfy requirement req1_1 : Req1_1 by system.sub1;
        satisfy requirement req1_2 : Req1_2 by system.sub2;
        #derivation connection : Req1_Derivation {
            end r1 ::> req1;
            end r1_1 ::> req1_1;
            end r1_2 ::> req1_2;
        }
    }

    part def Spec { requirement child : Req1_1; }
    part spec : Spec;
    requirement parent : Req1;
    #derivation connection standalone {
        end #original ::> parent;
        end #derive ::> spec.child;
    }
}
"#;

#[test]
fn derivation_connections_classify_their_original_and_derived_ends() {
    with_isolated_data_dir(|| {
        let workspace = tempfile::tempdir().expect("workspace");
        let file = workspace.path().join("model.sysml");
        std::fs::write(&file, MODEL).expect("model");
        let snapshot = load_snapshot_for_paths(&stdlib_cli(), &file, Some(workspace.path()), false)
            .expect("the model publishes");
        let model = snapshot.published_model_arc();

        // #221/#222: the connection inside the part body and the package-level one are both
        // modelled usages -- nothing is dropped as unsupported, nothing is a definition.
        let uri = url::Url::from_file_path(&file).expect("uri").to_string();
        let published = model.diagnostics().for_document(&uri);
        let diagnostics: Vec<String> = (0..published.len())
            .filter_map(|index| published.get(index))
            .map(|diagnostic| format!("{diagnostic:?}"))
            .collect();
        assert!(
            !diagnostics
                .iter()
                .any(|diagnostic| diagnostic.contains("unsupported")
                    || diagnostic.contains("unresolved")),
            "{diagnostics:#?}"
        );
        let QueryAnswer::Resolved(all) = model.inspection().all_elements().answer else {
            panic!("elements");
        };
        let standalone = all
            .iter()
            .find(|element| {
                model.qualified_name(element.entry.identity) == Some("Derivations::standalone")
            })
            .expect("standalone connection");
        assert_eq!(standalone.entry.kind, ElementKind::ConnectionUsage);

        let name = |endpoint: &DerivationEndpoint| match endpoint {
            DerivationEndpoint::Resolved(symbol) => {
                model.qualified_name(*symbol).unwrap_or("?").to_owned()
            }
            DerivationEndpoint::FeatureChain { path, .. } => path
                .iter()
                .map(|hop| model.qualified_name(*hop).unwrap_or("?").to_owned())
                .collect::<Vec<_>>()
                .join(" / "),
            other => format!("{other:?}"),
        };
        let QueryAnswer::Resolved(derivations) =
            model.inspection().derivation_relationships().answer
        else {
            panic!("derivation relationships");
        };
        let summary: Vec<(Vec<String>, Vec<String>, usize)> = derivations
            .iter()
            .map(|derivation| {
                (
                    derivation.original.iter().map(name).collect(),
                    derivation.derived.iter().map(name).collect(),
                    derivation.unclassified.len(),
                )
            })
            .collect();
        assert_eq!(
            summary,
            vec![
                (
                    vec!["Derivations::satisfactionContext::req1".to_owned()],
                    vec![
                        "Derivations::satisfactionContext::req1_1".to_owned(),
                        "Derivations::satisfactionContext::req1_2".to_owned(),
                    ],
                    0,
                ),
                (
                    vec!["Derivations::parent".to_owned()],
                    vec!["Derivations::spec / Derivations::Spec::child".to_owned()],
                    0,
                ),
            ]
        );

        // The same facts through the generator API a plugin reads.
        let generator = generator_api::GeneratorModelView::new(
            std::sync::Arc::clone(&model),
            model.publication().model_digest().to_string(),
            env!("CARGO_PKG_VERSION"),
            generator_api::QueryLimits::default(),
        )
        .expect("generator model");
        let summaries = generator.derivation_relationships().expect("derivations");
        assert_eq!(summaries.len(), 2);
        assert!(matches!(
            &summaries[1].derived[..],
            [generator_api::DerivationEndpointSummary::FeatureChain { path, .. }] if path.len() == 2
        ));
        assert!(summaries.iter().all(|summary| summary.original.len() == 1));
    });
}
