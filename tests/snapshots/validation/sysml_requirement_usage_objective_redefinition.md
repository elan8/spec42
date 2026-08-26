# META
~~~ini
description=SysML 8.3.21.9 checkRequirementUsageObjectiveRedefinition requires objective requirement usages to redefine inherited case objectives
specification=OMG SysML 2.0 (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.21.9:checkRequirementUsageObjectiveRedefinition
blocked_by=lowering-gap-redefinition-objective-case-endpoints
type=file
~~~
# SOURCE
~~~sysml
package Redefinition { requirement def Need; }
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics (redefinition-check (rule_id "sysml-2.0:8.3.21.9:checkRequirementUsageObjectiveRedefinition") (outcome satisfied)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_requirement_usage_objective_redefinition.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:de0f2a191e4715bb28569bb9f1c156b65c9aaef3441f91908bdfca2c677b4851") (contract-version "owned-cross-feature-typing-v4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition.md") (qualified-name "Redefinition"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition.md") (qualified-name "Redefinition::Need"))) (kind requirement-def) (membership (kind owning) (visibility default)))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
