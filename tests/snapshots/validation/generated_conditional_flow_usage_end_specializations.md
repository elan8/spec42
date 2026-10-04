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
  (relationship (kind subsetting) (source (anonymous (owner "FlowUsageEndSpecializations::Owner") (kind FlowConnectionUsage) (ordinal 0))) (target "Flows::flows") (provenance implied) (outcome resolved))
  (relationship (kind subsetting) (source (anonymous (owner "FlowUsageEndSpecializations::Owner") (kind FlowConnectionUsage) (ordinal 0))) (target "Transfers::flowTransfers") (provenance implied) (outcome resolved)))
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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:74231ce3649e26e169c66af22ca149550451266748663426ba2745bfdc04dff7") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (qualified-name "FlowUsageEndSpecializations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (qualified-name "FlowUsageEndSpecializations::Owner"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0))))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "source")) (flowTarget (reference "target")))))
    (declaration (id (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 0))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 0)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 1))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 1)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
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
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 0))))) (target (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 1))))) (target (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 1))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (qualified-name "FlowUsageEndSpecializations::Owner::source"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (qualified-name "FlowUsageEndSpecializations::Owner::source"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (qualified-name "FlowUsageEndSpecializations::Owner::source"))) (target (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (qualified-name "FlowUsageEndSpecializations::Owner"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (qualified-name "FlowUsageEndSpecializations::Owner::target"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (qualified-name "FlowUsageEndSpecializations::Owner::target"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
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
      (positional-ends (authored 2) (effective 2))
      (featured-by (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (qualified-name "FlowUsageEndSpecializations::Owner")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::Flow")) (source inherited) (from (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))))
      (effective-type (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::Message")) (source inherited) (from (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::FlowTransfer")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))))
      (effective-type (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::Flow")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::Message")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::MessageAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::FlowTransfer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (path (named (kind package) (name "FlowUsageEndSpecializations")) (named (kind action-def) (name "Owner")) (anonymous (kind flow) (ordinal 0)) (anonymous (kind flow-end) (ordinal 1)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (qualified-name "FlowUsageEndSpecializations::Owner::source")))
      (featured-by (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (qualified-name "FlowUsageEndSpecializations::Owner")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (qualified-name "FlowUsageEndSpecializations::Owner::target")))
      (featured-by (node (document "memory://snapshot/generated_conditional_flow_usage_end_specializations.md") (qualified-name "FlowUsageEndSpecializations::Owner")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
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
