# META
~~~ini
description=SysML 8.3.18.4 validateStateSubactionMembershipOwningType requires the owningType of a StateSubactionMembership to be a StateDefinition or a StateUsage
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.18.4 validateStateSubactionMembershipOwningType
source_expectation=accepted
rule_family=validate
expectation=by_construction
evidence_reference=file:tests/snapshots/validation/sysml_grammar_restricted_membership_owners.md
rule_id=sysml-2.0:8.3.18.4:validateStateSubactionMembershipOwningType
type=file
~~~
# SOURCE
~~~sysml
// The violating side has no textual counterpart: the grammar admits this element only in
// state bodies (EntryActionMember, DoActionMember and ExitActionMember in StateBodyItem), whose owners all satisfy the rule.
// sysml_grammar_restricted_membership_owners.md pins the parser rejecting a forbidden owner.
package States {
    // Conforming: the subaction membership is owned by a state definition.
    state def Good {
        entry action started;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_state_subaction_membership_owning_type.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 6 21) (end 6 28))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:e5f70a01cf2ec2c7fb02d9810c1f7296aefa6201cb48ab1ae079f21cccb1927e"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_state_subaction_membership_owning_type.md") (qualified-name "States"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_subaction_membership_owning_type.md") (qualified-name "States::Good"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_subaction_membership_owning_type.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Good")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entry-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (entryActionBinding (reference "started")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_state_subaction_membership_owning_type.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Good")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0))
      (authored-target "started")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_subaction_membership_owning_type.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Good")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_state_subaction_membership_owning_type.md") (qualified-name "States::Good"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_state_subaction_membership_owning_type.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Good")) (anonymous (kind entry-action-binding) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_state_subaction_membership_owning_type.md") (qualified-name "States::Good")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_state_subaction_membership_owning_type.md") (range (start 6 21) (end 6 28)) (probe (position 6 21))
    (reference (id (source (node (document "memory://snapshot/sysml_state_subaction_membership_owning_type.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Good")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0) (authored-target "started")
      (outcome (status unresolved)))
    )
  )
)
~~~
