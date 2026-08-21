# META
~~~ini
description=KerML 8.3.4.9.3 validateFlowEndOwningType requires the owningType of a FlowEnd to be a Flow
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.9.3 validateFlowEndOwningType
type=file
skip_validation=the pinned parser has no KerML flow production -- `flow of Thing from a to b;` resolves none of its tokens and is reported as a cascade of unresolved_reference -- so no Flow or FlowEnd reaches semantics
~~~
# SOURCE
~~~kerml
// Conforming: both flow ends below are owned by the flow that declares them.
//
// The violating side has no textual counterpart: a FlowEnd is only produced by the from/to
// clauses of a flow declaration, so a source document cannot give one a non-Flow owning type.
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
  (document "memory://snapshot/kerml_flow_end_owning_type.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_flow_end_owning_type.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 9 8) (end 9 44))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:2145a9dd20d21f311e765e48b71707d5798fbf044ba656ac14bd58e4ac7c0f65") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving"))) (kind kerml-behavior) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving::source"))) (target (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving::target"))) (target (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving::target"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving::source")))
      (featured-by (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving")))
      (type (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving::target")))
      (featured-by (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving")))
      (type (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Thing")))
      (subtype (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving::source")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving::target")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_flow_end_owning_type.md") (range (start 7 25) (end 7 30)) (probe (position 7 25))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving::source"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_flow_end_owning_type.md") (range (start 8 25) (end 8 30)) (probe (position 8 25))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving::target"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Thing")))))
    )
  )
)
~~~
