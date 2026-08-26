# META
~~~ini
description=KerML 8.3.4.4.2 validateAssociationBinarySpecialization forbids an Association with more than two associationEnds from specializing Links::BinaryLink
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.4.2 validateAssociationBinarySpecialization
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.4.4.2:validateAssociationBinarySpecialization
blocked_by=semantic-binary-association-end-count
type=file
libraries=standard
~~~
# SOURCE
~~~kerml
package Associations {
    // Conforming: a two-ended association may specialize BinaryLink.
    assoc Binary specializes Links::BinaryLink {
        end feature source : Base::Anything;
        end feature target : Base::Anything;
    }

    // Invalid: a three-ended association must not specialize BinaryLink.
    assoc Ternary specializes Links::BinaryLink {
        end feature source : Base::Anything;
        end feature middle : Base::Anything;
        end feature target : Base::Anything;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_association_binary_specialization.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "binary_association_end_count")
        (source "semantic")
        (range (start 8 30) (end 8 47))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_association_binary_specialization.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:0c145505999ca9183cf0e227df9c8fca4a7ad129d3f880e4790f00fb04aa94a7") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Binary"))) (kind kerml-association) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Links::BinaryLink")))))
    (declaration (id (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Binary::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Base::Anything")))))
    (declaration (id (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Binary::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Base::Anything")))))
    (declaration (id (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Ternary"))) (kind kerml-association) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Links::BinaryLink")))))
    (declaration (id (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Ternary::middle"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Base::Anything")))))
    (declaration (id (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Ternary::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Base::Anything")))))
    (declaration (id (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Ternary::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Base::Anything")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Binary"))) (kind specialization) (ordinal 0))
      (authored-target "Links::BinaryLink")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")))))
    (reference (id (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Binary::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base::Anything")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")))))
    (reference (id (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Binary::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base::Anything")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")))))
    (reference (id (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Ternary"))) (kind specialization) (ordinal 0))
      (authored-target "Links::BinaryLink")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")))))
    (reference (id (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Ternary::middle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base::Anything")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")))))
    (reference (id (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Ternary::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base::Anything")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")))))
    (reference (id (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Ternary::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base::Anything")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Binary"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Binary"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Binary::source"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Binary::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Binary::target"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Binary::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Ternary"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Ternary"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Ternary::middle"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Ternary::middle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Ternary::source"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Ternary::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Ternary::target"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Ternary::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Binary::source"))) (target (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Binary"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Binary::source"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Binary::source"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::source"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Binary::source"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Binary::target"))) (target (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Binary"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Binary::target"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Binary::target"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::target"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Binary::target"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Ternary::middle"))) (target (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Ternary"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Ternary::middle"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Ternary::middle"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Ternary::source"))) (target (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Ternary"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Ternary::source"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Ternary::source"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::source"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Ternary::source"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Ternary::target"))) (target (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Ternary"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Ternary::target"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Ternary::target"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::target"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Ternary::target"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Binary")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Binary::source")))
      (featured-by (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Binary")))
      (type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::participant"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::source"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::participant")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::source")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant")) (scopes any subclassification feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Binary::target")))
      (featured-by (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Binary")))
      (type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::participant"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::target"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::participant")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::target")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant")) (scopes any subclassification feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Ternary")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Ternary::middle")))
      (featured-by (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Ternary")))
      (type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Ternary::source")))
      (featured-by (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Ternary")))
      (type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::participant"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::source"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::participant")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::source")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant")) (scopes any subclassification feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Ternary::target")))
      (featured-by (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Ternary")))
      (type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::participant"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::target"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::participant")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::target")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant")) (scopes any subclassification feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_association_binary_specialization.md") (range (start 2 29) (end 2 46)) (probe (position 2 29))
    (reference (id (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Binary"))) (kind specialization) (ordinal 0) (authored-target "Links::BinaryLink")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")))))
    )
  )
  (query (document "memory://snapshot/kerml_association_binary_specialization.md") (range (start 3 29) (end 3 43)) (probe (position 3 29))
    (reference (id (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Binary::source"))) (kind featureTyping) (ordinal 0) (authored-target "Base::Anything")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")))))
    )
  )
  (query (document "memory://snapshot/kerml_association_binary_specialization.md") (range (start 4 29) (end 4 43)) (probe (position 4 29))
    (reference (id (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Binary::target"))) (kind featureTyping) (ordinal 0) (authored-target "Base::Anything")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")))))
    )
  )
  (query (document "memory://snapshot/kerml_association_binary_specialization.md") (range (start 8 30) (end 8 47)) (probe (position 8 30))
    (reference (id (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Ternary"))) (kind specialization) (ordinal 0) (authored-target "Links::BinaryLink")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")))))
    )
  )
  (query (document "memory://snapshot/kerml_association_binary_specialization.md") (range (start 10 29) (end 10 43)) (probe (position 10 29))
    (reference (id (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Ternary::middle"))) (kind featureTyping) (ordinal 0) (authored-target "Base::Anything")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")))))
    )
  )
  (query (document "memory://snapshot/kerml_association_binary_specialization.md") (range (start 9 29) (end 9 43)) (probe (position 9 29))
    (reference (id (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Ternary::source"))) (kind featureTyping) (ordinal 0) (authored-target "Base::Anything")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")))))
    )
  )
  (query (document "memory://snapshot/kerml_association_binary_specialization.md") (range (start 11 29) (end 11 43)) (probe (position 11 29))
    (reference (id (source (node (document "memory://snapshot/kerml_association_binary_specialization.md") (qualified-name "Associations::Ternary::target"))) (kind featureTyping) (ordinal 0) (authored-target "Base::Anything")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")))))
    )
  )
)
~~~
