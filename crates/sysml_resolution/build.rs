use std::env;
use std::fs;
use std::path::PathBuf;

use spec42_constraint_manifest::{
    ActionDerivedFactKind, BindingConnectorCheckKind, ConstraintManifest,
    ElementDerivedDocumentationKind, ElementDerivedOwnerKind, FeatureDerivedRelationshipKind,
    NamespaceDerivedElementKind, NamespaceImportDerivedElementKind, RedefinitionCheckKind,
    RequirementDerivedFactKind, SpecializationCheckKind, TypeDerivedElementKind,
    TypeDerivedFactKind, TypeDerivedRelationshipKind, TypeFeaturingCheckKind,
};

fn feature_collection(kind: FeatureDerivedRelationshipKind) -> &'static str {
    match kind {
        FeatureDerivedRelationshipKind::OwnedFeatureChaining => "OwnedFeatureChaining",
        FeatureDerivedRelationshipKind::OwnedRedefinition => "OwnedRedefinition",
        FeatureDerivedRelationshipKind::OwnedSubsetting => "OwnedSubsetting",
        FeatureDerivedRelationshipKind::OwnedTyping => "OwnedTyping",
        FeatureDerivedRelationshipKind::OwnedTypeFeaturing => "OwnedTypeFeaturing",
    }
}

fn type_collection(kind: TypeDerivedRelationshipKind) -> &'static str {
    match kind {
        TypeDerivedRelationshipKind::OwnedSpecialization => "OwnedSpecialization",
        TypeDerivedRelationshipKind::OwnedUnioning => "OwnedUnioning",
        TypeDerivedRelationshipKind::OwnedIntersecting => "OwnedIntersecting",
        TypeDerivedRelationshipKind::OwnedDifferencing => "OwnedDifferencing",
        TypeDerivedRelationshipKind::OwnedDisjoining => "OwnedDisjoining",
        TypeDerivedRelationshipKind::UnioningType => "UnioningType",
        TypeDerivedRelationshipKind::IntersectingType => "IntersectingType",
        TypeDerivedRelationshipKind::DifferencingType => "DifferencingType",
    }
}

fn type_element_collection(kind: TypeDerivedElementKind) -> &'static str {
    match kind {
        TypeDerivedElementKind::OwnedFeature => "OwnedFeature",
        TypeDerivedElementKind::OwnedEndFeature => "OwnedEndFeature",
    }
}

fn type_fact_collection(kind: TypeDerivedFactKind) -> &'static str {
    match kind {
        TypeDerivedFactKind::OwnedFeatureMembership => "OwnedFeatureMembership",
        TypeDerivedFactKind::FeatureMembership => "FeatureMembership",
        TypeDerivedFactKind::Feature => "Feature",
        TypeDerivedFactKind::EndFeature => "EndFeature",
        TypeDerivedFactKind::DirectedFeature => "DirectedFeature",
        TypeDerivedFactKind::InheritedMembership => "InheritedMembership",
        TypeDerivedFactKind::InheritedFeature => "InheritedFeature",
        TypeDerivedFactKind::Input => "Input",
        TypeDerivedFactKind::Output => "Output",
        TypeDerivedFactKind::Multiplicity => "Multiplicity",
        TypeDerivedFactKind::OwnedConjugator => "OwnedConjugator",
    }
}

