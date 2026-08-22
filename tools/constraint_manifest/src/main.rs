//! Explicit, local-only extraction of the normative KerML and SysML constraint inventory.
//!
//! Normal builds read the committed manifest and never fetch or discover OMG artifacts. Refresh
//! is deliberately an opt-in command whose two XMI inputs and expected content digests are named
//! on the command line.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::process::Command as ProcessCommand;

use clap::{Parser, Subcommand, ValueEnum};
use quick_xml::events::{BytesStart, Event};
use quick_xml::{Reader, XmlVersion};
use sha2::{Digest, Sha256};
use spec42_constraint_manifest::{
    ActionDerivedFactContract, ActionDerivedFactKind, BindingConnectorCheckContract,
    BindingConnectorCheckKind, ConditionalLibrarySpecializationContract, ConstraintFamily,
    ConstraintManifest, ConstraintManifestEntry, DefinitionUsageDerivedContract,
    ElementDerivedDocumentationContract, ElementDerivedDocumentationKind,
    ElementDerivedOwnerContract, ElementDerivedOwnerKind, FeatureDerivedRelationshipContract,
    FeatureDerivedRelationshipKind, LibraryRedefinitionContract, LibrarySpecializationContract,
    LibrarySpecializationPredicate, NamespaceDerivedElementContract, NamespaceDerivedElementKind,
    NamespaceImportDerivedElementContract, NamespaceImportDerivedElementKind, PinnedSpecification,
    RedefinitionCheckContract, RedefinitionCheckKind, RequirementDerivedFactContract,
    RequirementDerivedFactKind, SpecializationCheckContract, SpecializationCheckKind,
    SpecificationManifest, TypeDerivedElementContract, TypeDerivedElementKind,
    TypeDerivedFactContract, TypeDerivedFactKind, TypeDerivedRelationshipContract,
    TypeDerivedRelationshipKind, TypeFeaturingCheckContract, TypeFeaturingCheckKind,
    KERML10_SPECIFICATION, SCHEMA_VERSION, SYSML20_SPECIFICATION,
};

