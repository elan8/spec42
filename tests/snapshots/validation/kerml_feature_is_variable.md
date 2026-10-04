# META
~~~ini
description=KerML 8.3.3.3.4 validateFeatureIsVariable requires a Feature with isVariable = true to have an owningType that specializes Occurrences::Occurrence
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.3.4 validateFeatureIsVariable
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.3.3.4:validateFeatureIsVariable
libraries=standard
type=file
~~~
# SOURCE
~~~kerml
package Variables {
    classifier Thing;

    // Conforming: the owning type of a variable feature is an occurrence.
    class Happening {
        var feature snapshotted : Thing;
    }

    // Invalid: the owning type does not specialize Occurrences::Occurrence.
    datatype Value {
        var feature shifting : Thing;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_is_variable.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "variable_feature_owner_not_occurrence")
        (source "semantic")
        (range (start 10 8) (end 10 37))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_is_variable.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "variable_feature_owner_not_occurrence")
        (source "semantic")
        (range (start 10 8) (end 10 37))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:7c88ad9c392d088cbd8f7e19c9e13ce6a62eb3c82b37d47f0509239f3a87c2d1") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_is_variable.md") (qualified-name "Variables"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_is_variable.md") (qualified-name "Variables::Happening"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_is_variable.md") (qualified-name "Variables::Happening::snapshotted"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers var)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_is_variable.md") (qualified-name "Variables::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_is_variable.md") (qualified-name "Variables::Value"))) (kind kerml-datatype) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_is_variable.md") (qualified-name "Variables::Value::shifting"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers var)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_is_variable.md") (qualified-name "Variables::Happening::snapshotted"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_is_variable.md") (qualified-name "Variables::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_is_variable.md") (qualified-name "Variables::Value::shifting"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_is_variable.md") (qualified-name "Variables::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_is_variable.md") (qualified-name "Variables::Happening::snapshotted"))) (target (node (document "memory://snapshot/kerml_feature_is_variable.md") (qualified-name "Variables::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_is_variable.md") (qualified-name "Variables::Happening::snapshotted"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_is_variable.md") (qualified-name "Variables::Value::shifting"))) (target (node (document "memory://snapshot/kerml_feature_is_variable.md") (qualified-name "Variables::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_is_variable.md") (qualified-name "Variables::Value::shifting"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_feature_is_variable.md") (qualified-name "Variables::Happening"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_feature_is_variable.md") (qualified-name "Variables::Happening::snapshotted"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_feature_is_variable.md") (qualified-name "Variables::Value"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_feature_is_variable.md") (qualified-name "Variables::Value::shifting"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_is_variable.md") (qualified-name "Variables::Happening")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_is_variable.md") (qualified-name "Variables::Happening::snapshotted")))
      (type (node (document "memory://snapshot/kerml_feature_is_variable.md") (qualified-name "Variables::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_is_variable.md") (qualified-name "Variables::Thing")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/kerml_feature_is_variable.md") (qualified-name "Variables::Thing")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_is_variable.md") (qualified-name "Variables::Thing")))
      (subtype (node (document "memory://snapshot/kerml_feature_is_variable.md") (qualified-name "Variables::Happening::snapshotted")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_is_variable.md") (qualified-name "Variables::Value::shifting")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_is_variable.md") (qualified-name "Variables::Value")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_is_variable.md") (qualified-name "Variables::Value::shifting")))
      (type (node (document "memory://snapshot/kerml_feature_is_variable.md") (qualified-name "Variables::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_is_variable.md") (qualified-name "Variables::Thing")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/kerml_feature_is_variable.md") (qualified-name "Variables::Thing")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_is_variable.md") (range (start 5 34) (end 5 39)) (probe (position 5 34))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_is_variable.md") (qualified-name "Variables::Happening::snapshotted"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_is_variable.md") (qualified-name "Variables::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_is_variable.md") (range (start 10 31) (end 10 36)) (probe (position 10 31))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_is_variable.md") (qualified-name "Variables::Value::shifting"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_is_variable.md") (qualified-name "Variables::Thing")))))
    )
  )
)
~~~
