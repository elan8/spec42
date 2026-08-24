# META
~~~ini
description=KerML 8.3.3.3.4 validateFeatureChainingFeatureNotOne requires a Feature to have either no chainingFeatures or more than one
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.3.4 validateFeatureChainingFeatureNotOne
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.3.3.4:validateFeatureChainingFeatureNotOne
blocked_by=semantic-feature-chaining-rules
type=file
~~~
# SOURCE
~~~kerml
package Chains {
    classifier Thing {
        feature inner : Thing;
    }
    classifier Holder {
        feature outer : Thing;

        // Conforming: a chain of two chaining features.
        feature pair chains outer.inner;

        // Invalid: a chain of exactly one chaining feature.
        feature single chains outer;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "feature_chaining_single_operand")
        (source "semantic")
        (range (start 11 8) (end 11 36))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:e628da015705e64dee61557df7fdb7e604c164122196037973c2fee7d5259e9b") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Holder::outer"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Holder::pair"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureChaining (reference "outer::inner")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Holder::single"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureChaining (reference "outer")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Thing::inner"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Holder::outer"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Holder::pair"))) (kind featureChaining) (ordinal 0))
      (authored-target "outer::inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Thing::inner")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Holder::single"))) (kind featureChaining) (ordinal 0))
      (authored-target "outer")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Holder::outer")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Thing::inner"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Holder::outer"))) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Holder::outer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind featureChaining) (source (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Holder::pair"))) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Thing::inner"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Holder::pair"))) (kind featureChaining) (ordinal 0)))
    (relationship (kind featureChaining) (source (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Holder::single"))) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Holder::outer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Holder::single"))) (kind featureChaining) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Thing::inner"))) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Thing::inner"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Holder::outer"))) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Holder::pair"))) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Holder::single"))) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Thing::inner"))) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Thing"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Holder::outer")))
      (featured-by (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Holder")))
      (type (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Holder::single")))
      (featured-by (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Holder")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Thing")))
      (subtype (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Holder::outer")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Thing::inner")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Thing::inner")))
      (featured-by (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Thing")))
      (type (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Thing")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (range (start 5 24) (end 5 29)) (probe (position 5 24))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Holder::outer"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (range (start 8 28) (end 8 39)) (probe (position 8 28))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Holder::pair"))) (kind featureChaining) (ordinal 0) (authored-target "outer::inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Thing::inner")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (range (start 11 30) (end 11 35)) (probe (position 11 30))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Holder::single"))) (kind featureChaining) (ordinal 0) (authored-target "outer")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Holder::outer")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (range (start 2 24) (end 2 29)) (probe (position 2 24))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Thing::inner"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_not_one.md") (qualified-name "Chains::Thing")))))
    )
  )
)
~~~
