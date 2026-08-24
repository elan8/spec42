# META
~~~ini
description=SysML 8.3.18.4 validateStateSubactionMembershipOwningType requires the owningType of a StateSubactionMembership to be a StateDefinition or a StateUsage
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.18.4 validateStateSubactionMembershipOwningType
source_expectation=accepted
rule_family=validate
expectation=by_construction
rule_id=sysml-2.0:8.3.18.4:validateStateSubactionMembershipOwningType
blocked_by=abstract-syntax-invalid-state-subaction-owner
type=file
~~~
# SOURCE
~~~sysml
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
        (range (start 3 21) (end 3 28))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:b995c4fe9333c16fe09a1a2a335e210b820facda254707fd7da38590a5ac0dc9") (contract-version "lossless-publication-completeness-v3"))
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
  (query (document "memory://snapshot/sysml_state_subaction_membership_owning_type.md") (range (start 3 21) (end 3 28)) (probe (position 3 21))
    (reference (id (source (node (document "memory://snapshot/sysml_state_subaction_membership_owning_type.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Good")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0) (authored-target "started")
      (outcome (status unresolved)))
    )
  )
)
~~~
