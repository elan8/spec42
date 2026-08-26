# META
~~~ini
description=SysML 8.3.17.19 validateWhileLoopActionUsage requires a WhileLoopActionUsage to have at least two owned input parameters
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.17.19 validateWhileLoopActionUsage
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.17.19:validateWhileLoopActionUsage
type=file
~~~
# SOURCE
~~~sysml
// Conforming: the while loop below owns the condition and body input parameters its concrete
// syntax implies.
//
// The violating side has no textual counterpart: SysML while syntax always authors a condition
// and a body, so a source document cannot produce a WhileLoopActionUsage with fewer than two
// owned input parameters.
package Actions {
    action def Act {
        while true { action step; }
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_while_loop_action_usage.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_while_loop_action_usage.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:92dff2dc3deaa07a83da522f67dcb9aec561a825fc0d8b4c12e029a9dbbda9bc") (contract-version "operator-expression-arguments-v7"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_while_loop_action_usage.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_while_loop_action_usage.md") (qualified-name "Actions::Act"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_while_loop_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind while) (ordinal 0))))) (kind while) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_while_loop_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind while) (ordinal 0)) (named (kind action) (name "step"))))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_while_loop_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind while) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_while_loop_action_usage.md") (qualified-name "Actions::Act"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_while_loop_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind while) (ordinal 0)) (named (kind action) (name "step"))))) (target (node (document "memory://snapshot/sysml_while_loop_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind while) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/sysml_while_loop_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind while) (ordinal 0))))) (state literal) (value (kind boolean) (boolean true)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_while_loop_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind while) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_while_loop_action_usage.md") (qualified-name "Actions::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_while_loop_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind while) (ordinal 0)) (named (kind action) (name "step")))))
      (featured-by (node (document "memory://snapshot/sysml_while_loop_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind while) (ordinal 0)))))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
