# META
~~~ini
description=SysML 8.3.18.5 validateStateDefinitionParallelSubactions forbids the ownedActions of a parallel StateDefinition from having incoming or outgoing transitions
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.18.5 validateStateDefinitionParallelSubactions
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.18.5:validateStateDefinitionParallelSubactions
type=file
~~~
# SOURCE
~~~sysml
package States {
    // Conforming: a parallel state definition whose substates carry no transitions.
    state def Good parallel {
        state left;
        state right;
    }

    // Invalid: a parallel state definition whose substates are joined by a transition.
    state def Bad parallel {
        state left;
        state right;
        transition first left then right;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_state_definition_parallel_subactions.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_final_state")
        (source "semantic")
        (range (start 2 4) (end 5 5))
      )
      (diagnostic
        (severity information)
        (code "missing_initial_state")
        (source "semantic")
        (range (start 2 4) (end 5 5))
      )
      (diagnostic
        (severity information)
        (code "missing_final_state")
        (source "semantic")
        (range (start 8 4) (end 12 5))
      )
      (diagnostic
        (severity information)
        (code "missing_initial_state")
        (source "semantic")
        (range (start 8 4) (end 12 5))
      )
      (diagnostic
        (severity warning)
        (code "parallel_state_substate_transition")
        (source "semantic")
        (range (start 11 8) (end 11 41))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_state_definition_parallel_subactions.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_final_state")
        (source "semantic")
        (range (start 2 4) (end 5 5))
      )
      (diagnostic
        (severity information)
        (code "missing_initial_state")
        (source "semantic")
        (range (start 2 4) (end 5 5))
      )
      (diagnostic
        (severity information)
        (code "missing_final_state")
        (source "semantic")
        (range (start 8 4) (end 12 5))
      )
      (diagnostic
        (severity information)
        (code "missing_initial_state")
        (source "semantic")
        (range (start 8 4) (end 12 5))
      )
      (diagnostic
        (severity warning)
        (code "parallel_state_substate_transition")
        (source "semantic")
        (range (start 11 8) (end 11 41))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:37627ee1572344f43e132d19531dad5bf8d98eebd6fe8731d1faad49518e7ed5") (contract-version "feature-value-expression-results-v5"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (qualified-name "States"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (qualified-name "States::Bad"))) (kind state-def) (membership (kind owning) (visibility default)) (facts (modifiers parallel)))
    (declaration (id (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Bad")) (anonymous (kind transition) (ordinal 0))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "left")) (transitionTarget (reference "right")))))
    (declaration (id (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (qualified-name "States::Bad::left"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (qualified-name "States::Bad::right"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (qualified-name "States::Good"))) (kind state-def) (membership (kind owning) (visibility default)) (facts (modifiers parallel)))
    (declaration (id (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (qualified-name "States::Good::left"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (qualified-name "States::Good::right"))) (kind state) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Bad")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0))
      (authored-target "left")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (qualified-name "States::Bad::left")))))
    (reference (id (source (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Bad")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0))
      (authored-target "right")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (qualified-name "States::Bad::right")))))
  )
  (relationships
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Bad")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (qualified-name "States::Bad::left"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Bad")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Bad")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (qualified-name "States::Bad::right"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Bad")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Bad")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (qualified-name "States::Bad"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (qualified-name "States::Bad::left"))) (target (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (qualified-name "States::Bad"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (qualified-name "States::Bad::right"))) (target (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (qualified-name "States::Bad"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (qualified-name "States::Good::left"))) (target (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (qualified-name "States::Good"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (qualified-name "States::Good::right"))) (target (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (qualified-name "States::Good"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Bad")) (anonymous (kind transition) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (qualified-name "States::Bad")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (qualified-name "States::Bad::left")))
      (featured-by (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (qualified-name "States::Bad")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (qualified-name "States::Bad::right")))
      (featured-by (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (qualified-name "States::Bad")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (qualified-name "States::Good::left")))
      (featured-by (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (qualified-name "States::Good")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (qualified-name "States::Good::right")))
      (featured-by (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (qualified-name "States::Good")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (range (start 11 25) (end 11 29)) (probe (position 11 25))
    (reference (id (source (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Bad")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0) (authored-target "left")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (qualified-name "States::Bad::left")))))
    )
  )
  (query (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (range (start 11 35) (end 11 40)) (probe (position 11 35))
    (reference (id (source (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (path (named (kind package) (name "States")) (named (kind state-def) (name "Bad")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0) (authored-target "right")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_state_definition_parallel_subactions.md") (qualified-name "States::Bad::right")))))
    )
  )
)
~~~
