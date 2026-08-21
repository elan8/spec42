# META
~~~ini
description=KerML 8.3.3.3.2 validateCrossSubsettingCrossedFeature requires the crossedFeature of a CrossSubsetting to have exactly two chainingFeatures, the first of which is the other end Feature when the crossingFeature is one of two end Features
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.3.2 validateCrossSubsettingCrossedFeature
type=file
skip_validation=sysml_resolution does not lower KermlFeatureMember.crosses -- the parser types the clause, lower_kerml_feature_member reads only typing/subsets/redefines -- so no CrossSubsetting relationship is published
~~~
# SOURCE
~~~kerml
package Crossings {
    classifier Thing {
        feature inner : Thing;
    }
    assoc Binary {
        end feature source : Thing;
        end feature target : Thing;

        // Conforming: the crossed feature chains the other end feature first.
        end feature crossing : Thing crosses target.inner;
    }
    assoc Invalid {
        end feature source : Thing;
        end feature target : Thing;

        // Invalid: the crossed feature chains the crossing feature's own side first.
        end feature crossing : Thing crosses source.inner;
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
        (range (start 16 8) (end 16 58))
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
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:15fce48af9d8f2137bfa3879eb309062b0028eb03d63043f378729dd92b75b20") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary::crossing"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid::crossing"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing::inner"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary::crossing"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid::crossing"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing::inner"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary::crossing"))) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary::crossing"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary::source"))) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary::target"))) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid::crossing"))) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid::crossing"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid::source"))) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid::target"))) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing::inner"))) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing::inner"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary::crossing")))
      (featured-by (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary")))
      (type (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")) (scopes any))
    )
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
      (supertype (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid::crossing")))
      (featured-by (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid")))
      (type (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")) (scopes any))
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
      (supertype (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")))
      (subtype (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary::crossing")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary::source")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary::target")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid::crossing")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid::source")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid::target")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing::inner")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing::inner")))
      (featured-by (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")))
      (type (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (range (start 9 31) (end 9 36)) (probe (position 9 31))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary::crossing"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (range (start 5 29) (end 5 34)) (probe (position 5 29))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary::source"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (range (start 6 29) (end 6 34)) (probe (position 6 29))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Binary::target"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (range (start 16 31) (end 16 36)) (probe (position 16 31))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid::crossing"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (range (start 12 29) (end 12 34)) (probe (position 12 29))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid::source"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (range (start 13 29) (end 13 34)) (probe (position 13 29))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Invalid::target"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (range (start 2 24) (end 2 29)) (probe (position 2 24))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing::inner"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossed_feature.md") (qualified-name "Crossings::Thing")))))
    )
  )
)
~~~
