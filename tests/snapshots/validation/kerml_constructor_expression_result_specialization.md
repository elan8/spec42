# META
~~~ini
description=KerML checkConstructorExpressionResultSpecialization desired semantics
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.8.3:checkConstructorExpressionResultSpecialization
~~~
# SOURCE
~~~kerml
package Model {
  classifier Thing;
  feature prototype;
  feature classified = new Thing();
  feature subsetted = new prototype();
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics (specialization-check (rule_id "kerml-1.0:8.3.4.8.3:checkConstructorExpressionResultSpecialization") (outcome satisfied)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_constructor_expression_result_specialization.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:3a06313c1f301541b2dcca7ac0c7f9644ab836e9e88094318b5cf7bc28e77cf4") (contract-version "constructor-expression-specialization-v9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (qualified-name "Model::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (qualified-name "Model::classified"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "classified")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "classified")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "classified")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "classified")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (constructor-expression (result (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "classified")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (invocationCallee (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "classified")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (qualified-name "Model::prototype"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (qualified-name "Model::subsetted"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "subsetted")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "subsetted")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "subsetted")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "subsetted")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (constructor-expression (result (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "subsetted")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (invocationCallee (reference "prototype")))))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "subsetted")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "classified")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (qualified-name "Model::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "subsetted")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0))
      (authored-target "prototype")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (qualified-name "Model::prototype")))))
  )
  (relationships
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "classified")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (qualified-name "Model::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "classified")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "subsetted")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (qualified-name "Model::prototype"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "subsetted")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (qualified-name "Model::classified"))) (target (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "classified")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "classified")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (qualified-name "Model::Thing"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "classified")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "classified")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (qualified-name "Model::subsetted"))) (target (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "subsetted")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "subsetted")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (qualified-name "Model::prototype"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "subsetted")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "subsetted")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "classified")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "subsetted")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (qualified-name "Model::Thing")))
      (subtype (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "classified")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (qualified-name "Model::classified")))
      (effective-type (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (qualified-name "Model::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "classified")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))))
      (supertype (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (qualified-name "Model::Thing")) (scopes any))
      (supertype (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "classified")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "classified")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "classified")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (type (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (qualified-name "Model::Thing")) (provenance implied))
      (effective-type (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (qualified-name "Model::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (qualified-name "Model::Thing")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (qualified-name "Model::classified")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (qualified-name "Model::prototype")))
      (subtype (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "subsetted")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (qualified-name "Model::subsetted")))
      (supertype (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (qualified-name "Model::prototype")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "subsetted")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "subsetted")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "subsetted")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (qualified-name "Model::prototype")) (scopes any feature))
      (subtype (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (qualified-name "Model::subsetted")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (range (start 3 27) (end 3 32)) (probe (position 3 27))
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "classified")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (qualified-name "Model::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (range (start 4 26) (end 4 35)) (probe (position 4 26))
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "subsetted")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0) (authored-target "prototype")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_result_specialization.md") (qualified-name "Model::prototype")))))
    )
  )
)
~~~
