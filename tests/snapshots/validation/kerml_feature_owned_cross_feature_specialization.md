# META
~~~ini
description=KerML checkFeatureOwnedCrossFeatureSpecialization desired semantics
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.3.3.4:checkFeatureOwnedCrossFeatureSpecialization
type=file
~~~
# SOURCE
~~~kerml
package Crossings {
    classifier Thing;
    feature baseEndpoint : Thing;
    assoc Link {
        // The owned cross-feature must be typed from the end's effective type, including typing
        // inherited through redefinition rather than authored directly on the end.
        end crossing [1] feature endpoint :>> baseEndpoint;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship
    (kind feature_typing)
    (source "Crossings::Link::endpoint::crossing")
    (target "Crossings::Thing")
    (provenance implied)
    (outcome resolved))
  (specialization-check
    (rule_id "kerml-1.0:8.3.3.3.4:checkFeatureOwnedCrossFeatureSpecialization")
    (outcome satisfied)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:16a9c046aebc3586c59c30918eede45cafb82cc03c6f32ee80337020064c7a7f") (contract-version "feature-value-expression-results-v5"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::Link"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::Link::endpoint"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end) (cross-feature-projection (cross-feature (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::Link::endpoint::crossing"))) (owned-cross-feature (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::Link::endpoint::crossing"))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseEndpoint")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::Link::endpoint::crossing"))) (kind kerml-end) (membership (kind owning) (visibility default)) (facts (multiplicity (lower 1) (upper 1))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::baseEndpoint"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::Link::endpoint"))) (kind redefinition) (ordinal 0))
      (authored-target "baseEndpoint")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::baseEndpoint")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::baseEndpoint"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::Thing")))))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::Link::endpoint"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::baseEndpoint"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::Link::endpoint"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::baseEndpoint"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::baseEndpoint"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::Link::endpoint"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::Link"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::Link::endpoint::crossing"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::Thing"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::Link::endpoint")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::Link")))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::baseEndpoint"))))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::Thing")) (scopes any))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::baseEndpoint")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::Link::endpoint::crossing")))
      (type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::Thing")) (provenance implied))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::Thing")))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::Link::endpoint::crossing")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::baseEndpoint")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::baseEndpoint")))
      (type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::Thing")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::Link::endpoint")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (range (start 6 46) (end 6 58)) (probe (position 6 46))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::Link::endpoint"))) (kind redefinition) (ordinal 0) (authored-target "baseEndpoint")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::baseEndpoint")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (range (start 2 27) (end 2 32)) (probe (position 2 27))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::baseEndpoint"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_feature_specialization.md") (qualified-name "Crossings::Thing")))))
    )
  )
)
~~~
