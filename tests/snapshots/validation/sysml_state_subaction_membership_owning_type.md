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
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:d8378c5dc59b5b1b77154afe94aeff91b7b7dba715fb586b20ec599cf5b23b24"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_state_subaction_membership_owning_type.md") (qualified-name "States"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_subaction_membership_owning_type.md") (qualified-name "States::Good"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_subaction_membership_owning_type.md") (qualified-name "States::Good::started"))) (kind entry-action-binding) (membership (kind feature) (visibility default) (role entry)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_subaction_membership_owning_type.md") (qualified-name "States::Good::started"))) (target (node (document "memory://snapshot/sysml_state_subaction_membership_owning_type.md") (qualified-name "States::Good"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_state_subaction_membership_owning_type.md") (qualified-name "States::Good::started")))
      (featured-by (node (document "memory://snapshot/sysml_state_subaction_membership_owning_type.md") (qualified-name "States::Good")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
