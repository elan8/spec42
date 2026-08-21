# META
~~~ini
description=KerML 8.3.3.3.4 validateFeatureCrossFeatureType requires the crossFeature of a Feature to have the same types as the Feature
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.3.4 validateFeatureCrossFeatureType
type=file
skip_validation=the crosses clause now lowers to a crossSubsetting relationship, but its target settles as unsupported_reference, so no resolved crossFeature is published for the rule to inspect
~~~
# SOURCE
~~~kerml
package Crossings {
    classifier Thing;
    classifier Other;
    assoc Link {
        end feature source : Thing;
        end feature target : Other;

        // Conforming: the crossing feature and its crossed feature share a type.
        feature sameType : Thing crosses source;

        // Invalid: the crossed feature has a different type.
        feature otherType : Thing crosses target;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_cross_feature_type.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "cross_feature_type_mismatch")
        (source "semantic")
        (range (start 11 8) (end 11 49))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_cross_feature_type.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 8 41) (end 8 47))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 11 42) (end 11 48))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:3e6c09c13894182f7110720a4952fcc28be3356e90fdefcc8c817dd09c228ca0") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Link"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Link::otherType"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")) (crossSubsetting (reference "target")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Link::sameType"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")) (crossSubsetting (reference "source")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Link::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Link::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Other")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Other"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Link::otherType"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Link::otherType"))) (kind crossSubsetting) (ordinal 0))
      (authored-target "target")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Link::sameType"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Link::sameType"))) (kind crossSubsetting) (ordinal 0))
      (authored-target "source")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Link::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Link::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Other")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Other")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Link::otherType"))) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Link::otherType"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Link::sameType"))) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Link::sameType"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Link::source"))) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Link::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Link::target"))) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Other"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Link::target"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Link::otherType")))
      (featured-by (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Link")))
      (type (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Link::sameType")))
      (featured-by (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Link")))
      (type (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Link::source")))
      (featured-by (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Link")))
      (type (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Link::target")))
      (featured-by (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Link")))
      (type (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Other")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Other")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Other")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Other")))
      (subtype (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Link::target")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Thing")))
      (subtype (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Link::otherType")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Link::sameType")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Link::source")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_cross_feature_type.md") (range (start 11 28) (end 11 33)) (probe (position 11 28))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Link::otherType"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_cross_feature_type.md") (range (start 11 42) (end 11 48)) (probe (position 11 42))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Link::otherType"))) (kind crossSubsetting) (ordinal 0) (authored-target "target")
      (outcome (status unsupported)))
    )
  )
  (query (document "memory://snapshot/kerml_feature_cross_feature_type.md") (range (start 8 27) (end 8 32)) (probe (position 8 27))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Link::sameType"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_cross_feature_type.md") (range (start 8 41) (end 8 47)) (probe (position 8 41))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Link::sameType"))) (kind crossSubsetting) (ordinal 0) (authored-target "source")
      (outcome (status unsupported)))
    )
  )
  (query (document "memory://snapshot/kerml_feature_cross_feature_type.md") (range (start 4 29) (end 4 34)) (probe (position 4 29))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Link::source"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_cross_feature_type.md") (range (start 5 29) (end 5 34)) (probe (position 5 29))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Link::target"))) (kind featureTyping) (ordinal 0) (authored-target "Other")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_cross_feature_type.md") (qualified-name "Crossings::Other")))))
    )
  )
)
~~~
