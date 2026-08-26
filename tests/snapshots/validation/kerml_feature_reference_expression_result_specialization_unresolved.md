# META
~~~ini
description=KerML checkFeatureReferenceExpressionResultSpecialization remains unresolved when its referent cannot be resolved
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.8.5:checkFeatureReferenceExpressionResultSpecialization
~~~
# SOURCE
~~~kerml
package Model {
  classifier Thing {
    feature copied = missing;
  }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics (specialization-check (rule_id "kerml-1.0:8.3.4.8.5:checkFeatureReferenceExpressionResultSpecialization") (outcome unresolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_reference_expression_result_specialization_unresolved.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 2 21) (end 2 28))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:2397e75f32ef445611202e7b92825b4cfbc1c494cfb92ca6461d36b13af1cc50"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization_unresolved.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization_unresolved.md") (qualified-name "Model::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization_unresolved.md") (qualified-name "Model::Thing::copied"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Thing")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Thing")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Thing")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Thing")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "missing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Thing")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Thing")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "missing")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization_unresolved.md") (qualified-name "Model::Thing::copied"))) (target (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization_unresolved.md") (qualified-name "Model::Thing"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization_unresolved.md") (qualified-name "Model::Thing::copied"))) (target (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Thing")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Thing")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Thing")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Thing")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (state unresolved-operand))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization_unresolved.md") (qualified-name "Model::Thing::copied")))
      (featured-by (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization_unresolved.md") (qualified-name "Model::Thing")))
      (supertype (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Thing")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Thing")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Thing")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization_unresolved.md") (qualified-name "Model::Thing::copied")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_reference_expression_result_specialization_unresolved.md") (range (start 2 21) (end 2 28)) (probe (position 2 21))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_reference_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Thing")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "missing")
      (outcome (status unresolved)))
    )
  )
)
~~~
