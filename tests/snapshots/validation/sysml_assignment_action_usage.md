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
blocked_by=semantic-assignment-action-usage
type=file
~~~
# SOURCE
~~~sysml
package Actions {
    attribute def Reading;
    occurrence def Happening {
        // SysML RefPrefix has `constant` but no `var`/`variable` keyword (BNF 275-278).
        attribute tracked : Reading;
    }
    action def Act {
        constant attribute locked : Reading;

        // Conforming: the assigned feature is not constant, so it may time-vary.
        assign Happening::tracked := 1;

        // Invalid: a constant feature cannot have time-varying values.
        assign locked := 1;
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
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:68930a7ad2309c6961cbcde0c9b9d99063afbf53f126e2659a63c9e5cf66afb1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Act"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 0))))) (kind assign) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (assignTarget (reference "Happening::tracked")))))
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 1))))) (kind assign) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (assignTarget (reference "locked")))))
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Act::locked"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (modifiers constant)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Reading")))))
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Happening"))) (kind occurrence-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Happening::tracked"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Reading")))))
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Reading"))) (kind attribute-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 0))))) (kind assignTarget) (ordinal 0))
      (authored-target "Happening::tracked")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Happening::tracked")))))
    (reference (id (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 1))))) (kind assignTarget) (ordinal 0))
      (authored-target "locked")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Act::locked")))))
    (reference (id (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Act::locked"))) (kind featureTyping) (ordinal 0))
      (authored-target "Reading")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Reading")))))
    (reference (id (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Happening::tracked"))) (kind featureTyping) (ordinal 0))
      (authored-target "Reading")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Reading")))))
  )
  (relationships
    (relationship (kind assignTarget) (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Happening::tracked"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 0))))) (kind assignTarget) (ordinal 0)))
    (relationship (kind assignTarget) (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Act::locked"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 1))))) (kind assignTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Act::locked"))) (target (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Reading"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Act::locked"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Happening::tracked"))) (target (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Reading"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Happening::tracked"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Act"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Act"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Act::locked"))) (target (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Act"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Happening::tracked"))) (target (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Happening"))) (provenance implied))
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
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Act::locked")))
      (featured-by (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Act")))
      (type (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Reading")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Reading")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Reading")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Happening::tracked")))
      (featured-by (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Happening")))
      (type (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Reading")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Reading")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Reading")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Reading")))
      (subtype (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Act::locked")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Happening::tracked")) (scopes any))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 0))))) (outcome resolved) (literal (value (kind integer) (integer 1))))
  (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 1))))) (outcome resolved) (literal (value (kind integer) (integer 1))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_assignment_action_usage.md") (range (start 10 15) (end 10 33)) (probe (position 10 15))
    (reference (id (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 0))))) (kind assignTarget) (ordinal 0) (authored-target "Happening::tracked")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Happening::tracked")))))
    )
  )
  (query (document "memory://snapshot/sysml_assignment_action_usage.md") (range (start 13 15) (end 13 21)) (probe (position 13 15))
    (reference (id (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 1))))) (kind assignTarget) (ordinal 0) (authored-target "locked")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Act::locked")))))
    )
  )
  (query (document "memory://snapshot/sysml_assignment_action_usage.md") (range (start 7 36) (end 7 43)) (probe (position 7 36))
    (reference (id (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Act::locked"))) (kind featureTyping) (ordinal 0) (authored-target "Reading")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Reading")))))
    )
  )
  (query (document "memory://snapshot/sysml_assignment_action_usage.md") (range (start 4 28) (end 4 35)) (probe (position 4 28))
    (reference (id (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Happening::tracked"))) (kind featureTyping) (ordinal 0) (authored-target "Reading")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Reading")))))
    )
  )
)
~~~
