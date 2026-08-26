# META
~~~ini
description=SysML 8.3.18.5 validateStateDefinitionStateSubactionKind forbids a StateDefinition from owning more than one StateSubactionMembership of each kind
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.18.5 validateStateDefinitionStateSubactionKind
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.18.5:validateStateDefinitionStateSubactionKind
blocked_by=lowering-state-subaction-memberships
type=file
~~~
# SOURCE
~~~sysml
package States {
    // Conforming: one subaction membership of each kind.
    state def Good {
        entry action started;
        do action running;
        exit action finished;
    }

    // Invalid: two entry subaction memberships.
    state def Bad {
        entry action started;
        entry action restarted;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "state_duplicate_subaction_kind")
        (source "semantic")
        (range (start 11 8) (end 11 31))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 3 21) (end 3 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 4 18) (end 4 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 5 20) (end 5 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 21) (end 10 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 21) (end 11 30))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:75dcb5656fa8df9d3169287fa7ec8102a900fafdde78c5fabfd495a95145229b") (contract-version "owned-cross-feature-typing-v4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (qualified-name "States"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (qualified-name "States::Bad"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Bad")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entry-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (entryActionBinding (reference "started")))))
    (declaration (id (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Bad")) (anonymous (kind entry-action-binding) (ordinal 1))))) (kind entry-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (entryActionBinding (reference "restarted")))))
    (declaration (id (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (qualified-name "States::Good"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Good")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entry-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (entryActionBinding (reference "started")))))
    (declaration (id (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Good")) (anonymous (kind do-action-binding) (ordinal 0))))) (kind do-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (doActionBinding (reference "running")))))
    (declaration (id (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Good")) (anonymous (kind exit-action-binding) (ordinal 0))))) (kind exit-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (exitActionBinding (reference "finished")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Bad")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0))
      (authored-target "started")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Bad")) (anonymous (kind entry-action-binding) (ordinal 1))))) (kind entryActionBinding) (ordinal 0))
      (authored-target "restarted")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Good")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0))
      (authored-target "started")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Good")) (anonymous (kind do-action-binding) (ordinal 0))))) (kind doActionBinding) (ordinal 0))
      (authored-target "running")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Good")) (anonymous (kind exit-action-binding) (ordinal 0))))) (kind exitActionBinding) (ordinal 0))
      (authored-target "finished")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Bad")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (qualified-name "States::Bad"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Bad")) (anonymous (kind entry-action-binding) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (qualified-name "States::Bad"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Good")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (qualified-name "States::Good"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Good")) (anonymous (kind do-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (qualified-name "States::Good"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Good")) (anonymous (kind exit-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (qualified-name "States::Good"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Bad")) (anonymous (kind entry-action-binding) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (qualified-name "States::Bad")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Bad")) (anonymous (kind entry-action-binding) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (qualified-name "States::Bad")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Good")) (anonymous (kind entry-action-binding) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (qualified-name "States::Good")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Good")) (anonymous (kind do-action-binding) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (qualified-name "States::Good")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Good")) (anonymous (kind exit-action-binding) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (qualified-name "States::Good")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (range (start 10 21) (end 10 28)) (probe (position 10 21))
    (reference (id (source (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Bad")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0) (authored-target "started")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (range (start 11 21) (end 11 30)) (probe (position 11 21))
    (reference (id (source (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Bad")) (anonymous (kind entry-action-binding) (ordinal 1))))) (kind entryActionBinding) (ordinal 0) (authored-target "restarted")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (range (start 3 21) (end 3 28)) (probe (position 3 21))
    (reference (id (source (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Good")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0) (authored-target "started")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (range (start 4 18) (end 4 25)) (probe (position 4 18))
    (reference (id (source (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Good")) (anonymous (kind do-action-binding) (ordinal 0))))) (kind doActionBinding) (ordinal 0) (authored-target "running")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (range (start 5 20) (end 5 28)) (probe (position 5 20))
    (reference (id (source (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Good")) (anonymous (kind exit-action-binding) (ordinal 0))))) (kind exitActionBinding) (ordinal 0) (authored-target "finished")
      (outcome (status unresolved)))
    )
  )
)
~~~