fn action_fact_collection(kind: ActionDerivedFactKind) -> &'static str {
    match kind {
        ActionDerivedFactKind::ActionDefinitionAction => "ActionDefinitionAction",
        ActionDerivedFactKind::AssignmentValueExpression => "AssignmentValueExpression",
        ActionDerivedFactKind::AssignmentTargetArgument => "AssignmentTargetArgument",
        ActionDerivedFactKind::AssignmentReferent => "AssignmentReferent",
        ActionDerivedFactKind::ForLoopVariable => "ForLoopVariable",
        ActionDerivedFactKind::ForLoopSeqArgument => "ForLoopSeqArgument",
        ActionDerivedFactKind::LoopBodyAction => "LoopBodyAction",
        ActionDerivedFactKind::TerminateOccurrenceArgument => "TerminateOccurrenceArgument",
        ActionDerivedFactKind::AcceptPayloadArgument => "AcceptPayloadArgument",
        ActionDerivedFactKind::AcceptPayloadParameter => "AcceptPayloadParameter",
        ActionDerivedFactKind::AcceptReceiverArgument => "AcceptReceiverArgument",
        ActionDerivedFactKind::WhileArgument => "WhileArgument",
        ActionDerivedFactKind::UntilArgument => "UntilArgument",
        ActionDerivedFactKind::SendSenderArgument => "SendSenderArgument",
        ActionDerivedFactKind::SendReceiverArgument => "SendReceiverArgument",
        ActionDerivedFactKind::SendPayloadArgument => "SendPayloadArgument",
        ActionDerivedFactKind::IfThenAction => "IfThenAction",
        ActionDerivedFactKind::IfElseAction => "IfElseAction",
        ActionDerivedFactKind::IfArgument => "IfArgument",
    }
}

fn requirement_fact_collection(kind: RequirementDerivedFactKind) -> &'static str {
    match kind {
        RequirementDerivedFactKind::DefinitionActorParameter => "DefinitionActorParameter",
        RequirementDerivedFactKind::DefinitionSubjectParameter => "DefinitionSubjectParameter",
        RequirementDerivedFactKind::DefinitionText => "DefinitionText",
        RequirementDerivedFactKind::DefinitionRequiredConstraint => "DefinitionRequiredConstraint",
        RequirementDerivedFactKind::DefinitionAssumedConstraint => "DefinitionAssumedConstraint",
        RequirementDerivedFactKind::DefinitionFramedConcern => "DefinitionFramedConcern",
        RequirementDerivedFactKind::UsageActorParameter => "UsageActorParameter",
        RequirementDerivedFactKind::UsageSubjectParameter => "UsageSubjectParameter",
        RequirementDerivedFactKind::UsageText => "UsageText",
        RequirementDerivedFactKind::UsageRequiredConstraint => "UsageRequiredConstraint",
        RequirementDerivedFactKind::UsageAssumedConstraint => "UsageAssumedConstraint",
        RequirementDerivedFactKind::UsageFramedConcern => "UsageFramedConcern",
    }
}

fn type_featuring_check_kind(kind: TypeFeaturingCheckKind) -> &'static str {
    match kind {
        TypeFeaturingCheckKind::FeatureFeatureMembership => "FeatureFeatureMembership",
    }
}

fn element_owner_kind(kind: ElementDerivedOwnerKind) -> &'static str {
    match kind {
        ElementDerivedOwnerKind::Owner => "Owner",
    }
}

fn element_documentation_collection(kind: ElementDerivedDocumentationKind) -> &'static str {
    match kind {
        ElementDerivedDocumentationKind::Documentation => "Documentation",
        ElementDerivedDocumentationKind::TextualRepresentation => "TextualRepresentation",
    }
}

fn namespace_collection(kind: NamespaceDerivedElementKind) -> &'static str {
    match kind {
        NamespaceDerivedElementKind::OwnedMember => "OwnedMember",
        NamespaceDerivedElementKind::OwnedImport => "OwnedImport",
    }
}

fn namespace_import_element_kind(kind: NamespaceImportDerivedElementKind) -> &'static str {
    match kind {
        NamespaceImportDerivedElementKind::ImportedElement => "ImportedElement",
    }
}

