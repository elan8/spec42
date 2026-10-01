# META
~~~ini
description=KerML 8.3.4.10.2 validateFeatureValueOverriding requires every Feature directly or indirectly redefined by the featureWithValue of a FeatureValue to have only default FeatureValues
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.10.2 validateFeatureValueOverriding
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.4.10.2:validateFeatureValueOverriding
type=file
~~~
# SOURCE
~~~kerml
package Values {
    classifier Thing;
    classifier Base {
        feature fixed : Thing = null;
        feature defaulted : Thing default null;
    }
    classifier Conforming specializes Base {
        // Conforming: the redefined feature carries a default value.
        feature :>> defaulted = null;
    }
    classifier Invalid specializes Base {
        // Invalid: the redefined feature carries a non-default value.
        feature :>> fixed = null;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_value_overriding.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "feature_value_overrides_non_default")
        (source "semantic")
        (range (start 12 8) (end 12 33))
        (related-information
          (related
            (uri "memory://snapshot/kerml_feature_value_overriding.md")
            (range (start 3 8) (end 3 37))
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
  (document "memory://snapshot/kerml_feature_value_overriding.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "feature_value_overrides_non_default")
        (source "semantic")
        (range (start 12 8) (end 12 33))
        (related-information
          (related
            (uri "memory://snapshot/kerml_feature_value_overriding.md")
            (range (start 3 8) (end 3 37))
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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:93d3d15360bfa39e1ac9e297c730173b669a36910a44be4f6f928a2ac4a11d5c"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base::defaulted"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Base")) (named (kind kerml-feature) (name "defaulted")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Base")) (named (kind kerml-feature) (name "defaulted")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (default true) (operator false)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Base")) (named (kind kerml-feature) (name "defaulted")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Base")) (named (kind kerml-feature) (name "defaulted")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Base")) (named (kind kerml-feature) (name "defaulted")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base::fixed"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Base")) (named (kind kerml-feature) (name "fixed")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Base")) (named (kind kerml-feature) (name "fixed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Base")) (named (kind kerml-feature) (name "fixed")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Base")) (named (kind kerml-feature) (name "fixed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Base")) (named (kind kerml-feature) (name "fixed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Conforming"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Conforming")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (effective-identification (name "defaulted") (short-name absent) (provenance first-redefinition)) (feature-value (kind bind) (value (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Conforming")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Conforming")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "defaulted")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Conforming")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Conforming")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Conforming")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Invalid"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Invalid")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (effective-identification (name "fixed") (short-name absent) (provenance first-redefinition)) (feature-value (kind bind) (value (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Invalid")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Invalid")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "fixed")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Invalid")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Invalid")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Invalid")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base::defaulted"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base::fixed"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Conforming"))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Conforming")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "defaulted")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base::defaulted")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Invalid"))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Invalid")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "fixed")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base::fixed")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base::defaulted"))) (target (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base::defaulted"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base::fixed"))) (target (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base::fixed"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Conforming"))) (target (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Conforming"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Conforming")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base::defaulted"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Conforming")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Invalid"))) (target (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Invalid"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Invalid")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base::fixed"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Invalid")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base::defaulted"))) (target (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Base")) (named (kind kerml-feature) (name "defaulted")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Base")) (named (kind kerml-feature) (name "defaulted")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Base")) (named (kind kerml-feature) (name "defaulted")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base::fixed"))) (target (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Base")) (named (kind kerml-feature) (name "fixed")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Base")) (named (kind kerml-feature) (name "fixed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Base")) (named (kind kerml-feature) (name "fixed")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Conforming")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Conforming"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Conforming")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Conforming"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Conforming")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Conforming")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Invalid")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Invalid"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Invalid")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Invalid"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Invalid")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Invalid")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Base")) (named (kind kerml-feature) (name "defaulted")) (anonymous (kind kerml-expression) (ordinal 0))))) (state unsupported))
    (evaluated (declaration (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Base")) (named (kind kerml-feature) (name "fixed")) (anonymous (kind kerml-expression) (ordinal 0))))) (state unsupported))
    (evaluated (declaration (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Conforming")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (state unsupported))
    (evaluated (declaration (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Invalid")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (state unsupported))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base")))
      (subtype (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Conforming")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Invalid")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base::defaulted")))
      (featured-by (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base")))
      (type (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Thing")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Conforming")) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Base")) (named (kind kerml-feature) (name "defaulted")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Base")) (named (kind kerml-feature) (name "defaulted")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Base")) (named (kind kerml-feature) (name "defaulted")) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base::fixed")))
      (featured-by (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base")))
      (type (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Thing")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Invalid")) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Base")) (named (kind kerml-feature) (name "fixed")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Base")) (named (kind kerml-feature) (name "fixed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Base")) (named (kind kerml-feature) (name "fixed")) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Conforming")))
      (supertype (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Conforming")) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Conforming")))
      (effective-type (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base::defaulted"))))
      (supertype (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base::defaulted")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Conforming")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Conforming")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Conforming")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Conforming")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Invalid")))
      (supertype (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Invalid")) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Invalid")))
      (effective-type (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base::fixed"))))
      (supertype (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base::fixed")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Invalid")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Invalid")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Invalid")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Invalid")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Thing")))
      (subtype (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base::defaulted")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base::fixed")) (scopes any))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Base")) (named (kind kerml-feature) (name "defaulted")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome unsupported))
  (declaration (id (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Base")) (named (kind kerml-feature) (name "fixed")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome unsupported))
  (declaration (id (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Conforming")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome unsupported))
  (declaration (id (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Invalid")) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome unsupported))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_value_overriding.md") (range (start 4 28) (end 4 33)) (probe (position 4 28))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base::defaulted"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_value_overriding.md") (range (start 3 24) (end 3 29)) (probe (position 3 24))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base::fixed"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_value_overriding.md") (range (start 6 38) (end 6 42)) (probe (position 6 38))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Conforming"))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_value_overriding.md") (range (start 8 20) (end 8 29)) (probe (position 8 20))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Conforming")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "defaulted")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base::defaulted")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_value_overriding.md") (range (start 10 35) (end 10 39)) (probe (position 10 35))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Invalid"))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_value_overriding.md") (range (start 12 20) (end 12 25)) (probe (position 12 20))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_value_overriding.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Invalid")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "fixed")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_value_overriding.md") (qualified-name "Values::Base::fixed")))))
    )
  )
)
~~~
