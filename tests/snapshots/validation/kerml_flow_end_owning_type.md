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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 8) (end 9 12))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 13) (end 9 15))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 22) (end 9 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 27) (end 9 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 34) (end 9 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 37) (end 9 43))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:2145a9dd20d21f311e765e48b71707d5798fbf044ba656ac14bd58e4ac7c0f65") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving"))) (kind kerml-behavior) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "flow")) (expressionOperand (reference "of")) (expressionOperand (reference "Thing")) (expressionOperand (reference "from")) (expressionOperand (reference "source")) (expressionOperand (reference "to")) (expressionOperand (reference "target")))))
    (declaration (id (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving"))) (kind expressionOperand) (ordinal 0))
      (authored-target "flow")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving"))) (kind expressionOperand) (ordinal 1))
      (authored-target "of")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving"))) (kind expressionOperand) (ordinal 2))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving"))) (kind expressionOperand) (ordinal 3))
      (authored-target "from")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving"))) (kind expressionOperand) (ordinal 4))
      (authored-target "source")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving"))) (kind expressionOperand) (ordinal 5))
      (authored-target "to")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving"))) (kind expressionOperand) (ordinal 6))
      (authored-target "target")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Thing")))))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving"))) (target (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving"))) (kind expressionOperand) (ordinal 2)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving::source"))) (target (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving::target"))) (target (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving::target"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving"))) (state unresolved-operand))
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
  (query (document "memory://snapshot/kerml_flow_end_owning_type.md") (range (start 9 8) (end 9 12)) (probe (position 9 8))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving"))) (kind expressionOperand) (ordinal 0) (authored-target "flow")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/kerml_flow_end_owning_type.md") (range (start 9 13) (end 9 15)) (probe (position 9 13))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving"))) (kind expressionOperand) (ordinal 1) (authored-target "of")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/kerml_flow_end_owning_type.md") (range (start 9 16) (end 9 21)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving"))) (kind expressionOperand) (ordinal 2) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_flow_end_owning_type.md") (range (start 9 22) (end 9 26)) (probe (position 9 22))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving"))) (kind expressionOperand) (ordinal 3) (authored-target "from")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/kerml_flow_end_owning_type.md") (range (start 9 27) (end 9 33)) (probe (position 9 27))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving"))) (kind expressionOperand) (ordinal 4) (authored-target "source")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/kerml_flow_end_owning_type.md") (range (start 9 34) (end 9 36)) (probe (position 9 34))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving"))) (kind expressionOperand) (ordinal 5) (authored-target "to")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/kerml_flow_end_owning_type.md") (range (start 9 37) (end 9 43)) (probe (position 9 37))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_owning_type.md") (qualified-name "Flows::Moving"))) (kind expressionOperand) (ordinal 6) (authored-target "target")
      (outcome (status unresolved)))
    )
  )
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
