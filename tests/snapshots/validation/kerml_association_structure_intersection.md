# META
~~~ini
description=KerML 8.3.4.4.2 validateAssociationStructureIntersection requires an Association that is also a kind of Structure to be an AssociationStructure
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.4.2 validateAssociationStructureIntersection
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.4.4.2:validateAssociationStructureIntersection
type=file
~~~
# SOURCE
~~~kerml
// Conforming: an association that is also a structure is authored with the assoc struct
// keyword pair, which is exactly the AssociationStructure metaclass.
//
// The violating side has no textual counterpart: KerML concrete syntax has no spelling that
// makes a plain assoc also a kind of Structure, so a source document cannot author an
// Association that is a Structure without being an AssociationStructure.
package Associations {
    classifier Thing;
    assoc struct LinkObject {
        end feature source : Thing;
        end feature target : Thing;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_association_structure_intersection.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_association_structure_intersection.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:a0a1f6d0ea470baf21bb117374c68258e535e11b44c252080f1c30c0dbec5745") (contract-version "operator-expression-arguments-v7"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_association_structure_intersection.md") (qualified-name "Associations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_association_structure_intersection.md") (qualified-name "Associations::LinkObject"))) (kind kerml-association-structure) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_association_structure_intersection.md") (qualified-name "Associations::LinkObject::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_association_structure_intersection.md") (qualified-name "Associations::LinkObject::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_association_structure_intersection.md") (qualified-name "Associations::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_association_structure_intersection.md") (qualified-name "Associations::LinkObject::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_association_structure_intersection.md") (qualified-name "Associations::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_association_structure_intersection.md") (qualified-name "Associations::LinkObject::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_association_structure_intersection.md") (qualified-name "Associations::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_association_structure_intersection.md") (qualified-name "Associations::LinkObject::source"))) (target (node (document "memory://snapshot/kerml_association_structure_intersection.md") (qualified-name "Associations::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_association_structure_intersection.md") (qualified-name "Associations::LinkObject::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_association_structure_intersection.md") (qualified-name "Associations::LinkObject::target"))) (target (node (document "memory://snapshot/kerml_association_structure_intersection.md") (qualified-name "Associations::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_association_structure_intersection.md") (qualified-name "Associations::LinkObject::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_association_structure_intersection.md") (qualified-name "Associations::LinkObject::source"))) (target (node (document "memory://snapshot/kerml_association_structure_intersection.md") (qualified-name "Associations::LinkObject"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_association_structure_intersection.md") (qualified-name "Associations::LinkObject::target"))) (target (node (document "memory://snapshot/kerml_association_structure_intersection.md") (qualified-name "Associations::LinkObject"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_association_structure_intersection.md") (qualified-name "Associations::LinkObject::source")))
      (featured-by (node (document "memory://snapshot/kerml_association_structure_intersection.md") (qualified-name "Associations::LinkObject")))
      (type (node (document "memory://snapshot/kerml_association_structure_intersection.md") (qualified-name "Associations::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_association_structure_intersection.md") (qualified-name "Associations::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_association_structure_intersection.md") (qualified-name "Associations::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_association_structure_intersection.md") (qualified-name "Associations::LinkObject::target")))
      (featured-by (node (document "memory://snapshot/kerml_association_structure_intersection.md") (qualified-name "Associations::LinkObject")))
      (type (node (document "memory://snapshot/kerml_association_structure_intersection.md") (qualified-name "Associations::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_association_structure_intersection.md") (qualified-name "Associations::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_association_structure_intersection.md") (qualified-name "Associations::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_association_structure_intersection.md") (qualified-name "Associations::Thing")))
      (subtype (node (document "memory://snapshot/kerml_association_structure_intersection.md") (qualified-name "Associations::LinkObject::source")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_association_structure_intersection.md") (qualified-name "Associations::LinkObject::target")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_association_structure_intersection.md") (range (start 9 29) (end 9 34)) (probe (position 9 29))
    (reference (id (source (node (document "memory://snapshot/kerml_association_structure_intersection.md") (qualified-name "Associations::LinkObject::source"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_association_structure_intersection.md") (qualified-name "Associations::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_association_structure_intersection.md") (range (start 10 29) (end 10 34)) (probe (position 10 29))
    (reference (id (source (node (document "memory://snapshot/kerml_association_structure_intersection.md") (qualified-name "Associations::LinkObject::target"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_association_structure_intersection.md") (qualified-name "Associations::Thing")))))
    )
  )
)
~~~
