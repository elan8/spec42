# META
~~~ini
description=SysML ForLoopActionUsage seqArgument preserves its first canonical argument expression
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=sysml-2.0:8.3.17.9:deriveForLoopActionUsageSeqArgument
libraries=none
~~~
# SOURCE
~~~sysml
package Actions { action def Procedure { for item in (1) { action step; } } }
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics (action-derived-fact (rule_id "sysml-2.0:8.3.17.9:deriveForLoopActionUsageSeqArgument") (source "Actions::Procedure::") (position 1) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_for_loop_action_seq_argument.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:680b4ad039fe94f220cd6d54eaeea00d3c9cc68bec0d68d7bd129b81058feb86") (contract-version "lossless-publication-completeness-v3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_seq_argument.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_seq_argument.md") (qualified-name "Actions::Procedure"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_seq_argument.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind for-loop) (ordinal 0))))) (kind for-loop) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_seq_argument.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind for-loop) (ordinal 0)) (named (kind for-loop-variable) (name "item"))))) (kind for-loop-variable) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_seq_argument.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "step"))))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_for_loop_action_seq_argument.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind for-loop) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_for_loop_action_seq_argument.md") (qualified-name "Actions::Procedure"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_for_loop_action_seq_argument.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind for-loop) (ordinal 0)) (named (kind for-loop-variable) (name "item"))))) (target (node (document "memory://snapshot/sysml_for_loop_action_seq_argument.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind for-loop) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_for_loop_action_seq_argument.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "step"))))) (target (node (document "memory://snapshot/sysml_for_loop_action_seq_argument.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind for-loop) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/sysml_for_loop_action_seq_argument.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind for-loop) (ordinal 0))))) (state literal) (value (kind integer) (integer 1)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_seq_argument.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind for-loop) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_for_loop_action_seq_argument.md") (qualified-name "Actions::Procedure")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_seq_argument.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind for-loop) (ordinal 0)) (named (kind for-loop-variable) (name "item")))))
      (featured-by (node (document "memory://snapshot/sysml_for_loop_action_seq_argument.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind for-loop) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/sysml_for_loop_action_seq_argument.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind for-loop) (ordinal 0)) (named (kind action) (name "step")))))
      (featured-by (node (document "memory://snapshot/sysml_for_loop_action_seq_argument.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind for-loop) (ordinal 0)))))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