fn binding_connector_check_kind(kind: BindingConnectorCheckKind) -> &'static str {
    match kind {
        BindingConnectorCheckKind::FeatureValue => "FeatureValue",
        BindingConnectorCheckKind::ExpressionResult => "ExpressionResult",
        BindingConnectorCheckKind::FunctionResult => "FunctionResult",
        BindingConnectorCheckKind::ConstructorExpressionResultDefaultValueTbd => {
            "ConstructorExpressionResultDefaultValueTbd"
        }
        BindingConnectorCheckKind::FeatureReferenceExpression => "FeatureReferenceExpression",
        BindingConnectorCheckKind::InvocationExpressionBehavior => "InvocationExpressionBehavior",
        BindingConnectorCheckKind::InvocationExpressionDefaultValueTbd => {
            "InvocationExpressionDefaultValueTbd"
        }
        BindingConnectorCheckKind::AcceptActionUsageReceiver => "AcceptActionUsageReceiver",
        BindingConnectorCheckKind::TransitionUsageSource => "TransitionUsageSource",
        BindingConnectorCheckKind::TransitionUsageSuccession => "TransitionUsageSuccession",
        BindingConnectorCheckKind::SatisfyRequirementUsage => "SatisfyRequirementUsage",
    }
}

fn redefinition_check_kind(kind: RedefinitionCheckKind) -> &'static str {
    match kind {
        RedefinitionCheckKind::FeatureEnd => "FeatureEnd",
        RedefinitionCheckKind::FeatureFlowFeature => "FeatureFlowFeature",
        RedefinitionCheckKind::FeatureOwnedCrossFeatureSpecialization => {
            "FeatureOwnedCrossFeatureSpecialization"
        }
        RedefinitionCheckKind::FeatureParameter => "FeatureParameter",
        RedefinitionCheckKind::FeatureResult => "FeatureResult",
        RedefinitionCheckKind::ConstructorExpressionResultFeature => {
            "ConstructorExpressionResultFeature"
        }
        RedefinitionCheckKind::FeatureChainExpressionSourceTarget => {
            "FeatureChainExpressionSourceTarget"
        }
        RedefinitionCheckKind::FeatureChainExpressionTarget => "FeatureChainExpressionTarget",
        RedefinitionCheckKind::ActionUsageStateAction => "ActionUsageStateAction",
        RedefinitionCheckKind::AssignmentActionUsageAccessedFeature => {
            "AssignmentActionUsageAccessedFeature"
        }
        RedefinitionCheckKind::AssignmentActionUsageReferent => "AssignmentActionUsageReferent",
        RedefinitionCheckKind::AssignmentActionUsageStartingAt => "AssignmentActionUsageStartingAt",
        RedefinitionCheckKind::ForLoopActionUsageVar => "ForLoopActionUsageVar",
        RedefinitionCheckKind::RequirementUsageObjective => "RequirementUsageObjective",
        RedefinitionCheckKind::RenderingUsage => "RenderingUsage",
    }
}

fn specialization_check_kind(kind: SpecializationCheckKind) -> &'static str {
    match kind {
        SpecializationCheckKind::FeatureCrossing => "FeatureCrossing",
        SpecializationCheckKind::FeatureOwnedCrossFeature => "FeatureOwnedCrossFeature",
        SpecializationCheckKind::FeaturePortion => "FeaturePortion",
        SpecializationCheckKind::FeatureSubobject => "FeatureSubobject",
        SpecializationCheckKind::FeatureSuboccurrence => "FeatureSuboccurrence",
        SpecializationCheckKind::FeatureValuation => "FeatureValuation",
        SpecializationCheckKind::MetadataFeatureSemantic => "MetadataFeatureSemantic",
        SpecializationCheckKind::ConnectorBinaryObject => "ConnectorBinaryObject",
        SpecializationCheckKind::ConnectorObject => "ConnectorObject",
        SpecializationCheckKind::StepOwnedPerformance => "StepOwnedPerformance",
        SpecializationCheckKind::StepSubperformance => "StepSubperformance",
        SpecializationCheckKind::SelectExpressionResult => "SelectExpressionResult",
        SpecializationCheckKind::ConstructorExpressionResult => "ConstructorExpressionResult",
        SpecializationCheckKind::ConstructorExpression => "ConstructorExpression",
        SpecializationCheckKind::FeatureChainExpressionResult => "FeatureChainExpressionResult",
        SpecializationCheckKind::FeatureReferenceExpressionResult => {
            "FeatureReferenceExpressionResult"
        }
        SpecializationCheckKind::IndexExpressionResult => "IndexExpressionResult",
        SpecializationCheckKind::InvocationExpressionBehaviorResult => {
            "InvocationExpressionBehaviorResult"
        }
        SpecializationCheckKind::InvocationExpression => "InvocationExpression",
        SpecializationCheckKind::MergeNodeIncomingSuccession => "MergeNodeIncomingSuccession",
        SpecializationCheckKind::DecisionNodeOutgoingSuccession => "DecisionNodeOutgoingSuccession",
        SpecializationCheckKind::StateUsageExclusiveState => "StateUsageExclusiveState",
        SpecializationCheckKind::StateUsageSubstate => "StateUsageSubstate",
        SpecializationCheckKind::TransitionUsageAction => "TransitionUsageAction",
        SpecializationCheckKind::TransitionUsagePayload => "TransitionUsagePayload",
        SpecializationCheckKind::TransitionUsageState => "TransitionUsageState",
        SpecializationCheckKind::TransitionUsageSuccessionSource => {
            "TransitionUsageSuccessionSource"
        }
        SpecializationCheckKind::TransitionUsageTransitionFeature => {
            "TransitionUsageTransitionFeature"
        }
        SpecializationCheckKind::IncludeUseCase => "IncludeUseCase",
        SpecializationCheckKind::UsageVariationDefinition => "UsageVariationDefinition",
        SpecializationCheckKind::UsageVariationUsage => "UsageVariationUsage",
        SpecializationCheckKind::OccurrenceDefinitionMultiplicity => {
            "OccurrenceDefinitionMultiplicity"
        }
        SpecializationCheckKind::OccurrenceUsageSuboccurrence => "OccurrenceUsageSuboccurrence",
    }
}

