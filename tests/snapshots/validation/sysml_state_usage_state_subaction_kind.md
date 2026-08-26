# META
~~~ini
description=SysML 8.3.18.6 validateStateUsageStateSubactionKind forbids a StateUsage from owning more than one StateSubactionMembership of each kind
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.18.6 validateStateUsageStateSubactionKind
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.18.6:validateStateUsageStateSubactionKind
blocked_by=lowering-state-subaction-memberships
type=file
~~~
# SOURCE
~~~sysml
package States {
    state def Machine {
        // Conforming: one subaction membership of each kind.
        state good {
            entry action started;
            exit action finished;
        }

        // Invalid: two entry subaction memberships.
        state bad {
            entry action started;
            entry action restarted;
        }
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "state_duplicate_subaction_kind")
        (source "semantic")
        (range (start 11 12) (end 11 35))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_final_state")
        (source "semantic")
        (range (start 1 4) (end 13 5))
      )
      (diagnostic
        (severity information)
        (code "missing_initial_state")
        (source "semantic")
        (range (start 1 4) (end 13 5))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 4 25) (end 4 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 5 24) (end 5 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 25) (end 10 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 25) (end 11 34))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:2db1908bebac38df73047839c72274d5f005b8da0a7be5a92b557b832432e0d9") (contract-version "feature-value-expression-results-v5"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine::bad"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Machine")) (named (kind state) (name "bad")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entry-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (entryActionBinding (reference "started")))))
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Machine")) (named (kind state) (name "bad")) (anonymous (kind entry-action-binding) (ordinal 1))))) (kind entry-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (entryActionBinding (reference "restarted")))))
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine::good"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Machine")) (named (kind state) (name "good")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entry-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (entryActionBinding (reference "started")))))
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Machine")) (named (kind state) (name "good")) (anonymous (kind exit-action-binding) (ordinal 0))))) (kind exit-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (exitActionBinding (reference "finished")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Machine")) (named (kind state) (name "bad")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0))
      (authored-target "started")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Machine")) (named (kind state) (name "bad")) (anonymous (kind entry-action-binding) (ordinal 1))))) (kind entryActionBinding) (ordinal 0))
      (authored-target "restarted")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Machine")) (named (kind state) (name "good")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0))
      (authored-target "started")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Machine")) (named (kind state) (name "good")) (anonymous (kind exit-action-binding) (ordinal 0))))) (kind exitActionBinding) (ordinal 0))
      (authored-target "finished")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine::bad"))) (target (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Machine")) (named (kind state) (name "bad")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine::bad"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Machine")) (named (kind state) (name "bad")) (anonymous (kind entry-action-binding) (ordinal 1))))) (target (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine::bad"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine::good"))) (target (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Machine")) (named (kind state) (name "good")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine::good"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Machine")) (named (kind state) (name "good")) (anonymous (kind exit-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine::good"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine::bad")))
      (featured-by (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Machine")) (named (kind state) (name "bad")) (anonymous (kind entry-action-binding) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine::bad")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Machine")) (named (kind state) (name "bad")) (anonymous (kind entry-action-binding) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine::bad")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine::good")))
      (featured-by (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Machine")) (named (kind state) (name "good")) (anonymous (kind entry-action-binding) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine::good")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Machine")) (named (kind state) (name "good")) (anonymous (kind exit-action-binding) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine::good")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (range (start 10 25) (end 10 32)) (probe (position 10 25))
    (reference (id (source (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Machine")) (named (kind state) (name "bad")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0) (authored-target "started")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (range (start 11 25) (end 11 34)) (probe (position 11 25))
    (reference (id (source (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Machine")) (named (kind state) (name "bad")) (anonymous (kind entry-action-binding) (ordinal 1))))) (kind entryActionBinding) (ordinal 0) (authored-target "restarted")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (range (start 4 25) (end 4 32)) (probe (position 4 25))
    (reference (id (source (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Machine")) (named (kind state) (name "good")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0) (authored-target "started")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (range (start 5 24) (end 5 32)) (probe (position 5 24))
    (reference (id (source (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Machine")) (named (kind state) (name "good")) (anonymous (kind exit-action-binding) (ordinal 0))))) (kind exitActionBinding) (ordinal 0) (authored-target "finished")
      (outcome (status unresolved)))
    )
  )
)
~~~
