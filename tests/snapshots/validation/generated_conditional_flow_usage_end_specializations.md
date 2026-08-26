# META
~~~ini
description=Generated FlowUsage and KerML Flow specializations select exact anchors from typed anonymous flow endpoints
specification=OMG SysML 2.0 Language (formal/26-03-02); OMG KerML 1.0
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.16.3:checkFlowUsageFlowSpecialization
rule_id=kerml-1.0:8.3.4.9.2:checkFlowWithEndsSpecialization
blocked_by=library-gap-flow-end-specialization-anchors
type=file
libraries=standard
~~~
# SOURCE
~~~sysml
package FlowUsageEndSpecializations {
    action def Owner {
        action source;
        action target;
        flow from source to target;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind specialization) (source "FlowUsageEndSpecializations::Owner::<anonymous>") (target "Flows::flows") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "FlowUsageEndSpecializations::Owner::<anonymous>") (target "Transfers::flowTransfers") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:1c0f21d2266c5d2c8336ca3947e160c1e15e53065d121a89a5499945bc55acfc") (contract-version "feature-chain-expression-result-v10") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (qualified-name "FlowUsageEndSpecializations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (qualified-name "FlowUsageEndSpecializations::Owner"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0))))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "source")) (flowTarget (reference "target")))))
    (declaration (id (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (qualified-name "FlowUsageEndSpecializations::Owner::source"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (qualified-name "FlowUsageEndSpecializations::Owner::target"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0))))) (kind flowSource) (ordinal 0))
      (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (qualified-name "FlowUsageEndSpecializations::Owner::source")))))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0))))) (kind flowTarget) (ordinal 0))
      (authored-target "target")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (qualified-name "FlowUsageEndSpecializations::Owner::target")))))
  )
  (relationships
    (relationship (kind flowSource) (source (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (qualified-name "FlowUsageEndSpecializations::Owner::source"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0))))) (kind flowSource) (ordinal 0)))
    (relationship (kind flowTarget) (source (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (qualified-name "FlowUsageEndSpecializations::Owner::target"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0))))) (kind flowTarget) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (qualified-name "FlowUsageEndSpecializations::Owner"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (qualified-name "FlowUsageEndSpecializations::Owner"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (qualified-name "FlowUsageEndSpecializations::Owner::source"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (qualified-name "FlowUsageEndSpecializations::Owner::source"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (qualified-name "FlowUsageEndSpecializations::Owner::source"))) (target (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (qualified-name "FlowUsageEndSpecializations::Owner"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (qualified-name "FlowUsageEndSpecializations::Owner::target"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (qualified-name "FlowUsageEndSpecializations::Owner::target"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (qualified-name "FlowUsageEndSpecializations::Owner::target"))) (target (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (qualified-name "FlowUsageEndSpecializations::Owner"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (qualified-name "FlowUsageEndSpecializations::Owner")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0)))))
      (positional-ends (authored 0) (effective 2))
      (featured-by (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (qualified-name "FlowUsageEndSpecializations::Owner")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::Flow")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::Message")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::MessageAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::FlowTransfer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (qualified-name "FlowUsageEndSpecializations::Owner::source")))
      (featured-by (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (qualified-name "FlowUsageEndSpecializations::Owner")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (qualified-name "FlowUsageEndSpecializations::Owner::target")))
      (featured-by (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (qualified-name "FlowUsageEndSpecializations::Owner")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (range (start 4 18) (end 4 24)) (probe (position 4 18))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0))))) (kind flowSource) (ordinal 0) (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (qualified-name "FlowUsageEndSpecializations::Owner::source")))))
    )
  )
  (query (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (range (start 4 28) (end 4 34)) (probe (position 4 28))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0))))) (kind flowTarget) (ordinal 0) (authored-target "target")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (qualified-name "FlowUsageEndSpecializations::Owner::target")))))
    )
  )
)
~~~
