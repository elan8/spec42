# META
~~~ini
description=SysML 8.3.17.9 checkForLoopActionUsageVarRedefinition requires the loop variable to redefine Actions::ForLoopAction::var
specification=OMG SysML 2.0 (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.17.9:checkForLoopActionUsageVarRedefinition
blocked_by=lowering-gap-redefinition-for-loop-variable
type=file
~~~
# SOURCE
~~~sysml
package Redefinition { action def Work; }
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics (redefinition-check (rule_id "sysml-2.0:8.3.17.9:checkForLoopActionUsageVarRedefinition") (outcome satisfied)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:c101535fe066db867bb1ed8c152331cceb6c1acd320e29f207fb57996fba2b81") (contract-version "constructor-expression-specialization-v9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (qualified-name "Redefinition"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_usage_var_redefinition.md") (qualified-name "Redefinition::Work"))) (kind action-def) (membership (kind owning) (visibility default)))
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
