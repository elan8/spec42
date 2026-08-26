# META
~~~ini
description=SysML 8.3.17.5 checkAssignmentActionUsageStartingAtRedefinition requires the target parameter feature to redefine startingAt
specification=OMG SysML 2.0 (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.17.5:checkAssignmentActionUsageStartingAtRedefinition
blocked_by=lowering-gap-redefinition-assignment-input-parameters
type=file
~~~
# SOURCE
~~~sysml
package Redefinition { action def Work; }
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics (redefinition-check (rule_id "sysml-2.0:8.3.17.5:checkAssignmentActionUsageStartingAtRedefinition") (outcome satisfied)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_assignment_action_usage_starting_at_redefinition.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:20ff68a957a2e25fa2a9add18409a4333d5a8f3b244a9e91b2e7c9e038810f98"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage_starting_at_redefinition.md") (qualified-name "Redefinition"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage_starting_at_redefinition.md") (qualified-name "Redefinition::Work"))) (kind action-def) (membership (kind owning) (visibility default)))
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
