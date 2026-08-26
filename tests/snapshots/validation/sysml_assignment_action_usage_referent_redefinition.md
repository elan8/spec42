# META
~~~ini
description=SysML 8.3.17.5 checkAssignmentActionUsageReferentRedefinition requires the nested target feature to redefine the assignment referent
specification=OMG SysML 2.0 (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.17.5:checkAssignmentActionUsageReferentRedefinition
blocked_by=lowering-gap-redefinition-assignment-input-parameters
type=file
~~~
# SOURCE
~~~sysml
package Redefinition { action def Work; }
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics (redefinition-check (rule_id "sysml-2.0:8.3.17.5:checkAssignmentActionUsageReferentRedefinition") (outcome satisfied)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_assignment_action_usage_referent_redefinition.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:77021477e8e5c4f70f546c2b1204b4965a65a79197da57b0b39e8d44f4a1fa88") (contract-version "operator-expression-arguments-v7"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage_referent_redefinition.md") (qualified-name "Redefinition"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage_referent_redefinition.md") (qualified-name "Redefinition::Work"))) (kind action-def) (membership (kind owning) (visibility default)))
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
