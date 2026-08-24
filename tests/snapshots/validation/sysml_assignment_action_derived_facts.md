# META
~~~ini
description=SysML AssignmentActionUsage derives its target argument, value expression, and non-feature-membership referent from canonical action roles
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=sysml-2.0:8.3.17.5:deriveAssignmentActionUsageValueExpression
rule_id=sysml-2.0:8.3.17.5:deriveAssignmentUsageTargetArgument
blocked_by=lowering-gap-action-argument-identities
libraries=none
~~~
# SOURCE
~~~sysml
package Actions { action def Procedure { attribute target; assign target := 1; } }
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (action-derived-fact (rule_id "sysml-2.0:8.3.17.5:deriveAssignmentActionUsageValueExpression") (source "Actions::Procedure") (outcome resolved))
  (action-derived-fact (rule_id "sysml-2.0:8.3.17.5:deriveAssignmentUsageTargetArgument") (source "Actions::Procedure") (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_assignment_action_derived_facts.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:471ed72f0e7e9e1b007dd4096affbf7c95f008d4f6c5f7095cfebdce039a3bd7") (contract-version "parser-owned-resolution-v2"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_derived_facts.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_derived_facts.md") (qualified-name "Actions::Procedure"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind assign) (ordinal 0))))) (kind assign) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (assignTarget (reference "target")))))
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_derived_facts.md") (qualified-name "Actions::Procedure::target"))) (kind attribute) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_assignment_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind assign) (ordinal 0))))) (kind assignTarget) (ordinal 0))
      (authored-target "target")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_assignment_action_derived_facts.md") (qualified-name "Actions::Procedure::target")))))
  )
  (relationships
    (relationship (kind assignTarget) (source (node (document "memory://snapshot/sysml_assignment_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind assign) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_assignment_action_derived_facts.md") (qualified-name "Actions::Procedure::target"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_assignment_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind assign) (ordinal 0))))) (kind assignTarget) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_assignment_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind assign) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_assignment_action_derived_facts.md") (qualified-name "Actions::Procedure"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_assignment_action_derived_facts.md") (qualified-name "Actions::Procedure::target"))) (target (node (document "memory://snapshot/sysml_assignment_action_derived_facts.md") (qualified-name "Actions::Procedure"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/sysml_assignment_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind assign) (ordinal 0))))) (state literal) (value (kind integer) (integer 1)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind assign) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_assignment_action_derived_facts.md") (qualified-name "Actions::Procedure")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_derived_facts.md") (qualified-name "Actions::Procedure::target")))
      (featured-by (node (document "memory://snapshot/sysml_assignment_action_derived_facts.md") (qualified-name "Actions::Procedure")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_assignment_action_derived_facts.md") (range (start 0 66) (end 0 72)) (probe (position 0 66))
    (reference (id (source (node (document "memory://snapshot/sysml_assignment_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind assign) (ordinal 0))))) (kind assignTarget) (ordinal 0) (authored-target "target")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_assignment_action_derived_facts.md") (qualified-name "Actions::Procedure::target")))))
    )
  )
)
~~~
