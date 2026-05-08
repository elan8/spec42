//! Tests for the graph-driven sequence diagram extractor.
//!
//! These tests exercise the pipeline end-to-end through the real
//! `SemanticGraph` so cross-file specialization, custom lifeline/message
//! subtypes, and reference resolution all participate.

use std::collections::HashSet;
use std::env;

use url::Url;

use crate::semantic::extracted_model::SequenceDiagramDto;
use crate::workspace::{rebuild_all_document_links, store_document_text, ServerState};

use super::build_workspace_sequence_diagrams;

fn fake_uri(name: &str) -> Url {
    let path = env::temp_dir().join("spec42-tests").join(name);
    Url::from_file_path(&path).expect("file uri")
}

fn build_state(files: &[(&str, &str)]) -> (ServerState, Vec<Url>) {
    let mut state = ServerState::default();
    let mut uris = Vec::new();
    for (name, content) in files {
        let uri = fake_uri(name);
        store_document_text(&mut state, &uri, (*content).to_string());
        uris.push(uri);
    }
    rebuild_all_document_links(&mut state);
    (state, uris)
}

fn extract(files: &[(&str, &str)]) -> Vec<SequenceDiagramDto> {
    let (state, uris) = build_state(files);
    build_workspace_sequence_diagrams(&state.semantic_graph, &uris)
}

fn find_diagram<'a>(
    diagrams: &'a [SequenceDiagramDto],
    name: &str,
) -> &'a SequenceDiagramDto {
    diagrams
        .iter()
        .find(|d| d.name == name)
        .unwrap_or_else(|| panic!("expected diagram named {name}, got: {:#?}", diagrams))
}

const STDLIB: &str = r#"
package SoftwareInteractions {
    part def InteractionScenario;
    part def Lifeline;
    part def Activation;
    part def InteractionOperand;
    part def SynchronousCall;
    part def AsynchronousMessage;
    part def ReturnMessage;
    part def CreationMessage;
    part def AltFragment;
    part def OptFragment;
    part def LoopFragment;
    part def InteractionRef;
    part def CombinedFragment;
}
"#;

#[test]
fn graph_extractor_emits_basic_messages_lifelines_activations_and_fragments() {
    let demo = r#"
        package Demo {
            import SoftwareInteractions::*;

            part def CheckoutFlow :> InteractionScenario {
                part client : Lifeline;
                part api : Lifeline;

                part createOrder : SynchronousCall {
                    ref from : Lifeline = client;
                    ref to : Lifeline = api;
                    attribute label = "POST /orders";
                }

                part processing : Activation {
                    ref on : Lifeline = api;
                    ref startMessage : Message = createOrder;
                    ref finishMessage : Message = orderAccepted;
                }

                part validation : OptFragment {
                    part happyPath : InteractionOperand {
                        attribute guard = "valid order";
                        part orderAccepted : ReturnMessage {
                            ref from : Lifeline = api;
                            ref to : Lifeline = client;
                            attribute label = "202 Accepted";
                        }
                    }
                }
            }
        }
    "#;

    let diagrams = extract(&[("Stdlib.sysml", STDLIB), ("Demo.sysml", demo)]);
    let diagram = find_diagram(&diagrams, "CheckoutFlow");

    assert_eq!(diagram.package_path, "Demo");
    assert_eq!(diagram.lifelines.len(), 2);
    assert_eq!(diagram.messages.len(), 2);
    assert_eq!(diagram.messages[0].kind, "sync");
    assert_eq!(diagram.messages[1].kind, "return");
    assert_eq!(diagram.messages[0].order, 1);
    assert_eq!(diagram.messages[1].order, 2);
    assert_eq!(diagram.activations.len(), 1);
    assert_eq!(
        diagram.activations[0].on_lifeline,
        "Demo::CheckoutFlow::api"
    );
    assert_eq!(
        diagram.activations[0].start_message.as_deref(),
        Some("Demo::CheckoutFlow::createOrder")
    );
    assert_eq!(
        diagram.activations[0].finish_message.as_deref(),
        Some("Demo::CheckoutFlow::validation::happyPath::orderAccepted"),
        "expected forward reference into a fragment to resolve"
    );
    assert_eq!(diagram.fragments.len(), 1);
    assert_eq!(diagram.fragments[0].kind, "opt");
    assert_eq!(
        diagram.fragments[0].operands[0].guard.as_deref(),
        Some("valid order"),
    );
    assert_eq!(
        diagram.fragments[0].operands[0].message_ids,
        vec!["Demo::CheckoutFlow::validation::happyPath::orderAccepted".to_string()]
    );
    assert_eq!(diagram.messages[0].label.as_deref(), Some("POST /orders"));
}

#[test]
fn graph_extractor_handles_indirect_specialization_across_files() {
    let semantics = r#"
        package WebShopSemantics {
            import SoftwareInteractions::*;
            part def CommerceInteractionScenario :> InteractionScenario;
        }
    "#;
    let architecture = r#"
        package WebShopArchitecture {
            import SoftwareInteractions::*;
            import WebShopSemantics::*;

            part def CheckoutFlow :> CommerceInteractionScenario {
                part storefront : Lifeline;
                part apiGateway : Lifeline;
                part submitCheckout : SynchronousCall {
                    ref from : Lifeline = storefront;
                    ref to : Lifeline = apiGateway;
                }
            }
        }
    "#;

    let diagrams = extract(&[
        ("Stdlib.sysml", STDLIB),
        ("WebShopSemantics.sysml", semantics),
        ("WebShopArchitecture.sysml", architecture),
    ]);
    let diagram = find_diagram(&diagrams, "CheckoutFlow");

    assert_eq!(diagram.package_path, "WebShopArchitecture");
    assert_eq!(diagram.lifelines.len(), 2);
    assert_eq!(diagram.messages.len(), 1);
    assert_eq!(
        diagram.messages[0].from,
        "WebShopArchitecture::CheckoutFlow::storefront"
    );
    assert_eq!(
        diagram.messages[0].to,
        "WebShopArchitecture::CheckoutFlow::apiGateway"
    );
}

