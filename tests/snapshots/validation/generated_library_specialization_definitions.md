# META
~~~ini
description=Generated library-specialization checks publish implied canonical anchors for reachable SysML occurrence and link definitions
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.10.2:checkItemDefinitionSpecialization
rule_id=sysml-2.0:8.3.12.5:checkPortDefinitionSpecialization
rule_id=sysml-2.0:8.3.13.3:checkConnectionDefinitionSpecializations
rule_id=sysml-2.0:8.3.14.2:checkInterfaceDefinitionSpecialization
rule_id=sysml-2.0:8.3.15.2:checkAllocationDefinitionSpecialization
rule_id=sysml-2.0:8.3.16.2:checkFlowDefinitionSpecialization
type=file
libraries=standard
~~~
# SOURCE
~~~sysml
package GeneratedDefinitions {
    item def ItemDefinition;
    port def PortDefinition;
    abstract connection def ConnectionDefinition;
    abstract interface def InterfaceDefinition;
    abstract allocation def AllocationDefinition;
    abstract flow def FlowDefinition;
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind specialization) (source "GeneratedDefinitions::ItemDefinition") (target "Items::Item") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "GeneratedDefinitions::PortDefinition") (target "Ports::Port") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "GeneratedDefinitions::ConnectionDefinition") (target "Connections::Connection") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "GeneratedDefinitions::InterfaceDefinition") (target "Interfaces::Interface") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "GeneratedDefinitions::AllocationDefinition") (target "Allocations::Allocation") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "GeneratedDefinitions::FlowDefinition") (target "Flows::MessageAction") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_library_specialization_definitions.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:2d94af45d5beb978b1466a12d038964e8a6b062c139c774ba1a549a3fd6f0bf6") (contract-version "parser-owned-resolution-v2") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_definitions.md") (qualified-name "GeneratedDefinitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_definitions.md") (qualified-name "GeneratedDefinitions::AllocationDefinition"))) (kind allocation-def) (membership (kind owning) (visibility default)) (facts (modifiers abstract)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_definitions.md") (qualified-name "GeneratedDefinitions::ConnectionDefinition"))) (kind connection-def) (membership (kind owning) (visibility default)) (facts (modifiers abstract)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_definitions.md") (qualified-name "GeneratedDefinitions::FlowDefinition"))) (kind flow-def) (membership (kind owning) (visibility default)) (facts (modifiers abstract)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_definitions.md") (qualified-name "GeneratedDefinitions::InterfaceDefinition"))) (kind interface-def) (membership (kind owning) (visibility default)) (facts (modifiers abstract)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_definitions.md") (qualified-name "GeneratedDefinitions::ItemDefinition"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_definitions.md") (qualified-name "GeneratedDefinitions::PortDefinition"))) (kind port-def) (membership (kind owning) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_definitions.md") (qualified-name "GeneratedDefinitions::AllocationDefinition"))) (target (node (document "memory://snapshot/sysml.library/allocations.md") (qualified-name "Allocations::Allocation"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_definitions.md") (qualified-name "GeneratedDefinitions::ConnectionDefinition"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_definitions.md") (qualified-name "GeneratedDefinitions::FlowDefinition"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::MessageAction"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_definitions.md") (qualified-name "GeneratedDefinitions::InterfaceDefinition"))) (target (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::Interface"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_definitions.md") (qualified-name "GeneratedDefinitions::ItemDefinition"))) (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_definitions.md") (qualified-name "GeneratedDefinitions::PortDefinition"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_definitions.md") (qualified-name "GeneratedDefinitions::AllocationDefinition")))
      (positional-ends (authored 0) (effective 2))
      (supertype (node (document "memory://snapshot/sysml.library/allocations.md") (qualified-name "Allocations::Allocation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::BinaryConnection")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_definitions.md") (qualified-name "GeneratedDefinitions::ConnectionDefinition")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_definitions.md") (qualified-name "GeneratedDefinitions::FlowDefinition")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::MessageAction")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_definitions.md") (qualified-name "GeneratedDefinitions::InterfaceDefinition")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::Interface")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_definitions.md") (qualified-name "GeneratedDefinitions::ItemDefinition")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_definitions.md") (qualified-name "GeneratedDefinitions::PortDefinition")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
