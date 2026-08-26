# META
~~~ini
description=KerML 8.3.3.3.4 validateFeatureOwnedReferenceSubsetting allows a Feature at most one ownedSubsetting that is a ReferenceSubsetting
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.3.4 validateFeatureOwnedReferenceSubsetting
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.3.3.4:validateFeatureOwnedReferenceSubsetting
blocked_by=parser-gap-66-subsetting-clause-count
type=file
~~~
# SOURCE
~~~kerml
package References {
    classifier Thing;
    classifier Holder {
        feature first : Thing;
        feature second : Thing;

        // Conforming: a single references clause.
        feature one references first;

        // Invalid: two references clauses on one feature.
        feature two references first references second;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "feature_multiple_reference_subsettings")
        (source "semantic")
        (range (start 10 8) (end 10 55))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:5e46d3170659298e90d529886f135498c767b204c48cd5be9bf1d2b7d913020b") (contract-version "constructor-expression-specialization-v9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::first"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::one"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (referenceSubsetting (reference "first")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::second"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::two"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (referenceSubsetting (reference "first")) (referenceSubsetting (reference "second")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::first"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::one"))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "first")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::first")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::second"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::two"))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "first")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::first")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::two"))) (kind referenceSubsetting) (ordinal 1))
      (authored-target "second")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::second")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::first"))) (target (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::first"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::one"))) (target (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::first"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::one"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::second"))) (target (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::second"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::two"))) (target (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::first"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::two"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::two"))) (target (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::second"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::two"))) (kind referenceSubsetting) (ordinal 1)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::first"))) (target (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::one"))) (target (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::second"))) (target (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::two"))) (target (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::first")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder")))
      (type (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::one")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::second")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder")))
      (type (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::two")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Thing")))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::first")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::second")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (range (start 3 24) (end 3 29)) (probe (position 3 24))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::first"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (range (start 7 31) (end 7 36)) (probe (position 7 31))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::one"))) (kind referenceSubsetting) (ordinal 0) (authored-target "first")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::first")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (range (start 4 25) (end 4 30)) (probe (position 4 25))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::second"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (range (start 10 31) (end 10 36)) (probe (position 10 31))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::two"))) (kind referenceSubsetting) (ordinal 0) (authored-target "first")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::first")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (range (start 10 48) (end 10 54)) (probe (position 10 48))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::two"))) (kind referenceSubsetting) (ordinal 1) (authored-target "second")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_reference_subsetting.md") (qualified-name "References::Holder::second")))))
    )
  )
)
~~~