#[test]
fn graph_extractor_recognizes_custom_lifeline_subtype() {
    let semantics = r#"
        package WebShopSemantics {
            import SoftwareInteractions::*;
            part def ServiceLifeline :> Lifeline;
        }
    "#;
    let diagram_src = r#"
        package Flows {
            import SoftwareInteractions::*;
            import WebShopSemantics::*;

            part def OrderFlow :> InteractionScenario {
                part storefront : ServiceLifeline;
                part orders : ServiceLifeline;

                part placeOrder : SynchronousCall {
                    ref from : Lifeline = storefront;
                    ref to : Lifeline = orders;
                }
            }
        }
    "#;

    let diagrams = extract(&[
        ("Stdlib.sysml", STDLIB),
        ("WebShopSemantics.sysml", semantics),
        ("Flows.sysml", diagram_src),
    ]);
    let diagram = find_diagram(&diagrams, "OrderFlow");
    let lifeline_names: HashSet<_> = diagram.lifelines.iter().map(|l| l.name.clone()).collect();
    assert_eq!(
        lifeline_names,
        HashSet::from(["storefront".to_string(), "orders".to_string()])
    );
}

#[test]
fn graph_extractor_recognizes_custom_message_subtype() {
    let semantics = r#"
        package HttpSemantics {
            import SoftwareInteractions::*;
            part def HttpCall :> SynchronousCall;
        }
    "#;
    let diagram_src = r#"
        package Flows {
            import SoftwareInteractions::*;
            import HttpSemantics::*;

            part def HttpFlow :> InteractionScenario {
                part client : Lifeline;
                part api : Lifeline;

                part fetchOrder : HttpCall {
                    ref from : Lifeline = client;
                    ref to : Lifeline = api;
                    attribute label = "GET /orders/1";
                }
            }
        }
    "#;

    let diagrams = extract(&[
        ("Stdlib.sysml", STDLIB),
        ("HttpSemantics.sysml", semantics),
        ("Flows.sysml", diagram_src),
    ]);
    let diagram = find_diagram(&diagrams, "HttpFlow");
    assert_eq!(diagram.messages.len(), 1);
    assert_eq!(diagram.messages[0].kind, "sync");
    assert_eq!(diagram.messages[0].label.as_deref(), Some("GET /orders/1"));
}

#[test]
fn graph_extractor_orders_messages_by_source_range() {
    let demo = r#"
        package Demo {
            import SoftwareInteractions::*;

            part def OrderedFlow :> InteractionScenario {
                part client : Lifeline;
                part api : Lifeline;

                part one : SynchronousCall {
                    ref from : Lifeline = client;
                    ref to : Lifeline = api;
                }
                part two : SynchronousCall {
                    ref from : Lifeline = client;
                    ref to : Lifeline = api;
                }
                part three : SynchronousCall {
                    ref from : Lifeline = client;
                    ref to : Lifeline = api;
                }
            }
        }
    "#;

    let diagrams = extract(&[("Stdlib.sysml", STDLIB), ("Demo.sysml", demo)]);
    let diagram = find_diagram(&diagrams, "OrderedFlow");
    assert_eq!(diagram.messages.len(), 3);
    let names: Vec<_> = diagram.messages.iter().map(|m| m.name.clone()).collect();
    assert_eq!(names, vec!["one", "two", "three"]);
    let orders: Vec<_> = diagram.messages.iter().map(|m| m.order).collect();
    assert_eq!(orders, vec![1, 2, 3]);
}

#[test]
fn graph_extractor_supports_ref_fragments_and_nested_alternatives() {
    let demo = r#"
        package Demo {
            import SoftwareInteractions::*;

            part def RetryFlow :> InteractionScenario {
                part worker : Lifeline;
                part queue : Lifeline;
            }

            part def PaymentFlow :> InteractionScenario {
                part customer : Lifeline;
                part gateway : Lifeline;

                part authorize : SynchronousCall {
                    ref from : Lifeline = customer;
                    ref to : Lifeline = gateway;
                }

                part outcomes : AltFragment {
                    part approved : InteractionOperand {
                        attribute guard = "approved";
                        part complete : ReturnMessage {
                            ref from : Lifeline = gateway;
                            ref to : Lifeline = customer;
                        }
                    }

                    part declined : InteractionOperand {
                        attribute guard = "declined";
                        part retryFlow : InteractionRef {
                            ref target : InteractionScenario = RetryFlow;
                        }
                    }
                }
            }
        }
    "#;

    let diagrams = extract(&[("Stdlib.sysml", STDLIB), ("Demo.sysml", demo)]);
    let payment = find_diagram(&diagrams, "PaymentFlow");
    assert_eq!(payment.fragments.len(), 1);
    assert_eq!(payment.fragments[0].kind, "alt");
    assert_eq!(payment.fragments[0].operands.len(), 2);
    assert_eq!(payment.fragments[0].operands[1].fragments.len(), 1);
    assert_eq!(payment.fragments[0].operands[1].fragments[0].kind, "ref");
    assert_eq!(
        payment.fragments[0].operands[1].fragments[0]
            .target_ref
            .as_deref(),
        Some("RetryFlow")
    );
}
