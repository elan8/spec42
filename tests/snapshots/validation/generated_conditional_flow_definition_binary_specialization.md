# META
~~~ini
description=Generated FlowDefinition binary specialization selects Flows::Message from the exact two-flow-end predicate
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.16.2:checkFlowDefinitionBinarySpecialization
type=file
libraries=standard
~~~
# SOURCE
~~~sysml
package FlowDefinitionBinarySpecialization {
    part def Component;

    flow def Binary {
        end source : Component;
        end target : Component;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind specialization) (source "FlowDefinitionBinarySpecialization::Binary") (target "Flows::Message") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:6ad14ddbdaf377e61a1fabd167d52419bfed3d7e2ef2787e1b3123e4381c5e91") (contract-version "operator-expression-arguments-v7") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (qualified-name "FlowDefinitionBinarySpecialization"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (qualified-name "FlowDefinitionBinarySpecialization::Binary"))) (kind flow-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (qualified-name "FlowDefinitionBinarySpecialization::Binary::source"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (qualified-name "FlowDefinitionBinarySpecialization::Binary::target"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 1)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Component")))))
    (declaration (id (node (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (qualified-name "FlowDefinitionBinarySpecialization::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (qualified-name "FlowDefinitionBinarySpecialization::Binary::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (qualified-name "FlowDefinitionBinarySpecialization::Component")))))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (qualified-name "FlowDefinitionBinarySpecialization::Binary::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (qualified-name "FlowDefinitionBinarySpecialization::Component")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (qualified-name "FlowDefinitionBinarySpecialization::Binary::source"))) (target (node (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (qualified-name "FlowDefinitionBinarySpecialization::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (qualified-name "FlowDefinitionBinarySpecialization::Binary::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (qualified-name "FlowDefinitionBinarySpecialization::Binary::target"))) (target (node (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (qualified-name "FlowDefinitionBinarySpecialization::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (qualified-name "FlowDefinitionBinarySpecialization::Binary::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (qualified-name "FlowDefinitionBinarySpecialization::Binary"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::Message"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (qualified-name "FlowDefinitionBinarySpecialization::Binary"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::MessageAction"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (qualified-name "FlowDefinitionBinarySpecialization::Binary::source"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (qualified-name "FlowDefinitionBinarySpecialization::Binary::source"))) (target (node (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (qualified-name "FlowDefinitionBinarySpecialization::Binary"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (qualified-name "FlowDefinitionBinarySpecialization::Binary::target"))) (target (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (qualified-name "FlowDefinitionBinarySpecialization::Binary::target"))) (target (node (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (qualified-name "FlowDefinitionBinarySpecialization::Binary"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (qualified-name "FlowDefinitionBinarySpecialization::Component"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (qualified-name "FlowDefinitionBinarySpecialization::Binary")))
      (positional-ends (authored 2) (effective 2))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::Message")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::MessageAction")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (qualified-name "FlowDefinitionBinarySpecialization::Binary::source")))
      (featured-by (node (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (qualified-name "FlowDefinitionBinarySpecialization::Binary")))
      (type (node (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (qualified-name "FlowDefinitionBinarySpecialization::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (qualified-name "FlowDefinitionBinarySpecialization::Component")) (source direct))
      (supertype (node (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (qualified-name "FlowDefinitionBinarySpecialization::Component")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (qualified-name "FlowDefinitionBinarySpecialization::Binary::target")))
      (featured-by (node (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (qualified-name "FlowDefinitionBinarySpecialization::Binary")))
      (type (node (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (qualified-name "FlowDefinitionBinarySpecialization::Component")) (provenance authored))
      (effective-type (node (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (qualified-name "FlowDefinitionBinarySpecialization::Component")) (source direct))
      (supertype (node (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (qualified-name "FlowDefinitionBinarySpecialization::Component")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (qualified-name "FlowDefinitionBinarySpecialization::Component")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (qualified-name "FlowDefinitionBinarySpecialization::Binary::source")) (scopes any))
      (subtype (node (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (qualified-name "FlowDefinitionBinarySpecialization::Binary::target")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (range (start 4 21) (end 4 30)) (probe (position 4 21))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (qualified-name "FlowDefinitionBinarySpecialization::Binary::source"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (qualified-name "FlowDefinitionBinarySpecialization::Component")))))
    )
  )
  (query (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (range (start 5 21) (end 5 30)) (probe (position 5 21))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (qualified-name "FlowDefinitionBinarySpecialization::Binary::target"))) (kind featureTyping) (ordinal 0) (authored-target "Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_flow_definition_binary_specialization.md") (qualified-name "FlowDefinitionBinarySpecialization::Component")))))
    )
  )
)
~~~
