# META
~~~ini
description=KerML checkSelectExpressionResultSpecialization desired semantics
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.8.18:checkSelectExpressionResultSpecialization
blocked_by=lowering-gap-specialization-expression-argument-result
~~~
# SOURCE
~~~kerml
package Model { classifier Parent; classifier Child :> Parent; }
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics (specialization-check (rule_id "kerml-1.0:8.3.4.8.18:checkSelectExpressionResultSpecialization") (outcome satisfied)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_select_expression_result_specialization.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:d288960494263a9e43cc18e27f03900052098b68cf35e06ede534ba6e3600b3c") (contract-version "owned-cross-feature-typing-v4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (qualified-name "Model::Child"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Parent")))))
    (declaration (id (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (qualified-name "Model::Parent"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (qualified-name "Model::Child"))) (kind specialization) (ordinal 0))
      (authored-target "Parent")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (qualified-name "Model::Parent")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (qualified-name "Model::Child"))) (target (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (qualified-name "Model::Parent"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (qualified-name "Model::Child"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (qualified-name "Model::Child")))
      (supertype (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (qualified-name "Model::Parent")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (qualified-name "Model::Parent")))
      (subtype (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (qualified-name "Model::Child")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_select_expression_result_specialization.md") (range (start 0 55) (end 0 61)) (probe (position 0 55))
    (reference (id (source (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (qualified-name "Model::Child"))) (kind specialization) (ordinal 0) (authored-target "Parent")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_select_expression_result_specialization.md") (qualified-name "Model::Parent")))))
    )
  )
)
~~~
