# META
~~~ini
description=KerML 8.3.3.3.4 validateFeatureEndNoDirection forbids a Feature with isEnd = true from having a direction
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.3.4 validateFeatureEndNoDirection
type=file
skip_validation=parser gap 59 cannot represent an authored directed end feature
~~~
# SOURCE
~~~kerml
package Ends {
    classifier Thing;
    assoc DirectedEnd {
        // Conforming: an end feature without a direction.
        end feature plain : Thing;

        // Invalid: an end feature must not have an authored direction.
        in end feature directed : Thing;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_end_feature_direction.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "end_feature_invalid_restrictions")
        (source "semantic")
        (range (start 7 8) (end 7 40))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_end_feature_direction.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_calc_body_element")
        (source "parser")
        (range (start 7 8) (end 8 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:53963f905ea29fe6dd83088146c11a8e57916bdb2ef77105c162a93a08d4bb79") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::DirectedEnd"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::DirectedEnd::plain"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::DirectedEnd::plain"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::DirectedEnd::plain"))) (target (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::DirectedEnd::plain"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::DirectedEnd::plain")))
      (featured-by (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::DirectedEnd")))
      (type (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::Thing")))
      (subtype (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::DirectedEnd::plain")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_end_feature_direction.md") (range (start 4 28) (end 4 33)) (probe (position 4 28))
    (reference (id (source (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::DirectedEnd::plain"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::Thing")))))
    )
  )
)
~~~
