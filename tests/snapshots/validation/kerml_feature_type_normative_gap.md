# META
~~~ini
description=KerML 8.3.3.3.4 deriveFeatureType has a pinned body that is not evaluable OCL, so Feature::type is not published as an exact contract
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.3.3.3.4:deriveFeatureType
blocked_by=normative-specification-gap-feature-type-body
libraries=standard
type=file
~~~
# SOURCE
~~~kerml
package FeatureTypes {
    classifier General;
    classifier Specific :> General;
    classifier Holder {
        feature current : General, Specific;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (action-derived-fact (rule_id "kerml-1.0:8.3.3.3.4:deriveFeatureType") (source "FeatureTypes::Holder::current") (target "FeatureTypes::Specific") (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_type_normative_gap.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:8e41ab8e6c9a3cb2d585a0463b0063ff09858db766d12c9dc1d638827b1eb5db") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes::General"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes::Holder::current"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "General")) (featureTyping (reference "Specific")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes::Specific"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "General")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes::Holder::current"))) (kind featureTyping) (ordinal 0))
      (authored-target "General")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes::General")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes::Holder::current"))) (kind featureTyping) (ordinal 1))
      (authored-target "Specific")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes::Specific")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes::Specific"))) (kind specialization) (ordinal 0))
      (authored-target "General")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes::General")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes::Holder::current"))) (target (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes::General"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes::Holder::current"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes::Holder::current"))) (target (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes::Specific"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes::Holder::current"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes::Specific"))) (target (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes::General"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes::Specific"))) (kind specialization) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes::Holder::current"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes::Holder::current"))) (target (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes::Holder"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes::General")))
      (subtype (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes::Holder::current")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes::Specific")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes::Holder::current")))
      (featured-by (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes::Holder")))
      (type (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes::General")) (provenance authored))
      (type (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes::Specific")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes::General")) (source direct))
      (effective-type (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes::Specific")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes::General")) (scopes any))
      (supertype (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes::Specific")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes::Specific")))
      (supertype (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes::General")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes::Holder::current")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_type_normative_gap.md") (range (start 4 26) (end 4 33)) (probe (position 4 26))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes::Holder::current"))) (kind featureTyping) (ordinal 0) (authored-target "General")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes::General")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_type_normative_gap.md") (range (start 4 35) (end 4 43)) (probe (position 4 35))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes::Holder::current"))) (kind featureTyping) (ordinal 1) (authored-target "Specific")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes::Specific")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_type_normative_gap.md") (range (start 2 27) (end 2 34)) (probe (position 2 27))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes::Specific"))) (kind specialization) (ordinal 0) (authored-target "General")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_type_normative_gap.md") (qualified-name "FeatureTypes::General")))))
    )
  )
)
~~~
