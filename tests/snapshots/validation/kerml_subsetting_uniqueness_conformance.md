# META
~~~ini
description=KerML 8.3.3.3.10 validateSubsettingUniquenessConformance requires the subsettingFeature of a unique subsettedFeature to be unique
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.3.10 validateSubsettingUniquenessConformance
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.3.3.10:validateSubsettingUniquenessConformance
type=file
~~~
# SOURCE
~~~kerml
package Subsettings {
    classifier Thing;
    classifier Holder {
        feature base[0..*] : Thing;

        // Conforming: a unique feature subsets a unique feature.
        feature distinct : Thing[0..*] subsets base;

        // Invalid: a non-unique feature cannot subset a unique one.
        feature repeated : Thing[0..*] nonunique subsets base;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "subsetting_uniqueness_mismatch")
        (source "semantic")
        (range (start 9 57) (end 9 61))
        (related-information
          (related
            (uri "memory://snapshot/kerml_subsetting_uniqueness_conformance.md")
            (range (start 3 8) (end 3 35))
          )
        )
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "subsetting_uniqueness_mismatch")
        (source "semantic")
        (range (start 9 57) (end 9 61))
        (related-information
          (related
            (uri "memory://snapshot/kerml_subsetting_uniqueness_conformance.md")
            (range (start 3 8) (end 3 35))
          )
        )
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:47c4d1aa10e61758013ef947233284f76a61b03d752839fb3d8eeb3e09f12eb1") (contract-version "lossless-publication-completeness-v3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::base"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::distinct"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")) (subsetting (reference "base")))))
    (declaration (id (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::repeated"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers nonunique) (multiplicity (lower 0) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")) (subsetting (reference "base")))))
    (declaration (id (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::base"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::distinct"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::distinct"))) (kind subsetting) (ordinal 0))
      (authored-target "base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::base")))))
    (reference (id (source (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::repeated"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::repeated"))) (kind subsetting) (ordinal 0))
      (authored-target "base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::base")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::base"))) (target (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::base"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::distinct"))) (target (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::distinct"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::distinct"))) (target (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::distinct"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::repeated"))) (target (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::repeated"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::repeated"))) (target (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::repeated"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::base"))) (target (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::distinct"))) (target (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::repeated"))) (target (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::base")))
      (featured-by (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder")))
      (type (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Thing")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::distinct")) (scopes any feature))
      (subtype (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::repeated")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::distinct")))
      (featured-by (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder")))
      (type (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Thing")) (source direct))
      (effective-type (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::base"))))
      (supertype (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::base")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::repeated")))
      (featured-by (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder")))
      (type (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Thing")) (source direct))
      (effective-type (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::base"))))
      (supertype (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::base")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Thing")))
      (subtype (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::base")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::distinct")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::repeated")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (range (start 3 29) (end 3 34)) (probe (position 3 29))
    (reference (id (source (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::base"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (range (start 6 27) (end 6 32)) (probe (position 6 27))
    (reference (id (source (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::distinct"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (range (start 6 47) (end 6 51)) (probe (position 6 47))
    (reference (id (source (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::distinct"))) (kind subsetting) (ordinal 0) (authored-target "base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::base")))))
    )
  )
  (query (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (range (start 9 27) (end 9 32)) (probe (position 9 27))
    (reference (id (source (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::repeated"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (range (start 9 57) (end 9 61)) (probe (position 9 57))
    (reference (id (source (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::repeated"))) (kind subsetting) (ordinal 0) (authored-target "base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_subsetting_uniqueness_conformance.md") (qualified-name "Subsettings::Holder::base")))))
    )
  )
)
~~~
