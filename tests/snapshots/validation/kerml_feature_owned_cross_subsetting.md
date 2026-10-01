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
type=file
~~~
# SOURCE
~~~kerml
package Crossings {
    classifier Thing {
        feature inner : Thing;
    }
    assoc Link {
        end feature source : Thing;

        // Conforming: a single crosses clause.
        end feature target : Thing crosses source.inner;
    }
    assoc Twice {
        end feature source : Thing;

        // Invalid: two crosses clauses on one feature.
        end feature target : Thing crosses source.inner crosses source.inner;
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
        (range (start 14 64) (end 14 76))
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
      (diagnostic
        (severity warning)
        (code "feature_multiple_cross_subsettings")
        (source "semantic")
        (range (start 14 64) (end 14 76))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:0de2b9ce6f6e2a582834632c33004aa484cbb9eb80e3f07d6db1ef8b1ac06fcd"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")) (crossSubsetting (reference "source::inner")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing::inner"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Twice"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Twice::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Twice::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")) (crossSubsetting (reference "source::inner")) (crossSubsetting (reference "source::inner")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::target"))) (kind crossSubsetting) (ordinal 0))
      (authored-target "source::inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing::inner")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing::inner"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Twice::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Twice::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Twice::target"))) (kind crossSubsetting) (ordinal 0))
      (authored-target "source::inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing::inner")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Twice::target"))) (kind crossSubsetting) (ordinal 1))
      (authored-target "source::inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing::inner")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::source"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::target"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind crossSubsetting) (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::target"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing::inner"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::target"))) (kind crossSubsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing::inner"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing::inner"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Twice::source"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Twice::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Twice::target"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Twice::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind crossSubsetting) (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Twice::target"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing::inner"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Twice::target"))) (kind crossSubsetting) (ordinal 0)))
    (relationship (kind crossSubsetting) (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Twice::target"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing::inner"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Twice::target"))) (kind crossSubsetting) (ordinal 1)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::source"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::target"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing::inner"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Twice::source"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Twice"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Twice::target"))) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Twice"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
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
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing::inner"))))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")) (scopes any))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing::inner")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::source")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::target")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing::inner")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Twice::source")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Twice::target")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing::inner")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")))
      (type (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::target")) (scopes any feature))
      (subtype (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Twice::target")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Twice::source")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Twice")))
      (type (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Twice::target")))
      (featured-by (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Twice")))
      (type (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")) (source direct))
      (effective-type (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing::inner"))))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")) (scopes any))
      (supertype (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing::inner")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (range (start 5 29) (end 5 34)) (probe (position 5 29))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::source"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (range (start 8 29) (end 8 34)) (probe (position 8 29))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::target"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (range (start 8 43) (end 8 55)) (probe (position 8 43))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Link::target"))) (kind crossSubsetting) (ordinal 0) (authored-target "source::inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing::inner")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (range (start 2 24) (end 2 29)) (probe (position 2 24))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing::inner"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (range (start 11 29) (end 11 34)) (probe (position 11 29))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Twice::source"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (range (start 14 29) (end 14 34)) (probe (position 14 29))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Twice::target"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (range (start 14 43) (end 14 55)) (probe (position 14 43))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Twice::target"))) (kind crossSubsetting) (ordinal 0) (authored-target "source::inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing::inner")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (range (start 14 64) (end 14 76)) (probe (position 14 64))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Twice::target"))) (kind crossSubsetting) (ordinal 1) (authored-target "source::inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_owned_cross_subsetting.md") (qualified-name "Crossings::Thing::inner")))))
    )
  )
)
~~~
