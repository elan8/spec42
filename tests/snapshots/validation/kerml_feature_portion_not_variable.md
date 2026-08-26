# META
~~~ini
description=KerML 8.3.3.3.4 validateFeaturePortionNotVariable forbids a Feature with isPortion = true from having isVariable = true
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.3.4 validateFeaturePortionNotVariable
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.3.3.4:validateFeaturePortionNotVariable
blocked_by=semantic-portion-feature-is-variable
type=file
~~~
# SOURCE
~~~kerml
package Portions {
    classifier Thing;
    classifier Owner {
        // Conforming: a portion feature that is not variable.
        portion feature stable : Thing;

        // Invalid: a portion feature must not be variable.
        portion var feature shifting : Thing;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_portion_not_variable.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "portion_feature_is_variable")
        (source "semantic")
        (range (start 7 8) (end 7 45))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_portion_not_variable.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:b68e2eb2d011ec6826dcdd44718db6186bfdfdb466f8a7d6eba4fb1f0ff8813e") (contract-version "semantic-metadata-projection-v6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_portion_not_variable.md") (qualified-name "Portions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_portion_not_variable.md") (qualified-name "Portions::Owner"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_portion_not_variable.md") (qualified-name "Portions::Owner::shifting"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers portion var)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_portion_not_variable.md") (qualified-name "Portions::Owner::stable"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers portion)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_portion_not_variable.md") (qualified-name "Portions::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_portion_not_variable.md") (qualified-name "Portions::Owner::shifting"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_portion_not_variable.md") (qualified-name "Portions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_portion_not_variable.md") (qualified-name "Portions::Owner::stable"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_portion_not_variable.md") (qualified-name "Portions::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_portion_not_variable.md") (qualified-name "Portions::Owner::shifting"))) (target (node (document "memory://snapshot/kerml_feature_portion_not_variable.md") (qualified-name "Portions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_portion_not_variable.md") (qualified-name "Portions::Owner::shifting"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_portion_not_variable.md") (qualified-name "Portions::Owner::stable"))) (target (node (document "memory://snapshot/kerml_feature_portion_not_variable.md") (qualified-name "Portions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_portion_not_variable.md") (qualified-name "Portions::Owner::stable"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_portion_not_variable.md") (qualified-name "Portions::Owner::stable"))) (target (node (document "memory://snapshot/kerml_feature_portion_not_variable.md") (qualified-name "Portions::Owner"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_portion_not_variable.md") (qualified-name "Portions::Owner::shifting")))
      (type (node (document "memory://snapshot/kerml_feature_portion_not_variable.md") (qualified-name "Portions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_portion_not_variable.md") (qualified-name "Portions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_portion_not_variable.md") (qualified-name "Portions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_portion_not_variable.md") (qualified-name "Portions::Owner::stable")))
      (featured-by (node (document "memory://snapshot/kerml_feature_portion_not_variable.md") (qualified-name "Portions::Owner")))
      (type (node (document "memory://snapshot/kerml_feature_portion_not_variable.md") (qualified-name "Portions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_portion_not_variable.md") (qualified-name "Portions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_portion_not_variable.md") (qualified-name "Portions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_portion_not_variable.md") (qualified-name "Portions::Thing")))
      (subtype (node (document "memory://snapshot/kerml_feature_portion_not_variable.md") (qualified-name "Portions::Owner::shifting")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_portion_not_variable.md") (qualified-name "Portions::Owner::stable")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_portion_not_variable.md") (range (start 7 39) (end 7 44)) (probe (position 7 39))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_portion_not_variable.md") (qualified-name "Portions::Owner::shifting"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_portion_not_variable.md") (qualified-name "Portions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_portion_not_variable.md") (range (start 4 33) (end 4 38)) (probe (position 4 33))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_portion_not_variable.md") (qualified-name "Portions::Owner::stable"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_portion_not_variable.md") (qualified-name "Portions::Thing")))))
    )
  )
)
~~~
