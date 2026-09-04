# META
~~~ini
description=KerML constructor expression specialization remains unresolved without its exact semantic-library anchor
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.8.3:checkConstructorExpressionSpecialization
~~~
# SOURCE
~~~kerml
package Model {
  classifier Thing;
  feature made = new Thing();
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics (specialization-check (rule_id "kerml-1.0:8.3.4.8.3:checkConstructorExpressionSpecialization") (outcome unresolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_constructor_expression_specialization_missing_library.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:679a4fe06c7c67328c2bb621d7f2d87f3b5f5af082690d3de4ddbbbdf17a1c53"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_specialization_missing_library.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_specialization_missing_library.md") (qualified-name "Model::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_specialization_missing_library.md") (qualified-name "Model::made"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/kerml_constructor_expression_specialization_missing_library.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_constructor_expression_specialization_missing_library.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_specialization_missing_library.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_constructor_expression_specialization_missing_library.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (constructor-expression (result (node (document "memory://snapshot/kerml_constructor_expression_specialization_missing_library.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (invocationCallee (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_specialization_missing_library.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_specialization_missing_library.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_specialization_missing_library.md") (qualified-name "Model::Thing")))))
  )
  (relationships
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/kerml_constructor_expression_specialization_missing_library.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_constructor_expression_specialization_missing_library.md") (qualified-name "Model::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_constructor_expression_specialization_missing_library.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_constructor_expression_specialization_missing_library.md") (qualified-name "Model::made"))) (target (node (document "memory://snapshot/kerml_constructor_expression_specialization_missing_library.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_constructor_expression_specialization_missing_library.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_constructor_expression_specialization_missing_library.md") (qualified-name "Model::Thing"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_constructor_expression_specialization_missing_library.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_constructor_expression_specialization_missing_library.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_constructor_expression_specialization_missing_library.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_specialization_missing_library.md") (qualified-name "Model::Thing")))
      (subtype (node (document "memory://snapshot/kerml_constructor_expression_specialization_missing_library.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_specialization_missing_library.md") (qualified-name "Model::made")))
      (effective-type (node (document "memory://snapshot/kerml_constructor_expression_specialization_missing_library.md") (qualified-name "Model::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_constructor_expression_specialization_missing_library.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))))
      (supertype (node (document "memory://snapshot/kerml_constructor_expression_specialization_missing_library.md") (qualified-name "Model::Thing")) (scopes any))
      (supertype (node (document "memory://snapshot/kerml_constructor_expression_specialization_missing_library.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_specialization_missing_library.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_constructor_expression_specialization_missing_library.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (type (node (document "memory://snapshot/kerml_constructor_expression_specialization_missing_library.md") (qualified-name "Model::Thing")) (provenance implied))
      (effective-type (node (document "memory://snapshot/kerml_constructor_expression_specialization_missing_library.md") (qualified-name "Model::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_constructor_expression_specialization_missing_library.md") (qualified-name "Model::Thing")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_constructor_expression_specialization_missing_library.md") (qualified-name "Model::made")) (scopes any feature))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_specialization_missing_library.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (unsupported))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_constructor_expression_specialization_missing_library.md") (range (start 2 21) (end 2 26)) (probe (position 2 21))
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_specialization_missing_library.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_specialization_missing_library.md") (qualified-name "Model::Thing")))))
    )
  )
)
~~~
