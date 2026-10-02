# META
~~~ini
description=KerML 8.3.4.8.3 checkConstructorExpressionResultFeatureRedefinition is violated by an argument past the instantiated type's public features
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.8.3:checkConstructorExpressionResultFeatureRedefinition
coverage_role=secondary
type=file
~~~
# SOURCE
~~~kerml
package Constructors {
    classifier Pair { feature first; feature second; }
    // The third argument has no public feature of Pair to redefine.
    feature tooMany = new Pair(1, 2, 3);
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics (redefinition-check (rule_id "kerml-1.0:8.3.4.8.3:checkConstructorExpressionResultFeatureRedefinition") (outcome violated)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:41c347de4071f57eb175fa6055025f587d0530d3b919d85107fe400b75860cc9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (qualified-name "Constructors"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (qualified-name "Constructors::Pair"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (qualified-name "Constructors::Pair::first"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (qualified-name "Constructors::Pair::second"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (qualified-name "Constructors::tooMany"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (constructor-expression (result (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (invocationCallee (reference "Pair")))))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 2))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0))
      (authored-target "Pair")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (qualified-name "Constructors::Pair")))))
  )
  (relationships
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (qualified-name "Constructors::Pair"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (qualified-name "Constructors::Pair::first"))) (target (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (qualified-name "Constructors::Pair"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (qualified-name "Constructors::Pair::second"))) (target (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (qualified-name "Constructors::Pair"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (qualified-name "Constructors::tooMany"))) (target (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (qualified-name "Constructors::Pair"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (qualified-name "Constructors::Pair::first"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (target (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (qualified-name "Constructors::Pair::second"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (target (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2))))) (target (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (qualified-name "Constructors::Pair")))
      (subtype (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (qualified-name "Constructors::Pair::first")))
      (featured-by (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (qualified-name "Constructors::Pair")))
      (subtype (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (qualified-name "Constructors::Pair::second")))
      (featured-by (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (qualified-name "Constructors::Pair")))
      (subtype (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (qualified-name "Constructors::tooMany")))
      (effective-type (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (qualified-name "Constructors::Pair")) (source inherited) (from (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))))
      (supertype (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (qualified-name "Constructors::Pair")) (scopes any))
      (supertype (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (type (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (qualified-name "Constructors::Pair")) (provenance implied))
      (effective-type (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (qualified-name "Constructors::Pair")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (qualified-name "Constructors::Pair")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (qualified-name "Constructors::tooMany")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (qualified-name "Constructors::Pair::first")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (qualified-name "Constructors::Pair::second")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (unsupported (literal (value (kind integer) (integer 1))) (literal (value (kind integer) (integer 2))) (literal (value (kind integer) (integer 3)))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (range (start 3 26) (end 3 30)) (probe (position 3 26))
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (path (named (kind package) (name "Constructors")) (named (kind kerml-feature) (name "tooMany")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0) (authored-target "Pair")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_result_feature_redefinition_violated.md") (qualified-name "Constructors::Pair")))))
    )
  )
)
~~~
