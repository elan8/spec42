# META
~~~ini
description=Generated Feature category specializations use direct DataType typing and Association end ownership
specification=OMG KerML 1.0
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.3.3.4:checkFeatureDataValueSpecialization
rule_id=kerml-1.0:8.3.3.3.4:checkFeatureEndSpecialization
type=file
libraries=standard
~~~
# SOURCE
~~~kerml
package FeatureCategorySpecializations {
    datatype Value;

    class Owner {
        feature data : Value;
    }

    assoc Association {
        end feature participant : Value;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind specialization) (source "FeatureCategorySpecializations::Owner::data") (target "Base::dataValues") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "FeatureCategorySpecializations::Association::participant") (target "Links::Link::participant") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_conditional_feature_category_specializations.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:331a4230d0693e966fca0aee63e04f521c9e1e39dcf2805a0eadaa9bc9a2658e") (contract-version "feature-chain-expression-result-v10") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Association"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Association::participant"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Value")))))
    (declaration (id (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Owner"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Owner::data"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Value")))))
    (declaration (id (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Value"))) (kind kerml-datatype) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Association::participant"))) (kind featureTyping) (ordinal 0))
      (authored-target "Value")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Value")))))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Owner::data"))) (kind featureTyping) (ordinal 0))
      (authored-target "Value")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Value")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Association::participant"))) (target (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Value"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Association::participant"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Owner::data"))) (target (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Value"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Owner::data"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Association"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Association::participant"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Association::participant"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Association::participant"))) (target (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Association"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Association::participant"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Owner"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Owner::data"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Owner::data"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Owner::data"))) (target (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Owner"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Value"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Association")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Association::participant")))
      (featured-by (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Association")))
      (type (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Value")) (provenance authored))
      (effective-type (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Value")) (source direct))
      (supertype (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Value")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Owner")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Owner::data")))
      (featured-by (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Owner")))
      (type (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Value")) (provenance authored))
      (effective-type (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Value")) (source direct))
      (supertype (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Value")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Value")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Association::participant")) (scopes any))
      (subtype (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Owner::data")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (range (start 8 34) (end 8 39)) (probe (position 8 34))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Association::participant"))) (kind featureTyping) (ordinal 0) (authored-target "Value")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Value")))))
    )
  )
  (query (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (range (start 4 23) (end 4 28)) (probe (position 4 23))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Owner::data"))) (kind featureTyping) (ordinal 0) (authored-target "Value")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_feature_category_specializations.md") (qualified-name "FeatureCategorySpecializations::Value")))))
    )
  )
)
~~~
