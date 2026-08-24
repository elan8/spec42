# META
~~~ini
description=KerML 8.3.4.9.2 validateFlowPayloadFeature allows a Flow at most one ownedFeature that is a PayloadFeature
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.9.2 validateFlowPayloadFeature
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.4.9.2:validateFlowPayloadFeature
blocked_by=parser-gap-62-kerml-flow
type=file
~~~
# SOURCE
~~~kerml
package Flows {
    classifier Thing;
    behavior Moving {
        feature source : Thing;
        feature target : Thing;

        // Conforming: a single payload feature.
        flow of Thing from source to target;

        // Invalid: two payload features on one flow.
        flow of Thing of Thing from source to target;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_flow_payload_feature.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "flow_multiple_payload_features")
        (source "semantic")
        (range (start 10 8) (end 10 53))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_flow_payload_feature.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_calc_body_element")
        (source "parser")
        (range (start 10 8) (end 11 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:73c254581b289ebf821351c109c56924c016a399439cffdce63de00bb3a17cf4") (contract-version "parser-owned-resolution-v2"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Moving"))) (kind kerml-behavior) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_flow_payload_feature.md") (path (named (kind package) (name "Flows")) (named (kind kerml-behavior) (name "Moving")) (anonymous (kind flow) (ordinal 0))))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "source")) (flowTarget (reference "target")) (flowPayloadType (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Moving::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Moving::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_flow_payload_feature.md") (path (named (kind package) (name "Flows")) (named (kind kerml-behavior) (name "Moving")) (anonymous (kind flow) (ordinal 0))))) (kind flowSource) (ordinal 0))
      (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Moving::source")))))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_payload_feature.md") (path (named (kind package) (name "Flows")) (named (kind kerml-behavior) (name "Moving")) (anonymous (kind flow) (ordinal 0))))) (kind flowTarget) (ordinal 0))
      (authored-target "target")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Moving::target")))))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_payload_feature.md") (path (named (kind package) (name "Flows")) (named (kind kerml-behavior) (name "Moving")) (anonymous (kind flow) (ordinal 0))))) (kind flowPayloadType) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Moving::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Moving::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Thing")))))
  )
  (relationships
    (relationship (kind flowSource) (source (node (document "memory://snapshot/kerml_flow_payload_feature.md") (path (named (kind package) (name "Flows")) (named (kind kerml-behavior) (name "Moving")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Moving::source"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_flow_payload_feature.md") (path (named (kind package) (name "Flows")) (named (kind kerml-behavior) (name "Moving")) (anonymous (kind flow) (ordinal 0))))) (kind flowSource) (ordinal 0)))
    (relationship (kind flowTarget) (source (node (document "memory://snapshot/kerml_flow_payload_feature.md") (path (named (kind package) (name "Flows")) (named (kind kerml-behavior) (name "Moving")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Moving::target"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_flow_payload_feature.md") (path (named (kind package) (name "Flows")) (named (kind kerml-behavior) (name "Moving")) (anonymous (kind flow) (ordinal 0))))) (kind flowTarget) (ordinal 0)))
    (relationship (kind flowPayloadType) (source (node (document "memory://snapshot/kerml_flow_payload_feature.md") (path (named (kind package) (name "Flows")) (named (kind kerml-behavior) (name "Moving")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_flow_payload_feature.md") (path (named (kind package) (name "Flows")) (named (kind kerml-behavior) (name "Moving")) (anonymous (kind flow) (ordinal 0))))) (kind flowPayloadType) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Moving::source"))) (target (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Moving::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Moving::target"))) (target (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Moving::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_flow_payload_feature.md") (path (named (kind package) (name "Flows")) (named (kind kerml-behavior) (name "Moving")) (anonymous (kind flow) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Moving"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Moving::source"))) (target (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Moving"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Moving::target"))) (target (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Moving"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_flow_payload_feature.md") (path (named (kind package) (name "Flows")) (named (kind kerml-behavior) (name "Moving")) (anonymous (kind flow) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Moving")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Moving::source")))
      (featured-by (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Moving")))
      (type (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Moving::target")))
      (featured-by (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Moving")))
      (type (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Thing")))
      (subtype (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Moving::source")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Moving::target")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_flow_payload_feature.md") (range (start 7 27) (end 7 33)) (probe (position 7 27))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_payload_feature.md") (path (named (kind package) (name "Flows")) (named (kind kerml-behavior) (name "Moving")) (anonymous (kind flow) (ordinal 0))))) (kind flowSource) (ordinal 0) (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Moving::source")))))
    )
  )
  (query (document "memory://snapshot/kerml_flow_payload_feature.md") (range (start 7 37) (end 7 43)) (probe (position 7 37))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_payload_feature.md") (path (named (kind package) (name "Flows")) (named (kind kerml-behavior) (name "Moving")) (anonymous (kind flow) (ordinal 0))))) (kind flowTarget) (ordinal 0) (authored-target "target")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Moving::target")))))
    )
  )
  (query (document "memory://snapshot/kerml_flow_payload_feature.md") (range (start 7 16) (end 7 21)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_payload_feature.md") (path (named (kind package) (name "Flows")) (named (kind kerml-behavior) (name "Moving")) (anonymous (kind flow) (ordinal 0))))) (kind flowPayloadType) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_flow_payload_feature.md") (range (start 3 25) (end 3 30)) (probe (position 3 25))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Moving::source"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_flow_payload_feature.md") (range (start 4 25) (end 4 30)) (probe (position 4 25))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Moving::target"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_flow_payload_feature.md") (qualified-name "Flows::Thing")))))
    )
  )
)
~~~
