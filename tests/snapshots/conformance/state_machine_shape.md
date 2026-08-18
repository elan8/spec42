# META
~~~ini
description=Initial-state targets, final-state cardinality, guards and transition contexts
type=file
~~~
# SOURCE
~~~sysml
package Machines {
    part def Structure;

    state def InitialMarkerNamesANonState {
        then Structure;
        state armed;
        final disarmed;
    }

    state def TwoFinalStates {
        then armed;
        state armed;
        final first;
        final second;
    }

    state def GuardIsNotBoolean {
        then armed;
        state armed;
        final disarmed;
        transition first armed if 1 then disarmed;
    }

    state def Left {
        then idle;
        state idle;
        final done;
    }

    state def TransitionLeavesItsOwnContext {
        then ready;
        state ready;
        final done;
        transition first ready then Left::idle;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/state_machine_shape.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "initial_state_invalid_target")
        (source "semantic")
        (range (start 4 13) (end 4 22))
        (related-information
          (related
            (uri "memory://snapshot/state_machine_shape.md")
            (range (start 1 4) (end 1 23))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "multiple_final_states")
        (source "semantic")
        (range (start 9 4) (end 14 5))
      )
      (diagnostic
        (severity warning)
        (code "transition_guard_non_boolean")
        (source "semantic")
        (range (start 20 8) (end 20 50))
      )
      (diagnostic
        (severity warning)
        (code "transition_endpoint_invalid_context")
        (source "semantic")
        (range (start 33 8) (end 33 47))
        (related-information
          (related
            (uri "memory://snapshot/state_machine_shape.md")
            (range (start 29 4) (end 34 5))
          )
          (related
            (uri "memory://snapshot/state_machine_shape.md")
            (range (start 23 4) (end 27 5))
          )
        )
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:dff8c15c9f2a6b586b685035c900542641528f563e0d736025fcad8bbef61dfe") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::GuardIsNotBoolean"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "GuardIsNotBoolean")) (anonymous (kind initial-state) (ordinal 0))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "armed")))))
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "GuardIsNotBoolean")) (anonymous (kind transition) (ordinal 0))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "armed")) (transitionTarget (reference "disarmed")))))
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::GuardIsNotBoolean::armed"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::GuardIsNotBoolean::disarmed"))) (kind final-state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::InitialMarkerNamesANonState"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "InitialMarkerNamesANonState")) (anonymous (kind initial-state) (ordinal 0))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "Structure")))))
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::InitialMarkerNamesANonState::armed"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::InitialMarkerNamesANonState::disarmed"))) (kind final-state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::Left"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "Left")) (anonymous (kind initial-state) (ordinal 0))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "idle")))))
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::Left::done"))) (kind final-state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::Left::idle"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::Structure"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::TransitionLeavesItsOwnContext"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "TransitionLeavesItsOwnContext")) (anonymous (kind initial-state) (ordinal 0))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "ready")))))
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "TransitionLeavesItsOwnContext")) (anonymous (kind transition) (ordinal 0))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "ready")) (transitionTarget (reference "Left::idle")))))
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::TransitionLeavesItsOwnContext::done"))) (kind final-state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::TransitionLeavesItsOwnContext::ready"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::TwoFinalStates"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "TwoFinalStates")) (anonymous (kind initial-state) (ordinal 0))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "armed")))))
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::TwoFinalStates::armed"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::TwoFinalStates::first"))) (kind final-state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::TwoFinalStates::second"))) (kind final-state) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "GuardIsNotBoolean")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0))
      (authored-target "armed")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::GuardIsNotBoolean::armed")))))
    (reference (id (source (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "GuardIsNotBoolean")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0))
      (authored-target "armed")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::GuardIsNotBoolean::armed")))))
    (reference (id (source (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "GuardIsNotBoolean")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0))
      (authored-target "disarmed")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::GuardIsNotBoolean::disarmed")))))
    (reference (id (source (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "InitialMarkerNamesANonState")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0))
      (authored-target "Structure")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::Structure")))))
    (reference (id (source (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "Left")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::Left::idle")))))
    (reference (id (source (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "TransitionLeavesItsOwnContext")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0))
      (authored-target "ready")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::TransitionLeavesItsOwnContext::ready")))))
    (reference (id (source (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "TransitionLeavesItsOwnContext")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0))
      (authored-target "ready")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::TransitionLeavesItsOwnContext::ready")))))
    (reference (id (source (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "TransitionLeavesItsOwnContext")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0))
      (authored-target "Left::idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::Left::idle")))))
    (reference (id (source (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "TwoFinalStates")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0))
      (authored-target "armed")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::TwoFinalStates::armed")))))
  )
  (relationships
    (relationship (kind initialState) (source (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "GuardIsNotBoolean")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::GuardIsNotBoolean::armed"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "GuardIsNotBoolean")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "GuardIsNotBoolean")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::GuardIsNotBoolean::armed"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "GuardIsNotBoolean")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "GuardIsNotBoolean")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::GuardIsNotBoolean::disarmed"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "GuardIsNotBoolean")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind initialState) (source (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "InitialMarkerNamesANonState")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::Structure"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "InitialMarkerNamesANonState")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0)))
    (relationship (kind initialState) (source (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "Left")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::Left::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "Left")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0)))
    (relationship (kind initialState) (source (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "TransitionLeavesItsOwnContext")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::TransitionLeavesItsOwnContext::ready"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "TransitionLeavesItsOwnContext")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "TransitionLeavesItsOwnContext")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::TransitionLeavesItsOwnContext::ready"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "TransitionLeavesItsOwnContext")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "TransitionLeavesItsOwnContext")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::Left::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "TransitionLeavesItsOwnContext")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind initialState) (source (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "TwoFinalStates")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::TwoFinalStates::armed"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "TwoFinalStates")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "GuardIsNotBoolean")) (anonymous (kind transition) (ordinal 0))))) (state literal) (value (kind integer) (integer 1)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "GuardIsNotBoolean")) (anonymous (kind initial-state) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::GuardIsNotBoolean")))
    )
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "GuardIsNotBoolean")) (anonymous (kind transition) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::GuardIsNotBoolean")))
    )
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::GuardIsNotBoolean::armed")))
      (featured-by (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::GuardIsNotBoolean")))
    )
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::GuardIsNotBoolean::disarmed")))
      (featured-by (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::GuardIsNotBoolean")))
    )
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "InitialMarkerNamesANonState")) (anonymous (kind initial-state) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::InitialMarkerNamesANonState")))
    )
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::InitialMarkerNamesANonState::armed")))
      (featured-by (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::InitialMarkerNamesANonState")))
    )
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::InitialMarkerNamesANonState::disarmed")))
      (featured-by (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::InitialMarkerNamesANonState")))
    )
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "Left")) (anonymous (kind initial-state) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::Left")))
    )
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::Left::done")))
      (featured-by (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::Left")))
    )
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::Left::idle")))
      (featured-by (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::Left")))
    )
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "TransitionLeavesItsOwnContext")) (anonymous (kind initial-state) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::TransitionLeavesItsOwnContext")))
    )
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "TransitionLeavesItsOwnContext")) (anonymous (kind transition) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::TransitionLeavesItsOwnContext")))
    )
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::TransitionLeavesItsOwnContext::done")))
      (featured-by (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::TransitionLeavesItsOwnContext")))
    )
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::TransitionLeavesItsOwnContext::ready")))
      (featured-by (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::TransitionLeavesItsOwnContext")))
    )
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "TwoFinalStates")) (anonymous (kind initial-state) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::TwoFinalStates")))
    )
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::TwoFinalStates::armed")))
      (featured-by (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::TwoFinalStates")))
    )
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::TwoFinalStates::first")))
      (featured-by (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::TwoFinalStates")))
    )
    (declaration (id (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::TwoFinalStates::second")))
      (featured-by (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::TwoFinalStates")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/state_machine_shape.md") (range (start 17 13) (end 17 18)) (probe (position 17 13))
    (reference (id (source (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "GuardIsNotBoolean")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0) (authored-target "armed")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::GuardIsNotBoolean::armed")))))
    )
  )
  (query (document "memory://snapshot/state_machine_shape.md") (range (start 20 25) (end 20 30)) (probe (position 20 25))
    (reference (id (source (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "GuardIsNotBoolean")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0) (authored-target "armed")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::GuardIsNotBoolean::armed")))))
    )
  )
  (query (document "memory://snapshot/state_machine_shape.md") (range (start 20 41) (end 20 49)) (probe (position 20 41))
    (reference (id (source (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "GuardIsNotBoolean")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0) (authored-target "disarmed")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::GuardIsNotBoolean::disarmed")))))
    )
  )
  (query (document "memory://snapshot/state_machine_shape.md") (range (start 4 13) (end 4 22)) (probe (position 4 13))
    (reference (id (source (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "InitialMarkerNamesANonState")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0) (authored-target "Structure")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::Structure")))))
    )
  )
  (query (document "memory://snapshot/state_machine_shape.md") (range (start 24 13) (end 24 17)) (probe (position 24 13))
    (reference (id (source (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "Left")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::Left::idle")))))
    )
  )
  (query (document "memory://snapshot/state_machine_shape.md") (range (start 30 13) (end 30 18)) (probe (position 30 13))
    (reference (id (source (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "TransitionLeavesItsOwnContext")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0) (authored-target "ready")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::TransitionLeavesItsOwnContext::ready")))))
    )
  )
  (query (document "memory://snapshot/state_machine_shape.md") (range (start 33 25) (end 33 30)) (probe (position 33 25))
    (reference (id (source (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "TransitionLeavesItsOwnContext")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0) (authored-target "ready")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::TransitionLeavesItsOwnContext::ready")))))
    )
  )
  (query (document "memory://snapshot/state_machine_shape.md") (range (start 33 36) (end 33 46)) (probe (position 33 36))
    (reference (id (source (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "TransitionLeavesItsOwnContext")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0) (authored-target "Left::idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::Left::idle")))))
    )
  )
  (query (document "memory://snapshot/state_machine_shape.md") (range (start 10 13) (end 10 18)) (probe (position 10 13))
    (reference (id (source (node (document "memory://snapshot/state_machine_shape.md") (path (named (kind package) (name "Machines")) (named (kind state-def) (name "TwoFinalStates")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0) (authored-target "armed")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_machine_shape.md") (qualified-name "Machines::TwoFinalStates::armed")))))
    )
  )
)
~~~
