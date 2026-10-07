# META
~~~ini
description=KerML 8.3.3.3.2 validateCrossSubsettingCrossedFeature requires the crossedFeature of a CrossSubsetting to have exactly two chainingFeatures, the first of which is the other end Feature when the crossingFeature is one of two end Features
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.3.2 validateCrossSubsettingCrossedFeature
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.3.3.2:validateCrossSubsettingCrossedFeature
type=file
~~~
# SOURCE
~~~kerml
package Crossings {
    classifier Thing {
        feature inner : Thing;
    }
    assoc Binary {
        end feature source : Thing;

        // Conforming: the crossed feature chains the other end feature first.
        end feature target : Thing crosses source.inner;
    }
    assoc Invalid {
        end feature source : Thing;

        // Invalid: the crossed feature chains the crossing feature's own side first.
        end feature target : Thing crosses target.inner;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "cross_subsetting_crossed_feature_invalid")
        (source "semantic")
        (range (start 14 43) (end 14 55))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "cross_subsetting_crossed_feature_invalid")
        (source "semantic")
        (range (start 14 43) (end 14 55))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:67c75756b2acb5f9e006083fcc152e509034f7fd3299df483be5e21553ccb87d"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")) (crossSubsetting (reference "source::inner")))))
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")) (crossSubsetting (reference "target::inner")))))
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing::inner"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary::target"))) (kind crossSubsetting) (ordinal 0))
      (authored-target "source::inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing::inner")))))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid::target"))) (kind crossSubsetting) (ordinal 0))
      (authored-target "target::inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing::inner")))))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing::inner"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary::source"))) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary::target"))) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind crossSubsetting) (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary::target"))) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing::inner"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary::target"))) (kind crossSubsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid::source"))) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid::target"))) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind crossSubsetting) (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid::target"))) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing::inner"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid::target"))) (kind crossSubsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing::inner"))) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing::inner"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary::source"))) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary::target"))) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid::source"))) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid::target"))) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing::inner"))) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary::source")))
      (featured-by (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary")))
      (type (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary::target")))
      (featured-by (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary")))
      (type (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")) (source direct))
      (effective-type (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing::inner"))))
      (supertype (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")) (scopes any))
      (supertype (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing::inner")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid::source")))
      (featured-by (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid")))
      (type (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid::target")))
      (featured-by (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid")))
      (type (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")) (source direct))
      (effective-type (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing::inner"))))
      (supertype (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")) (scopes any))
      (supertype (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing::inner")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")))
      (subtype (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary::source")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary::target")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid::source")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid::target")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing::inner")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing::inner")))
      (featured-by (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")))
      (type (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary::target")) (scopes any feature))
      (subtype (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid::target")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (range (start 5 29) (end 5 34)) (probe (position 5 29))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary::source"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (range (start 8 29) (end 8 34)) (probe (position 8 29))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary::target"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (range (start 8 43) (end 8 55)) (probe (position 8 43))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary::target"))) (kind crossSubsetting) (ordinal 0) (authored-target "source::inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing::inner")))))
    )
  )
  (query (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (range (start 11 29) (end 11 34)) (probe (position 11 29))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid::source"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (range (start 14 29) (end 14 34)) (probe (position 14 29))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid::target"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (range (start 14 43) (end 14 55)) (probe (position 14 43))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid::target"))) (kind crossSubsetting) (ordinal 0) (authored-target "target::inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing::inner")))))
    )
  )
  (query (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (range (start 2 24) (end 2 29)) (probe (position 2 24))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing::inner"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")))))
    )
  )
)
~~~
