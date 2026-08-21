# META
~~~ini
description=KerML 8.3.3.3.4 validateFeatureChainingFeaturesNotSelf forbids a Feature from being one of its own chainingFeatures
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.3.4 validateFeatureChainingFeaturesNotSelf
type=file
skip_validation=the chains clause reaches semantics as FeatureRelationshipPart::Chaining, which lower_kerml_feature_relationship_parts reports as an unsupported member, so no featureChaining relationship is published
~~~
# SOURCE
~~~kerml
package Chains {
    classifier Thing {
        feature inner : Thing;
    }
    classifier Holder {
        feature outer : Thing;

        // Conforming: no chaining feature is the chained feature itself.
        feature good chains outer.inner;

        // Invalid: the chained feature is one of its own chaining features.
        feature bad chains bad.inner;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_chaining_features_not_self.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "feature_chaining_includes_self")
        (source "semantic")
        (range (start 11 8) (end 11 37))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_chaining_features_not_self.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 8 21) (end 8 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 11 20) (end 11 36))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:a72a36a73aca7458eec31e78f726dcdfa82e1002afe576dfa7b8e344918f9d7f") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_features_not_self.md") (qualified-name "Chains"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_features_not_self.md") (qualified-name "Chains::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_features_not_self.md") (qualified-name "Chains::Holder::bad"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_features_not_self.md") (qualified-name "Chains::Holder::good"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_features_not_self.md") (qualified-name "Chains::Holder::outer"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_features_not_self.md") (qualified-name "Chains::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_features_not_self.md") (qualified-name "Chains::Thing::inner"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chaining_features_not_self.md") (qualified-name "Chains::Holder::outer"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chaining_features_not_self.md") (qualified-name "Chains::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chaining_features_not_self.md") (qualified-name "Chains::Thing::inner"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chaining_features_not_self.md") (qualified-name "Chains::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_chaining_features_not_self.md") (qualified-name "Chains::Holder::outer"))) (target (node (document "memory://snapshot/kerml_feature_chaining_features_not_self.md") (qualified-name "Chains::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_chaining_features_not_self.md") (qualified-name "Chains::Holder::outer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_chaining_features_not_self.md") (qualified-name "Chains::Thing::inner"))) (target (node (document "memory://snapshot/kerml_feature_chaining_features_not_self.md") (qualified-name "Chains::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_chaining_features_not_self.md") (qualified-name "Chains::Thing::inner"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_features_not_self.md") (qualified-name "Chains::Holder::bad")))
      (featured-by (node (document "memory://snapshot/kerml_feature_chaining_features_not_self.md") (qualified-name "Chains::Holder")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_features_not_self.md") (qualified-name "Chains::Holder::good")))
      (featured-by (node (document "memory://snapshot/kerml_feature_chaining_features_not_self.md") (qualified-name "Chains::Holder")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_features_not_self.md") (qualified-name "Chains::Holder::outer")))
      (featured-by (node (document "memory://snapshot/kerml_feature_chaining_features_not_self.md") (qualified-name "Chains::Holder")))
      (type (node (document "memory://snapshot/kerml_feature_chaining_features_not_self.md") (qualified-name "Chains::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_chaining_features_not_self.md") (qualified-name "Chains::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_chaining_features_not_self.md") (qualified-name "Chains::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_features_not_self.md") (qualified-name "Chains::Thing")))
      (subtype (node (document "memory://snapshot/kerml_feature_chaining_features_not_self.md") (qualified-name "Chains::Holder::outer")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_chaining_features_not_self.md") (qualified-name "Chains::Thing::inner")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chaining_features_not_self.md") (qualified-name "Chains::Thing::inner")))
      (featured-by (node (document "memory://snapshot/kerml_feature_chaining_features_not_self.md") (qualified-name "Chains::Thing")))
      (type (node (document "memory://snapshot/kerml_feature_chaining_features_not_self.md") (qualified-name "Chains::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_chaining_features_not_self.md") (qualified-name "Chains::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_chaining_features_not_self.md") (qualified-name "Chains::Thing")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_chaining_features_not_self.md") (range (start 5 24) (end 5 29)) (probe (position 5 24))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chaining_features_not_self.md") (qualified-name "Chains::Holder::outer"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chaining_features_not_self.md") (qualified-name "Chains::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_chaining_features_not_self.md") (range (start 2 24) (end 2 29)) (probe (position 2 24))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chaining_features_not_self.md") (qualified-name "Chains::Thing::inner"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chaining_features_not_self.md") (qualified-name "Chains::Thing")))))
    )
  )
)
~~~
