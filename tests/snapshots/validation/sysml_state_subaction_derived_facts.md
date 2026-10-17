# META
~~~ini
description=State definitions and usages derive their first entry, do, and exit subaction
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=sysml-2.0:8.3.18.5:deriveStateDefinitionEntryAction
rule_id=sysml-2.0:8.3.18.5:deriveStateDefinitionDoAction
rule_id=sysml-2.0:8.3.18.5:deriveStateDefinitionExitAction
rule_id=sysml-2.0:8.3.18.6:deriveStateUsageEntryAction
rule_id=sysml-2.0:8.3.18.6:deriveStateUsageDoAction
rule_id=sysml-2.0:8.3.18.6:deriveStateUsageExitAction
rule_id=sysml-2.0:8.3.18.5:deriveStateDefinitionState
libraries=none
~~~
# SOURCE
~~~sysml
package States {
    state def Machine {
        entry action enter { }
        do action run { }
        exit action leave { }
        state nested;
    }
    state machine {
        entry action enter { }
        do action run { }
        exit action leave { }
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (action-derived-fact (rule_id "sysml-2.0:8.3.18.5:deriveStateDefinitionDoAction") (source "States::Machine") (target "States::Machine::run") (outcome resolved))
  (action-derived-fact (rule_id "sysml-2.0:8.3.18.5:deriveStateDefinitionEntryAction") (source "States::Machine") (target "States::Machine::enter") (outcome resolved))
  (action-derived-fact (rule_id "sysml-2.0:8.3.18.5:deriveStateDefinitionExitAction") (source "States::Machine") (target "States::Machine::leave") (outcome resolved))
  (action-derived-fact (rule_id "sysml-2.0:8.3.18.6:deriveStateUsageDoAction") (source "States::machine") (target "States::machine::run") (outcome resolved))
  (action-derived-fact (rule_id "sysml-2.0:8.3.18.6:deriveStateUsageEntryAction") (source "States::machine") (target "States::machine::enter") (outcome resolved))
  (action-derived-fact (rule_id "sysml-2.0:8.3.18.6:deriveStateUsageExitAction") (source "States::machine") (target "States::machine::leave") (outcome resolved))
  (action-derived-fact (rule_id "sysml-2.0:8.3.18.5:deriveStateDefinitionState") (source "States::Machine") (target "States::Machine::nested") (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_state_subaction_derived_facts.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_initial_state")
        (source "semantic")
        (range (start 1 4) (end 6 5))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:ef25dcc339ab9a7610d6f1aa8f477cea16ab700a8430484534943c5def20b639"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_state_subaction_derived_facts.md") (qualified-name "States"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_subaction_derived_facts.md") (qualified-name "States::Machine"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_subaction_derived_facts.md") (qualified-name "States::Machine::enter"))) (kind entry-action-binding) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_subaction_derived_facts.md") (qualified-name "States::Machine::leave"))) (kind exit-action-binding) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_subaction_derived_facts.md") (qualified-name "States::Machine::nested"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_subaction_derived_facts.md") (qualified-name "States::Machine::run"))) (kind do-action-binding) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_subaction_derived_facts.md") (qualified-name "States::machine"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_subaction_derived_facts.md") (qualified-name "States::machine::enter"))) (kind entry-action-binding) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_subaction_derived_facts.md") (qualified-name "States::machine::leave"))) (kind exit-action-binding) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_subaction_derived_facts.md") (qualified-name "States::machine::run"))) (kind do-action-binding) (membership (kind feature) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_subaction_derived_facts.md") (qualified-name "States::Machine::enter"))) (target (node (document "memory://snapshot/sysml_state_subaction_derived_facts.md") (qualified-name "States::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_subaction_derived_facts.md") (qualified-name "States::Machine::leave"))) (target (node (document "memory://snapshot/sysml_state_subaction_derived_facts.md") (qualified-name "States::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_subaction_derived_facts.md") (qualified-name "States::Machine::nested"))) (target (node (document "memory://snapshot/sysml_state_subaction_derived_facts.md") (qualified-name "States::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_subaction_derived_facts.md") (qualified-name "States::Machine::run"))) (target (node (document "memory://snapshot/sysml_state_subaction_derived_facts.md") (qualified-name "States::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_subaction_derived_facts.md") (qualified-name "States::machine::enter"))) (target (node (document "memory://snapshot/sysml_state_subaction_derived_facts.md") (qualified-name "States::machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_subaction_derived_facts.md") (qualified-name "States::machine::leave"))) (target (node (document "memory://snapshot/sysml_state_subaction_derived_facts.md") (qualified-name "States::machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_subaction_derived_facts.md") (qualified-name "States::machine::run"))) (target (node (document "memory://snapshot/sysml_state_subaction_derived_facts.md") (qualified-name "States::machine"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_state_subaction_derived_facts.md") (qualified-name "States::Machine::enter")))
      (featured-by (node (document "memory://snapshot/sysml_state_subaction_derived_facts.md") (qualified-name "States::Machine")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_state_subaction_derived_facts.md") (qualified-name "States::Machine::leave")))
      (featured-by (node (document "memory://snapshot/sysml_state_subaction_derived_facts.md") (qualified-name "States::Machine")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_state_subaction_derived_facts.md") (qualified-name "States::Machine::nested")))
      (featured-by (node (document "memory://snapshot/sysml_state_subaction_derived_facts.md") (qualified-name "States::Machine")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_state_subaction_derived_facts.md") (qualified-name "States::Machine::run")))
      (featured-by (node (document "memory://snapshot/sysml_state_subaction_derived_facts.md") (qualified-name "States::Machine")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_state_subaction_derived_facts.md") (qualified-name "States::machine::enter")))
      (featured-by (node (document "memory://snapshot/sysml_state_subaction_derived_facts.md") (qualified-name "States::machine")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_state_subaction_derived_facts.md") (qualified-name "States::machine::leave")))
      (featured-by (node (document "memory://snapshot/sysml_state_subaction_derived_facts.md") (qualified-name "States::machine")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_state_subaction_derived_facts.md") (qualified-name "States::machine::run")))
      (featured-by (node (document "memory://snapshot/sysml_state_subaction_derived_facts.md") (qualified-name "States::machine")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