#[derive(Debug, Parser)]
#[command(
    name = "spec42-constraint-manifest",
    about = "Refresh or audit the pinned KerML and SysML constraint manifest"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Extract the manifest from explicitly supplied, digest-pinned OMG XMI and PDF artifacts.
    Refresh {
        #[arg(long)]
        kerml: PathBuf,
        #[arg(long)]
        sysml: PathBuf,
        #[arg(long)]
        kerml_pdf: PathBuf,
        #[arg(long)]
        sysml_pdf: PathBuf,
        #[arg(long, default_value = "specifications/constraint_manifest.toml")]
        output: PathBuf,
        #[arg(long, value_enum, default_value_t = ManifestFormat::Toml)]
        format: ManifestFormat,
    },
    /// Verify a committed manifest against explicitly supplied, digest-pinned OMG XMI and PDF artifacts.
    Audit {
        #[arg(long)]
        kerml: PathBuf,
        #[arg(long)]
        sysml: PathBuf,
        #[arg(long)]
        kerml_pdf: PathBuf,
        #[arg(long)]
        sysml_pdf: PathBuf,
        #[arg(long, default_value = "specifications/constraint_manifest.toml")]
        manifest: PathBuf,
        #[arg(long, value_enum, default_value_t = ManifestFormat::Toml)]
        format: ManifestFormat,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum ManifestFormat {
    Toml,
    Json,
}

#[derive(Debug, Clone)]
enum Scope {
    RootPackage,
    Package(String),
    Metaclass(String),
}

#[derive(Debug, Clone)]
struct ElementFrame {
    scope: Option<Scope>,
    constraint_index: Option<usize>,
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();
    match cli.command {
        Command::Refresh {
            kerml,
            sysml,
            kerml_pdf,
            sysml_pdf,
            output,
            format,
        } => {
            let manifest = extract_manifest(&kerml, &sysml, &kerml_pdf, &sysml_pdf)?;
            let rendered = render_manifest(&manifest, format)?;
            fs::write(&output, rendered)
                .map_err(|error| format!("{}: write failed: {error}", output.display()))?;
        }
        Command::Audit {
            kerml,
            sysml,
            kerml_pdf,
            sysml_pdf,
            manifest,
            format,
        } => audit_manifest(&kerml, &sysml, &kerml_pdf, &sysml_pdf, &manifest, format)?,
    }
    Ok(())
}

fn extract_manifest(
    kerml: &Path,
    sysml: &Path,
    kerml_pdf: &Path,
    sysml_pdf: &Path,
) -> Result<ConstraintManifest, String> {
    let mut specifications = vec![
        extract_specification(KERML10_SPECIFICATION, kerml, kerml_pdf)?,
        extract_specification(SYSML20_SPECIFICATION, sysml, sysml_pdf)?,
    ];
    specifications.sort_by(|left, right| left.name.cmp(&right.name));
    Ok(ConstraintManifest {
        schema_version: SCHEMA_VERSION,
        specifications,
    })
}

fn extract_specification(
    definition: PinnedSpecification,
    xmi_path: &Path,
    pdf_path: &Path,
) -> Result<SpecificationManifest, String> {
    let bytes = fs::read(xmi_path)
        .map_err(|error| format!("{}: read failed: {error}", xmi_path.display()))?;
    let actual_digest = sha256_hex(&bytes);
    if actual_digest != definition.expected_sha256 {
        return Err(format!(
            "{}: SHA-256 mismatch for {} {}: expected {}, got {}",
            xmi_path.display(),
            definition.name,
            definition.xmi_file_id,
            definition.expected_sha256,
            actual_digest
        ));
    }
    let mut constraints = extract_constraints(&bytes)?;
    let (pdf_sha256, clauses) = extract_pdf_clauses(pdf_path, definition)?;
    reconcile_constraint_clauses(&mut constraints, &clauses, definition, pdf_path)?;
    constraints.sort_by(|left, right| left.rule_id.cmp(&right.rule_id));
    let mut ids = BTreeSet::new();
    for entry in &constraints {
        if !ids.insert(entry.rule_id.as_str()) {
            return Err(format!(
                "{}: duplicate stable rule identity {:?}",
                xmi_path.display(),
                entry.rule_id
            ));
        }
    }
    if constraints.is_empty() {
        return Err(format!(
            "{}: {} contains no derive*, check*, or validate* constraints",
            xmi_path.display(),
            definition.name
        ));
    }
    Ok(SpecificationManifest {
        name: definition.name.to_string(),
        version: definition.version.to_string(),
        formal_document_id: definition.formal_document_id.to_string(),
        xmi_file_id: definition.xmi_file_id.to_string(),
        xmi_sha256: actual_digest,
        pdf_sha256,
        constraints,
    })
}

/// Parse the XMI structure with an XML parser; no regular-expression or line-oriented XML
/// matching is used. `ownedRule` belongs to the nearest owning UML metaclass on the element stack.
fn extract_constraints(bytes: &[u8]) -> Result<Vec<ConstraintManifestEntry>, String> {
    let mut reader = Reader::from_reader(Cursor::new(bytes));
    reader.config_mut().trim_text(true);
    let mut buffer = Vec::new();
    let mut stack = Vec::<ElementFrame>::new();
    let mut constraints = Vec::new();

    loop {
        match reader.read_event_into(&mut buffer) {
            Ok(Event::Start(start)) => {
                let constraint_index = process_start(&start, &stack, &mut constraints)?;
                if local_name(start.name().as_ref()) == "specification" {
                    attach_library_contract(&start, &stack, &mut constraints)?;
                }
                stack.push(ElementFrame {
                    scope: scope_for_start(&start, &stack)?,
                    constraint_index,
                });
            }
            Ok(Event::Empty(start)) => {
                let _ = process_start(&start, &stack, &mut constraints)?;
                if local_name(start.name().as_ref()) == "specification" {
                    attach_library_contract(&start, &stack, &mut constraints)?;
                }
            }
            Ok(Event::End(_)) => {
                stack
                    .pop()
                    .ok_or_else(|| "XMI closes an element without an open element".to_string())?;
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(error) => {
                return Err(format!(
                    "invalid XML at byte {}: {error}",
                    reader.buffer_position()
                ))
            }
        }
        buffer.clear();
    }
    if !stack.is_empty() {
        return Err("XMI ended before every element closed".to_string());
    }
    Ok(constraints)
}

fn process_start(
    start: &BytesStart<'_>,
    stack: &[ElementFrame],
    constraints: &mut Vec<ConstraintManifestEntry>,
) -> Result<Option<usize>, String> {
    if local_name(start.name().as_ref()) != "ownedRule" {
        return Ok(None);
    }
    let attributes = attributes(start)?;
    // UML uses an `ownedRule` element with only `xmi:idref` to point at inherited operation
    // pre/postconditions. It is not a constraint declaration owned by this metaclass.
    if attributes.contains_key("idref") {
        return Ok(None);
    }
    let Some(name) = attributes.get("name") else {
        return Err("ownedRule has no name".to_string());
    };
    let Some(family) = ConstraintFamily::from_constraint_name(name) else {
        return Ok(None);
    };
    let metaclass = stack.iter().rev().find_map(|frame| match &frame.scope {
        Some(Scope::Metaclass(name)) => Some(name.as_str()),
        _ => None,
    });
    let Some(metaclass) = metaclass else {
        return Err(format!("constraint {name:?} has no owning UML metaclass"));
    };
    let package_parts = stack.iter().filter_map(|frame| match &frame.scope {
        Some(Scope::Package(name)) => Some(name.as_str()),
        _ => None,
    });
    let package = package_parts.collect::<Vec<_>>().join("::");
    if package.is_empty() {
        return Err(format!("constraint {name:?} has no owning package"));
    }
    constraints.push(ConstraintManifestEntry {
        // The stable rule identity depends on the PDF clause and is assigned only after
        // deterministic XMI-to-PDF reconciliation succeeds.
        rule_id: String::new(),
        package,
        metaclass: metaclass.to_string(),
        constraint: name.clone(),
        family,
        clause: String::new(),
        specializes_from_library: None,
        conditional_specializes_from_library: None,
        redefines_from_library: None,
        feature_derived_relationship: None,
        type_derived_relationship: None,
        type_derived_element: None,
        type_derived_fact: None,
        type_featuring_check: None,
        redefinition_check: None,
        specialization_check: None,
        element_derived_owner: None,
        element_derived_documentation: None,
        namespace_derived_element: None,
        namespace_import_derived_element: None,
        binding_connector_check: None,
        definition_usage_derived: None,
        action_derived_fact: None,
        requirement_derived_fact: None,
    });
    Ok(Some(constraints.len() - 1))
}

fn attach_library_contract(
    start: &BytesStart<'_>,
    stack: &[ElementFrame],
    constraints: &mut [ConstraintManifestEntry],
) -> Result<(), String> {
    let attributes = attributes(start)?;
    let Some(body) = attributes.get("body") else {
        return Ok(());
    };
    let Some(index) = stack.iter().rev().find_map(|frame| frame.constraint_index) else {
        return Ok(());
    };
    let Some(entry) = constraints.get_mut(index) else {
        return Err("constraint specification refers to an invalid owner".to_string());
    };
    if let Some(anchor) = exact_library_specialization_anchor(body) {
        entry.specializes_from_library = Some(LibrarySpecializationContract { anchor });
    }
    if let Some(contract) = exact_conditional_library_specialization(body) {
        entry.conditional_specializes_from_library = Some(contract);
    }
    if let Some(anchor) = exact_library_redefinition_anchor(body) {
        entry.redefines_from_library = Some(LibraryRedefinitionContract { anchor });
    }
    if let Some(kind) = exact_feature_derived_relationship(body) {
        entry.feature_derived_relationship = Some(FeatureDerivedRelationshipContract { kind });
    }
    if let Some(kind) = exact_type_derived_relationship(body) {
        entry.type_derived_relationship = Some(TypeDerivedRelationshipContract { kind });
    }
    if let Some(kind) = exact_type_derived_element(body) {
        entry.type_derived_element = Some(TypeDerivedElementContract { kind });
    }
    if let Some(kind) = exact_type_derived_fact(&entry.constraint, body) {
        entry.type_derived_fact = Some(TypeDerivedFactContract { kind });
    }
    if let Some(kind) = exact_type_featuring_check(&entry.constraint, body) {
        entry.type_featuring_check = Some(TypeFeaturingCheckContract {
            kind,
            body_sha256: sha256_hex(body.as_bytes()),
        });
    }
    if let Some(kind) = exact_redefinition_check(&entry.constraint, body) {
        entry.redefinition_check = Some(RedefinitionCheckContract {
            kind,
            body_sha256: sha256_hex(body.as_bytes()),
        });
    }
    if let Some(kind) = exact_specialization_check(&entry.constraint, body) {
        entry.specialization_check = Some(SpecializationCheckContract {
            kind,
            body_sha256: sha256_hex(body.as_bytes()),
        });
    }
    if let Some(kind) = exact_element_derived_owner(body) {
        entry.element_derived_owner = Some(ElementDerivedOwnerContract { kind });
    }
    if let Some(kind) = exact_element_derived_documentation(body) {
        entry.element_derived_documentation = Some(ElementDerivedDocumentationContract { kind });
    }
    if let Some(kind) = exact_namespace_derived_element(body) {
        entry.namespace_derived_element = Some(NamespaceDerivedElementContract { kind });
    }
    if let Some(kind) = exact_namespace_import_derived_element(body) {
        entry.namespace_import_derived_element =
            Some(NamespaceImportDerivedElementContract { kind });
    }
    if let Some(kind) = exact_binding_connector_check(&entry.constraint, body) {
        entry.binding_connector_check = Some(BindingConnectorCheckContract {
            kind,
            body_sha256: sha256_hex(body.as_bytes()),
        });
    }
    let definition_usage_body_sha256 = sha256_hex(body.as_bytes());
    if let Some(contract) = DefinitionUsageDerivedContract::from_exact_pinned_body(
        &entry.constraint,
        &definition_usage_body_sha256,
    ) {
        entry.definition_usage_derived = Some(contract);
    }
    if let Some(kind) = exact_action_derived_fact(&entry.constraint, body) {
        entry.action_derived_fact = Some(ActionDerivedFactContract {
            kind,
            body_sha256: sha256_hex(body.as_bytes()),
        });
    }
    if let Some(kind) = exact_requirement_derived_fact(&entry.constraint, body) {
        entry.requirement_derived_fact = Some(RequirementDerivedFactContract {
            kind,
            body_sha256: sha256_hex(body.as_bytes()),
        });
    }
    Ok(())
}

/// Extracts only complete Feature relationship-collection bodies. These bodies select one
/// already-owned relationship metaclass; complex closure, `let`, `if`, or predicate bodies stay
/// outside the contract rather than becoming a partial OCL interpreter.
fn exact_feature_derived_relationship(body: &str) -> Option<FeatureDerivedRelationshipKind> {
    match body {
        "ownedFeatureChaining = ownedRelationship->selectByKind(FeatureChaining)" => {
            Some(FeatureDerivedRelationshipKind::OwnedFeatureChaining)
        }
        "ownedRedefinition = ownedSubsetting->selectByKind(Redefinition)" => {
            Some(FeatureDerivedRelationshipKind::OwnedRedefinition)
        }
        "ownedSubsetting = ownedSpecialization->selectByKind(Subsetting)" => {
            Some(FeatureDerivedRelationshipKind::OwnedSubsetting)
        }
        "ownedTyping = ownedGeneralization->selectByKind(FeatureTyping)" => {
            Some(FeatureDerivedRelationshipKind::OwnedTyping)
        }
        "ownedTypeFeaturing = ownedRelationship->selectByKind(TypeFeaturing)-> select(tf | tf.featureOfType = self)" => {
            Some(FeatureDerivedRelationshipKind::OwnedTypeFeaturing)
        }
        _ => None,
    }
}

/// Extract only complete pinned Type relationship-collection and operand bodies. The operand
/// variants deliberately preserve the relationship fact instead of interpreting an arbitrary
/// property path or materializing a second target collection.
fn exact_type_derived_relationship(body: &str) -> Option<TypeDerivedRelationshipKind> {
    match body {
        "ownedRelationship->selectByKind(Intersecting)" => {
            Some(TypeDerivedRelationshipKind::OwnedIntersecting)
        }
        "ownedDisjoining = ownedRelationship->selectByKind(Disjoining)" => {
            Some(TypeDerivedRelationshipKind::OwnedDisjoining)
        }
        "ownedDifferencing = ownedRelationship->selectByKind(Differencing)" => {
            Some(TypeDerivedRelationshipKind::OwnedDifferencing)
        }
        "ownedSpecialization = ownedRelationship->selectByKind(Specialization)-> select(s | s.special = self) " => {
            Some(TypeDerivedRelationshipKind::OwnedSpecialization)
        }
        "ownedUnioning = ownedRelationship->selectByKind(Unioning)" => {
            Some(TypeDerivedRelationshipKind::OwnedUnioning)
        }
        "unioningType = ownedUnioning.unioningType" => {
            Some(TypeDerivedRelationshipKind::UnioningType)
        }
        "intersectingType = ownedIntersecting.intersectingType" => {
            Some(TypeDerivedRelationshipKind::IntersectingType)
        }
        "differencingType = ownedDifferencing.differencingType" => {
            Some(TypeDerivedRelationshipKind::DifferencingType)
        }
        _ => None,
    }
}

/// Extract only the complete final `Type` element projections. The publication owns its direct
/// declaration owner, Feature-membership, and feature-prefix modifier facts, which are sufficient
/// to return the selected member elements without fabricating intermediate FeatureMembership
/// identities.
/// Do not admit `feature` or `featureMembership`: each requires inherited memberships, whose
/// canonical owner has not yet published a complete closure.
fn exact_type_derived_element(body: &str) -> Option<TypeDerivedElementKind> {
    match body {
        "ownedFeature = ownedFeatureMembership.ownedMemberFeature" => {
            Some(TypeDerivedElementKind::OwnedFeature)
        }
        "ownedEndFeature = ownedFeature->select(isEnd)" => {
            Some(TypeDerivedElementKind::OwnedEndFeature)
        }
        _ => None,
    }
}

/// Exact `Type` derivations whose result type is normative but whose first canonical prerequisite
/// is not yet owned by the published semantic model. Whitespace normalization is lossless for
/// these complete token sequences and does not admit prefixes or general OCL shapes.
fn exact_type_derived_fact(constraint: &str, body: &str) -> Option<TypeDerivedFactKind> {
    let body = body.split_whitespace().collect::<Vec<_>>().join(" ");
    match (constraint, body.as_str()) {
        ("deriveTypeOwnedFeatureMembership", "ownedFeatureMembership = ownedRelationship->selectByKind(FeatureMembership)") => Some(TypeDerivedFactKind::OwnedFeatureMembership),
        ("deriveTypeFeatureMembership", "featureMembership = ownedFeatureMembership->union( inheritedMembership->selectByKind(FeatureMembership))") => Some(TypeDerivedFactKind::FeatureMembership),
        ("deriveTypeFeature", "feature = featureMembership.ownedMemberFeature") => Some(TypeDerivedFactKind::Feature),
        ("deriveTypeEndFeature", "endFeature = feature->select(isEnd)") => Some(TypeDerivedFactKind::EndFeature),
        ("deriveTypeDirectedFeature", "directedFeature = feature->select(f | directionOf(f) <> null)") => Some(TypeDerivedFactKind::DirectedFeature),
        ("deriveTypeInheritedMembership", "inheritedMembership = inheritedMemberships(Set{}, Set{}, false)") => Some(TypeDerivedFactKind::InheritedMembership),
        ("deriveTypeInheritedFeature", "inheritedFeature = inheritedMemberships-> selectByKind(FeatureMembership).memberFeature") => Some(TypeDerivedFactKind::InheritedFeature),
        ("deriveTypeInput", "input = feature->select(f | let direction: FeatureDirectionKind = directionOf(f) in direction = FeatureDirectionKind::_'in' or direction = FeatureDirectionKind::inout)") => Some(TypeDerivedFactKind::Input),
        ("deriveTypeOutput", "output = feature->select(f | let direction: FeatureDirectionKind = directionOf(f) in direction = FeatureDirectionKind::out or direction = FeatureDirectionKind::inout)") => Some(TypeDerivedFactKind::Output),
        ("deriveTypeMultiplicity", "multiplicity = let ownedMultiplicities: Sequence(Multiplicity) = ownedMember->selectByKind(Multiplicity) in if ownedMultiplicities->isEmpty() then null else ownedMultiplicities->first() endif") => Some(TypeDerivedFactKind::Multiplicity),
        ("deriveTypeOwnedConjugator", "ownedConjugator = let ownedConjugators: Sequence(Conjugator) = ownedRelationship->selectByKind(Conjugation) in if ownedConjugators->isEmpty() then null else ownedConjugators->at(1) endif") => Some(TypeDerivedFactKind::OwnedConjugator),
        _ => None,
    }
}

/// Extract only the complete FeatureMembership TypeFeaturing implication. Its antecedent is the
/// canonical FeatureMembership fact and its consequent is the canonical effective TypeFeaturing
/// collection, so no source reconstruction or partial OCL interpretation is needed.
fn exact_type_featuring_check(constraint: &str, body: &str) -> Option<TypeFeaturingCheckKind> {
    match (constraint, body) {
        (
            "checkFeatureFeatureMembershipTypeFeaturing",
            "owningFeatureMembership <> null implies featuringTypes->exists(t | isFeaturingType(t))",
        ) => Some(TypeFeaturingCheckKind::FeatureFeatureMembership),
        _ => None,
    }
}

/// Exact Systems::Actions property bodies.  The SHA matches the unmodified pinned XMI body, so
/// whitespace, branch, ordinal, or required metaclass changes cannot silently widen a contract.
fn exact_action_derived_fact(constraint: &str, body: &str) -> Option<ActionDerivedFactKind> {
    match (constraint, sha256_hex(body.as_bytes()).as_str()) {
        (
            "deriveActionDefinitionAction",
            "7872e4a616e8fc27a45bd4fec622d9dcb206b22e23a576fe39dfe4f481fe0139",
        ) => Some(ActionDerivedFactKind::ActionDefinitionAction),
        (
            "deriveAssignmentActionUsageValueExpression",
            "610474804569285f5825176504f20da26768043b79361cea7664b4ff36e26388",
        ) => Some(ActionDerivedFactKind::AssignmentValueExpression),
        (
            "deriveAssignmentUsageTargetArgument",
            "5ce7ace2a22d8f31a549d22e68bbeefb3902cc3d36a98c5a839740a735fb967a",
        ) => Some(ActionDerivedFactKind::AssignmentTargetArgument),
        (
            "deriveAssignmentActionUsageReferent",
            "8773c7a04945562abc19317dd5bda6e972f1fa8781260b740797beb9c3efc1d0",
        ) => Some(ActionDerivedFactKind::AssignmentReferent),
        (
            "deriveForLoopActionUsageLoopVariable",
            "d90dcabc5291320811c4654e43162c2eed89e16b78d3f967b1eca3ee75c6fc8c",
        ) => Some(ActionDerivedFactKind::ForLoopVariable),
        (
            "deriveForLoopActionUsageSeqArgument",
            "4c171f9ec57ef018961d7e40675fc0b5456546208c77b7844aa47f2f904f7a80",
        ) => Some(ActionDerivedFactKind::ForLoopSeqArgument),
        (
            "deriveLoopActionUsageBodyAction",
            "4f5670d8bfc41a23f7f107604343b5ac79f253010596dc8024cf63e2da013c5a",
        ) => Some(ActionDerivedFactKind::LoopBodyAction),
        (
            "deriveTerminateActionUsageTerminatedOccurrenceArgument",
            "97b5591eef2caa83652f019244ec9b4ddf791a650106dd5507f7fa6d8af9fb8c",
        ) => Some(ActionDerivedFactKind::TerminateOccurrenceArgument),
        (
            "deriveAcceptActionUsagePayloadArgument",
            "f35de7ce2869094f5f28977cf462f3a510f854f54f04dabc80b98e307eadae4d",
        ) => Some(ActionDerivedFactKind::AcceptPayloadArgument),
        (
            "deriveAcceptActionUsagePayloadParameter",
            "246b4f4f6cc4870aee84d9cac36287008d1d5ac2b59046206aae646ce47cfbbe",
        ) => Some(ActionDerivedFactKind::AcceptPayloadParameter),
        (
            "deriveAcceptActionUsageReceiverArgument",
            "88e4e8cb4e0e15eee528fedaee4baa0468887e59daa388e91efa0b12045b0f99",
        ) => Some(ActionDerivedFactKind::AcceptReceiverArgument),
        (
            "deriveWhileLoopActionUsageWhileArgument",
            "8572cdc7b35d0e7accac9dad478bd1442ea643ac68b6186276ea5022a8df9d8e",
        ) => Some(ActionDerivedFactKind::WhileArgument),
        (
            "deriveWhileLoopActionUsageUntilArgument",
            "e808b11abde7b571384b9ac8a6a7d29f378a462cbe752e2d9c9bfbd07501670f",
        ) => Some(ActionDerivedFactKind::UntilArgument),
        (
            "deriveSendActionUsageSenderArgument",
            "322943a677fed299bd1052e5277afa0798dfb84e1b99dd6172604c7e83d048d6",
        ) => Some(ActionDerivedFactKind::SendSenderArgument),
        (
            "deriveSendActionUsageReceiverArgument",
            "9be5035d34c99f4cca19c8dcb85cbd9d3514b8ae583de48dcfe333eb19fad5ad",
        ) => Some(ActionDerivedFactKind::SendReceiverArgument),
        (
            "deriveSendActionUsagePayloadArgument",
            "f35de7ce2869094f5f28977cf462f3a510f854f54f04dabc80b98e307eadae4d",
        ) => Some(ActionDerivedFactKind::SendPayloadArgument),
        (
            "deriveIfActionUsageThenAction",
            "d27bb6b629ce48335295d47e6ab2c375e3e8c61fde1b546dc487126ee04cb020",
        ) => Some(ActionDerivedFactKind::IfThenAction),
        (
            "deriveIfActionUsageElseAction",
            "347397f8e7db08658345fc0a2a92f3cdb3e0d4a8b76b4506d300c1051761ce15",
        ) => Some(ActionDerivedFactKind::IfElseAction),
        (
            "deriveIfActionUsageIfArgument",
            "7dc57a8093de6438288f6693d9bde3fbfa1648a41713512361efce3fe7cfa942",
        ) => Some(ActionDerivedFactKind::IfArgument),
        _ => None,
    }
}

/// Select only the complete Systems::Requirements bodies whose inputs are already published as
/// canonical membership roles or documentation records.  The digest commits the exact pinned OCL
/// spelling: malformed stakeholder, reference-target, and binding predicates deliberately stay
/// outside this contract instead of being repaired by an extractor.
fn exact_requirement_derived_fact(
    constraint: &str,
    body: &str,
) -> Option<RequirementDerivedFactKind> {
    // These six complete OCL bodies contain only XML attribute-layout whitespace differences.
    // Normalize that whitespace at this extraction boundary while retaining the unmodified body
    // digest in the emitted contract. Every token and every branch remains closed here.
    let normalized = body.split_whitespace().collect::<Vec<_>>().join(" ");
    match (constraint, normalized.as_str()) {
        ("deriveRequirementDefinitionSubjectParameter", "subjectParameter = let subjects : OrderedSet(SubjectMembership) = featureMembership->selectByKind(SubjectMembership) in if subjects->isEmpty() then null else subjects->first().ownedSubjectParameter endif") => return Some(RequirementDerivedFactKind::DefinitionSubjectParameter),
        ("deriveRequirementDefinitionRequiredConstraint", "requiredConstraint = ownedFeatureMembership-> selectByKind(RequirementConstraintMembership)-> select(kind = RequirementConstraintKind::requirement). ownedConstraint") => return Some(RequirementDerivedFactKind::DefinitionRequiredConstraint),
        ("deriveRequirementDefinitionAssumedConstraint", "assumedConstraint = ownedFeatureMembership-> selectByKind(RequirementConstraintMembership)-> select(kind = RequirementConstraintKind::assumption). ownedConstraint") => return Some(RequirementDerivedFactKind::DefinitionAssumedConstraint),
        ("deriveRequirementUsageSubjectParameter", "subjectParameter = let subjects : OrderedSet(SubjectMembership) = featureMembership->selectByKind(SubjectMembership) in if subjects->isEmpty() then null else subjects->first().ownedSubjectParameter endif") => return Some(RequirementDerivedFactKind::UsageSubjectParameter),
        ("deriveRequirementUsageRequiredConstraint", "requiredConstraint = ownedFeatureMembership-> selectByKind(RequirementConstraintMembership)-> select(kind = RequirementConstraintKind::requirement). ownedConstraint") => return Some(RequirementDerivedFactKind::UsageRequiredConstraint),
        ("deriveRequirementUsageAssumedConstraint", "assumedConstraint = ownedFeatureMembership-> selectByKind(RequirementConstraintMembership)-> select(kind = RequirementConstraintKind::assumption). ownedConstraint") => return Some(RequirementDerivedFactKind::UsageAssumedConstraint),
        _ => {}
    }
    match (constraint, sha256_hex(body.as_bytes()).as_str()) {
        (
            "deriveRequirementDefinitionActorParameter",
            "4491cc267914513216e0de41b0fab6f99422326393c937b7499fc16c9fde61a9",
        ) => Some(RequirementDerivedFactKind::DefinitionActorParameter),
        (
            "deriveRequirementDefinitionSubjectParameter",
            "a974bff5d79e5ed8c5fbb92464594e2dcbc25c9780cc5b414d65204077d098cd",
        ) => Some(RequirementDerivedFactKind::DefinitionSubjectParameter),
        (
            "deriveRequirementDefinitionText",
            "f0940bd08b9bf742aa3c4dc08462b7957c1f15729f6845f95010e273adbf9016",
        ) => Some(RequirementDerivedFactKind::DefinitionText),
        (
            "deriveRequirementDefinitionRequiredConstraint",
            "c6be589ce8372b1af2003aa0c0a2703bdbc1f436d3a77502964285a5a068f14a",
        ) => Some(RequirementDerivedFactKind::DefinitionRequiredConstraint),
        (
            "deriveRequirementDefinitionAssumedConstraint",
            "fe3d349fead7db1e9dc5c437397c551aed9c41028a56bafd8b337506198a6f91",
        ) => Some(RequirementDerivedFactKind::DefinitionAssumedConstraint),
        (
            "deriveRequirementDefinitionFramedConcern",
            "c915c2298f5bfebaa9a0e06d06f3d83d3f4c2c2601cc17f332926bf72ea94e3e",
        ) => Some(RequirementDerivedFactKind::DefinitionFramedConcern),
        (
            "deriveRequirementUsageActorParameter",
            "4491cc267914513216e0de41b0fab6f99422326393c937b7499fc16c9fde61a9",
        ) => Some(RequirementDerivedFactKind::UsageActorParameter),
        (
            "deriveRequirementUsageSubjectParameter",
            "a974bff5d79e5ed8c5fbb92464594e2dcbc25c9780cc5b414d65204077d098cd",
        ) => Some(RequirementDerivedFactKind::UsageSubjectParameter),
        (
            "deriveRequirementUsageText",
            "f0940bd08b9bf742aa3c4dc08462b7957c1f15729f6845f95010e273adbf9016",
        ) => Some(RequirementDerivedFactKind::UsageText),
        (
            "deriveRequirementUsageRequiredConstraint",
            "c6be589ce8372b1af2003aa0c0a2703bdbc1f436d3a77502964285a5a068f14a",
        ) => Some(RequirementDerivedFactKind::UsageRequiredConstraint),
        (
            "deriveRequirementUsageAssumedConstraint",
            "fe3d349fead7db1e9dc5c437397c551aed9c41028a56bafd8b337506198a6f91",
        ) => Some(RequirementDerivedFactKind::UsageAssumedConstraint),
        (
            "deriveRequirementUsageFramedConcern",
            "c915c2298f5bfebaa9a0e06d06f3d83d3f4c2c2601cc17f332926bf72ea94e3e",
        ) => Some(RequirementDerivedFactKind::UsageFramedConcern),
        _ => None,
    }
}

/// Selects only the complete pinned redefinition predicates by constraint identity and exact
/// source-body digest. This is intentionally a closed fingerprint table, not an OCL classifier:
/// changing one character in the official predicate makes the contract absent until its semantic
/// owner and tests have been reviewed together.
fn exact_redefinition_check(constraint: &str, body: &str) -> Option<RedefinitionCheckKind> {
    match (constraint, sha256_hex(body.as_bytes()).as_str()) {
        (
            "checkFeatureEndRedefinition",
            "8747431465ceef72b37b67feb11775341732ee6afbe957818bc1ef380788cc2e",
        ) => Some(RedefinitionCheckKind::FeatureEnd),
        (
            "checkFeatureFlowFeatureRedefinition",
            "86ee51338f0bc7bf1cc6eddf42a4491372734a399eac9dbb3893d42c04d4f867",
        ) => Some(RedefinitionCheckKind::FeatureFlowFeature),
        (
            "checkFeatureOwnedCrossFeatureRedefinitionSpecialization",
            "6e3cea90693ed3c8a67a21a46598afea8eece7451fbc58d0cd0ad5ed3d15112f",
        ) => Some(RedefinitionCheckKind::FeatureOwnedCrossFeatureSpecialization),
        (
            "checkFeatureParameterRedefinition",
            "7be82dad9b00543740c0c374334407fe4b559b8e2e0747d0d4525485d3afeba2",
        ) => Some(RedefinitionCheckKind::FeatureParameter),
        (
            "checkFeatureResultRedefinition",
            "791eae28342f20cafe62680bfa95a70718b99dd1ead744425c27204146eab0ef",
        ) => Some(RedefinitionCheckKind::FeatureResult),
        (
            "checkConstructorExpressionResultFeatureRedefinition",
            "894296d0076037f3cb3b3783f970c8b90fef382f9296aa2d8672c69c93c80f4c",
        ) => Some(RedefinitionCheckKind::ConstructorExpressionResultFeature),
        (
            "checkFeatureChainExpressionSourceTargetRedefinition",
            "3976c4b5eddf14a5252c3a8656d8ef65895ae5a149b6d41b3d2254118798afb7",
        ) => Some(RedefinitionCheckKind::FeatureChainExpressionSourceTarget),
        (
            "checkFeatureChainExpressionTargetRedefinition",
            "c8df46d3fd5145a0a6ed30e6a5c05f0514485a1c1179be9a2f3a9ac1004fe6c1",
        ) => Some(RedefinitionCheckKind::FeatureChainExpressionTarget),
        (
            "checkActionUsageStateActionRedefinition",
            "b8bc90ee469a0e87a536bdc6bcbed80e6b6cc5a718e5aef488203f1fa06fce2e",
        ) => Some(RedefinitionCheckKind::ActionUsageStateAction),
        (
            "checkAssignmentActionUsageAccessedFeatureRedefinition",
            "b147fe42157e4fc77f1704eba0f01a2020c05281b82f9203c6fa947b08f651a2",
        ) => Some(RedefinitionCheckKind::AssignmentActionUsageAccessedFeature),
        (
            "checkAssignmentActionUsageReferentRedefinition",
            "54c689a5405db06ff2b1ab4ec13966118456aeb731bd34543496bbe6d1aa1c9d",
        ) => Some(RedefinitionCheckKind::AssignmentActionUsageReferent),
        (
            "checkAssignmentActionUsageStartingAtRedefinition",
            "8d8d0297978140ec9695701694537ce12923b72cf5b389665fcb5ffe15a2fbb7",
        ) => Some(RedefinitionCheckKind::AssignmentActionUsageStartingAt),
        (
            "checkForLoopActionUsageVarRedefinition",
            "5cf6b791a9152a5e230937017653dbb283682ffe933cc1fe254dc4df32857223",
        ) => Some(RedefinitionCheckKind::ForLoopActionUsageVar),
        (
            "checkRequirementUsageObjectiveRedefinition",
            "04697a0cf644432753919c6ee374f3cc0d8cbe89cf4d4ffd2bc902b914738fbd",
        ) => Some(RedefinitionCheckKind::RequirementUsageObjective),
        (
            "checkRenderingUsageRedefinition",
            "12fce1a5188723cc69e60d2c908c496b542fb94aef868b9a3197ad8ccc18de6f",
        ) => Some(RedefinitionCheckKind::RenderingUsage),
        _ => None,
    }
}

/// Selects the remaining complete named specialization checks by their unmodified pinned-body
/// fingerprints.  This is a boundary declaration, not a general OCL matcher: each enum value
/// names the whole predicate and lets the resolver state its first missing canonical input.
fn exact_specialization_check(constraint: &str, body: &str) -> Option<SpecializationCheckKind> {
    match (constraint, sha256_hex(body.as_bytes()).as_str()) {
        (
            "checkFeatureCrossingSpecialization",
            "47881c2d357962093cdbf0443dadd77640285876d021f6114451df24919f7345",
        ) => Some(SpecializationCheckKind::FeatureCrossing),
        (
            "checkFeatureObjectSpecialization",
            "1f2b529eb4ca82e07862f31cfe545dfdf957a1d13647f0b13ab3bdb7a52528ab",
        ) => Some(SpecializationCheckKind::FeatureObject),
        (
            "checkFeatureOccurrenceSpecialization",
            "630ce8c543298b5047aae6f40b0077429014535533ed623a643ee59d0f50595e",
        ) => Some(SpecializationCheckKind::FeatureOccurrence),
        (
            "checkFeatureOwnedCrossFeatureSpecialization",
            "f4b9efd26d1bda827b8c555bccce877c10449ce09b96447edb1f969825aac76c",
        ) => Some(SpecializationCheckKind::FeatureOwnedCrossFeature),
        (
            "checkFeaturePortionSpecialization",
            "b787bc9258cd436c1755ddabfa52081cf11dcf182d512fd0b8716d6691d0f1fd",
        ) => Some(SpecializationCheckKind::FeaturePortion),
        (
            "checkFeatureSubobjectSpecialization",
            "4e25db6b99093b5d97599bc57600501477f59a86a095b82e57b3d753d982166a",
        ) => Some(SpecializationCheckKind::FeatureSubobject),
        (
            "checkFeatureSuboccurrenceSpecialization",
            "83634d519b643f9eb356c1756dc2061df5e307fde0f698e34d797e9dfd508028",
        ) => Some(SpecializationCheckKind::FeatureSuboccurrence),
        (
            "checkFeatureValuationSpecialization",
            "522b1613e844c2e80a7799539cffb138ab4db04e536587596bdd0cd0bec61e2a",
        ) => Some(SpecializationCheckKind::FeatureValuation),
        (
            "checkMetadataFeatureSemanticSpecialization",
            "936940ed125c2c95d5d880dc54f0116978ccd57d6bc97d53b10559ed7633d6d2",
        ) => Some(SpecializationCheckKind::MetadataFeatureSemantic),
        (
            "checkConnectorBinaryObjectSpecialization",
            "ef980ca3cf7281d3144fadfdfd58f140ad6e820bd1018d454600c432c58d4482",
        ) => Some(SpecializationCheckKind::ConnectorBinaryObject),
        (
            "checkConnectorObjectSpecialization",
            "9ae2298b97dbbf830e4340a5775ed25be9a04cf483dbbeb8418570eba4ec9092",
        ) => Some(SpecializationCheckKind::ConnectorObject),
        (
            "checkStepOwnedPerformanceSpecialization",
            "e01a90308904c0884f8ac137304b7b6e6a0542f6b2048f34af2b2b8f0e976089",
        ) => Some(SpecializationCheckKind::StepOwnedPerformance),
        (
            "checkStepSubperformanceSpecialization",
            "50f033eae06d065541811f26cd541408d80c8a869fe2adbff37b896b1d6196c3",
        ) => Some(SpecializationCheckKind::StepSubperformance),
        (
            "checkSelectExpressionResultSpecialization",
            "a9b331c1590b638129c082c7e7b85a7544f3c71703d363351a6a254334487144",
        ) => Some(SpecializationCheckKind::SelectExpressionResult),
        (
            "checkConstructorExpressionResultSpecialization",
            "5eb1f49d18322a9f0d90ea6e5e22a212746672c27bf87c3ce9d17d95f5b478f4",
        ) => Some(SpecializationCheckKind::ConstructorExpressionResult),
        (
            "checkConstructorExpressionSpecialization",
            "66dd4fa3be2fb4306c333f7de59ab2173098a94d54cb8f78223f0f814d628dd7",
        ) => Some(SpecializationCheckKind::ConstructorExpression),
        (
            "checkFeatureChainExpressionResultSpecialization",
            "673cf0957eedc0885b7ab2f127aa1f8e6b8b0322b0b1daffce5aafcb21883ebf",
        ) => Some(SpecializationCheckKind::FeatureChainExpressionResult),
        (
            "checkFeatureReferenceExpressionResultSpecialization",
            "9bcffa10b905b42ab89228954819fc2ce14a5919b4ec35c9dc18d3e1962f154c",
        ) => Some(SpecializationCheckKind::FeatureReferenceExpressionResult),
        (
            "checkIndexExpressionResultSpecialization",
            "44a5e254e58ddb9ec88d2a602274005579323e9901a94e65c6bc256ba2683911",
        ) => Some(SpecializationCheckKind::IndexExpressionResult),
        (
            "checkInvocationExpressionBehaviorResultSpecialization",
            "ba28607d7a256ae92b12ce3be5bf2dfa5152175cda1e25b10a3cbd1403605170",
        ) => Some(SpecializationCheckKind::InvocationExpressionBehaviorResult),
        (
            "checkInvocationExpressionSpecialization",
            "590e9ab6b0c19b9b8254adc7a7c8879642b9d4ab8b08572a0a583f1c4f575597",
        ) => Some(SpecializationCheckKind::InvocationExpression),
        (
            "checkMergeNodeIncomingSuccessionSpecialization",
            "5b836063bce12a1d40e5870b188e457bab4798a4ac7d3c15cb9b1f061d67af2f",
        ) => Some(SpecializationCheckKind::MergeNodeIncomingSuccession),
        (
            "checkDecisionNodeOutgoingSuccessionSpecialization",
            "6f5518e6f7b338a460191d277f3ac752f21c254101c25fc1c0c22732cf1a7a6c",
        ) => Some(SpecializationCheckKind::DecisionNodeOutgoingSuccession),
        (
            "checkStateUsageExclusiveStateSpecialization",
            "b898b22cb8d2191cb0860e84b77b941851b1e6a6b107f4d50ab68d6ec2638af7",
        ) => Some(SpecializationCheckKind::StateUsageExclusiveState),
        (
            "checkStateUsageSubstateSpecialization",
            "54a5dd705707d6d2350b6ff36e886174b59950c6f42605ea1f398a77bda5535d",
        ) => Some(SpecializationCheckKind::StateUsageSubstate),
        (
            "checkTransitionUsageActionSpecialization",
            "6b7e5eabb948223437e1ec76e48394ca6f268c8a7eb90832306395da63e4320d",
        ) => Some(SpecializationCheckKind::TransitionUsageAction),
        (
            "checkTransitionUsagePayloadSpecialization",
            "c1ae18465b56201790dfbfadc0a99ef06c335896bd1ee1567c39f5b619642f2d",
        ) => Some(SpecializationCheckKind::TransitionUsagePayload),
        (
            "checkTransitionUsageStateSpecialization",
            "6a12483ddf6ab1063595ec94dc583fa3b345649f621c3031f94d5f2ef170e0b4",
        ) => Some(SpecializationCheckKind::TransitionUsageState),
        (
            "checkTransitionUsageSuccessionSourceSpecialization",
            "6ee42713077e669332e3bc50eb3c14925b0a55824a145333a789a1119346b33f",
        ) => Some(SpecializationCheckKind::TransitionUsageSuccessionSource),
        (
            "checkTransitionUsageTransitionFeatureSpecialization",
            "14316db507c88bb3779a97184f7bc81291654f7bdaf89ab374478ad3b8ce8730",
        ) => Some(SpecializationCheckKind::TransitionUsageTransitionFeature),
        (
            "checkIncludeUseCaseSpecialization",
            "ecb0c3b855cac97c010ad87f8737f04179cd8de522bf5e04b941111c3fe8ae58",
        ) => Some(SpecializationCheckKind::IncludeUseCase),
        (
            "checkUsageVariationDefinitionSpecialization",
            "fdf18171e224258850523b58fe9ae23f572597e25192ddf02bf83f28fd57e5ba",
        ) => Some(SpecializationCheckKind::UsageVariationDefinition),
        (
            "checkUsageVariationUsageSpecialization",
            "6a33a2111fdc24d151969bcd3ad52ebfb97291e534546a20c0ec886735f84ff7",
        ) => Some(SpecializationCheckKind::UsageVariationUsage),
        (
            "checkOccurrenceDefinitionMultiplicitySpecialization",
            "51cbfb36ae596a4f59f5fc4b0154199dcea4f8481e40451109b321407e4c7d90",
        ) => Some(SpecializationCheckKind::OccurrenceDefinitionMultiplicity),
        (
            "checkOccurrenceUsageSuboccurrenceSpecialization",
            "a82787fc405ae7a6cd05beecc04374f7fe670e8ff13ee0eff3efa43ff9ad4afc",
        ) => Some(SpecializationCheckKind::OccurrenceUsageSuboccurrence),
        _ => None,
    }
}

/// Extract only the complete `Element::owner` equation. The semantic model already owns this
/// declaration-structure fact; adjacent Element rules call operations, branch, or project
/// relationship objects the publication does not expose as first-class facts.
fn exact_element_derived_owner(body: &str) -> Option<ElementDerivedOwnerKind> {
    match body {
        "owner = owningRelationship.owningRelatedElement" => Some(ElementDerivedOwnerKind::Owner),
        _ => None,
    }
}

/// Extract only complete Element documentation-form selections from the canonical owned-element
/// collection. Other annotation rules require relationship identities or predicates that this
/// semantic publication does not expose as a complete fact.
fn exact_element_derived_documentation(body: &str) -> Option<ElementDerivedDocumentationKind> {
    match body {
        "documentation = ownedElement->selectByKind(Documentation)" => {
            Some(ElementDerivedDocumentationKind::Documentation)
        }
        "textualRepresentation = ownedElement->selectByKind(TextualRepresentation)" => {
            Some(ElementDerivedDocumentationKind::TextualRepresentation)
        }
        _ => None,
    }
}

/// Extract only complete Namespace projections that can be read without inventing a relationship
/// identity: direct owning memberships already retain their member and membership kind, while
/// authored imports are canonical owned declarations. The adjacent imported-membership operation
/// and relationship-valued rules intentionally remain outside this closed matcher.
fn exact_namespace_derived_element(body: &str) -> Option<NamespaceDerivedElementKind> {
    match body {
        "ownedMember = ownedMembership->selectByKind(OwningMembership).ownedMemberElement" => {
            Some(NamespaceDerivedElementKind::OwnedMember)
        }
        "ownedImport = ownedRelationship->selectByKind(Import)" => {
            Some(NamespaceDerivedElementKind::OwnedImport)
        }
        _ => None,
    }
}

/// The direct NamespaceImport target is already an authored canonical reference. Do not admit the
/// MembershipImport sibling: its `importedMembership.memberElement` needs a first-class
/// membership relationship identity, which this publication deliberately does not synthesize.
fn exact_namespace_import_derived_element(body: &str) -> Option<NamespaceImportDerivedElementKind> {
    match body {
        "importedElement = importedNamespace" => {
            Some(NamespaceImportDerivedElementKind::ImportedElement)
        }
        _ => None,
    }
}

/// Match only complete named BindingConnector check bodies from the pinned XMI. Whitespace in
/// long OCL attributes is normalized, but every remaining token and the owning rule name must
/// match exactly; this is not a general BindingConnector OCL recognizer.
fn exact_binding_connector_check(
    constraint: &str,
    body: &str,
) -> Option<BindingConnectorCheckKind> {
    let body = body.split_whitespace().collect::<Vec<_>>().join(" ");
    let body = body.as_str();
    match (constraint, body) {
        ("checkFeatureValueBindingConnector", "not isDefault implies featureWithValue.ownedMember-> selectByKind(BindingConnector)->exists(b | b.relatedFeature->includes(featureWithValue) and b.relatedFeature->exists(f | f.chainingFeature = Sequence{value, value.result}) and if not isInitial then b.featuringType = featureWithValue.featuringType else b.featuringType->exists(t | t.oclIsKindOf(Feature) and t.oclAsType(Feature).chainingFeature = Sequence{ resolveGlobal('Base::things::that'). memberElement, resolveGlobal('Occurrences::Occurrence::startShot'). memberElement } ) endif)") => Some(BindingConnectorCheckKind::FeatureValue),
        ("checkExpressionResultBindingConnector", "ownedMembership.selectByKind(ResultExpressionMembership)-> forAll(mem | ownedFeature.selectByKind(BindingConnector)-> exists(binding | binding.relatedFeature->includes(result) and binding.relatedFeature->includes(mem.ownedResultExpression.result)))") => Some(BindingConnectorCheckKind::ExpressionResult),
        ("checkFunctionResultBindingConnector", "ownedMembership.selectByKind(ResultExpressionMembership)-> forAll(mem | ownedFeature.selectByKind(BindingConnector)-> exists(binding | binding.relatedFeature->includes(result) and binding.relatedFeature->includes(mem.ownedResultExpression.result)))") => Some(BindingConnectorCheckKind::FunctionResult),
        ("checkConstructorExpressionResultDefaultValueBindingConnector", "TBD") => Some(BindingConnectorCheckKind::ConstructorExpressionResultDefaultValueTbd),
        ("checkFeatureReferenceExpressionBindingConnector", "ownedMember->selectByKind(BindingConnector)->exists(b | b.relatedFeatures->includes(targetFeature) and b.relatedFeatures->includes(result))") => Some(BindingConnectorCheckKind::FeatureReferenceExpression),
        ("checkInvocationExpressionBehaviorBindingConnector", "not instantiatedType.oclIsKindOf(Function) and not (instantiatedType.oclIsKindOf(Feature) and instantiatedType.oclAsType(Feature).type->exists(oclIsKindOf(Function))) implies ownedFeature.selectByKind(BindingConnector)->exists( relatedFeature->includes(self) and relatedFeature->includes(result))") => Some(BindingConnectorCheckKind::InvocationExpressionBehavior),
        ("checkInvocationExpressionDefaultValueBindingConnector", "TBD") => Some(BindingConnectorCheckKind::InvocationExpressionDefaultValueTbd),
        ("checkAcceptActionUsageReceiverBindingConnector", "payloadArgument <> null and payloadArgument.oclIsKindOf(TriggerInvocationExpression) implies let invocation : Expression = payloadArgument.oclAsType(Expression) in parameter->size() >= 2 and invocation.parameter->size() >= 2 and ownedFeature->selectByKind(BindingConnector)->exists(b | b.relatedFeatures->includes(parameter->at(2)) and b.relatedFeatures->includes(invocation.parameter->at(2)))") => Some(BindingConnectorCheckKind::AcceptActionUsageReceiver),
        ("checkTransitionUsageSourceBindingConnector", "ownedMember->selectByKind(BindingConnector)->exists(b | b.relatedFeatures->includes(source) and b.relatedFeatures->includes(inputParameter(1)))") => Some(BindingConnectorCheckKind::TransitionUsageSource),
        ("checkTransitionUsageSuccessionBindingConnector", "ownedMember->selectByKind(BindingConnector)->exists(b | b.relatedFeatures->includes(succession) and b.relatedFeatures->includes(resolveGlobal( 'TransitionPerformances::TransitionPerformance::transitionLink')))") => Some(BindingConnectorCheckKind::TransitionUsageSuccession),
        ("checkSatisfyRequirementUsageBindingConnector", "ownedMember->selectByKind(BindingConnector)-> select(b | b.relatedElement->includes(subjectParameter) and b.relatedElement->exists(r | r <> subjectParameter))-> size() = 1") => Some(BindingConnectorCheckKind::SatisfyRequirementUsage),
        _ => None,
    }
}

fn exact_library_specialization_anchor(body: &str) -> Option<String> {
    exact_library_anchor(body, "specializesFromLibrary('")
}

fn exact_library_redefinition_anchor(body: &str) -> Option<String> {
    exact_library_anchor(body, "redefinesFromLibrary('")
}

fn exact_conditional_library_specialization(
    body: &str,
) -> Option<ConditionalLibrarySpecializationContract> {
    // XMI preserves formatting newlines in long OCL bodies. Whitespace is not a semantic token
    // in any closed shape below, so normalize it once before applying the complete-body matcher.
    // This does not turn a partial OCL match into a contract: every non-whitespace token and the
    // full anchor suffix still have to match exactly.
    let normalized = body.split_whitespace().collect::<Vec<_>>().join(" ");
    let body = normalized.as_str();
    if let Some(contract) = exact_membership_role_specialization(body) {
        return Some(contract);
    }
    if let Some(contract) = exact_feature_category_specialization(body) {
        return Some(contract);
    }
    if let Some(contract) = exact_connector_association_specialization(body) {
        return Some(contract);
    }
    let (predicate, owner_metaclasses, prefix) = match body {
        _ if body.starts_with("isIndividual implies specializesFromLibrary('") => (
            LibrarySpecializationPredicate::IsIndividual,
            Vec::new(),
            "isIndividual implies specializesFromLibrary('",
        ),
        _ if body.starts_with(
            "portionKind = PortionKind::snapshot implies specializesFromLibrary('",
        ) =>
        {
            (
                LibrarySpecializationPredicate::PortionKindSnapshot,
                Vec::new(),
                "portionKind = PortionKind::snapshot implies specializesFromLibrary('",
            )
        }
        _ if body.starts_with(
            "portionKind = PortionKind::timeslice implies specializesFromLibrary('",
        ) =>
        {
            (
                LibrarySpecializationPredicate::PortionKindTimeslice,
                Vec::new(),
                "portionKind = PortionKind::timeslice implies specializesFromLibrary('",
            )
        }
        _ if body.starts_with("ownedEndFeature->size() = 2 implies specializesFromLibrary('") => (
            LibrarySpecializationPredicate::OwnedEndFeatureCountIsTwo,
            Vec::new(),
            "ownedEndFeature->size() = 2 implies specializesFromLibrary('",
        ),
        _ if body.starts_with("connectorEnd->size() = 2 implies specializesFromLibrary('") => (
            LibrarySpecializationPredicate::ConnectorEndCountIsTwo,
            Vec::new(),
            "connectorEnd->size() = 2 implies specializesFromLibrary('",
        ),
        _ if body.starts_with("associationEnd->size() = 2 implies specializesFromLibrary('") => (
            LibrarySpecializationPredicate::AssociationEndCountIsTwo,
            Vec::new(),
            "associationEnd->size() = 2 implies specializesFromLibrary('",
        ),
        _ if body.starts_with("endFeature->size() = 2 implies specializesFromLibrary('") => (
            LibrarySpecializationPredicate::EndFeatureCountIsTwo,
            Vec::new(),
            "endFeature->size() = 2 implies specializesFromLibrary('",
        ),
        _ if body.starts_with("flowEnd->size() = 2 implies specializesFromLibrary('") => (
            LibrarySpecializationPredicate::FlowEndCountIsTwo,
            Vec::new(),
            "flowEnd->size() = 2 implies specializesFromLibrary('",
        ),
        _ if body.starts_with("ownedEndFeatures->notEmpty() implies specializesFromLibrary('") => (
            LibrarySpecializationPredicate::OwnedEndFeaturesNotEmpty,
            Vec::new(),
            "ownedEndFeatures->notEmpty() implies specializesFromLibrary('",
        ),
        _ if body.starts_with("isSubactionUsage() implies specializesFromLibrary('") => (
            LibrarySpecializationPredicate::IsSubactionUsage,
            Vec::new(),
            "isSubactionUsage() implies specializesFromLibrary('",
        ),
        _ if body.starts_with("not isTriggerAction() implies specializesFromLibrary('") => (
            LibrarySpecializationPredicate::IsNotTriggerAction,
            Vec::new(),
            "not isTriggerAction() implies specializesFromLibrary('",
        ),
        _ if body.starts_with(
            "isSubactionUsage() and not isTriggerAction() implies specializesFromLibrary('",
        ) =>
        {
            (
                LibrarySpecializationPredicate::IsSubactionUsageAndNotTriggerAction,
                Vec::new(),
                "isSubactionUsage() and not isTriggerAction() implies specializesFromLibrary('",
            )
        }
        _ if body.starts_with("isTriggerAction() implies specializesFromLibrary('") => (
            LibrarySpecializationPredicate::IsTriggerAction,
            Vec::new(),
            "isTriggerAction() implies specializesFromLibrary('",
        ),
        _ if body.starts_with("if isNegated then specializesFromLibrary('") => {
            return exact_polarity_branch_specialization(body);
        }
        _ if body.starts_with("if elseAction = null then specializesFromLibrary('") => {
            return exact_else_action_branch_specialization(body);
        }
        _ => {
            return exact_composite_owner_specialization(body)
                .or_else(|| exact_step_subperformance_specialization(body))
                .or_else(|| exact_owner_specialization(body));
        }
    };
    let anchor = exact_library_anchor(body, prefix)?;
    Some(ConditionalLibrarySpecializationContract {
        predicate,
        owner_metaclasses,
        anchor,
        true_anchor: None,
    })
}

/// Extract the complete Connector predicate whose `association` collection is exactly the direct
/// canonical typing relation filtered to `AssociationStructure`.
fn exact_connector_association_specialization(
    body: &str,
) -> Option<ConditionalLibrarySpecializationContract> {
    const ASSOCIATION_STRUCTURE: &str =
        "association->exists(oclIsKindOf(AssociationStructure)) implies specializesFromLibrary('";
    let anchor = exact_library_anchor(body, ASSOCIATION_STRUCTURE)?;
    Some(ConditionalLibrarySpecializationContract {
        predicate: LibrarySpecializationPredicate::ConnectorAssociationStructure,
        owner_metaclasses: Vec::new(),
        anchor,
        true_anchor: None,
    })
}

/// Extract only the complete Feature category predicates whose direct source facts are already
/// published by the lowering/resolution boundary. The remaining Feature category checks require
/// a first-class metaclass-subtyping fact and deliberately remain outside this closed extractor.
fn exact_feature_category_specialization(
    body: &str,
) -> Option<ConditionalLibrarySpecializationContract> {
    const DATA_VALUE: &str =
        "ownedTyping.type->exists(selectByKind(DataType)) implies specializesFromLibrary('";
    const END: &str = "isEnd and owningType <> null and (owningType.oclIsKindOf(Association) or owningType.oclIsKindOf(Connector)) implies specializesFromLibrary('";
    let predicate = if body.starts_with(DATA_VALUE) {
        LibrarySpecializationPredicate::OwnedTypingDataType
    } else if body.starts_with(END) {
        LibrarySpecializationPredicate::EndOwnedByAssociationOrConnector
    } else {
        return None;
    };
    let prefix = match predicate {
        LibrarySpecializationPredicate::OwnedTypingDataType => DATA_VALUE,
        LibrarySpecializationPredicate::EndOwnedByAssociationOrConnector => END,
        _ => unreachable!("feature category extractor owns only its two exact predicates"),
    };
    let anchor = exact_library_anchor(body, prefix)?;
    Some(ConditionalLibrarySpecializationContract {
        predicate,
        owner_metaclasses: Vec::new(),
        true_anchor: None,
        anchor,
    })
}

/// Extract only the five complete membership-role OCL bodies whose role is already an owned,
/// typed semantic fact. These are full-token patterns, not a rule-name convention or a general
/// OCL interpretation.
fn exact_membership_role_specialization(
    body: &str,
) -> Option<ConditionalLibrarySpecializationContract> {
    const FRAMED_CONCERN: &str = "owningFeatureMembership <> null and owningFeatureMembership.oclIsKindOf(FramedConcernMembership) implies specializesFromLibrary('";
    const STAKEHOLDER: &str = "owningFeatureMembership <> null and owningFeatureMembership.oclIsKindOf(StakeholderMembership) implies specializesFromLibrary('";
    const REQUIREMENT_VERIFICATION: &str = "owningFeatureMembership <> null and owningFeatureMembership.oclIsKindOf(RequirementVerificationMembership) implies specializesFromLibrary('";
    const REQUIREMENT_CONSTRAINT: &str = "owningFeatureMembership <> null and owningFeatureMembership.oclIsKindOf(RequirementConstraintMembership) implies if owningFeatureMembership.oclAsType(RequirementConstraintMembership).kind = RequirementConstraintKind::assumption then specializesFromLibrary('";
    const ACTOR: &str = "owningFeatureMembership <> null and owningFeatureMembership.oclIsKindOf(ActorMembership) implies if owningType.oclIsKindOf(RequirementDefinition) or owningType.oclIsKindOf(RequirementUsage) then specializesFromLibrary('";

    for (predicate, prefix) in [
        (
            LibrarySpecializationPredicate::FramedConcernMembership,
            FRAMED_CONCERN,
        ),
        (
            LibrarySpecializationPredicate::StakeholderMembership,
            STAKEHOLDER,
        ),
        (
            LibrarySpecializationPredicate::RequirementVerificationMembership,
            REQUIREMENT_VERIFICATION,
        ),
    ] {
        if let Some(anchor) = exact_library_anchor(body, prefix) {
            return Some(ConditionalLibrarySpecializationContract {
                predicate,
                owner_metaclasses: Vec::new(),
                true_anchor: None,
                anchor,
            });
        }
    }
    exact_if_then_else_specialization(
        body,
        LibrarySpecializationPredicate::RequirementConstraintMembershipKind,
        REQUIREMENT_CONSTRAINT,
    )
    .or_else(|| {
        // The pinned SysML XMI body for the ActorMembership rule is the exact two-arm OCL token
        // sequence but omits the terminal `endif`. Preserve that source form explicitly rather
        // than accepting a loose prefix or normalizing it into a different contract.
        exact_if_then_else_specialization_without_endif(
            body,
            LibrarySpecializationPredicate::ActorMembershipOwningRequirement,
            ACTOR,
        )
    })
}

fn exact_if_then_else_specialization(
    body: &str,
    predicate: LibrarySpecializationPredicate,
    prefix: &str,
) -> Option<ConditionalLibrarySpecializationContract> {
    const BETWEEN: &str = "') else specializesFromLibrary('";
    const SUFFIX: &str = "') endif";
    let rest = body.strip_prefix(prefix)?;
    let (true_anchor, rest) = rest.split_once(BETWEEN)?;
    let false_anchor = rest.strip_suffix(SUFFIX)?;
    let valid = |anchor: &str| {
        exact_library_anchor(
            &format!("specializesFromLibrary('{anchor}')"),
            "specializesFromLibrary('",
        )
        .is_some()
    };
    (valid(true_anchor) && valid(false_anchor)).then(|| ConditionalLibrarySpecializationContract {
        predicate,
        owner_metaclasses: Vec::new(),
        true_anchor: Some(true_anchor.to_string()),
        anchor: false_anchor.to_string(),
    })
}

fn exact_if_then_else_specialization_without_endif(
    body: &str,
    predicate: LibrarySpecializationPredicate,
    prefix: &str,
) -> Option<ConditionalLibrarySpecializationContract> {
    const BETWEEN: &str = "') else specializesFromLibrary('";
    let rest = body.strip_prefix(prefix)?;
    let (true_anchor, false_anchor) = rest.split_once(BETWEEN)?;
    let false_anchor = false_anchor.strip_suffix("')")?;
    let valid = |anchor: &str| {
        exact_library_anchor(
            &format!("specializesFromLibrary('{anchor}')"),
            "specializesFromLibrary('",
        )
        .is_some()
    };
    (valid(true_anchor) && valid(false_anchor)).then(|| ConditionalLibrarySpecializationContract {
        predicate,
        owner_metaclasses: Vec::new(),
        true_anchor: Some(true_anchor.to_string()),
        anchor: false_anchor.to_string(),
    })
}

fn exact_polarity_branch_specialization(
    body: &str,
) -> Option<ConditionalLibrarySpecializationContract> {
    const PREFIX: &str = "if isNegated then specializesFromLibrary('";
    exact_if_then_else_specialization(body, LibrarySpecializationPredicate::PolarityBranch, PREFIX)
}

/// Extract the complete `IfActionUsage` branch exactly as serialized in the pinned XMI. The
/// manifest predicate is positive (`hasElseAction`), while the OCL condition is its inverse: the
/// `then` arm is the no-else anchor and the `else` arm is the predicate-true anchor.
fn exact_else_action_branch_specialization(
    body: &str,
) -> Option<ConditionalLibrarySpecializationContract> {
    const PREFIX: &str = "if elseAction = null then specializesFromLibrary('";
    const BETWEEN: &str = "') else specializesFromLibrary('";
    const SUFFIX: &str = "') endif";
    let rest = body.strip_prefix(PREFIX)?;
    let (without_else_anchor, rest) = rest.split_once(BETWEEN)?;
    let with_else_anchor = rest.strip_suffix(SUFFIX)?;
    let valid = |anchor: &str| {
        exact_library_anchor(
            &format!("specializesFromLibrary('{anchor}')"),
            "specializesFromLibrary('",
        )
        .is_some()
    };
    (valid(without_else_anchor) && valid(with_else_anchor)).then(|| {
        ConditionalLibrarySpecializationContract {
            predicate: LibrarySpecializationPredicate::HasElseActionBranch,
            owner_metaclasses: Vec::new(),
            true_anchor: Some(with_else_anchor.to_string()),
            anchor: without_else_anchor.to_string(),
        }
    })
}

/// Extract one complete two-owner composite predicate. This is deliberately a structural match
/// over the full body, rather than an OCL parser: a third disjunct, a different property, or any
/// additional conjunct does not produce a contract.
fn exact_composite_owner_specialization(
    body: &str,
) -> Option<ConditionalLibrarySpecializationContract> {
    const PREFIX: &str = "isComposite and owningType <> null and (owningType.oclIsKindOf(";
    const BETWEEN: &str = ") or owningType.oclIsKindOf(";
    const SUFFIX: &str = ")) implies specializesFromLibrary('";
    let remainder = body.strip_prefix(PREFIX)?;
    let (first, remainder) = remainder.split_once(BETWEEN)?;
    let (second, _) = remainder.split_once(SUFFIX)?;
    let prefix_len = body.len() - remainder.len() + second.len() + SUFFIX.len();
    let anchor = exact_library_anchor(body, &body[..prefix_len])?;
    let is_metaclass = |value: &str| {
        !value.is_empty()
            && value
                .chars()
                .all(|character| character.is_ascii_alphanumeric() || character == '_')
    };
    (is_metaclass(first) && is_metaclass(second)).then(|| {
        ConditionalLibrarySpecializationContract {
            predicate: LibrarySpecializationPredicate::CompositeOwnedBy,
            owner_metaclasses: vec![first.to_string(), second.to_string()],
            anchor,
            true_anchor: None,
        }
    })
}

/// `Step::checkStepSubperformanceSpecialization` is the one pinned complete owner predicate
/// whose equivalent composite conjunct is spelled `self.isComposite` and follows the owner
/// disjunction. Keep its grammar exact and rule-scoped rather than treating `self` as a generic
/// OCL alias.
fn exact_step_subperformance_specialization(
    body: &str,
) -> Option<ConditionalLibrarySpecializationContract> {
    const PREFIX: &str = "owningType <> null and (owningType.oclIsKindOf(Behavior) or owningType.oclIsKindOf(Step)) and self.isComposite implies specializesFromLibrary('";
    let anchor = exact_library_anchor(body, PREFIX)?;
    Some(ConditionalLibrarySpecializationContract {
        predicate: LibrarySpecializationPredicate::CompositeOwnedBy,
        owner_metaclasses: vec!["Behavior".to_string(), "Step".to_string()],
        anchor,
        true_anchor: None,
    })
}

/// Extract the companion complete two-owner predicate without the `isComposite` conjunct.
fn exact_owner_specialization(body: &str) -> Option<ConditionalLibrarySpecializationContract> {
    const PREFIX: &str = "owningType <> null and (owningType.oclIsKindOf(";
    const BETWEEN: &str = ") or owningType.oclIsKindOf(";
    const SUFFIX: &str = ")) implies specializesFromLibrary('";
    let remainder = body.strip_prefix(PREFIX)?;
    let (first, remainder) = remainder.split_once(BETWEEN)?;
    let (second, _) = remainder.split_once(SUFFIX)?;
    let prefix_len = body.len() - remainder.len() + second.len() + SUFFIX.len();
    let anchor = exact_library_anchor(body, &body[..prefix_len])?;
    let is_metaclass = |value: &str| {
        !value.is_empty()
            && value
                .chars()
                .all(|character| character.is_ascii_alphanumeric() || character == '_')
    };
    (is_metaclass(first) && is_metaclass(second)).then(|| {
        ConditionalLibrarySpecializationContract {
            predicate: LibrarySpecializationPredicate::OwnedBy,
            owner_metaclasses: vec![first.to_string(), second.to_string()],
            anchor,
            true_anchor: None,
        }
    })
}

fn exact_library_anchor(body: &str, prefix: &str) -> Option<String> {
    let anchor = body.strip_prefix(prefix)?.strip_suffix("')")?;
    (!anchor.is_empty()
        && anchor.split("::").all(|part| {
            !part.is_empty()
                && part
                    .chars()
                    .all(|character| character.is_ascii_alphanumeric() || character == '_')
        }))
    .then(|| anchor.to_string())
}

/// Extract abstract-syntax clause headings from the official PDF through `pdftotext`. The PDF
/// itself is content-pinned before conversion, so the text is evidence from the exact published
/// artifact rather than an unversioned local rendering.
fn extract_pdf_clauses(
    path: &Path,
    definition: PinnedSpecification,
) -> Result<(String, BTreeMap<String, Vec<String>>), String> {
    let bytes =
        fs::read(path).map_err(|error| format!("{}: read failed: {error}", path.display()))?;
    let actual_digest = sha256_hex(&bytes);
    if actual_digest != definition.expected_pdf_sha256 {
        return Err(format!(
            "{}: SHA-256 mismatch for {} {} PDF: expected {}, got {}",
            path.display(),
            definition.name,
            definition.formal_document_id,
            definition.expected_pdf_sha256,
            actual_digest
        ));
    }
    let output = ProcessCommand::new("pdftotext")
        .args(["-layout"])
        .arg(path)
        .arg("-")
        .output()
        .map_err(|error| {
            format!(
                "{}: cannot run required local PDF text extractor `pdftotext`: {error}",
                path.display()
            )
        })?;
    if !output.status.success() {
        return Err(format!(
            "{}: pdftotext failed: {}",
            path.display(),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    let text = String::from_utf8(output.stdout).map_err(|error| {
        format!(
            "{}: pdftotext did not return UTF-8: {error}",
            path.display()
        )
    })?;
    let mut clauses = BTreeMap::<String, Vec<String>>::new();
    for line in text.lines() {
        let Some((clause, heading)) = parse_abstract_syntax_heading(line) else {
            continue;
        };
        clauses
            .entry(normalize_heading(heading))
            .or_default()
            .push(clause.to_string());
    }
    for values in clauses.values_mut() {
        values.sort();
        values.dedup();
    }
    if clauses.is_empty() {
        return Err(format!(
            "{}: no numbered 8.3 abstract-syntax headings were found in pinned {} PDF",
            path.display(),
            definition.name
        ));
    }
    Ok((actual_digest, clauses))
}

fn parse_abstract_syntax_heading(line: &str) -> Option<(&str, &str)> {
    let trimmed = line.trim();
    let split = trimmed.find(char::is_whitespace)?;
    let (clause, heading) = trimmed.split_at(split);
    let heading = heading.trim();
    let is_numbered = clause.starts_with("8.3.")
        && clause
            .split('.')
            .all(|part| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_digit()));
    (is_numbered && !heading.is_empty()).then_some((clause, heading))
}

fn normalize_heading(heading: &str) -> String {
    heading
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .flat_map(char::to_lowercase)
        .collect()
}

fn reconcile_constraint_clauses(
    constraints: &mut [ConstraintManifestEntry],
    clauses: &BTreeMap<String, Vec<String>>,
    definition: PinnedSpecification,
    pdf_path: &Path,
) -> Result<(), String> {
    for entry in constraints {
        let heading = normalize_heading(&entry.metaclass);
        let Some(matches) = clauses.get(&heading) else {
            return Err(format!(
                "{}: no exact 8.3 abstract-syntax heading for {} metaclass {:?}; refusing to claim a normative clause",
                pdf_path.display(),
                definition.name,
                entry.metaclass
            ));
        };
        let [clause] = matches.as_slice() else {
            return Err(format!(
                "{}: ambiguous 8.3 abstract-syntax headings {:?} for {} metaclass {:?}; refusing to claim a normative clause",
                pdf_path.display(),
                matches,
                definition.name,
                entry.metaclass
            ));
        };
        entry.clause = clause.clone();
        entry.rule_id = format!(
            "{}-{}:{}:{}",
            definition.slug, definition.version, entry.clause, entry.constraint
        );
    }
    Ok(())
}

fn scope_for_start(
    start: &BytesStart<'_>,
    stack: &[ElementFrame],
) -> Result<Option<Scope>, String> {
    let attributes = attributes(start)?;
    let qualified_element_name = start.name();
    let element_name = local_name(qualified_element_name.as_ref());
    let type_name = attributes
        .get("type")
        .map(String::as_str)
        .unwrap_or_default();
    let name = attributes.get("name").filter(|name| !name.is_empty());
    let package_count = stack
        .iter()
        .filter(|frame| matches!(frame.scope, Some(Scope::RootPackage | Scope::Package(_))))
        .count();
    if let (true, Some(name)) = (
        element_name == "Package" || type_name.ends_with(":Package"),
        name,
    ) {
        return Ok(Some(if package_count == 0 {
            Scope::RootPackage
        } else {
            Scope::Package(name.clone())
        }));
    }
    if let (true, Some(name)) = (type_name.ends_with(":Class"), name) {
        return Ok(Some(Scope::Metaclass(name.clone())));
    }
    Ok(None)
}

fn attributes(
    start: &BytesStart<'_>,
) -> Result<std::collections::BTreeMap<String, String>, String> {
    let mut values = std::collections::BTreeMap::new();
    for attribute in start.attributes() {
        let attribute = attribute.map_err(|error| format!("invalid XML attribute: {error}"))?;
        let key = local_name(attribute.key.as_ref()).to_string();
        let value = attribute
            .normalized_value(XmlVersion::Implicit1_0)
            .map_err(|error| format!("invalid XML attribute value: {error}"))?
            .into_owned();
        if values.insert(key.clone(), value).is_some() {
            return Err(format!("duplicate XML attribute local name {key:?}"));
        }
    }
    Ok(values)
}

fn local_name(name: &[u8]) -> &str {
    std::str::from_utf8(name)
        .unwrap_or("")
        .rsplit(':')
        .next()
        .unwrap_or("")
}

fn render_manifest(
    manifest: &ConstraintManifest,
    format: ManifestFormat,
) -> Result<String, String> {
    match format {
        ManifestFormat::Toml => toml::to_string_pretty(manifest)
            .map_err(|error| format!("manifest TOML serialization failed: {error}")),
        ManifestFormat::Json => serde_json::to_string_pretty(manifest)
            .map(|text| format!("{text}\n"))
            .map_err(|error| format!("manifest JSON serialization failed: {error}")),
    }
}

fn audit_manifest(
    kerml: &Path,
    sysml: &Path,
    kerml_pdf: &Path,
    sysml_pdf: &Path,
    manifest_path: &Path,
    format: ManifestFormat,
) -> Result<(), String> {
    let expected = extract_manifest(kerml, sysml, kerml_pdf, sysml_pdf)?;
    let text = fs::read_to_string(manifest_path)
        .map_err(|error| format!("{}: read failed: {error}", manifest_path.display()))?;
    let actual: ConstraintManifest = match format {
        ManifestFormat::Toml => toml::from_str(&text).map_err(|error| {
            format!(
                "{}: invalid manifest TOML: {error}",
                manifest_path.display()
            )
        })?,
        ManifestFormat::Json => serde_json::from_str(&text).map_err(|error| {
            format!(
                "{}: invalid manifest JSON: {error}",
                manifest_path.display()
            )
        })?,
    };
    if actual != expected {
        return Err(format!(
            "{}: manifest facts differ from the supplied pinned XMI artifacts; run refresh",
            manifest_path.display()
        ));
    }
    let canonical = render_manifest(&expected, format)?;
    if text != canonical {
        return Err(format!(
            "{}: manifest is not in canonical deterministic {} form; run refresh",
            manifest_path.display(),
            match format {
                ManifestFormat::Toml => "TOML",
                ManifestFormat::Json => "JSON",
            }
        ));
    }
    Ok(())
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SPEC: PinnedSpecification = PinnedSpecification {
        specification_id: spec42_constraint_manifest::SpecificationId::KerML10,
        rule_id_prefix: "testml-1.0",
        name: "TestML",
        slug: "testml",
        version: "1.0",
        formal_document_id: "formal/test",
        xmi_file_id: "ptc/test",
        expected_sha256: "not used by XML parser tests",
        expected_pdf_sha256: "not used by XML parser tests",
    };

    const XMI: &str = r#"<?xml version="1.0"?>
<xmi:XMI xmlns:xmi="urn:xmi" xmlns:uml="urn:uml">
  <uml:Package xmi:id="Top" name="TestML">
    <packagedElement xmi:type="uml:Package" name="Core">
      <packagedElement xmi:type="uml:Package" name="Types">
        <packagedElement xmi:type="uml:Class" name="Element">
          <ownedRule xmi:type="uml:Constraint" name="deriveOwner" />
          <ownedRule xmi:type="uml:Constraint" name="checkOwner" />
          <ownedRule xmi:type="uml:Constraint" name="validateOwner" />
          <ownedRule xmi:type="uml:Constraint" name="helper" />
        </packagedElement>
      </packagedElement>
    </packagedElement>
  </uml:Package>
</xmi:XMI>"#;

    #[test]
    fn extracts_closed_constraint_families_from_xml_structure() {
        let entries = extract_constraints(XMI.as_bytes()).unwrap();
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0].package, "Core::Types");
        assert_eq!(entries[0].metaclass, "Element");
        assert_eq!(entries[0].family, ConstraintFamily::Derive);
        assert_eq!(entries[2].family, ConstraintFamily::Validate);
        assert!(entries.iter().all(|entry| entry.rule_id.is_empty()));
    }

    #[test]
    fn malformed_xml_is_rejected() {
        let error = extract_constraints(b"<xmi:XMI><ownedRule").unwrap_err();
        assert!(error.contains("invalid XML"));
    }

    #[test]
    fn rendering_is_deterministic_in_both_formats() {
        let manifest = ConstraintManifest {
            schema_version: SCHEMA_VERSION,
            specifications: vec![SpecificationManifest {
                name: "TestML".to_string(),
                version: "1.0".to_string(),
                formal_document_id: "formal/test".to_string(),
                xmi_file_id: "ptc/test".to_string(),
                xmi_sha256: "digest".to_string(),
                pdf_sha256: "pdf-digest".to_string(),
                constraints: extract_constraints(XMI.as_bytes()).unwrap(),
            }],
        };
        for format in [ManifestFormat::Toml, ManifestFormat::Json] {
            assert_eq!(
                render_manifest(&manifest, format).unwrap(),
                render_manifest(&manifest, format).unwrap()
            );
        }
    }

    #[test]
    fn refresh_rejects_an_unpinned_xmi_digest_before_extraction() {
        let input = tempfile::NamedTempFile::new().unwrap();
        fs::write(input.path(), XMI).unwrap();
        let error = extract_specification(
            PinnedSpecification {
                expected_sha256: "0000000000000000000000000000000000000000000000000000000000000000",
                ..TEST_SPEC
            },
            input.path(),
            input.path(),
        )
        .unwrap_err();
        assert!(error.contains("SHA-256 mismatch"));
    }

    #[test]
    fn reconciliation_assigns_the_exact_heading_clause() {
        let mut constraints = extract_constraints(XMI.as_bytes()).unwrap();
        let clauses = BTreeMap::from([(String::from("element"), vec![String::from("8.3.7.4")])]);
        reconcile_constraint_clauses(
            &mut constraints,
            &clauses,
            TEST_SPEC,
            Path::new("TestML.pdf"),
        )
        .unwrap();
        assert!(constraints.iter().all(|entry| entry.clause == "8.3.7.4"));
        assert_eq!(constraints[1].rule_id, "testml-1.0:8.3.7.4:checkOwner");
    }

    #[test]
    fn clause_parser_accepts_only_numbered_abstract_syntax_headings() {
        assert_eq!(
            parse_abstract_syntax_heading("  8.3.4.6.2 Behavior  "),
            Some(("8.3.4.6.2", "Behavior"))
        );
        assert_eq!(parse_abstract_syntax_heading("8.4.1 Behavior"), None);
        assert_eq!(parse_abstract_syntax_heading("8.3.4.6.2"), None);
    }

    #[test]
    fn library_specialization_contract_requires_the_complete_unconditional_ocl_body() {
        assert_eq!(
            exact_library_specialization_anchor("specializesFromLibrary('Parts::Part')"),
            Some("Parts::Part".to_string())
        );
        assert_eq!(
            exact_library_specialization_anchor(
                "isComposite implies specializesFromLibrary('Parts::Part')"
            ),
            None
        );
    }

    #[test]
    fn conditional_library_specialization_contracts_are_closed_exact_ocl_shapes() {
        assert_eq!(
            exact_conditional_library_specialization(
                "isIndividual implies specializesFromLibrary('Occurrences::Life')"
            ),
            Some(ConditionalLibrarySpecializationContract {
                predicate: LibrarySpecializationPredicate::IsIndividual,
                owner_metaclasses: Vec::new(),
                anchor: "Occurrences::Life".to_string(),
                true_anchor: None,
            })
        );
        assert_eq!(
            exact_conditional_library_specialization(
                "ownedEndFeature->size() = 2 implies specializesFromLibrary('Connections::BinaryConnections')"
            ),
            Some(ConditionalLibrarySpecializationContract {
                predicate: LibrarySpecializationPredicate::OwnedEndFeatureCountIsTwo,
                owner_metaclasses: Vec::new(),
                anchor: "Connections::BinaryConnections".to_string(),
                true_anchor: None,
            })
        );
        assert_eq!(
            exact_conditional_library_specialization(
                "ownedTyping.type->exists(selectByKind(DataType)) implies specializesFromLibrary('Base::dataValues')"
            ),
            Some(ConditionalLibrarySpecializationContract {
                predicate: LibrarySpecializationPredicate::OwnedTypingDataType,
                owner_metaclasses: Vec::new(),
                anchor: "Base::dataValues".to_string(),
                true_anchor: None,
            })
        );
        assert_eq!(
            exact_conditional_library_specialization(
                "isEnd and owningType <> null and (owningType.oclIsKindOf(Association) or owningType.oclIsKindOf(Connector)) implies specializesFromLibrary('Links::Link::participant')"
            ),
            Some(ConditionalLibrarySpecializationContract {
                predicate: LibrarySpecializationPredicate::EndOwnedByAssociationOrConnector,
                owner_metaclasses: Vec::new(),
                anchor: "Links::Link::participant".to_string(),
                true_anchor: None,
            })
        );
        assert_eq!(
            exact_conditional_library_specialization(
                "association->exists(oclIsKindOf(AssociationStructure)) implies specializesFromLibrary('Objects::linkObjects')"
            ),
            Some(ConditionalLibrarySpecializationContract {
                predicate: LibrarySpecializationPredicate::ConnectorAssociationStructure,
                owner_metaclasses: Vec::new(),
                anchor: "Objects::linkObjects".to_string(),
                true_anchor: None,
            })
        );
        assert!(exact_conditional_library_specialization(
            "connectorEnds->size() = 2 and association->exists(oclIsKindOf(AssociationStructure)) implies specializesFromLibrary('Objects::binaryLinkObjects')"
        )
        .is_none());
        assert_eq!(
            exact_conditional_library_specialization(
                "owningType <> null and (owningType.oclIsKindOf(Behavior) or owningType.oclIsKindOf(Step)) and self.isComposite implies specializesFromLibrary('Performances::Performance::subperformance')"
            ),
            Some(ConditionalLibrarySpecializationContract {
                predicate: LibrarySpecializationPredicate::CompositeOwnedBy,
                owner_metaclasses: vec!["Behavior".to_string(), "Step".to_string()],
                anchor: "Performances::Performance::subperformance".to_string(),
                true_anchor: None,
            })
        );
        assert_eq!(
            exact_conditional_library_specialization(
                "flowEnd->size() = 2 implies specializesFromLibrary('Flows::Message')"
            ),
            Some(ConditionalLibrarySpecializationContract {
                predicate: LibrarySpecializationPredicate::FlowEndCountIsTwo,
                owner_metaclasses: Vec::new(),
                anchor: "Flows::Message".to_string(),
                true_anchor: None,
            })
        );
        assert_eq!(
            exact_conditional_library_specialization(
                "ownedEndFeatures->notEmpty() implies specializesFromLibrary('Flows::flows')"
            ),
            Some(ConditionalLibrarySpecializationContract {
                predicate: LibrarySpecializationPredicate::OwnedEndFeaturesNotEmpty,
                owner_metaclasses: Vec::new(),
                anchor: "Flows::flows".to_string(),
                true_anchor: None,
            })
        );
        assert_eq!(
            exact_conditional_library_specialization(
                "portionKind = PortionKind::snapshot implies specializesFromLibrary('Occurrences::Occurrence::snapshots')"
            ),
            Some(ConditionalLibrarySpecializationContract {
                predicate: LibrarySpecializationPredicate::PortionKindSnapshot,
                owner_metaclasses: Vec::new(),
                anchor: "Occurrences::Occurrence::snapshots".to_string(),
                true_anchor: None,
            })
        );
        assert_eq!(
            exact_conditional_library_specialization(
                "isComposite and owningType <> null and (owningType.oclIsKindOf(PartDefinition) or owningType.oclIsKindOf(PartUsage)) implies specializesFromLibrary('Parts::Part::ownedActions')"
            ),
            Some(ConditionalLibrarySpecializationContract {
                predicate: LibrarySpecializationPredicate::CompositeOwnedBy,
                owner_metaclasses: vec!["PartDefinition".to_string(), "PartUsage".to_string()],
                anchor: "Parts::Part::ownedActions".to_string(),
                true_anchor: None,
            })
        );
        assert_eq!(
            exact_conditional_library_specialization(
                "if isNegated then specializesFromLibrary('Requirements::notSatisfiedRequirementChecks') else specializesFromLibrary('Requirements::satisfiedRequirementChecks') endif"
            ),
            Some(ConditionalLibrarySpecializationContract {
                predicate: LibrarySpecializationPredicate::PolarityBranch,
                owner_metaclasses: Vec::new(),
                true_anchor: Some("Requirements::notSatisfiedRequirementChecks".to_string()),
                anchor: "Requirements::satisfiedRequirementChecks".to_string(),
            })
        );
        assert_eq!(
            exact_conditional_library_specialization(
                "if elseAction = null then specializesFromLibrary('Actions::ifThenActions') else specializesFromLibrary('Actions::ifThenElseActions') endif"
            ),
            Some(ConditionalLibrarySpecializationContract {
                predicate: LibrarySpecializationPredicate::HasElseActionBranch,
                owner_metaclasses: Vec::new(),
                true_anchor: Some("Actions::ifThenElseActions".to_string()),
                anchor: "Actions::ifThenActions".to_string(),
            })
        );
        assert_eq!(
            exact_conditional_library_specialization(
                "owningFeatureMembership <> null and owningFeatureMembership.oclIsKindOf(RequirementConstraintMembership) implies if owningFeatureMembership.oclAsType(RequirementConstraintMembership).kind = RequirementConstraintKind::assumption then specializesFromLibrary('Requirements::RequirementCheck::assumptions') else specializesFromLibrary('Requirements::RequirementCheck::constraints') endif"
            ),
            Some(ConditionalLibrarySpecializationContract {
                predicate: LibrarySpecializationPredicate::RequirementConstraintMembershipKind,
                owner_metaclasses: Vec::new(),
                true_anchor: Some("Requirements::RequirementCheck::assumptions".to_string()),
                anchor: "Requirements::RequirementCheck::constraints".to_string(),
            })
        );
        assert_eq!(
            exact_conditional_library_specialization(
                "owningFeatureMembership <> null and owningFeatureMembership.oclIsKindOf(ActorMembership) implies if owningType.oclIsKindOf(RequirementDefinition) or owningType.oclIsKindOf(RequirementUsage) then specializesFromLibrary('Requirements::RequirementCheck::actors') else specializesFromLibrary('Cases::Case::actors')"
            ),
            Some(ConditionalLibrarySpecializationContract {
                predicate: LibrarySpecializationPredicate::ActorMembershipOwningRequirement,
                owner_metaclasses: Vec::new(),
                true_anchor: Some("Requirements::RequirementCheck::actors".to_string()),
                anchor: "Cases::Case::actors".to_string(),
            })
        );
        assert_eq!(
            exact_conditional_library_specialization(
                "owningFeatureMembership <> null and owningFeatureMembership.oclIsKindOf(FramedConcernMembership) implies specializesFromLibrary('Requirements::RequirementCheck::concerns')"
            ),
            Some(ConditionalLibrarySpecializationContract {
                predicate: LibrarySpecializationPredicate::FramedConcernMembership,
                owner_metaclasses: Vec::new(),
                true_anchor: None,
                anchor: "Requirements::RequirementCheck::concerns".to_string(),
            })
        );
        assert_eq!(
            exact_conditional_library_specialization(
                "not isTriggerAction() implies specializesFromLibrary('Actions::acceptActions')"
            ),
            Some(ConditionalLibrarySpecializationContract {
                predicate: LibrarySpecializationPredicate::IsNotTriggerAction,
                owner_metaclasses: Vec::new(),
                true_anchor: None,
                anchor: "Actions::acceptActions".to_string(),
            })
        );
        assert_eq!(
            exact_conditional_library_specialization(
                "isSubactionUsage() and not isTriggerAction() implies specializesFromLibrary('Actions::Action::acceptSubactions')"
            ),
            Some(ConditionalLibrarySpecializationContract {
                predicate: LibrarySpecializationPredicate::IsSubactionUsageAndNotTriggerAction,
                owner_metaclasses: Vec::new(),
                true_anchor: None,
                anchor: "Actions::Action::acceptSubactions".to_string(),
            })
        );
        assert_eq!(
            exact_conditional_library_specialization(
                "isTriggerAction() implies specializesFromLibrary('Actions::TransitionAction::accepter')"
            ),
            Some(ConditionalLibrarySpecializationContract {
                predicate: LibrarySpecializationPredicate::IsTriggerAction,
                owner_metaclasses: Vec::new(),
                true_anchor: None,
                anchor: "Actions::TransitionAction::accepter".to_string(),
            })
        );
        assert!(exact_conditional_library_specialization(
            "isIndividual and isComposite implies specializesFromLibrary('Occurrences::Life')"
        )
        .is_none());
        assert!(exact_conditional_library_specialization(
            "flowEnd->size() = 2 implies specializesFromLibrary('Flows::Message') and true"
        )
        .is_none());
        assert!(exact_conditional_library_specialization(
            "if elseAction = null then specializesFromLibrary('Actions::ifThenActions') else specializesFromLibrary('Actions::ifThenElseActions')"
        )
        .is_none());
        assert!(exact_conditional_library_specialization(
            "if isNegated then specializesFromLibrary('Requirements::notSatisfiedRequirementChecks') else specializesFromLibrary('Requirements::satisfiedRequirementChecks')"
        )
        .is_none());
        assert!(exact_conditional_library_specialization(
            "owningFeatureMembership.oclIsKindOf(ActorMembership) implies if owningType.oclIsKindOf(RequirementDefinition) or owningType.oclIsKindOf(RequirementUsage) then specializesFromLibrary('Requirements::RequirementCheck::actors') else specializesFromLibrary('Cases::Case::actors')"
        )
        .is_none());
        assert!(exact_conditional_library_specialization(
            "isSubactionUsage() and isTriggerAction() implies specializesFromLibrary('Actions::Action::acceptSubactions')"
        )
        .is_none());
    }

    #[test]
    fn library_redefinition_contract_requires_the_complete_unconditional_ocl_body() {
        assert_eq!(
            exact_library_redefinition_anchor(
                "redefinesFromLibrary('Transfers::Transfer::payload')"
            ),
            Some("Transfers::Transfer::payload".to_string())
        );
        assert_eq!(
            exact_library_redefinition_anchor(
                "payload &lt;&gt; null and payload.redefinesFromLibrary('Transfers::Transfer::payload')"
            ),
            None
        );
    }

    #[test]
    fn feature_relationship_derivation_contracts_are_closed_exact_ocl_bodies() {
        assert_eq!(
            exact_feature_derived_relationship(
                "ownedFeatureChaining = ownedRelationship->selectByKind(FeatureChaining)"
            ),
            Some(FeatureDerivedRelationshipKind::OwnedFeatureChaining)
        );
        assert_eq!(
            exact_feature_derived_relationship(
                "ownedTypeFeaturing = ownedRelationship->selectByKind(TypeFeaturing)-> select(tf | tf.featureOfType = self)"
            ),
            Some(FeatureDerivedRelationshipKind::OwnedTypeFeaturing)
        );
        assert!(exact_feature_derived_relationship(
            "ownedTyping = ownedGeneralization->selectByKind(FeatureTyping)->asOrderedSet()"
        )
        .is_none());
        assert!(
            exact_feature_derived_relationship("type = closure(typingFeatures()).typing.type")
                .is_none()
        );
    }

    #[test]
    fn type_relationship_derivation_contracts_are_closed_exact_ocl_bodies() {
        assert_eq!(
            exact_type_derived_relationship("ownedRelationship->selectByKind(Intersecting)"),
            Some(TypeDerivedRelationshipKind::OwnedIntersecting)
        );
        assert_eq!(
            exact_type_derived_relationship(
                "ownedSpecialization = ownedRelationship->selectByKind(Specialization)-> select(s | s.special = self) "
            ),
            Some(TypeDerivedRelationshipKind::OwnedSpecialization)
        );
        assert_eq!(
            exact_type_derived_relationship("unioningType = ownedUnioning.unioningType"),
            Some(TypeDerivedRelationshipKind::UnioningType)
        );
        assert!(exact_type_derived_relationship(
            "unioningType = ownedUnioning->collect(unioningType)"
        )
        .is_none());
        assert!(exact_type_derived_relationship(
            "multiplicity = ownedSpecialization.general.multiplicity"
        )
        .is_none());
    }

    #[test]
    fn type_element_derivation_contracts_are_closed_complete_equations() {
        assert_eq!(
            exact_type_derived_element("ownedFeature = ownedFeatureMembership.ownedMemberFeature"),
            Some(TypeDerivedElementKind::OwnedFeature)
        );
        assert_eq!(
            exact_type_derived_element("ownedEndFeature = ownedFeature->select(isEnd)"),
            Some(TypeDerivedElementKind::OwnedEndFeature)
        );
        assert!(
            exact_type_derived_element("feature = featureMembership.ownedMemberFeature").is_none()
        );
        assert!(exact_type_derived_element(
            "ownedFeature = ownedFeatureMembership->collect(ownedMemberFeature)"
        )
        .is_none());
        assert!(exact_type_derived_element("ownedEndFeature = feature->select(isEnd)").is_none());
    }

    #[test]
    fn type_fact_derivation_contracts_require_the_complete_pinned_body() {
        assert_eq!(
            exact_type_derived_fact(
                "deriveTypeOwnedFeatureMembership",
                "ownedFeatureMembership = ownedRelationship->selectByKind(FeatureMembership)",
            ),
            Some(TypeDerivedFactKind::OwnedFeatureMembership)
        );
        assert_eq!(
            exact_type_derived_fact(
                "deriveTypeMultiplicity",
                "multiplicity = let ownedMultiplicities: Sequence(Multiplicity) = ownedMember->selectByKind(Multiplicity) in if ownedMultiplicities->isEmpty() then null else ownedMultiplicities->first() endif",
            ),
            Some(TypeDerivedFactKind::Multiplicity)
        );
        assert!(exact_type_derived_fact(
            "deriveTypeMultiplicity",
            "multiplicity = ownedMember->selectByKind(Multiplicity)->first()",
        )
        .is_none());
        assert!(exact_type_derived_fact(
            "deriveTypeFeature",
            "feature = featureMembership->select(memberFeature)",
        )
        .is_none());
    }

    #[test]
    fn action_derivation_contracts_require_the_complete_pinned_body() {
        assert_eq!(
            exact_action_derived_fact(
                "deriveActionDefinitionAction",
                "action = usage->selectByKind(ActionUsage)",
            ),
            Some(ActionDerivedFactKind::ActionDefinitionAction)
        );
        assert_eq!(
            exact_action_derived_fact(
                "deriveSendActionUsagePayloadArgument",
                "payloadArgument = argument(1)",
            ),
            Some(ActionDerivedFactKind::SendPayloadArgument)
        );
        assert!(exact_action_derived_fact(
            "deriveSendActionUsagePayloadArgument",
            "payloadArgument = argument(2)",
        )
        .is_none());
        assert!(exact_action_derived_fact(
            "deriveActionDefinitionAction",
            "action = ownedUsage->selectByKind(ActionUsage)",
        )
        .is_none());
    }

    #[test]
    fn requirement_derivation_contracts_are_closed_pinned_body_fingerprints() {
        assert_eq!(
            exact_requirement_derived_fact(
                "deriveRequirementDefinitionActorParameter",
                "actorParameter = featureMembership-> selectByKind(ActorMembership). ownedActorParameter",
            ),
            Some(RequirementDerivedFactKind::DefinitionActorParameter)
        );
        assert_eq!(
            exact_requirement_derived_fact(
                "deriveRequirementUsageText",
                "text = documentation.body",
            ),
            Some(RequirementDerivedFactKind::UsageText)
        );
        assert!(exact_requirement_derived_fact(
            "deriveRequirementUsageActorParameter",
            "actorParameter = featureMembership->selectByKind(ActorMembership).ownedActorParameter",
        )
        .is_none());
        assert!(exact_requirement_derived_fact(
            "deriveRequirementDefinitionStakeholderParameter",
            "stakeholderParameter = featureMembership-> selectByKind(StakholderMembership). ownedStakeholderParameter",
        )
        .is_none());
    }

    #[test]
    fn type_featuring_check_contract_is_the_complete_feature_membership_implication() {
        assert_eq!(
            exact_type_featuring_check(
                "checkFeatureFeatureMembershipTypeFeaturing",
                "owningFeatureMembership <> null implies featuringTypes->exists(t | isFeaturingType(t))",
            ),
            Some(TypeFeaturingCheckKind::FeatureFeatureMembership)
        );
        assert!(exact_type_featuring_check(
            "checkFeatureFeatureMembershipTypeFeaturing",
            "owningFeatureMembership <> null implies featuringType->exists(t | isFeaturingType(t))",
        )
        .is_none());
        assert!(exact_type_featuring_check(
            "checkFeatureOwnedCrossFeatureTypeFeaturing",
            "owningFeatureMembership <> null implies featuringTypes->exists(t | isFeaturingType(t))",
        )
        .is_none());
    }

    #[test]
    fn element_owner_derivation_contract_is_the_complete_exact_equation() {
        assert_eq!(
            exact_element_derived_owner("owner = owningRelationship.owningRelatedElement"),
            Some(ElementDerivedOwnerKind::Owner)
        );
        assert!(exact_element_derived_owner("owner = owningRelationship.relatedElement").is_none());
        assert!(
            exact_element_derived_owner("owner = owningRelationship.owningRelatedElement\n")
                .is_none()
        );
    }

    #[test]
    fn element_documentation_derivation_contracts_are_closed_exact_equations() {
        assert_eq!(
            exact_element_derived_documentation(
                "documentation = ownedElement->selectByKind(Documentation)"
            ),
            Some(ElementDerivedDocumentationKind::Documentation)
        );
        assert_eq!(
            exact_element_derived_documentation(
                "textualRepresentation = ownedElement->selectByKind(TextualRepresentation)"
            ),
            Some(ElementDerivedDocumentationKind::TextualRepresentation)
        );
        assert!(exact_element_derived_documentation(
            "documentation = ownedElement->selectByKind(Documentation)->asOrderedSet()"
        )
        .is_none());
        assert!(exact_element_derived_documentation(
            "ownedAnnotation = ownedRelationship->selectByKind(Annotation)"
        )
        .is_none());
    }

    #[test]
    fn namespace_element_derivation_contracts_are_closed_exact_equations() {
        assert_eq!(
            exact_namespace_derived_element(
                "ownedMember = ownedMembership->selectByKind(OwningMembership).ownedMemberElement"
            ),
            Some(NamespaceDerivedElementKind::OwnedMember)
        );
        assert_eq!(
            exact_namespace_derived_element(
                "ownedImport = ownedRelationship->selectByKind(Import)"
            ),
            Some(NamespaceDerivedElementKind::OwnedImport)
        );
        assert!(exact_namespace_derived_element(
            "ownedMembership = ownedRelationship->selectByKind(Membership)"
        )
        .is_none());
        assert!(exact_namespace_derived_element(
            "ownedMember = ownedMembership->selectByKind(OwningMembership).memberElement"
        )
        .is_none());
    }

    #[test]
    fn namespace_import_element_derivation_contract_is_the_complete_exact_equation() {
        assert_eq!(
            exact_namespace_import_derived_element("importedElement = importedNamespace"),
            Some(NamespaceImportDerivedElementKind::ImportedElement)
        );
        assert!(exact_namespace_import_derived_element(
            "importedElement = importedMembership.memberElement"
        )
        .is_none());
        assert!(
            exact_namespace_import_derived_element("importedElement = importedNamespace\n")
                .is_none()
        );
    }

    #[test]
    fn type_featuring_candidates_are_not_promoted_without_an_exact_relationship_contract() {
        // These are the authoritative type-featuring candidate bodies from the pinned KerML and
        // SysML XMI. None constructs a TypeFeaturing relationship: they respectively derive a
        // collection through `let`/`if`, select already-owned relationships, or constrain a
        // conditional/equality result. Treating any as an implied edge would interpret general
        // OCL instead of extracting an exact contract.
        const TYPE_FEATURING_XMI: &str = r#"<?xml version="1.0"?>
<xmi:XMI xmlns:xmi="urn:xmi" xmlns:uml="urn:uml">
  <uml:Package xmi:id="Top" name="KerML">
    <packagedElement xmi:type="uml:Package" name="Core">
      <packagedElement xmi:type="uml:Package" name="Features">
        <packagedElement xmi:type="uml:Class" name="Feature">
          <ownedRule xmi:type="uml:Constraint" name="deriveFeatureFeaturingType">
            <specification xmi:type="uml:OpaqueExpression" body="featuringType = let featuringTypes : OrderedSet(Type) = featuring.type-&gt;asOrderedSet() in if chainingFeature-&gt;isEmpty() then featuringTypes else featuringTypes-&gt;union(chainingFeature-&gt;first().featuringType)-&gt;asOrderedSet() endif" language="OCL2.0"/>
          </ownedRule>
          <ownedRule xmi:type="uml:Constraint" name="deriveFeatureOwnedTypeFeaturing">
            <specification xmi:type="uml:OpaqueExpression" body="ownedTypeFeaturing = ownedRelationship-&gt;selectByKind(TypeFeaturing)-&gt;select(tf | tf.featureOfType = self)" language="OCL2.0"/>
          </ownedRule>
          <ownedRule xmi:type="uml:Constraint" name="checkFeatureFeatureMembershipTypeFeaturing">
            <specification xmi:type="uml:OpaqueExpression" body="owningFeatureMembership &lt;&gt; null implies featuringTypes-&gt;exists(t | isFeaturingType(t))" language="OCL2.0"/>
          </ownedRule>
        </packagedElement>
      </packagedElement>
    </packagedElement>
  </uml:Package>
</xmi:XMI>"#;

        let entries = extract_constraints(TYPE_FEATURING_XMI.as_bytes()).unwrap();
        assert_eq!(entries.len(), 3);
        assert!(entries.iter().all(|entry| {
            entry.specializes_from_library.is_none() && entry.redefines_from_library.is_none()
        }));
    }

    #[test]
    fn binding_connector_contracts_are_extracted_only_from_exact_pinned_bodies() {
        // Authoritative named binding-connector checks from the pinned KerML/SysML XMI. Each is
        // a validation over an existing connector (`exists`/`forAll`), a conditional rule, or a
        // deliberate TBD. The extractor may classify only an exact named body; it never infers
        // connector endpoints or evaluates general OCL.
        const BINDING_XMI: &str = r#"<?xml version="1.0"?>
<xmi:XMI xmlns:xmi="urn:xmi" xmlns:uml="urn:uml">
  <uml:Package xmi:id="Top" name="KerML">
    <packagedElement xmi:type="uml:Package" name="Kernel">
      <packagedElement xmi:type="uml:Package" name="Expressions">
        <packagedElement xmi:type="uml:Class" name="FeatureReferenceExpression">
          <ownedRule xmi:type="uml:Constraint" name="checkFeatureReferenceExpressionBindingConnector">
            <specification xmi:type="uml:OpaqueExpression" body="ownedMember-&gt;selectByKind(BindingConnector)-&gt;exists(b | b.relatedFeatures-&gt;includes(targetFeature) and b.relatedFeatures-&gt;includes(result))" language="OCL2.0"/>
          </ownedRule>
        </packagedElement>
        <packagedElement xmi:type="uml:Class" name="InvocationExpression">
          <ownedRule xmi:type="uml:Constraint" name="checkInvocationExpressionBehaviorBindingConnector">
            <specification xmi:type="uml:OpaqueExpression" body="not instantiatedType.oclIsKindOf(Function) implies ownedFeature.selectByKind(BindingConnector)-&gt;exists(relatedFeature-&gt;includes(result))" language="OCL2.0"/>
          </ownedRule>
          <ownedRule xmi:type="uml:Constraint" name="checkInvocationExpressionDefaultValueBindingConnector">
            <specification xmi:type="uml:OpaqueExpression" body="TBD" language="OCL2.0"/>
          </ownedRule>
        </packagedElement>
      </packagedElement>
      <packagedElement xmi:type="uml:Package" name="Functions">
        <packagedElement xmi:type="uml:Class" name="Expression">
          <ownedRule xmi:type="uml:Constraint" name="checkExpressionResultBindingConnector">
            <specification xmi:type="uml:OpaqueExpression" body="ownedMembership.selectByKind(ResultExpressionMembership)-&gt;forAll(mem | ownedFeature.selectByKind(BindingConnector)-&gt;exists(binding | binding.relatedFeature-&gt;includes(result)))" language="OCL2.0"/>
          </ownedRule>
        </packagedElement>
      </packagedElement>
    </packagedElement>
  </uml:Package>
</xmi:XMI>"#;

        let entries = extract_constraints(BINDING_XMI.as_bytes()).unwrap();
        assert_eq!(entries.len(), 4);
        let kinds = entries
            .iter()
            .filter_map(|entry| {
                entry
                    .binding_connector_check
                    .as_ref()
                    .map(|contract| contract.kind)
            })
            .collect::<Vec<_>>();
        assert_eq!(
            kinds,
            vec![
                BindingConnectorCheckKind::FeatureReferenceExpression,
                BindingConnectorCheckKind::InvocationExpressionDefaultValueTbd,
            ]
        );
        assert!(entries.iter().all(|entry| {
            entry.specializes_from_library.is_none() && entry.redefines_from_library.is_none()
        }));
    }

    #[test]
    fn specialization_check_contracts_require_exact_pinned_body_fingerprints() {
        assert_eq!(
            exact_specialization_check(
                "checkInvocationExpressionSpecialization",
                "specializes(instantiatedType)",
            ),
            Some(SpecializationCheckKind::InvocationExpression)
        );
        assert_eq!(
            exact_specialization_check(
                "checkInvocationExpressionSpecialization",
                "specializes(instantiatedType) and true",
            ),
            None
        );
        assert_eq!(
            exact_specialization_check(
                "checkUsageVariationUsageSpecialization",
                "owningVariationUsage <> null implies specializes(owningVariationUsage)",
            ),
            Some(SpecializationCheckKind::UsageVariationUsage)
        );
    }

    #[test]
    fn reconciliation_rejects_missing_and_ambiguous_headings() {
        let mut constraints = extract_constraints(XMI.as_bytes()).unwrap();
        let missing = reconcile_constraint_clauses(
            &mut constraints,
            &BTreeMap::new(),
            TEST_SPEC,
            Path::new("TestML.pdf"),
        )
        .unwrap_err();
        assert!(missing.contains("no exact"));

        let ambiguous = reconcile_constraint_clauses(
            &mut constraints,
            &BTreeMap::from([(
                String::from("element"),
                vec![String::from("8.3.1"), String::from("8.3.2")],
            )]),
            TEST_SPEC,
            Path::new("TestML.pdf"),
        )
        .unwrap_err();
        assert!(ambiguous.contains("ambiguous"));
    }
}
