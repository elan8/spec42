# META
~~~ini
description=KerML checkInvocationExpressionSpecialization remains unresolved when instantiatedType cannot be derived
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.8.8:checkInvocationExpressionSpecialization
coverage_role=secondary
~~~
# SOURCE
~~~kerml
package Model {
  feature invoked = Missing();
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics (specialization-check (rule_id "kerml-1.0:8.3.4.8.8:checkInvocationExpressionSpecialization") (outcome unresolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_invocation_expression_specialization_unresolved.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 1 20) (end 1 27))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:9946392a8c349885eefe591c2e68bbea022a1775ee1ac54e5a81a47e436d2974"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_specialization_unresolved.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_specialization_unresolved.md") (qualified-name "Model::invoked"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/kerml_invocation_expression_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "invoked")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_invocation_expression_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "invoked")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "invoked")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_invocation_expression_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "invoked")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (invocationCallee (reference "Missing")))))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "invoked")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "invoked")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0))
      (authored-target "Missing")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_invocation_expression_specialization_unresolved.md") (qualified-name "Model::invoked"))) (target (node (document "memory://snapshot/kerml_invocation_expression_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "invoked")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_invocation_expression_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "invoked")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_invocation_expression_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "invoked")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_invocation_expression_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "invoked")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_specialization_unresolved.md") (qualified-name "Model::invoked")))
      (supertype (node (document "memory://snapshot/kerml_invocation_expression_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "invoked")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "invoked")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_invocation_expression_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "invoked")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/kerml_invocation_expression_specialization_unresolved.md") (qualified-name "Model::invoked")) (scopes any feature))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "invoked")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (unsupported))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_invocation_expression_specialization_unresolved.md") (range (start 1 20) (end 1 27)) (probe (position 1 20))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "invoked")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0) (authored-target "Missing")
      (outcome (status unresolved)))
    )
  )
)
~~~
