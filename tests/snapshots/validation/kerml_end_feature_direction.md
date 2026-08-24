# META
~~~ini
description=KerML 8.3.3.3.4 validateFeatureEndNoDirection forbids a Feature with isEnd = true from having a direction
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.3.4 validateFeatureEndNoDirection
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.3.3.4:validateFeatureEndNoDirection
type=file
~~~
# SOURCE
~~~sysml
package Ends {
    part def Thing;
    connection def DirectedEnd {
        // Conforming: an end feature without a direction.
        end plain : Thing;

        // Invalid: an end feature must not have an authored direction. KerML's own
        // `EndFeaturePrefix` spells only `const? end`, so the textual spelling that authors
        // both is SysML's `DefaultReferenceUsage` (`( 'end' )? RefPrefix UsageDeclaration`).
        end in directed : Thing;
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
        (code "end_feature_has_direction")
        (source "semantic")
        (range (start 9 8) (end 9 32))
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
        (severity warning)
        (code "end_feature_has_direction")
        (source "semantic")
        (range (start 9 8) (end 9 32))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:17c0df53be6ca4450153057dc3bb30cc68c3478cafe4ee7b4d7c47452e6510b1") (contract-version "lossless-publication-completeness-v3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::DirectedEnd"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::DirectedEnd::directed"))) (kind connection) (membership (kind feature) (visibility default)) (facts (direction in) (positional-end 1)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::DirectedEnd::plain"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::Thing"))) (kind part-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::DirectedEnd::directed"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::DirectedEnd::plain"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::DirectedEnd::directed"))) (target (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::DirectedEnd::directed"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::DirectedEnd::plain"))) (target (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::DirectedEnd::plain"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::DirectedEnd::directed"))) (target (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::DirectedEnd"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::DirectedEnd::plain"))) (target (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::DirectedEnd"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::DirectedEnd")))
      (positional-ends (authored 2) (effective 2))
    )
    (declaration (id (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::DirectedEnd::directed")))
      (featured-by (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::DirectedEnd")))
      (type (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::DirectedEnd::plain")))
      (featured-by (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::DirectedEnd")))
      (type (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::Thing")))
      (subtype (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::DirectedEnd::directed")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::DirectedEnd::plain")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_end_feature_direction.md") (range (start 9 26) (end 9 31)) (probe (position 9 26))
    (reference (id (source (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::DirectedEnd::directed"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_end_feature_direction.md") (range (start 4 20) (end 4 25)) (probe (position 4 20))
    (reference (id (source (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::DirectedEnd::plain"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_end_feature_direction.md") (qualified-name "Ends::Thing")))))
    )
  )
)
~~~
