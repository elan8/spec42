# META
~~~ini
description=SysML 8.3.17.5 validateAssignmentActionUsage requires the featureTarget of the referent of an AssignmentActionUsage to be able to have time-varying values
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.17.5 validateAssignmentActionUsage
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.17.5:validateAssignmentActionUsage
blocked_by=parser-gap-52-var-modifier
type=file
~~~
# SOURCE
~~~sysml
package Actions {
    attribute def Reading;
    occurrence def Happening {
        var attribute tracked : Reading;
    }
    action def Act {
        ref attribute constant : Reading;

        // Conforming: the assigned feature may vary over time.
        assign Happening::tracked := 1;

        // Invalid: the assigned feature cannot have time-varying values.
        assign constant := 1;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_assignment_action_usage.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "assignment_target_not_time_varying")
        (source "semantic")
        (range (start 12 8) (end 12 29))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_assignment_action_usage.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 3 8) (end 4 4))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 15) (end 9 33))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:ad3bd4be3cdd6a192ab46a4439f1c008923329802fc48b5d2426daf79c4826a5") (contract-version "operator-expression-arguments-v7"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Act"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 0))))) (kind assign) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (assignTarget (reference "Happening::tracked")))))
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 1))))) (kind assign) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (assignTarget (reference "constant")))))
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Act::constant"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (modifiers reference)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Reading")))))
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Happening"))) (kind occurrence-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Reading"))) (kind attribute-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 0))))) (kind assignTarget) (ordinal 0))
      (authored-target "Happening::tracked")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 1))))) (kind assignTarget) (ordinal 0))
      (authored-target "constant")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Act::constant")))))
    (reference (id (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Act::constant"))) (kind featureTyping) (ordinal 0))
      (authored-target "Reading")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Reading")))))
  )
  (relationships
    (relationship (kind assignTarget) (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Act::constant"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 1))))) (kind assignTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Act::constant"))) (target (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Reading"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Act::constant"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Act"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Act"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Act::constant"))) (target (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Act"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/sysml_assignment_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 0))))) (state literal) (value (kind integer) (integer 1)))
    (evaluated (declaration (node (document "memory://snapshot/sysml_assignment_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 1))))) (state literal) (value (kind integer) (integer 1)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Act::constant")))
      (featured-by (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Act")))
      (type (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Reading")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Reading")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Reading")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Reading")))
      (subtype (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Act::constant")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_assignment_action_usage.md") (range (start 9 15) (end 9 33)) (probe (position 9 15))
    (reference (id (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 0))))) (kind assignTarget) (ordinal 0) (authored-target "Happening::tracked")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/sysml_assignment_action_usage.md") (range (start 12 15) (end 12 23)) (probe (position 12 15))
    (reference (id (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 1))))) (kind assignTarget) (ordinal 0) (authored-target "constant")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Act::constant")))))
    )
  )
  (query (document "memory://snapshot/sysml_assignment_action_usage.md") (range (start 6 33) (end 6 40)) (probe (position 6 33))
    (reference (id (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Act::constant"))) (kind featureTyping) (ordinal 0) (authored-target "Reading")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Reading")))))
    )
  )
)
~~~
