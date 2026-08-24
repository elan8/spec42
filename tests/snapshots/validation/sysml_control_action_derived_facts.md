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
blocked_by=lowering-gap-action-input-parameter-identities
libraries=none
~~~
# SOURCE
~~~sysml
package Actions { action def Procedure { if true { action thenStep; } else { action elseStep; } while true { action loopStep; } } }
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (action-derived-fact (rule_id "sysml-2.0:8.3.17.10:deriveIfActionUsageElseAction") (source "Actions::Procedure") (target "Actions::Procedure::elseStep") (outcome resolved))
  (action-derived-fact (rule_id "sysml-2.0:8.3.17.10:deriveIfActionUsageIfArgument") (source "Actions::Procedure") (outcome resolved))
  (action-derived-fact (rule_id "sysml-2.0:8.3.17.10:deriveIfActionUsageThenAction") (source "Actions::Procedure") (target "Actions::Procedure::thenStep") (outcome resolved))
  (action-derived-fact (rule_id "sysml-2.0:8.3.17.12:deriveLoopActionUsageBodyAction") (source "Actions::Procedure") (target "Actions::Procedure::loopStep") (outcome resolved))
  (action-derived-fact (rule_id "sysml-2.0:8.3.17.19:deriveWhileLoopActionUsageUntilArgument") (source "Actions::Procedure") (outcome resolved))
  (action-derived-fact (rule_id "sysml-2.0:8.3.17.19:deriveWhileLoopActionUsageWhileArgument") (source "Actions::Procedure") (outcome resolved)))
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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:8a834d3a69289cabbb3194d8b706d9c7100d0748104c3170a06efd7e93a95dde") (contract-version "lossless-publication-completeness-v3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (qualified-name "Actions::Procedure"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind if) (ordinal 0))))) (kind if) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind while) (ordinal 0))))) (kind while) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "elseStep"))))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind while) (ordinal 0)) (named (kind action) (name "loopStep"))))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "thenStep"))))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind if) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (qualified-name "Actions::Procedure"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind while) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (qualified-name "Actions::Procedure"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "elseStep"))))) (target (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind if) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind while) (ordinal 0)) (named (kind action) (name "loopStep"))))) (target (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind while) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "thenStep"))))) (target (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind if) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind if) (ordinal 0))))) (state literal) (value (kind boolean) (boolean true)))
    (evaluated (declaration (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind while) (ordinal 0))))) (state literal) (value (kind boolean) (boolean true)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind if) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (qualified-name "Actions::Procedure")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind while) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (qualified-name "Actions::Procedure")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "elseStep")))))
      (featured-by (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind if) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind while) (ordinal 0)) (named (kind action) (name "loopStep")))))
      (featured-by (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind while) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "thenStep")))))
      (featured-by (node (document "memory://snapshot/sysml_control_action_derived_facts.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind if) (ordinal 0)))))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
