# META
~~~ini
description=SysML If, Loop, and While action derivations select exact ordered input-parameter roles
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=sysml-2.0:8.3.17.10:deriveIfActionUsageElseAction
rule_id=sysml-2.0:8.3.17.10:deriveIfActionUsageIfArgument
rule_id=sysml-2.0:8.3.17.10:deriveIfActionUsageThenAction
rule_id=sysml-2.0:8.3.17.12:deriveLoopActionUsageBodyAction
rule_id=sysml-2.0:8.3.17.19:deriveWhileLoopActionUsageUntilArgument
rule_id=sysml-2.0:8.3.17.19:deriveWhileLoopActionUsageWhileArgument
libraries=none
~~~
# SOURCE
~~~sysml
package Actions {
    action def IfProcedure {
        if true { action thenStep; } else { action elseStep; }
    }
    action def WhileProcedure {
        while true { action loopStep; }
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (action-derived-fact (rule_id "sysml-2.0:8.3.17.10:deriveIfActionUsageElseAction") (source "Actions::IfProcedure::") (target "Actions::IfProcedure::::elseStep") (outcome resolved))
  (action-derived-fact (rule_id "sysml-2.0:8.3.17.10:deriveIfActionUsageIfArgument") (source "Actions::IfProcedure::") (position 1) (outcome resolved))
  (action-derived-fact (rule_id "sysml-2.0:8.3.17.10:deriveIfActionUsageThenAction") (source "Actions::IfProcedure::") (target "Actions::IfProcedure::::thenStep") (outcome resolved))
  (action-derived-fact (rule_id "sysml-2.0:8.3.17.12:deriveLoopActionUsageBodyAction") (source "Actions::WhileProcedure::") (target "Actions::WhileProcedure::::loopStep") (outcome resolved))
  (action-derived-fact (rule_id "sysml-2.0:8.3.17.19:deriveWhileLoopActionUsageUntilArgument") (source "Actions::WhileProcedure::") (outcome absent))
  (action-derived-fact (rule_id "sysml-2.0:8.3.17.19:deriveWhileLoopActionUsageWhileArgument") (source "Actions::WhileProcedure::") (position 1) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_control_action_derived_facts.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:23c723007904efc3cf05c9c9b70c3a25828fd242ff58e139bafd01a913e75c58") (contract-version "feature-value-expression-results-v5"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (qualified-name "Actions::IfProcedure"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "IfProcedure")) (anonymous (kind if) (ordinal 0))))) (kind if) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "IfProcedure")) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "elseStep"))))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "IfProcedure")) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "thenStep"))))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (qualified-name "Actions::WhileProcedure"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "WhileProcedure")) (anonymous (kind while) (ordinal 0))))) (kind while) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "WhileProcedure")) (anonymous (kind while) (ordinal 0)) (named (kind action) (name "loopStep"))))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "IfProcedure")) (anonymous (kind if) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (qualified-name "Actions::IfProcedure"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "IfProcedure")) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "elseStep"))))) (target (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "IfProcedure")) (anonymous (kind if) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "IfProcedure")) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "thenStep"))))) (target (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "IfProcedure")) (anonymous (kind if) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "WhileProcedure")) (anonymous (kind while) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (qualified-name "Actions::WhileProcedure"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "WhileProcedure")) (anonymous (kind while) (ordinal 0)) (named (kind action) (name "loopStep"))))) (target (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "WhileProcedure")) (anonymous (kind while) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "IfProcedure")) (anonymous (kind if) (ordinal 0))))) (state literal) (value (kind boolean) (boolean true)))
    (evaluated (declaration (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "WhileProcedure")) (anonymous (kind while) (ordinal 0))))) (state literal) (value (kind boolean) (boolean true)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "IfProcedure")) (anonymous (kind if) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (qualified-name "Actions::IfProcedure")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "IfProcedure")) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "elseStep")))))
      (featured-by (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "IfProcedure")) (anonymous (kind if) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "IfProcedure")) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "thenStep")))))
      (featured-by (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "IfProcedure")) (anonymous (kind if) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "WhileProcedure")) (anonymous (kind while) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (qualified-name "Actions::WhileProcedure")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "WhileProcedure")) (anonymous (kind while) (ordinal 0)) (named (kind action) (name "loopStep")))))
      (featured-by (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "WhileProcedure")) (anonymous (kind while) (ordinal 0)))))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
