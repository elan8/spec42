# META
~~~ini
description=KerML checkConstructorExpressionSpecialization desired semantics
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.8.3:checkConstructorExpressionSpecialization
libraries=standard
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
(fixture-semantics (specialization-check (rule_id "kerml-1.0:8.3.4.8.3:checkConstructorExpressionSpecialization") (outcome satisfied)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_constructor_expression_specialization.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:026cf3977ddcfe3abe622fbfae6def29eee17d27c6df7564e5e25ad68d4f81d4") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (qualified-name "Model::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (qualified-name "Model::made"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (constructor-expression (result (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (invocationCallee (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (qualified-name "Model::Thing")))))
  )
  (relationships
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (qualified-name "Model::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (qualified-name "Model::made"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (qualified-name "Model::made"))) (target (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::constructorEvaluations"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (qualified-name "Model::Thing"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (qualified-name "Model::Thing")))
      (subtype (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (qualified-name "Model::made")))
      (effective-type (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (qualified-name "Model::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (qualified-name "Model::Thing")) (scopes any))
      (supertype (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::constructorEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (type (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (qualified-name "Model::Thing")) (provenance implied))
      (effective-type (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (qualified-name "Model::Thing")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (qualified-name "Model::Thing")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (subtype (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (qualified-name "Model::made")) (scopes any feature))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (unsupported))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_constructor_expression_specialization.md") (range (start 2 21) (end 2 26)) (probe (position 2 21))
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "made")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (qualified-name "Model::Thing")))))
    )
  )
)
~~~
