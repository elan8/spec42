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
type=file
~~~
# SOURCE
~~~sysml
package States {
    // Conforming: one subaction membership of each kind.
    state def Good {
        entry action started { }
        do action running { }
        exit action finished { }
    }

    // Invalid: two entry subaction memberships.
    state def Bad {
        entry action started { }
        entry action restarted { }
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
        (range (start 11 8) (end 11 34))
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
        (code "state_duplicate_subaction_kind")
        (source "semantic")
        (range (start 11 8) (end 11 34))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:b7c107c7736ff6107e981c814388fc84d7e0f0e5f8f25d21f820bbe9a9b35580"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (qualified-name "States"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (qualified-name "States::Bad"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (qualified-name "States::Bad::restarted"))) (kind entry-action-binding) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (qualified-name "States::Bad::started"))) (kind entry-action-binding) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (qualified-name "States::Good"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (qualified-name "States::Good::finished"))) (kind exit-action-binding) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (qualified-name "States::Good::running"))) (kind do-action-binding) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (qualified-name "States::Good::started"))) (kind entry-action-binding) (membership (kind feature) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (qualified-name "States::Bad::restarted"))) (target (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (qualified-name "States::Bad"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (qualified-name "States::Bad::started"))) (target (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (qualified-name "States::Bad"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (qualified-name "States::Good::finished"))) (target (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (qualified-name "States::Good"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (qualified-name "States::Good::running"))) (target (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (qualified-name "States::Good"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (qualified-name "States::Good::started"))) (target (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (qualified-name "States::Good"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (qualified-name "States::Bad::restarted")))
      (featured-by (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (qualified-name "States::Bad")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (qualified-name "States::Bad::started")))
      (featured-by (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (qualified-name "States::Bad")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (qualified-name "States::Good::finished")))
      (featured-by (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (qualified-name "States::Good")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (qualified-name "States::Good::running")))
      (featured-by (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (qualified-name "States::Good")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (qualified-name "States::Good::started")))
      (featured-by (node (document "memory://snapshot/sysml_state_definition_state_subaction_kind.md") (qualified-name "States::Good")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
