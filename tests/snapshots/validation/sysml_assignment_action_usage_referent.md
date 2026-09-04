# META
~~~ini
description=SysML 8.3.17.5 validateAssignmentActionUsageReferent requires an AssignmentActionUsage to have an ownedMembership that is not an OwningMembership and whose memberElement is a Feature
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.17.5 validateAssignmentActionUsageReferent
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.17.5:validateAssignmentActionUsageReferent
type=file
~~~
# SOURCE
~~~sysml
// Conforming: the assignment below names an existing feature as its referent, which is the only
// shape the assign syntax produces.
//
// The violating side has no textual counterpart: SysML assign syntax always authors a reference
// to the assigned feature, so a source document cannot produce an AssignmentActionUsage without
// that membership.
//
// Note: the referent membership of an assignment action is not published in SMG; this fixture pins only that the accepted side reports nothing.
package Actions {
    action def Act {
        ref attribute counter;
        assign counter := 1;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_assignment_action_usage_referent.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_assignment_action_usage_referent.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:40955cd791e238a100309db2e062212e2b142156a3b63e20593e0128be532efa"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage_referent.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage_referent.md") (qualified-name "Actions::Act"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage_referent.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 0))))) (kind assign) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (assignTarget (reference "counter")))))
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage_referent.md") (qualified-name "Actions::Act::counter"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (modifiers reference)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_assignment_action_usage_referent.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 0))))) (kind assignTarget) (ordinal 0))
      (authored-target "counter")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_assignment_action_usage_referent.md") (qualified-name "Actions::Act::counter")))))
  )
  (relationships
    (relationship (kind assignTarget) (source (node (document "memory://snapshot/sysml_assignment_action_usage_referent.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_assignment_action_usage_referent.md") (qualified-name "Actions::Act::counter"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_assignment_action_usage_referent.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 0))))) (kind assignTarget) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_assignment_action_usage_referent.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_assignment_action_usage_referent.md") (qualified-name "Actions::Act"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_assignment_action_usage_referent.md") (qualified-name "Actions::Act::counter"))) (target (node (document "memory://snapshot/sysml_assignment_action_usage_referent.md") (qualified-name "Actions::Act"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/sysml_assignment_action_usage_referent.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 0))))) (state literal) (value (kind integer) (integer 1)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage_referent.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_assignment_action_usage_referent.md") (qualified-name "Actions::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage_referent.md") (qualified-name "Actions::Act::counter")))
      (featured-by (node (document "memory://snapshot/sysml_assignment_action_usage_referent.md") (qualified-name "Actions::Act")))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage_referent.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 0))))) (outcome resolved) (literal (value (kind integer) (integer 1))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_assignment_action_usage_referent.md") (range (start 11 15) (end 11 22)) (probe (position 11 15))
    (reference (id (source (node (document "memory://snapshot/sysml_assignment_action_usage_referent.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Act")) (anonymous (kind assign) (ordinal 0))))) (kind assignTarget) (ordinal 0) (authored-target "counter")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_assignment_action_usage_referent.md") (qualified-name "Actions::Act::counter")))))
    )
  )
)
~~~
