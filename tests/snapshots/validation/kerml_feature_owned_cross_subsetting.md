# META
~~~ini
description=KerML 8.3.3.3.4 validateFeatureOwnedCrossSubsetting allows a Feature at most one ownedSubsetting that is a CrossSubsetting
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.3.4 validateFeatureOwnedCrossSubsetting
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.3.3.4:validateFeatureOwnedCrossSubsetting
blocked_by=parser-gap-66-subsetting-clause-count
type=file
~~~
# SOURCE
~~~kerml
package Crossings {
    classifier Thing;
    assoc Link {
        end feature source : Thing;
        end feature target : Thing;

        // Conforming: a single crosses clause.
        feature one crosses source;

        // Invalid: two crosses clauses on one feature.
        feature two crosses source crosses target;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "feature_multiple_cross_subsettings")
        (source "semantic")
        (range (start 10 8) (end 10 50))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:f03f31fa4546734ad2c2e5d09e0ea98827c105d80cea03fb3c1c6542e1008424") (contract-version "semantic-metadata-projection-v6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::one"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (crossSubsetting (reference "source")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::two"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (crossSubsetting (reference "source")) (crossSubsetting (reference "target")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::one"))) (kind crossSubsetting) (ordinal 0))
      (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::source")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::two"))) (kind crossSubsetting) (ordinal 0))
      (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::source")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::two"))) (kind crossSubsetting) (ordinal 1))
      (authored-target "target")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::target")))))
  )
  (relationships
    (relationship (kind crossSubsetting) (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::one"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::source"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::one"))) (kind crossSubsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::source"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::target"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind crossSubsetting) (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::two"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::source"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::two"))) (kind crossSubsetting) (ordinal 0)))
    (relationship (kind crossSubsetting) (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::two"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::target"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::two"))) (kind crossSubsetting) (ordinal 1)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::one"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::source"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::target"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::two"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::one")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::source")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link")))
      (type (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::target")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link")))
      (type (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::two")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::source")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::target")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (range (start 7 28) (end 7 34)) (probe (position 7 28))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::one"))) (kind crossSubsetting) (ordinal 0) (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::source")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (range (start 3 29) (end 3 34)) (probe (position 3 29))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::source"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (range (start 4 29) (end 4 34)) (probe (position 4 29))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::target"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (range (start 10 28) (end 10 34)) (probe (position 10 28))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::two"))) (kind crossSubsetting) (ordinal 0) (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::source")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (range (start 10 43) (end 10 49)) (probe (position 10 43))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::two"))) (kind crossSubsetting) (ordinal 1) (authored-target "target")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::target")))))
    )
  )
)
~~~
