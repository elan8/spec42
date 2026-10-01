# META
~~~ini
description=KerML 8.3.3.3.4 checkFeatureEndRedefinition pairs the ends of a binary association with Links::BinaryLink::source and target from its implied library supertype
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.3.3.4:checkFeatureEndRedefinition
coverage_role=secondary
type=file
libraries=standard
~~~
# SOURCE
~~~kerml
package Redefinition {
    classifier A;
    classifier B;
    assoc Pair { end feature first : A; end feature second : B; }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind redefinition) (source "Redefinition::Pair::first") (target "Links::BinaryLink::source") (provenance implied) (outcome resolved))
  (relationship (kind redefinition) (source "Redefinition::Pair::second") (target "Links::BinaryLink::target") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "association_end_type_not_one")
        (source "semantic")
        (range (start 3 17) (end 3 39))
      )
      (diagnostic
        (severity warning)
        (code "association_end_type_not_one")
        (source "semantic")
        (range (start 3 40) (end 3 63))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:6a8de24a315c2b2a484921849d5d99c9a0d4b339e0e07cd3139d6bd5e6962797") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::A"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::B"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::Pair"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::Pair::first"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::Pair::second"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "B")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::Pair::first"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::A")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::Pair::second"))) (kind featureTyping) (ordinal 0))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::B")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::Pair::first"))) (target (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::Pair::first"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::Pair::second"))) (target (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::Pair::second"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::Pair"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::Pair"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::Pair::first"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::Pair::first"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::source"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::Pair::first"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::Pair::first"))) (target (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::Pair"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::Pair::second"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::Pair::second"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::target"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::Pair::second"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::Pair::second"))) (target (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::Pair"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::A")))
      (subtype (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::Pair::first")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::B")))
      (subtype (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::Pair::second")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::Pair")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::Pair::first")))
      (featured-by (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::Pair")))
      (type (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::A")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::participant"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::source"))))
      (supertype (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::A")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::participant")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::source")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::Pair::second")))
      (featured-by (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::Pair")))
      (type (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::B")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::B")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::participant"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::target"))))
      (supertype (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::B")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::participant")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::target")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (range (start 3 37) (end 3 38)) (probe (position 3 37))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::Pair::first"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::A")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (range (start 3 61) (end 3 62)) (probe (position 3 61))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::Pair::second"))) (kind featureTyping) (ordinal 0) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_redefinition_library_ends.md") (qualified-name "Redefinition::B")))))
    )
  )
)
~~~