fn write_generated(output: &PathBuf, name: &str, rows: Vec<String>) {
    let rows = rows.join("\n");
    let text = format!(
        "pub(crate) const {name}: &[{name_type}] = &[\n{rows}\n];\n",
        name_type = match name {
            "GENERATED_LIBRARY_SPECIALIZATION_RULES" => "LibrarySpecializationRule",
            "GENERATED_CONDITIONAL_LIBRARY_SPECIALIZATION_RULES" => {
                "ConditionalLibrarySpecializationRule"
            }
            "GENERATED_LIBRARY_REDEFINITION_RULES" => "LibraryRedefinitionRule",
            "GENERATED_FEATURE_DERIVED_RELATIONSHIP_RULES" => "FeatureDerivedRelationshipRule",
            "GENERATED_TYPE_DERIVED_RELATIONSHIP_RULES" => "TypeDerivedRelationshipRule",
            "GENERATED_TYPE_DERIVED_ELEMENT_RULES" => "TypeDerivedElementRule",
            "GENERATED_TYPE_DERIVED_FACT_RULES" => "TypeDerivedFactRule",
            "GENERATED_TYPE_FEATURING_CHECK_RULES" => "TypeFeaturingCheckRule",
            "GENERATED_ELEMENT_DERIVED_OWNER_RULES" => "ElementDerivedOwnerRule",
            "GENERATED_ELEMENT_DERIVED_DOCUMENTATION_RULES" => {
                "ElementDerivedDocumentationRule"
            }
            "GENERATED_NAMESPACE_DERIVED_ELEMENT_RULES" => "NamespaceDerivedElementRule",
            "GENERATED_NAMESPACE_IMPORT_DERIVED_ELEMENT_RULES" => {
                "NamespaceImportDerivedElementRule"
            }
            "GENERATED_BINDING_CONNECTOR_CHECK_RULES" => "BindingConnectorCheckRule",
            "GENERATED_REDEFINITION_CHECK_RULES" => "RedefinitionCheckRule",
            "GENERATED_SPECIALIZATION_CHECK_RULES" => "SpecializationCheckRule",
            "GENERATED_DEFINITION_USAGE_DERIVED_RULES" => "DefinitionUsageDerivedRule",
            "GENERATED_ACTION_DERIVED_FACT_RULES" => "ActionDerivedFactRule",
            "GENERATED_REQUIREMENT_DERIVED_FACT_RULES" => "RequirementDerivedFactRule",
            _ => unreachable!("closed generated table name"),
        }
    );
    fs::write(output, text).expect("write generated manifest binding");
}

