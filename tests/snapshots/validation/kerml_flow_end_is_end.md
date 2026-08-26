# META
~~~ini
description=KerML 8.3.4.9.3 validateFlowEndIsEnd requires a FlowEnd to be an end Feature
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.9.3 validateFlowEndIsEnd
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.4.9.3:validateFlowEndIsEnd
type=file
~~~
# SOURCE
~~~kerml
// Conforming: the flow ends below are authored by the from/to clauses, which create
// EndFeatureMemberships and so make each FlowEnd an end feature.
//
// The violating side has no textual counterpart: KerML flow syntax produces a FlowEnd only
// through a FlowEndMember, which is an EndFeatureMembership, so a source document cannot author
// a FlowEnd that is not an end feature.
package Flows {
    classifier Thing;
    behavior Moving {
        feature source : Thing;
        feature target : Thing;
        flow of Thing from source to target;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_flow_end_is_end.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_flow_end_is_end.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:91de64d03c0611c6faf70c6e9fd13d7a6a66c2e7b24bc2e3e2358b8f26fff15d") (contract-version "feature-value-expression-results-v5"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving"))) (kind kerml-behavior) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_flow_end_is_end.md") (path (named (kind package) (name "Flows")) (named (kind kerml-behavior) (name "Moving")) (anonymous (kind flow) (ordinal 0))))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "source")) (flowTarget (reference "target")) (flowPayloadType (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (path (named (kind package) (name "Flows")) (named (kind kerml-behavior) (name "Moving")) (anonymous (kind flow) (ordinal 0))))) (kind flowSource) (ordinal 0))
      (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving::source")))))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (path (named (kind package) (name "Flows")) (named (kind kerml-behavior) (name "Moving")) (anonymous (kind flow) (ordinal 0))))) (kind flowTarget) (ordinal 0))
      (authored-target "target")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving::target")))))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (path (named (kind package) (name "Flows")) (named (kind kerml-behavior) (name "Moving")) (anonymous (kind flow) (ordinal 0))))) (kind flowPayloadType) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Thing")))))
  )
  (relationships
    (relationship (kind flowSource) (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (path (named (kind package) (name "Flows")) (named (kind kerml-behavior) (name "Moving")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving::source"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (path (named (kind package) (name "Flows")) (named (kind kerml-behavior) (name "Moving")) (anonymous (kind flow) (ordinal 0))))) (kind flowSource) (ordinal 0)))
    (relationship (kind flowTarget) (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (path (named (kind package) (name "Flows")) (named (kind kerml-behavior) (name "Moving")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving::target"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (path (named (kind package) (name "Flows")) (named (kind kerml-behavior) (name "Moving")) (anonymous (kind flow) (ordinal 0))))) (kind flowTarget) (ordinal 0)))
    (relationship (kind flowPayloadType) (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (path (named (kind package) (name "Flows")) (named (kind kerml-behavior) (name "Moving")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (path (named (kind package) (name "Flows")) (named (kind kerml-behavior) (name "Moving")) (anonymous (kind flow) (ordinal 0))))) (kind flowPayloadType) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving::source"))) (target (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving::target"))) (target (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (path (named (kind package) (name "Flows")) (named (kind kerml-behavior) (name "Moving")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving::source"))) (target (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving::target"))) (target (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_flow_end_is_end.md") (path (named (kind package) (name "Flows")) (named (kind kerml-behavior) (name "Moving")) (anonymous (kind flow) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving::source")))
      (featured-by (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving")))
      (type (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving::target")))
      (featured-by (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving")))
      (type (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Thing")))
      (subtype (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving::source")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving::target")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_flow_end_is_end.md") (range (start 11 27) (end 11 33)) (probe (position 11 27))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (path (named (kind package) (name "Flows")) (named (kind kerml-behavior) (name "Moving")) (anonymous (kind flow) (ordinal 0))))) (kind flowSource) (ordinal 0) (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving::source")))))
    )
  )
  (query (document "memory://snapshot/kerml_flow_end_is_end.md") (range (start 11 37) (end 11 43)) (probe (position 11 37))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (path (named (kind package) (name "Flows")) (named (kind kerml-behavior) (name "Moving")) (anonymous (kind flow) (ordinal 0))))) (kind flowTarget) (ordinal 0) (authored-target "target")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving::target")))))
    )
  )
  (query (document "memory://snapshot/kerml_flow_end_is_end.md") (range (start 11 16) (end 11 21)) (probe (position 11 16))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (path (named (kind package) (name "Flows")) (named (kind kerml-behavior) (name "Moving")) (anonymous (kind flow) (ordinal 0))))) (kind flowPayloadType) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_flow_end_is_end.md") (range (start 9 25) (end 9 30)) (probe (position 9 25))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving::source"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_flow_end_is_end.md") (range (start 10 25) (end 10 30)) (probe (position 10 25))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving::target"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Thing")))))
    )
  )
)
~~~
