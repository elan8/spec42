# META
~~~ini
description=KerML checkFeatureReferenceExpressionResultSpecialization desired semantics
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.8.5:checkFeatureReferenceExpressionResultSpecialization
~~~
# SOURCE
~~~kerml
package Model {
  classifier Thing {
    feature original : Thing;
    feature copied = original;
  }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics (specialization-check (rule_id "kerml-1.0:8.3.4.8.5:checkFeatureReferenceExpressionResultSpecialization") (outcome satisfied)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:db1f89eac6c27d944a229d2f14ae0f0b14c28abc8a2a182dbde670a911ec1861"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (qualified-name "Model::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (qualified-name "Model::Thing::copied"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Thing")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Thing")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Thing")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Thing")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "original")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Thing")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (qualified-name "Model::Thing::original"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Thing")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "original")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (qualified-name "Model::Thing::original")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (qualified-name "Model::Thing::original"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (qualified-name "Model::Thing")))))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Thing")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (qualified-name "Model::Thing::original"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Thing")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (qualified-name "Model::Thing::original"))) (target (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (qualified-name "Model::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (qualified-name "Model::Thing::original"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (qualified-name "Model::Thing::copied"))) (target (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (qualified-name "Model::Thing"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (qualified-name "Model::Thing::copied"))) (target (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Thing")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Thing")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Thing")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Thing")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (qualified-name "Model::Thing::original"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (qualified-name "Model::Thing::original"))) (target (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (qualified-name "Model::Thing"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Thing")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (qualified-name "Model::Thing")))
      (subtype (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (qualified-name "Model::Thing::original")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (qualified-name "Model::Thing::copied")))
      (featured-by (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (qualified-name "Model::Thing")))
      (effective-type (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (qualified-name "Model::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (qualified-name "Model::Thing::original"))))
      (supertype (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (qualified-name "Model::Thing")) (scopes any))
      (supertype (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Thing")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (qualified-name "Model::Thing::original")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Thing")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Thing")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (qualified-name "Model::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (qualified-name "Model::Thing::original"))))
      (supertype (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (qualified-name "Model::Thing")) (scopes any))
      (supertype (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (qualified-name "Model::Thing::original")) (scopes any feature))
      (subtype (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (qualified-name "Model::Thing::copied")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (qualified-name "Model::Thing::original")))
      (featured-by (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (qualified-name "Model::Thing")))
      (type (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (qualified-name "Model::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (qualified-name "Model::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (qualified-name "Model::Thing")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Thing")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Thing")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (feature-reference "original" (target (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (qualified-name "Model::Thing::original")))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (range (start 3 21) (end 3 29)) (probe (position 3 21))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Thing")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "original")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (qualified-name "Model::Thing::original")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (range (start 2 23) (end 2 28)) (probe (position 2 23))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (qualified-name "Model::Thing::original"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization.md") (qualified-name "Model::Thing")))))
    )
  )
)
~~~
