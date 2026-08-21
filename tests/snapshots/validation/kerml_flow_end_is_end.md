# META
~~~ini
description=KerML 8.3.4.9.3 validateFlowEndIsEnd requires a FlowEnd to be an end Feature
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.9.3 validateFlowEndIsEnd
type=file
skip_validation=the pinned parser has no KerML flow production -- `flow of Thing from a to b;` resolves none of its tokens and is reported as a cascade of unresolved_reference -- so no Flow or FlowEnd reaches semantics
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
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 8) (end 11 12))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 13) (end 11 15))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 22) (end 11 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 27) (end 11 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 34) (end 11 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 37) (end 11 43))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:91de64d03c0611c6faf70c6e9fd13d7a6a66c2e7b24bc2e3e2358b8f26fff15d") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving"))) (kind kerml-behavior) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "flow")) (expressionOperand (reference "of")) (expressionOperand (reference "Thing")) (expressionOperand (reference "from")) (expressionOperand (reference "source")) (expressionOperand (reference "to")) (expressionOperand (reference "target")))))
    (declaration (id (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving"))) (kind expressionOperand) (ordinal 0))
      (authored-target "flow")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving"))) (kind expressionOperand) (ordinal 1))
      (authored-target "of")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving"))) (kind expressionOperand) (ordinal 2))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving"))) (kind expressionOperand) (ordinal 3))
      (authored-target "from")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving"))) (kind expressionOperand) (ordinal 4))
      (authored-target "source")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving"))) (kind expressionOperand) (ordinal 5))
      (authored-target "to")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving"))) (kind expressionOperand) (ordinal 6))
      (authored-target "target")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Thing")))))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving"))) (target (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving"))) (kind expressionOperand) (ordinal 2)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving::source"))) (target (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving::target"))) (target (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving::target"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving"))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving"))) (state unresolved-operand))
  )
)
~~~
# TYPES
~~~sexpr
(types
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
  (query (document "memory://snapshot/kerml_flow_end_is_end.md") (range (start 11 8) (end 11 12)) (probe (position 11 8))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving"))) (kind expressionOperand) (ordinal 0) (authored-target "flow")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/kerml_flow_end_is_end.md") (range (start 11 13) (end 11 15)) (probe (position 11 13))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving"))) (kind expressionOperand) (ordinal 1) (authored-target "of")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/kerml_flow_end_is_end.md") (range (start 11 16) (end 11 21)) (probe (position 11 16))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving"))) (kind expressionOperand) (ordinal 2) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_flow_end_is_end.md") (range (start 11 22) (end 11 26)) (probe (position 11 22))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving"))) (kind expressionOperand) (ordinal 3) (authored-target "from")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/kerml_flow_end_is_end.md") (range (start 11 27) (end 11 33)) (probe (position 11 27))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving"))) (kind expressionOperand) (ordinal 4) (authored-target "source")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/kerml_flow_end_is_end.md") (range (start 11 34) (end 11 36)) (probe (position 11 34))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving"))) (kind expressionOperand) (ordinal 5) (authored-target "to")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/kerml_flow_end_is_end.md") (range (start 11 37) (end 11 43)) (probe (position 11 37))
    (reference (id (source (node (document "memory://snapshot/kerml_flow_end_is_end.md") (qualified-name "Flows::Moving"))) (kind expressionOperand) (ordinal 6) (authored-target "target")
      (outcome (status unresolved)))
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
