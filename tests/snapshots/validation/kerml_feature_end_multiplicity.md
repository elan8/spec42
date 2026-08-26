# META
~~~ini
description=KerML 8.3.3.3.4 validateFeatureEndMultiplicity requires a Feature with isEnd = true to have multiplicity 1..1
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.3.4 validateFeatureEndMultiplicity
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.3.3.4:validateFeatureEndMultiplicity
blocked_by=semantic-end-feature-multiplicity-not-one
type=file
~~~
# SOURCE
~~~kerml
package Ends {
    classifier Thing;
    assoc Multiplicities {
        // Conforming: an end feature with the required 1..1 multiplicity.
        end feature exactlyOne[1] : Thing;

        // Invalid: an end feature must not widen its multiplicity.
        end feature many[0..2] : Thing;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_end_multiplicity.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "end_feature_multiplicity_not_one")
        (source "semantic")
        (range (start 7 8) (end 7 39))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_end_multiplicity.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:ffbbd3d9599689e9b9a0c9130e1d6335b710b256eb5e048c586dafd965af4b39") (contract-version "operator-expression-arguments-v7"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_multiplicity.md") (qualified-name "Ends"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_multiplicity.md") (qualified-name "Ends::Multiplicities"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_multiplicity.md") (qualified-name "Ends::Multiplicities::exactlyOne"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_multiplicity.md") (qualified-name "Ends::Multiplicities::many"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end) (multiplicity (lower 0) (upper 2))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_multiplicity.md") (qualified-name "Ends::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_multiplicity.md") (qualified-name "Ends::Multiplicities::exactlyOne"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_multiplicity.md") (qualified-name "Ends::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_multiplicity.md") (qualified-name "Ends::Multiplicities::many"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_multiplicity.md") (qualified-name "Ends::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_end_multiplicity.md") (qualified-name "Ends::Multiplicities::exactlyOne"))) (target (node (document "memory://snapshot/kerml_feature_end_multiplicity.md") (qualified-name "Ends::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_end_multiplicity.md") (qualified-name "Ends::Multiplicities::exactlyOne"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_end_multiplicity.md") (qualified-name "Ends::Multiplicities::many"))) (target (node (document "memory://snapshot/kerml_feature_end_multiplicity.md") (qualified-name "Ends::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_end_multiplicity.md") (qualified-name "Ends::Multiplicities::many"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_end_multiplicity.md") (qualified-name "Ends::Multiplicities::exactlyOne"))) (target (node (document "memory://snapshot/kerml_feature_end_multiplicity.md") (qualified-name "Ends::Multiplicities"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_end_multiplicity.md") (qualified-name "Ends::Multiplicities::many"))) (target (node (document "memory://snapshot/kerml_feature_end_multiplicity.md") (qualified-name "Ends::Multiplicities"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_multiplicity.md") (qualified-name "Ends::Multiplicities::exactlyOne")))
      (featured-by (node (document "memory://snapshot/kerml_feature_end_multiplicity.md") (qualified-name "Ends::Multiplicities")))
      (type (node (document "memory://snapshot/kerml_feature_end_multiplicity.md") (qualified-name "Ends::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_end_multiplicity.md") (qualified-name "Ends::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_end_multiplicity.md") (qualified-name "Ends::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_multiplicity.md") (qualified-name "Ends::Multiplicities::many")))
      (featured-by (node (document "memory://snapshot/kerml_feature_end_multiplicity.md") (qualified-name "Ends::Multiplicities")))
      (type (node (document "memory://snapshot/kerml_feature_end_multiplicity.md") (qualified-name "Ends::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_end_multiplicity.md") (qualified-name "Ends::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_end_multiplicity.md") (qualified-name "Ends::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_multiplicity.md") (qualified-name "Ends::Thing")))
      (subtype (node (document "memory://snapshot/kerml_feature_end_multiplicity.md") (qualified-name "Ends::Multiplicities::exactlyOne")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_end_multiplicity.md") (qualified-name "Ends::Multiplicities::many")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_end_multiplicity.md") (range (start 4 36) (end 4 41)) (probe (position 4 36))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_multiplicity.md") (qualified-name "Ends::Multiplicities::exactlyOne"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_multiplicity.md") (qualified-name "Ends::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_end_multiplicity.md") (range (start 7 33) (end 7 38)) (probe (position 7 33))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_multiplicity.md") (qualified-name "Ends::Multiplicities::many"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_multiplicity.md") (qualified-name "Ends::Thing")))))
    )
  )
)
~~~