fn main() {
    let manifest_path =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"))
            .join("../../specifications/constraint_manifest.toml");
    println!("cargo:rerun-if-changed={}", manifest_path.display());
    let manifest = ConstraintManifest::load_toml(&manifest_path)
        .expect("load validated constraint manifest for generated resolver bindings");
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR"));

    let mut specializations = Vec::new();
    let mut conditional_specializations = Vec::new();
    let mut redefinitions = Vec::new();
    let mut feature_derivations = Vec::new();
    let mut type_derivations = Vec::new();
    let mut type_derived_elements = Vec::new();
    let mut type_derived_facts = Vec::new();
    let mut type_featuring_checks = Vec::new();
    let mut element_owners = Vec::new();
    let mut element_documentation = Vec::new();
    let mut namespace_derived_elements = Vec::new();
    let mut namespace_import_derived_elements = Vec::new();
    let mut binding_connector_checks = Vec::new();
    let mut redefinition_checks = Vec::new();
    let mut specialization_checks = Vec::new();
    let mut definition_usage_derived = Vec::new();
    let mut action_derived_facts = Vec::new();
    let mut requirement_derived_facts = Vec::new();
    for entry in manifest
        .specifications
        .iter()
        .flat_map(|specification| &specification.constraints)
    {
        if let Some(contract) = &entry.specializes_from_library {
            specializations.push(format!(
                "    LibrarySpecializationRule {{ rule_id: {:?}, metaclass: {:?}, anchor: {:?} }},",
                entry.rule_id, entry.metaclass, contract.anchor
            ));
        }
        if let Some(contract) = &entry.conditional_specializes_from_library {
            let anchor =
                manifest.executable_library_anchor(&entry.rule_id, contract.anchor.as_str());
            conditional_specializations.push(format!(
                "    ConditionalLibrarySpecializationRule {{ rule_id: {:?}, metaclass: {:?}, predicate: LibrarySpecializationPredicate::{:?}, owner_metaclasses: &{:?}, true_anchor: {:?}, anchor: {:?} }},",
                entry.rule_id,
                entry.metaclass,
                contract.predicate,
                contract.owner_metaclasses,
                contract.true_anchor,
                anchor,
            ));
        }
        if let Some(contract) = &entry.redefines_from_library {
            redefinitions.push(format!(
                "    LibraryRedefinitionRule {{ rule_id: {:?}, metaclass: {:?}, anchor: {:?} }},",
                entry.rule_id, entry.metaclass, contract.anchor
            ));
        }
        if let Some(contract) = &entry.feature_derived_relationship {
            feature_derivations.push(format!(
                "    FeatureDerivedRelationshipRule {{ rule_id: {:?}, metaclass: {:?}, collection: FeatureDerivedRelationshipCollection::{} }},",
                entry.rule_id,
                entry.metaclass,
                feature_collection(contract.kind),
            ));
        }
        if let Some(contract) = &entry.type_derived_relationship {
            type_derivations.push(format!(
                "    TypeDerivedRelationshipRule {{ rule_id: {:?}, metaclass: {:?}, collection: TypeDerivedRelationshipCollection::{} }},",
                entry.rule_id,
                entry.metaclass,
                type_collection(contract.kind),
            ));
        }
        if let Some(contract) = &entry.type_derived_element {
            type_derived_elements.push(format!(
                "    TypeDerivedElementRule {{ rule_id: {:?}, metaclass: {:?}, collection: TypeDerivedElementCollection::{} }},",
                entry.rule_id,
                entry.metaclass,
                type_element_collection(contract.kind),
            ));
        }
        if let Some(contract) = &entry.type_derived_fact {
            type_derived_facts.push(format!(
                "    TypeDerivedFactRule {{ rule_id: {:?}, metaclass: {:?}, collection: TypeDerivedFactCollection::{} }},",
                entry.rule_id,
                entry.metaclass,
                type_fact_collection(contract.kind),
            ));
        }
        if let Some(contract) = &entry.type_featuring_check {
            type_featuring_checks.push(format!(
                "    TypeFeaturingCheckRule {{ rule_id: {:?}, metaclass: {:?}, kind: TypeFeaturingCheckKind::{} }},",
                entry.rule_id,
                entry.metaclass,
                type_featuring_check_kind(contract.kind),
            ));
        }
        if let Some(contract) = &entry.element_derived_owner {
            element_owners.push(format!(
                "    ElementDerivedOwnerRule {{ rule_id: {:?}, metaclass: {:?}, kind: ElementDerivedOwnerKind::{} }},",
                entry.rule_id,
                entry.metaclass,
                element_owner_kind(contract.kind),
            ));
        }
        if let Some(contract) = &entry.element_derived_documentation {
            element_documentation.push(format!(
                "    ElementDerivedDocumentationRule {{ rule_id: {:?}, metaclass: {:?}, collection: ElementDerivedDocumentationCollection::{} }},",
                entry.rule_id,
                entry.metaclass,
                element_documentation_collection(contract.kind),
            ));
        }
        if let Some(contract) = &entry.namespace_derived_element {
            namespace_derived_elements.push(format!(
                "    NamespaceDerivedElementRule {{ rule_id: {:?}, metaclass: {:?}, collection: NamespaceDerivedElementCollection::{} }},",
                entry.rule_id,
                entry.metaclass,
                namespace_collection(contract.kind),
            ));
        }
        if let Some(contract) = &entry.namespace_import_derived_element {
            namespace_import_derived_elements.push(format!(
                "    NamespaceImportDerivedElementRule {{ rule_id: {:?}, metaclass: {:?}, kind: NamespaceImportDerivedElementKind::{} }},",
                entry.rule_id,
                entry.metaclass,
                namespace_import_element_kind(contract.kind),
            ));
        }
        if let Some(contract) = &entry.binding_connector_check {
            binding_connector_checks.push(format!(
                "    BindingConnectorCheckRule {{ rule_id: {:?}, metaclass: {:?}, kind: BindingConnectorCheckKind::{} }},",
                entry.rule_id,
                entry.metaclass,
                binding_connector_check_kind(contract.kind),
            ));
        }
        if let Some(contract) = &entry.redefinition_check {
            redefinition_checks.push(format!(
                "    RedefinitionCheckRule {{ rule_id: {:?}, metaclass: {:?}, kind: RedefinitionCheckKind::{} }},",
                entry.rule_id,
                entry.metaclass,
                redefinition_check_kind(contract.kind),
            ));
        }
        if let Some(contract) = &entry.specialization_check {
            specialization_checks.push(format!(
                "    SpecializationCheckRule {{ rule_id: {:?}, metaclass: {:?}, kind: SpecializationCheckKind::{} }},",
                entry.rule_id,
                entry.metaclass,
                specialization_check_kind(contract.kind),
            ));
        }
        if let Some(contract) = &entry.definition_usage_derived {
            definition_usage_derived.push(format!(
                "    DefinitionUsageDerivedRule {{ rule_id: {:?}, metaclass: {:?}, kind: DefinitionUsageDerivedKind::{:?} }},",
                entry.rule_id, entry.metaclass, contract.kind,
            ));
        }
        if let Some(contract) = &entry.action_derived_fact {
            action_derived_facts.push(format!(
                "    ActionDerivedFactRule {{ rule_id: {:?}, metaclass: {:?}, collection: ActionDerivedFactCollection::{} }},",
                entry.rule_id, entry.metaclass, action_fact_collection(contract.kind),
            ));
        }
        if let Some(contract) = &entry.requirement_derived_fact {
            requirement_derived_facts.push(format!(
                "    RequirementDerivedFactRule {{ rule_id: {:?}, metaclass: {:?}, collection: RequirementDerivedFactCollection::{} }},",
                entry.rule_id, entry.metaclass, requirement_fact_collection(contract.kind),
            ));
        }
    }
    for rows in [
        &mut specializations,
        &mut conditional_specializations,
        &mut redefinitions,
        &mut feature_derivations,
        &mut type_derivations,
        &mut type_derived_elements,
        &mut type_derived_facts,
        &mut type_featuring_checks,
        &mut element_owners,
        &mut element_documentation,
        &mut namespace_derived_elements,
        &mut namespace_import_derived_elements,
        &mut binding_connector_checks,
        &mut redefinition_checks,
        &mut specialization_checks,
        &mut definition_usage_derived,
        &mut action_derived_facts,
        &mut requirement_derived_facts,
    ] {
        rows.sort();
    }
    write_generated(
        &out_dir.join("library_specialization_rules.rs"),
        "GENERATED_LIBRARY_SPECIALIZATION_RULES",
        specializations,
    );
    write_generated(
        &out_dir.join("conditional_library_specialization_rules.rs"),
        "GENERATED_CONDITIONAL_LIBRARY_SPECIALIZATION_RULES",
        conditional_specializations,
    );
    write_generated(
        &out_dir.join("library_redefinition_rules.rs"),
        "GENERATED_LIBRARY_REDEFINITION_RULES",
        redefinitions,
    );
    write_generated(
        &out_dir.join("feature_derived_relationship_rules.rs"),
        "GENERATED_FEATURE_DERIVED_RELATIONSHIP_RULES",
        feature_derivations,
    );
    write_generated(
        &out_dir.join("type_derived_relationship_rules.rs"),
        "GENERATED_TYPE_DERIVED_RELATIONSHIP_RULES",
        type_derivations,
    );
    write_generated(
        &out_dir.join("type_derived_element_rules.rs"),
        "GENERATED_TYPE_DERIVED_ELEMENT_RULES",
        type_derived_elements,
    );
    write_generated(
        &out_dir.join("type_derived_fact_rules.rs"),
        "GENERATED_TYPE_DERIVED_FACT_RULES",
        type_derived_facts,
    );
    write_generated(
        &out_dir.join("type_featuring_check_rules.rs"),
        "GENERATED_TYPE_FEATURING_CHECK_RULES",
        type_featuring_checks,
    );
    write_generated(
        &out_dir.join("element_derived_owner_rules.rs"),
        "GENERATED_ELEMENT_DERIVED_OWNER_RULES",
        element_owners,
    );
    write_generated(
        &out_dir.join("element_derived_documentation_rules.rs"),
        "GENERATED_ELEMENT_DERIVED_DOCUMENTATION_RULES",
        element_documentation,
    );
    write_generated(
        &out_dir.join("namespace_derived_element_rules.rs"),
        "GENERATED_NAMESPACE_DERIVED_ELEMENT_RULES",
        namespace_derived_elements,
    );
    write_generated(
        &out_dir.join("namespace_import_derived_element_rules.rs"),
        "GENERATED_NAMESPACE_IMPORT_DERIVED_ELEMENT_RULES",
        namespace_import_derived_elements,
    );
    write_generated(
        &out_dir.join("binding_connector_check_rules.rs"),
        "GENERATED_BINDING_CONNECTOR_CHECK_RULES",
        binding_connector_checks,
    );
    write_generated(
        &out_dir.join("redefinition_check_rules.rs"),
        "GENERATED_REDEFINITION_CHECK_RULES",
        redefinition_checks,
    );
    write_generated(
        &out_dir.join("specialization_check_rules.rs"),
        "GENERATED_SPECIALIZATION_CHECK_RULES",
        specialization_checks,
    );
    write_generated(
        &out_dir.join("definition_usage_derived_rules.rs"),
        "GENERATED_DEFINITION_USAGE_DERIVED_RULES",
        definition_usage_derived,
    );
    write_generated(
        &out_dir.join("action_derived_fact_rules.rs"),
        "GENERATED_ACTION_DERIVED_FACT_RULES",
        action_derived_facts,
    );
    write_generated(
        &out_dir.join("requirement_derived_fact_rules.rs"),
        "GENERATED_REQUIREMENT_DERIVED_FACT_RULES",
        requirement_derived_facts,
    );
}
