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
type=file
~~~
# SOURCE
~~~kerml
package Associations {
    struct Thing;
    struct Other;

    // Conforming: every owned end feature has exactly one type.
    assoc Typed {
        end feature source : Thing;
        end feature target : Thing;
    }

    // An end with no authored type takes its type from the library end it redefines
    // (`Links::BinaryLink::source` / `target`). No library is admitted here, so its types are
    // unsettled and the rule is left unanswered rather than reported.
    assoc Untyped {
        end feature source;
        end feature target;
    }

    assoc TwoTypes {
        end feature source : Thing;

        // Invalid: an owned end feature with two types.
        end feature target : Thing, Other;
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
        (range (start 22 8) (end 22 42))
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
      (diagnostic
        (severity warning)
        (code "association_end_type_not_one")
        (source "semantic")
        (range (start 22 8) (end 22 42))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:cdbdedd36d3f85fca90dd82f1f70174863c7b958c0318eb49701dcf069faa363"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Other"))) (kind kerml-structure) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Thing"))) (kind kerml-structure) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::TwoTypes"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::TwoTypes::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::TwoTypes::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")) (featureTyping (reference "Other")))))
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Typed"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Typed::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Typed::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Untyped"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Untyped::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)))
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Untyped::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::TwoTypes::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::TwoTypes::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::TwoTypes::target"))) (kind featureTyping) (ordinal 1))
      (authored-target "Other")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Other")))))
    (reference (id (source (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Typed::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Typed::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::TwoTypes::source"))) (target (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::TwoTypes::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::TwoTypes::target"))) (target (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::TwoTypes::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::TwoTypes::target"))) (target (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Other"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::TwoTypes::target"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Typed::source"))) (target (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Typed::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Typed::target"))) (target (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Typed::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::TwoTypes::source"))) (target (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::TwoTypes"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::TwoTypes::target"))) (target (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::TwoTypes"))) (provenance implied))
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
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Other")))
      (subtype (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::TwoTypes::target")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Thing")))
      (subtype (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::TwoTypes::source")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::TwoTypes::target")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Typed::source")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Typed::target")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::TwoTypes::source")))
      (featured-by (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::TwoTypes")))
      (type (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::TwoTypes::target")))
      (featured-by (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::TwoTypes")))
      (type (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Other")) (provenance authored))
      (type (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Other")) (source direct))
      (effective-type (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Other")) (scopes any))
      (supertype (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Thing")) (scopes any))
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
    )
    (declaration (id (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Untyped::target")))
      (featured-by (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Untyped")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_association_end_types.md") (range (start 19 29) (end 19 34)) (probe (position 19 29))
    (reference (id (source (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::TwoTypes::source"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_association_end_types.md") (range (start 22 29) (end 22 34)) (probe (position 22 29))
    (reference (id (source (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::TwoTypes::target"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_association_end_types.md") (range (start 22 36) (end 22 41)) (probe (position 22 36))
    (reference (id (source (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::TwoTypes::target"))) (kind featureTyping) (ordinal 1) (authored-target "Other")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_association_end_types.md") (qualified-name "Associations::Other")))))
    )
  )
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
)
~~~
