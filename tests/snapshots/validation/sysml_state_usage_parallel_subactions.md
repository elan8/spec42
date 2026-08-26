# META
~~~ini
description=SysML 8.3.18.6 validateStateUsageParallelSubactions forbids the nestedActions of a parallel StateUsage from having incoming or outgoing transitions
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.18.6 validateStateUsageParallelSubactions
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.18.6:validateStateUsageParallelSubactions
type=file
~~~
# SOURCE
~~~sysml
package States {
    part def Holder {
        // Conforming: a parallel state whose substates carry no transitions.
        state good parallel {
            state left;
            state right;
        }

        // Invalid: a parallel state whose substates are joined by a transition.
        state bad parallel {
            state left;
            state right;
            transition first left then right;
        }
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_state_usage_parallel_subactions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "parallel_state_substate_transition")
        (source "semantic")
        (range (start 12 12) (end 12 45))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_state_usage_parallel_subactions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "parallel_state_substate_transition")
        (source "semantic")
        (range (start 12 12) (end 12 45))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:013851768cf1a5b6ec3cd8ce4ab0ba6417b0745636118f610c13b13041eebe7d") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (qualified-name "States"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (qualified-name "States::Holder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (qualified-name "States::Holder::bad"))) (kind state) (membership (kind feature) (visibility default)) (facts (modifiers parallel)))
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (path (named (kind package) (name "States")) (named (kind part-def) (name "Holder")) (named (kind state) (name "bad")) (anonymous (kind transition) (ordinal 0))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "left")) (transitionTarget (reference "right")))))
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (qualified-name "States::Holder::bad::left"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (qualified-name "States::Holder::bad::right"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (qualified-name "States::Holder::good"))) (kind state) (membership (kind feature) (visibility default)) (facts (modifiers parallel)))
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (qualified-name "States::Holder::good::left"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (qualified-name "States::Holder::good::right"))) (kind state) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (path (named (kind package) (name "States")) (named (kind part-def) (name "Holder")) (named (kind state) (name "bad")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0))
      (authored-target "left")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (qualified-name "States::Holder::bad::left")))))
    (reference (id (source (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (path (named (kind package) (name "States")) (named (kind part-def) (name "Holder")) (named (kind state) (name "bad")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0))
      (authored-target "right")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (qualified-name "States::Holder::bad::right")))))
  )
  (relationships
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (path (named (kind package) (name "States")) (named (kind part-def) (name "Holder")) (named (kind state) (name "bad")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (qualified-name "States::Holder::bad::left"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (path (named (kind package) (name "States")) (named (kind part-def) (name "Holder")) (named (kind state) (name "bad")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (path (named (kind package) (name "States")) (named (kind part-def) (name "Holder")) (named (kind state) (name "bad")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (qualified-name "States::Holder::bad::right"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (path (named (kind package) (name "States")) (named (kind part-def) (name "Holder")) (named (kind state) (name "bad")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (qualified-name "States::Holder::bad"))) (target (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (qualified-name "States::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (path (named (kind package) (name "States")) (named (kind part-def) (name "Holder")) (named (kind state) (name "bad")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (qualified-name "States::Holder::bad"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (qualified-name "States::Holder::bad::left"))) (target (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (qualified-name "States::Holder::bad"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (qualified-name "States::Holder::bad::right"))) (target (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (qualified-name "States::Holder::bad"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (qualified-name "States::Holder::good"))) (target (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (qualified-name "States::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (qualified-name "States::Holder::good::left"))) (target (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (qualified-name "States::Holder::good"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (qualified-name "States::Holder::good::right"))) (target (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (qualified-name "States::Holder::good"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (qualified-name "States::Holder::bad")))
      (featured-by (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (qualified-name "States::Holder")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (path (named (kind package) (name "States")) (named (kind part-def) (name "Holder")) (named (kind state) (name "bad")) (anonymous (kind transition) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (qualified-name "States::Holder::bad")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (qualified-name "States::Holder::bad::left")))
      (featured-by (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (qualified-name "States::Holder::bad")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (qualified-name "States::Holder::bad::right")))
      (featured-by (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (qualified-name "States::Holder::bad")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (qualified-name "States::Holder::good")))
      (featured-by (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (qualified-name "States::Holder")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (qualified-name "States::Holder::good::left")))
      (featured-by (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (qualified-name "States::Holder::good")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (qualified-name "States::Holder::good::right")))
      (featured-by (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (qualified-name "States::Holder::good")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (range (start 12 29) (end 12 33)) (probe (position 12 29))
    (reference (id (source (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (path (named (kind package) (name "States")) (named (kind part-def) (name "Holder")) (named (kind state) (name "bad")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0) (authored-target "left")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (qualified-name "States::Holder::bad::left")))))
    )
  )
  (query (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (range (start 12 39) (end 12 44)) (probe (position 12 39))
    (reference (id (source (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (path (named (kind package) (name "States")) (named (kind part-def) (name "Holder")) (named (kind state) (name "bad")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0) (authored-target "right")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_state_usage_parallel_subactions.md") (qualified-name "States::Holder::bad::right")))))
    )
  )
)
~~~
