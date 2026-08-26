# META
~~~ini
description=KerML 8.3.3.3.4 checkFeatureOwnedCrossFeatureTypeFeaturing requires an owned crossFeature to have featuringTypes consistent with the other ends
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.3.3.4:checkFeatureOwnedCrossFeatureTypeFeaturing
blocked_by=lowering-kerml-feature-relationships
type=file
~~~
# SOURCE
~~~kerml
package Crossings {
    classifier Thing;
    assoc Link {
        end feature source : Thing;
        end feature target : Thing;
        feature crossing crosses source;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship
    (kind type_featuring)
    (source "Crossings::Link::crossing")
    (target "Crossings::Thing")
    (provenance implied)
    (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:19a65111642347f67df6945ced5db6e64454fd17fe30f373e61e168b07bbe813") (contract-version "semantic-metadata-projection-v6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::crossing"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (crossSubsetting (reference "source")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::crossing"))) (kind crossSubsetting) (ordinal 0))
      (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::source")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Thing")))))
  )
  (relationships
    (relationship (kind crossSubsetting) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::crossing"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::source"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::crossing"))) (kind crossSubsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::source"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::target"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::crossing"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::source"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::target"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::crossing")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::source")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link")))
      (type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::target")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link")))
      (type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Thing")))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::source")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::target")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (range (start 5 33) (end 5 39)) (probe (position 5 33))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::crossing"))) (kind crossSubsetting) (ordinal 0) (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::source")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (range (start 3 29) (end 3 34)) (probe (position 3 29))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::source"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (range (start 4 29) (end 4 34)) (probe (position 4 29))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Link::target"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_type_featuring.md") (qualified-name "Crossings::Thing")))))
    )
  )
)
~~~
