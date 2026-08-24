# META
~~~ini
description=SysML 8.3.17.4 checkActionUsageStateActionRedefinition requires entry, do, or exit action usages to redefine their StateAction feature
specification=OMG SysML 2.0 (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.17.4:checkActionUsageStateActionRedefinition
blocked_by=lowering-gap-redefinition-state-subaction-membership
type=file
~~~
# SOURCE
~~~sysml
package Redefinition { action def Work; }
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics (redefinition-check (rule_id "sysml-2.0:8.3.17.4:checkActionUsageStateActionRedefinition") (outcome satisfied)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_action_usage_state_action_redefinition.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:ab0feb3ad95d162313a3e4e46bfdcc6de55b12a27312c56984eb59d9fb5ae094") (contract-version "parser-owned-resolution-v2"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_action_usage_state_action_redefinition.md") (qualified-name "Redefinition"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_action_usage_state_action_redefinition.md") (qualified-name "Redefinition::Work"))) (kind action-def) (membership (kind owning) (visibility default)))
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
