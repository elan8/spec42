# META
~~~ini
description=KerML 8.3.3.3.4 validateFeatureChainingFeatureConformance requires each chainingFeature after the first to be featured within the previous chainingFeature
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.3.4 validateFeatureChainingFeatureConformance
type=file
skip_validation=the pinned parser drops the `chains` clause -- a chaining feature publishes no featureChaining relationship -- so the rule has no chainingFeature list to check
~~~
# SOURCE
~~~kerml
package Chains {
    classifier Thing {
        feature inner : Thing;
    }
    classifier Other {
        feature unrelated : Thing;
    }
    classifier Holder {
        feature outer : Thing;

        // Conforming: inner is featured within outer's type.
        feature good chains outer.inner;

        // Invalid: unrelated is featured within Other, not within outer.
        feature bad chains outer.unrelated;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "feature_chaining_not_featured_within_previous")
        (source "semantic")
        (range (start 14 8) (end 14 43))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:d1a5dea51279844787b29fa786e9ec23645acce844479dcaac8ad66b62f3b5e5") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder::bad"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder::good"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder::outer"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Other"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Other::unrelated"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing::inner"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder::outer"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Other::unrelated"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing::inner"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder::outer"))) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder::outer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Other::unrelated"))) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Other::unrelated"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing::inner"))) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing::inner"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder::bad")))
      (featured-by (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder::good")))
      (featured-by (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder::outer")))
      (featured-by (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder")))
      (type (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Other::unrelated")))
      (featured-by (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Other")))
      (type (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing")))
      (subtype (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder::outer")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Other::unrelated")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing::inner")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing::inner")))
      (featured-by (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing")))
      (type (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (range (start 8 24) (end 8 29)) (probe (position 8 24))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Holder::outer"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (range (start 5 28) (end 5 33)) (probe (position 5 28))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Other::unrelated"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (range (start 2 24) (end 2 29)) (probe (position 2 24))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing::inner"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chaining_feature_conformance.md") (qualified-name "Chains::Thing")))))
    )
  )
)
~~~
