# META
~~~ini
description=SysML 8.3.18.4 validateStateSubactionMembershipOwningType requires the owningType of a StateSubactionMembership to be a StateDefinition or a StateUsage
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.18.4 validateStateSubactionMembershipOwningType
type=file
skip_validation=the pinned parser has no production for an entry subaction outside a state, so the invalid owner is reported as unexpected_keyword_in_scope and never reaches semantics
~~~
# SOURCE
~~~sysml
package States {
    // Conforming: the subaction membership is owned by a state definition.
    state def Good {
        entry action started;
    }

    // Invalid: the subaction membership is owned by a part definition.
    part def Bad {
        entry action started;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_state_subaction_membership_owning_type.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "state_subaction_invalid_owner")
        (source "semantic")
        (range (start 7 4) (end 7 18))
      )
    )
  )
)
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
        (range (start 3 21) (end 3 28))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 8 8) (end 9 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:34572facffb3e244c6ac56335bdf455920fef4d657e5fc15de94e1fad71c6042") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_state_subaction_membership_owning_type.md") (qualified-name "States"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_subaction_membership_owning_type.md") (qualified-name "States::Bad"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_subaction_membership_owning_type.md") (qualified-name "States::Good"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_subaction_membership_owning_type.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Good")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entry-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (entryActionBinding (reference "started")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_state_subaction_membership_owning_type.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Good")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0))
      (authored-target "started")
      (outcome (status unresolved)))
  )
  (relationships
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
  (query (document "memory://snapshot/sysml_state_subaction_membership_owning_type.md") (range (start 3 21) (end 3 28)) (probe (position 3 21))
    (reference (id (source (node (document "memory://snapshot/sysml_state_subaction_membership_owning_type.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Good")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0) (authored-target "started")
      (outcome (status unresolved)))
    )
  )
)
~~~
