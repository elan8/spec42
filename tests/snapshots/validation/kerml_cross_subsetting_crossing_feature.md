# META
~~~ini
description=KerML 8.3.3.3.2 validateCrossSubsettingCrossingFeature requires the crossingFeature of a CrossSubsetting to be an end Feature owned by a Type with at least two end Features
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.3.2 validateCrossSubsettingCrossingFeature
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.3.3.2:validateCrossSubsettingCrossingFeature
blocked_by=lowering-kerml-feature-relationships
type=file
~~~
# SOURCE
~~~kerml
package Crossings {
    classifier Thing;
    assoc Binary {
        end feature source : Thing;
        end feature target : Thing;

        // Conforming: the crossing feature is an end feature of a type with two end features.
        end feature crossing : Thing crosses source;
    }
    classifier Holder {
        feature plain : Thing;

        // Invalid: the crossing feature is not an end feature of a type with two end features.
        feature crossing : Thing crosses plain;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "cross_subsetting_crossing_feature_invalid")
        (source "semantic")
        (range (start 13 8) (end 13 47))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:6bc84072f2c3ac4e93f3ce2526cef7122a8d64bbaea4c15c46a730488a68c82c") (contract-version "lossless-publication-completeness-v3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Binary"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Binary::crossing"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")) (crossSubsetting (reference "source")))))
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Binary::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Binary::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Holder::crossing"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")) (crossSubsetting (reference "plain")))))
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Holder::plain"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Binary::crossing"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Binary::crossing"))) (kind crossSubsetting) (ordinal 0))
      (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Binary::source")))))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Binary::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Binary::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Holder::crossing"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Holder::crossing"))) (kind crossSubsetting) (ordinal 0))
      (authored-target "plain")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Holder::plain")))))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Holder::plain"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Binary::crossing"))) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Binary::crossing"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind crossSubsetting) (source (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Binary::crossing"))) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Binary::source"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Binary::crossing"))) (kind crossSubsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Binary::source"))) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Binary::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Binary::target"))) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Binary::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Holder::crossing"))) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Holder::crossing"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind crossSubsetting) (source (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Holder::crossing"))) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Holder::plain"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Holder::crossing"))) (kind crossSubsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Holder::plain"))) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Holder::plain"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Binary::crossing"))) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Binary"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Binary::source"))) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Binary"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Binary::target"))) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Binary"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Holder::crossing"))) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Holder::plain"))) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Holder"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Binary::crossing")))
      (featured-by (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Binary")))
      (type (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Binary::source")))
      (featured-by (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Binary")))
      (type (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Binary::target")))
      (featured-by (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Binary")))
      (type (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Holder::crossing")))
      (featured-by (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Holder")))
      (type (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Holder::plain")))
      (featured-by (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Holder")))
      (type (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Thing")))
      (subtype (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Binary::crossing")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Binary::source")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Binary::target")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Holder::crossing")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Holder::plain")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (range (start 7 31) (end 7 36)) (probe (position 7 31))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Binary::crossing"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (range (start 7 45) (end 7 51)) (probe (position 7 45))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Binary::crossing"))) (kind crossSubsetting) (ordinal 0) (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Binary::source")))))
    )
  )
  (query (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (range (start 3 29) (end 3 34)) (probe (position 3 29))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Binary::source"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (range (start 4 29) (end 4 34)) (probe (position 4 29))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Binary::target"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (range (start 13 27) (end 13 32)) (probe (position 13 27))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Holder::crossing"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (range (start 13 41) (end 13 46)) (probe (position 13 41))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Holder::crossing"))) (kind crossSubsetting) (ordinal 0) (authored-target "plain")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Holder::plain")))))
    )
  )
  (query (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (range (start 10 24) (end 10 29)) (probe (position 10 24))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Holder::plain"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_subsetting_crossing_feature.md") (qualified-name "Crossings::Thing")))))
    )
  )
)
~~~
