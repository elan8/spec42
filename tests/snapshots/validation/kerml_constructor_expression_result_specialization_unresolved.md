# META
~~~ini
description=KerML constructor result specialization remains unresolved when instantiatedType is unresolved
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.8.3:checkConstructorExpressionResultSpecialization
~~~
# SOURCE
~~~kerml
package Model {
  feature made = new Missing();
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics (specialization-check (rule_id "kerml-1.0:8.3.4.8.3:checkConstructorExpressionResultSpecialization") (outcome unresolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_constructor_expression_result_specialization_unresolved.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 1 21) (end 1 28))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:3165b6b7c0dc004bac95fe5522d73b77b2c63c9fe7969118996fae91dfffa202") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_specialization_unresolved.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_specialization_unresolved.md") (qualified-name "Model::made"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/kerml_constructor_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_constructor_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_constructor_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (constructor-expression (result (node (document "memory://snapshot/kerml_constructor_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (invocationCallee (reference "Missing")))))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0))
      (authored-target "Missing")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_constructor_expression_result_specialization_unresolved.md") (qualified-name "Model::made"))) (target (node (document "memory://snapshot/kerml_constructor_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_constructor_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_constructor_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_constructor_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_specialization_unresolved.md") (qualified-name "Model::made")))
      (supertype (node (document "memory://snapshot/kerml_constructor_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_constructor_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/kerml_constructor_expression_result_specialization_unresolved.md") (qualified-name "Model::made")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_constructor_expression_result_specialization_unresolved.md") (range (start 1 21) (end 1 28)) (probe (position 1 21))
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0) (authored-target "Missing")
      (outcome (status unresolved)))
    )
  )
)
~~~
