# META
~~~ini
description=KerML 8.3.4.4.2 validateAssociationEndTypes requires each ownedEndFeature of an Association to have exactly one type
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.4.2 validateAssociationEndTypes
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.4.4.2:validateAssociationEndTypes
blocked_by=semantic-association-end-type-not-one
type=file
~~~
# SOURCE
~~~kerml
package Associations {
    classifier Thing;
    classifier Other;

    // Conforming: every owned end feature has exactly one type.
    assoc Typed {
        end feature source : Thing;
        end feature target : Thing;
    }

    assoc Untyped {
        end feature source : Thing;

        // Invalid: an owned end feature with no type.
        end feature target;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_association_end_types.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "association_end_type_not_one")
        (source "semantic")
        (range (start 14 8) (end 14 27))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_association_end_types.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:32ba0ba84018c13d48cae0f2b8ac728832f3b70d8fb0cd59f488800d16d4d730") (contract-version "parser-owned-resolution-v2"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Other"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Typed"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Typed::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Typed::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Untyped"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Untyped::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Untyped::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Typed::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Typed::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Untyped::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Typed::source"))) (target (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Typed::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Typed::target"))) (target (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Typed::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Untyped::source"))) (target (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Untyped::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Typed::source"))) (target (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Typed"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Typed::target"))) (target (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Typed"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Untyped::source"))) (target (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Untyped"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Untyped::target"))) (target (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Untyped"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Thing")))
      (subtype (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Typed::source")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Typed::target")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Untyped::source")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Typed::source")))
      (featured-by (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Typed")))
      (type (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Typed::target")))
      (featured-by (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Typed")))
      (type (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Untyped::source")))
      (featured-by (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Untyped")))
      (type (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Untyped::target")))
      (featured-by (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Untyped")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_association_end_types.md") (range (start 6 29) (end 6 34)) (probe (position 6 29))
    (reference (id (source (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Typed::source"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_association_end_types.md") (range (start 7 29) (end 7 34)) (probe (position 7 29))
    (reference (id (source (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Typed::target"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_association_end_types.md") (range (start 11 29) (end 11 34)) (probe (position 11 29))
    (reference (id (source (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Untyped::source"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Thing")))))
    )
  )
)
~~~
