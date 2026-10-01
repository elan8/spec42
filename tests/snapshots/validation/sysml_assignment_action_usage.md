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
type=file
~~~
# SOURCE
~~~sysml
// SysML has no `var` keyword: a Usage's isVariable is the derived mayTimeVary, true for a feature
// of an occurrence definition or usage unless it is a portion, a link participant, or a composite
// action. A package-owned feature has no owningType and so is never variable.
package Actions {
    attribute def Reading;
    attribute fixed : Reading;
    occurrence def Happening {
        attribute tracked : Reading;
    }
    action def Act {
        // Conforming: the assigned feature may vary over time.
        assign Happening::tracked := 1;

        // Invalid: the assigned feature cannot have time-varying values.
        assign fixed := 1;
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
        (range (start 14 8) (end 14 26))
        (related-information
          (related
            (uri "memory://snapshot/sysml_assignment_action_usage.md")
            (range (start 5 4) (end 5 30))
          )
        )
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
        (severity warning)
        (code "assignment_target_not_time_varying")
        (source "semantic")
        (range (start 14 8) (end 14 26))
        (related-information
          (related
            (uri "memory://snapshot/sysml_assignment_action_usage.md")
            (range (start 5 4) (end 5 30))
          )
        )
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:2929097ac3ea6e56306c715c0635b052f2c9987cef77f5eccfa056738e256d31"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Act"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 0))))) (kind assign) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (assignTarget (reference "Happening::tracked")))))
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 1))))) (kind assign) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (assignTarget (reference "fixed")))))
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Happening"))) (kind occurrence-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Happening::tracked"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Reading")))))
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Reading"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::fixed"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Reading")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 0))))) (kind assignTarget) (ordinal 0))
      (authored-target "Happening::tracked")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Happening::tracked")))))
    (reference (id (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 1))))) (kind assignTarget) (ordinal 0))
      (authored-target "fixed")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::fixed")))))
    (reference (id (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Happening::tracked"))) (kind featureTyping) (ordinal 0))
      (authored-target "Reading")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Reading")))))
    (reference (id (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::fixed"))) (kind featureTyping) (ordinal 0))
      (authored-target "Reading")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Reading")))))
  )
  (relationships
    (relationship (kind assignTarget) (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Happening::tracked"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 0))))) (kind assignTarget) (ordinal 0)))
    (relationship (kind assignTarget) (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::fixed"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 1))))) (kind assignTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Happening::tracked"))) (target (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Reading"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Happening::tracked"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::fixed"))) (target (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Reading"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::fixed"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Act"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Act"))) (provenance implied))
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
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Happening::tracked")))
      (featured-by (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Happening")))
      (type (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Reading")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Reading")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Reading")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Reading")))
      (subtype (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Happening::tracked")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::fixed")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::fixed")))
      (type (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Reading")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Reading")) (source direct))
      (supertype (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Reading")) (scopes any))
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
  (query (document "memory://snapshot/sysml_assignment_action_usage.md") (range (start 11 15) (end 11 33)) (probe (position 11 15))
    (reference (id (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 0))))) (kind assignTarget) (ordinal 0) (authored-target "Happening::tracked")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Happening::tracked")))))
    )
  )
  (query (document "memory://snapshot/sysml_assignment_action_usage.md") (range (start 14 15) (end 14 20)) (probe (position 14 15))
    (reference (id (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 1))))) (kind assignTarget) (ordinal 0) (authored-target "fixed")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::fixed")))))
    )
  )
  (query (document "memory://snapshot/sysml_assignment_action_usage.md") (range (start 7 28) (end 7 35)) (probe (position 7 28))
    (reference (id (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Happening::tracked"))) (kind featureTyping) (ordinal 0) (authored-target "Reading")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Reading")))))
    )
  )
  (query (document "memory://snapshot/sysml_assignment_action_usage.md") (range (start 5 22) (end 5 29)) (probe (position 5 22))
    (reference (id (source (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::fixed"))) (kind featureTyping) (ordinal 0) (authored-target "Reading")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_assignment_action_usage.md") (qualified-name "Actions::Reading")))))
    )
  )
)
~~~
