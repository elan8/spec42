# META
~~~ini
description=KerML 8.3.4.9.5 checkPayloadFeatureRedefinition needs the PayloadFeature a flow owns, which is not lowered
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.9.5:checkPayloadFeatureRedefinition
blocked_by=lowering-gap-flow-payload-feature
libraries=standard
type=file
~~~
# SOURCE
~~~kerml
package Payloads {
    classifier Fuel;
    classifier Tank {
        feature outlet;
        feature inlet;
        flow transfer of Fuel from outlet to inlet;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind redefinition) (source (anonymous (owner "Payloads::Tank::transfer") (kind Feature) (ordinal 2))) (target "Transfers::Transfer::payload") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:9fc22b56c1a835a00194846c20874b5fccc8579b24f424db928ac98a2afd0310") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Fuel"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank::inlet"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank::outlet"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank::transfer"))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "outlet")) (flowTarget (reference "inlet")) (flowPayloadType (reference "Fuel")))))
    (declaration (id (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (path (named (kind package) (name "Payloads")) (named (kind kerml-classifier) (name "Tank")) (named (kind flow) (name "transfer")) (anonymous (kind flow-end) (ordinal 0))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 0)))
    (declaration (id (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (path (named (kind package) (name "Payloads")) (named (kind kerml-classifier) (name "Tank")) (named (kind flow) (name "transfer")) (anonymous (kind flow-end) (ordinal 1))))) (kind flow-end) (membership (kind feature) (visibility default)) (facts (positional-end 1)))
    (declaration (id (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (path (named (kind package) (name "Payloads")) (named (kind kerml-classifier) (name "Tank")) (named (kind flow) (name "transfer")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (path (named (kind package) (name "Payloads")) (named (kind kerml-classifier) (name "Tank")) (named (kind flow) (name "transfer")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank::transfer"))) (kind flowSource) (ordinal 0))
      (authored-target "outlet")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank::outlet")))))
    (reference (id (source (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank::transfer"))) (kind flowTarget) (ordinal 0))
      (authored-target "inlet")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank::inlet")))))
    (reference (id (source (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank::transfer"))) (kind flowPayloadType) (ordinal 0))
      (authored-target "Fuel")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Fuel")))))
  )
  (relationships
    (relationship (kind flowSource) (source (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank::transfer"))) (target (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank::outlet"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank::transfer"))) (kind flowSource) (ordinal 0)))
    (relationship (kind flowTarget) (source (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank::transfer"))) (target (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank::inlet"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank::transfer"))) (kind flowTarget) (ordinal 0)))
    (relationship (kind flowPayloadType) (source (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank::transfer"))) (target (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Fuel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank::transfer"))) (kind flowPayloadType) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank::inlet"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank::inlet"))) (target (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank::outlet"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank::outlet"))) (target (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank::transfer"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank::transfer"))) (target (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank::transfer"))) (target (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank::transfer"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank::transfer"))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (path (named (kind package) (name "Payloads")) (named (kind kerml-classifier) (name "Tank")) (named (kind flow) (name "transfer")) (anonymous (kind flow-end) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank::transfer"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (path (named (kind package) (name "Payloads")) (named (kind kerml-classifier) (name "Tank")) (named (kind flow) (name "transfer")) (anonymous (kind flow-end) (ordinal 1))))) (target (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank::transfer"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (path (named (kind package) (name "Payloads")) (named (kind kerml-classifier) (name "Tank")) (named (kind flow) (name "transfer")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (path (named (kind package) (name "Payloads")) (named (kind kerml-classifier) (name "Tank")) (named (kind flow) (name "transfer")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (path (named (kind package) (name "Payloads")) (named (kind kerml-classifier) (name "Tank")) (named (kind flow) (name "transfer")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (path (named (kind package) (name "Payloads")) (named (kind kerml-classifier) (name "Tank")) (named (kind flow) (name "transfer")) (anonymous (kind flow-end) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (path (named (kind package) (name "Payloads")) (named (kind kerml-classifier) (name "Tank")) (named (kind flow) (name "transfer")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (path (named (kind package) (name "Payloads")) (named (kind kerml-classifier) (name "Tank")) (named (kind flow) (name "transfer")) (anonymous (kind flow-end) (ordinal 1))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (path (named (kind package) (name "Payloads")) (named (kind kerml-classifier) (name "Tank")) (named (kind flow) (name "transfer")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (path (named (kind package) (name "Payloads")) (named (kind kerml-classifier) (name "Tank")) (named (kind flow) (name "transfer")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank::inlet")))
      (featured-by (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank::outlet")))
      (featured-by (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank::transfer")))
      (positional-ends (authored 2) (effective 2))
      (featured-by (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::Flow")) (source inherited) (from (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::flows"))))
      (effective-type (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::Message")) (source inherited) (from (node (document "memory://snapshot/sysml.library/flows.md") (qualified-name "Flows::messages"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
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
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::FlowTransfer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::flowTransfers")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::transfers")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (path (named (kind package) (name "Payloads")) (named (kind kerml-classifier) (name "Tank")) (named (kind flow) (name "transfer")) (anonymous (kind flow-end) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank::transfer")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (path (named (kind package) (name "Payloads")) (named (kind kerml-classifier) (name "Tank")) (named (kind flow) (name "transfer")) (anonymous (kind flow-end) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank::transfer")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (path (named (kind package) (name "Payloads")) (named (kind kerml-classifier) (name "Tank")) (named (kind flow) (name "transfer")) (anonymous (kind flow-end) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (path (named (kind package) (name "Payloads")) (named (kind kerml-classifier) (name "Tank")) (named (kind flow) (name "transfer")) (anonymous (kind flow-end) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::source::sourceOutput")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (path (named (kind package) (name "Payloads")) (named (kind kerml-classifier) (name "Tank")) (named (kind flow) (name "transfer")) (anonymous (kind flow-end) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (path (named (kind package) (name "Payloads")) (named (kind kerml-classifier) (name "Tank")) (named (kind flow) (name "transfer")) (anonymous (kind flow-end) (ordinal 1)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::Transfer::target::targetInput")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (range (start 5 35) (end 5 41)) (probe (position 5 35))
    (reference (id (source (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank::transfer"))) (kind flowSource) (ordinal 0) (authored-target "outlet")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank::outlet")))))
    )
  )
  (query (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (range (start 5 45) (end 5 50)) (probe (position 5 45))
    (reference (id (source (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank::transfer"))) (kind flowTarget) (ordinal 0) (authored-target "inlet")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank::inlet")))))
    )
  )
  (query (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (range (start 5 25) (end 5 29)) (probe (position 5 25))
    (reference (id (source (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Tank::transfer"))) (kind flowPayloadType) (ordinal 0) (authored-target "Fuel")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_payload_feature_redefinition_lowering_gap.md") (qualified-name "Payloads::Fuel")))))
    )
  )
)
~~~
