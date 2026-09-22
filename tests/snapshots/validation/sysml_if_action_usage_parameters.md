# META
~~~ini
description=SysML 8.3.17.10 validateIfActionUsageParameters requires an IfActionUsage to have at least two owned input parameters
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.17.10 validateIfActionUsageParameters
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.17.10:validateIfActionUsageParameters
type=file
~~~
# SOURCE
~~~sysml
// Conforming: IfNode (SysML BNF 1123-1138) authors a condition plus a braced then-branch.
// There is no `then` keyword; ActionBodyParameter is always `{ ActionBodyItem* }`.
//
// The violating side has no textual counterpart: the production always authors a condition and
// at least one branch, so a source document cannot produce an IfActionUsage with fewer than two
// owned input parameters.
package Actions {
    action def Act {
        if true {
            action a1;
        } else {
            action a2;
        }
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_if_action_usage_parameters.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_if_action_usage_parameters.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:e4d0ae60ff4633a8b6d66289f87800dc4682b84f91aba866723362fc116d5008"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (qualified-name "Actions::Act"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind if) (ordinal 0))))) (kind if) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "a1"))))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "a2"))))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind if) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (qualified-name "Actions::Act"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "a1"))))) (target (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind if) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "a2"))))) (target (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind if) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind if) (ordinal 0))))) (state literal) (value (kind boolean) (boolean true)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind if) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (qualified-name "Actions::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "a1")))))
      (featured-by (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind if) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind if) (ordinal 0)) (named (kind action) (name "a2")))))
      (featured-by (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind if) (ordinal 0)))))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/sysml_if_action_usage_parameters.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind if) (ordinal 0))))) (outcome resolved) (literal (value (kind boolean) (boolean true))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
