# META
~~~ini
description=KerML checkConstructorExpressionSpecialization desired semantics
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.8.3:checkConstructorExpressionSpecialization
blocked_by=lowering-gap-specialization-library-anchor-implied-edge
~~~
# SOURCE
~~~kerml
package Model { classifier Parent; classifier Child :> Parent; }
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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:370366f2defcf1963d4504332faa76b0e945b793f3777e811bd638d0544a3cd7") (contract-version "constructor-expression-result-v8"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (qualified-name "Model::Child"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Parent")))))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (qualified-name "Model::Parent"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (qualified-name "Model::Child"))) (kind specialization) (ordinal 0))
      (authored-target "Parent")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (qualified-name "Model::Parent")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (qualified-name "Model::Child"))) (target (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (qualified-name "Model::Parent"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (qualified-name "Model::Child"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (qualified-name "Model::Child")))
      (supertype (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (qualified-name "Model::Parent")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (qualified-name "Model::Parent")))
      (subtype (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (qualified-name "Model::Child")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_constructor_expression_specialization.md") (range (start 0 55) (end 0 61)) (probe (position 0 55))
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (qualified-name "Model::Child"))) (kind specialization) (ordinal 0) (authored-target "Parent")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_specialization.md") (qualified-name "Model::Parent")))))
    )
  )
)
~~~
