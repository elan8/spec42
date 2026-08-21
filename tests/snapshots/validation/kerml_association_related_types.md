# META
~~~ini
description=KerML 8.3.4.4.2 validateAssociationRelatedTypes requires a concrete Association to have at least two relatedTypes
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.4.2 validateAssociationRelatedTypes
type=file
skip_validation=no semantic rule checks the relatedType count of a concrete KerML association; the canonical code association_related_types_insufficient does not exist yet
~~~
# SOURCE
~~~kerml
package Associations {
    classifier Thing;

    // Conforming: a concrete association with two related types.
    assoc Binary {
        end feature source : Thing;
        end feature target : Thing;
    }

    // Conforming: an abstract association is exempt from the rule.
    abstract assoc Partial {
        end feature only : Thing;
    }

    // Invalid: a concrete association with fewer than two related types.
    assoc Unary {
        end feature only : Thing;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_association_related_types.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "association_related_types_insufficient")
        (source "semantic")
        (range (start 15 4) (end 15 17))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_association_related_types.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:ecc45616a9ece40c6db379720f95bb5b181835609b1e8d70230bc0ce7cebc4ac") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Binary"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Binary::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Binary::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Partial"))) (kind kerml-association) (membership (kind owning) (visibility default)) (facts (modifiers abstract)))
    (declaration (id (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Partial::only"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Unary"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Unary::only"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Binary::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Binary::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Partial::only"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Unary::only"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Binary::source"))) (target (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Binary::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Binary::target"))) (target (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Binary::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Partial::only"))) (target (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Partial::only"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Unary::only"))) (target (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Unary::only"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Binary::source")))
      (featured-by (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Binary")))
      (type (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Binary::target")))
      (featured-by (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Binary")))
      (type (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Partial::only")))
      (featured-by (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Partial")))
      (type (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Thing")))
      (subtype (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Binary::source")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Binary::target")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Partial::only")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Unary::only")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Unary::only")))
      (featured-by (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Unary")))
      (type (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Thing")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_association_related_types.md") (range (start 5 29) (end 5 34)) (probe (position 5 29))
    (reference (id (source (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Binary::source"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_association_related_types.md") (range (start 6 29) (end 6 34)) (probe (position 6 29))
    (reference (id (source (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Binary::target"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_association_related_types.md") (range (start 11 27) (end 11 32)) (probe (position 11 27))
    (reference (id (source (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Partial::only"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_association_related_types.md") (range (start 16 27) (end 16 32)) (probe (position 16 27))
    (reference (id (source (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Unary::only"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Thing")))))
    )
  )
)
~~~
