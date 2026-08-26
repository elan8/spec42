# META
~~~ini
description=SysML TerminateActionUsage derives its terminated occurrence argument through an addressable canonical action identity
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=sysml-2.0:8.3.17.16:deriveTerminateActionUsageTerminatedOccurrenceArgument
libraries=none
~~~
# SOURCE
~~~sysml
package Actions { action def Procedure { action target; terminate target; } }
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (action-derived-fact (rule_id "sysml-2.0:8.3.17.16:deriveTerminateActionUsageTerminatedOccurrenceArgument") (source "Actions::Procedure::") (position 1) (outcome resolved))
  (action-derived-fact (rule_id "sysml-2.0:8.3.17.16:deriveTerminateActionUsageTerminatedOccurrenceArgument") (source "Actions::Procedure") (outcome absent)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_terminate_action_derived_fact.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:f2c5f97e672194110c1fb9446f52592c0a49d947de66f7d2362556867a7e6a42") (contract-version "constructor-expression-result-v8"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_terminate_action_derived_fact.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_terminate_action_derived_fact.md") (qualified-name "Actions::Procedure"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_terminate_action_derived_fact.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind terminate-action) (ordinal 0))))) (kind terminate-action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (terminateTarget (reference "target")))))
    (declaration (id (node (document "memory://snapshot/sysml_terminate_action_derived_fact.md") (qualified-name "Actions::Procedure::target"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_terminate_action_derived_fact.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind terminate-action) (ordinal 0))))) (kind terminateTarget) (ordinal 0))
      (authored-target "target")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_terminate_action_derived_fact.md") (qualified-name "Actions::Procedure::target")))))
  )
  (relationships
    (relationship (kind terminateTarget) (source (node (document "memory://snapshot/sysml_terminate_action_derived_fact.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind terminate-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_terminate_action_derived_fact.md") (qualified-name "Actions::Procedure::target"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_terminate_action_derived_fact.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind terminate-action) (ordinal 0))))) (kind terminateTarget) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_terminate_action_derived_fact.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind terminate-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_terminate_action_derived_fact.md") (qualified-name "Actions::Procedure"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_terminate_action_derived_fact.md") (qualified-name "Actions::Procedure::target"))) (target (node (document "memory://snapshot/sysml_terminate_action_derived_fact.md") (qualified-name "Actions::Procedure"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_terminate_action_derived_fact.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind terminate-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_terminate_action_derived_fact.md") (qualified-name "Actions::Procedure")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_terminate_action_derived_fact.md") (qualified-name "Actions::Procedure::target")))
      (featured-by (node (document "memory://snapshot/sysml_terminate_action_derived_fact.md") (qualified-name "Actions::Procedure")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_terminate_action_derived_fact.md") (range (start 0 66) (end 0 72)) (probe (position 0 66))
    (reference (id (source (node (document "memory://snapshot/sysml_terminate_action_derived_fact.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind terminate-action) (ordinal 0))))) (kind terminateTarget) (ordinal 0) (authored-target "target")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_terminate_action_derived_fact.md") (qualified-name "Actions::Procedure::target")))))
    )
  )
)
~~~
