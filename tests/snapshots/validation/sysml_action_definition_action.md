# META
~~~ini
description=SysML ActionDefinition action retains ActionUsage values from effective usage
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=sysml-2.0:8.3.17.3:deriveActionDefinitionAction
blocked_by=lowering-gap-action-effective-usage-closure
libraries=none
~~~
# SOURCE
~~~sysml
package Actions { action def Procedure { action step; } }
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics (action-derived-fact (rule_id "sysml-2.0:8.3.17.3:deriveActionDefinitionAction") (source "Actions::Procedure") (target "Actions::Procedure::step") (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_action_definition_action.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:4aebe133566e17697390873d094d0bb06191cc919a9ff6aedf75a9ac4108fd1d") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Procedure"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Procedure::step"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Procedure::step"))) (target (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Procedure"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Procedure::step")))
      (featured-by (node (document "memory://snapshot/sysml_action_definition_action.md") (qualified-name "Actions::Procedure")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
