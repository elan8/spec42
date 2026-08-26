# META
~~~ini
description=SysML checkUsageVariationUsageSpecialization desired semantics
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.6.4:checkUsageVariationUsageSpecialization
blocked_by=lowering-gap-specialization-usage-variation-owner
~~~
# SOURCE
~~~sysml
package Model { part def Parent; part def Child :> Parent; }
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics (specialization-check (rule_id "sysml-2.0:8.3.6.4:checkUsageVariationUsageSpecialization") (outcome satisfied)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_usage_variation_usage_specialization.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:c95e186c4d5d174f762d78504d09cf7e5fdf47c156aec8d3704fffbb12c9beb4") (contract-version "owned-cross-feature-typing-v4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (qualified-name "Model::Child"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Parent")))))
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (qualified-name "Model::Parent"))) (kind part-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (qualified-name "Model::Child"))) (kind specialization) (ordinal 0))
      (authored-target "Parent")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (qualified-name "Model::Parent")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (qualified-name "Model::Child"))) (target (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (qualified-name "Model::Parent"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (qualified-name "Model::Child"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (qualified-name "Model::Child")))
      (supertype (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (qualified-name "Model::Parent")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (qualified-name "Model::Parent")))
      (subtype (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (qualified-name "Model::Child")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (range (start 0 51) (end 0 57)) (probe (position 0 51))
    (reference (id (source (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (qualified-name "Model::Child"))) (kind specialization) (ordinal 0) (authored-target "Parent")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_usage_variation_usage_specialization.md") (qualified-name "Model::Parent")))))
    )
  )
)
~~~
