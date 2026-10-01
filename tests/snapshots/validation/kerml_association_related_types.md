# META
~~~ini
description=KerML 8.3.4.4.2 validateAssociationRelatedTypes requires a concrete Association to have at least two relatedTypes
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.4.2 validateAssociationRelatedTypes
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.4.4.2:validateAssociationRelatedTypes
type=file
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

    // Conforming: both related types are inherited through the supertype's end features.
    assoc Derived specializes Binary;

    // Invalid: the only end feature, inherited from the supertype, relates one type.
    assoc DerivedUnary specializes Unary;
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
        (range (start 15 4) (end 17 5))
      )
      (diagnostic
        (severity warning)
        (code "association_related_types_insufficient")
        (source "semantic")
        (range (start 23 4) (end 23 41))
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
      (diagnostic
        (severity warning)
        (code "association_related_types_insufficient")
        (source "semantic")
        (range (start 15 4) (end 17 5))
      )
      (diagnostic
        (severity warning)
        (code "association_related_types_insufficient")
        (source "semantic")
        (range (start 23 4) (end 23 41))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:d92ee53911763de74dc4a4bf64dd33561ebbc282665bf7269b9abe1f44bad141"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Binary"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Binary::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Binary::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Derived"))) (kind kerml-association) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Binary")))))
    (declaration (id (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::DerivedUnary"))) (kind kerml-association) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Unary")))))
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
    (reference (id (source (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Derived"))) (kind specialization) (ordinal 0))
      (authored-target "Binary")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Binary")))))
    (reference (id (source (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::DerivedUnary"))) (kind specialization) (ordinal 0))
      (authored-target "Unary")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Unary")))))
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
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Derived"))) (target (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Binary"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Derived"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::DerivedUnary"))) (target (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Unary"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::DerivedUnary"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Partial::only"))) (target (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Partial::only"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Unary::only"))) (target (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Unary::only"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Binary::source"))) (target (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Binary"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Binary::target"))) (target (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Binary"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Partial::only"))) (target (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Partial"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Unary::only"))) (target (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Unary"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Binary")))
      (subtype (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Derived")) (scopes any subclassification))
    )
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
    (declaration (id (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Derived")))
      (supertype (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Binary")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::DerivedUnary")))
      (supertype (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Unary")) (scopes any subclassification))
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
    (declaration (id (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Unary")))
      (subtype (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::DerivedUnary")) (scopes any subclassification))
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
  (query (document "memory://snapshot/kerml_association_related_types.md") (range (start 20 30) (end 20 36)) (probe (position 20 30))
    (reference (id (source (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Derived"))) (kind specialization) (ordinal 0) (authored-target "Binary")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Binary")))))
    )
  )
  (query (document "memory://snapshot/kerml_association_related_types.md") (range (start 23 35) (end 23 40)) (probe (position 23 35))
    (reference (id (source (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::DerivedUnary"))) (kind specialization) (ordinal 0) (authored-target "Unary")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_association_related_types.md") (qualified-name "Associations::Unary")))))
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
