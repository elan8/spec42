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
type=file
~~~
# SOURCE
~~~sysml
package States {
    part def Machine {
        // Conforming: one subaction membership of each kind.
        state good {
            entry action started { }
            exit action finished { }
        }

        // Invalid: two entry subaction memberships.
        state bad {
            entry action started { }
            entry action restarted { }
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
        (range (start 11 12) (end 11 38))
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
        (severity warning)
        (code "state_duplicate_subaction_kind")
        (source "semantic")
        (range (start 11 12) (end 11 38))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:af5bbb839319eeabd3856751bacfdc48905d2f6040c09b09859974a3745a9fab"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine::bad"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine::bad::restarted"))) (kind entry-action-binding) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine::bad::started"))) (kind entry-action-binding) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine::good"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine::good::finished"))) (kind exit-action-binding) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine::good::started"))) (kind entry-action-binding) (membership (kind feature) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine::bad"))) (target (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine::bad::restarted"))) (target (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine::bad"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine::bad::started"))) (target (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine::bad"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine::good"))) (target (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine::good::finished"))) (target (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine::good"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine::good::started"))) (target (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine::good"))) (provenance implied))
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
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine::bad::restarted")))
      (featured-by (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine::bad")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine::bad::started")))
      (featured-by (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine::bad")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine::good")))
      (featured-by (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine::good::finished")))
      (featured-by (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine::good")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine::good::started")))
      (featured-by (node (document "memory://snapshot/sysml_state_usage_state_subaction_kind.md") (qualified-name "States::Machine::good")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
