# META
~~~ini
description=SysML checkMergeNodeIncomingSuccessionSpecialization desired semantics
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.17.13:checkMergeNodeIncomingSuccessionSpecialization
blocked_by=lowering-gap-specialization-succession-endpoint-subsetting
~~~
# SOURCE
~~~sysml
package Model { part def Parent; part def Child :> Parent; }
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics (specialization-check (rule_id "sysml-2.0:8.3.17.13:checkMergeNodeIncomingSuccessionSpecialization") (outcome satisfied)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_merge_node_incoming_succession_specialization.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:7f4094e38dc9b20dba31efad1ebdde8a9810d14b65296fd19166abc63cb6ab90") (contract-version "owned-cross-feature-typing-v4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_merge_node_incoming_succession_specialization.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_merge_node_incoming_succession_specialization.md") (qualified-name "Model::Child"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Parent")))))
    (declaration (id (node (document "memory://snapshot/sysml_merge_node_incoming_succession_specialization.md") (qualified-name "Model::Parent"))) (kind part-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_merge_node_incoming_succession_specialization.md") (qualified-name "Model::Child"))) (kind specialization) (ordinal 0))
      (authored-target "Parent")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_merge_node_incoming_succession_specialization.md") (qualified-name "Model::Parent")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_merge_node_incoming_succession_specialization.md") (qualified-name "Model::Child"))) (target (node (document "memory://snapshot/sysml_merge_node_incoming_succession_specialization.md") (qualified-name "Model::Parent"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_merge_node_incoming_succession_specialization.md") (qualified-name "Model::Child"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_merge_node_incoming_succession_specialization.md") (qualified-name "Model::Child")))
      (supertype (node (document "memory://snapshot/sysml_merge_node_incoming_succession_specialization.md") (qualified-name "Model::Parent")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/sysml_merge_node_incoming_succession_specialization.md") (qualified-name "Model::Parent")))
      (subtype (node (document "memory://snapshot/sysml_merge_node_incoming_succession_specialization.md") (qualified-name "Model::Child")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_merge_node_incoming_succession_specialization.md") (range (start 0 51) (end 0 57)) (probe (position 0 51))
    (reference (id (source (node (document "memory://snapshot/sysml_merge_node_incoming_succession_specialization.md") (qualified-name "Model::Child"))) (kind specialization) (ordinal 0) (authored-target "Parent")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_merge_node_incoming_succession_specialization.md") (qualified-name "Model::Parent")))))
    )
  )
)
~~~
