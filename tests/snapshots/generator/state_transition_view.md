# META
~~~ini
description=An authored StateTransitionView resolves its normative type and exposed state machine
type=file
libraries=standard
~~~
# SOURCE
~~~sysml
package DoorController {
    private import StandardViewDefinitions::*;

    item def OpenRequested;

    state def DoorLifecycle {
        then closed;
        state closed;
        state open;
        final retired;
        transition open_door first closed accept OpenRequested then open;
        transition retire first open then retired;
    }

    view lifecycle : StateTransitionView {
        expose DoorLifecycle;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/state_transition_view.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:581ae56601182f8d949f19ea5ed75dbda9ea8dabb5e13ed5d99fa6cb34fb8eac") (contract-version "parser-owned-resolution-v1") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_transition_view.md") (path (named (kind package) (name "DoorController")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_transition_view.md") (path (named (kind package) (name "DoorController")) (named (kind state-def) (name "DoorLifecycle")) (anonymous (kind initial-state) (ordinal 0))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "closed")))))
    (declaration (id (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::closed"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::open"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::open_door"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "closed")) (transitionTarget (reference "open")) (transitionTrigger (reference "OpenRequested")))))
    (declaration (id (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::retire"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "open")) (transitionTarget (reference "retired")))))
    (declaration (id (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::retired"))) (kind final-state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::OpenRequested"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::lifecycle"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StateTransitionView")))))
    (declaration (id (node (document "memory://snapshot/state_transition_view.md") (path (named (kind package) (name "DoorController")) (named (kind view) (name "lifecycle")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "DoorLifecycle")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/state_transition_view.md") (path (named (kind package) (name "DoorController")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/state_transition_view.md") (path (named (kind package) (name "DoorController")) (named (kind state-def) (name "DoorLifecycle")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0))
      (authored-target "closed")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::closed")))))
    (reference (id (source (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::open_door"))) (kind transitionSource) (ordinal 0))
      (authored-target "closed")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::closed")))))
    (reference (id (source (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::open_door"))) (kind transitionTarget) (ordinal 0))
      (authored-target "open")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::open")))))
    (reference (id (source (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::open_door"))) (kind transitionTrigger) (ordinal 0))
      (authored-target "OpenRequested")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::OpenRequested")))))
    (reference (id (source (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::retire"))) (kind transitionSource) (ordinal 0))
      (authored-target "open")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::open")))))
    (reference (id (source (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::retire"))) (kind transitionTarget) (ordinal 0))
      (authored-target "retired")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::retired")))))
    (reference (id (source (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::lifecycle"))) (kind featureTyping) (ordinal 0))
      (authored-target "StateTransitionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView")))))
    (reference (id (source (node (document "memory://snapshot/state_transition_view.md") (path (named (kind package) (name "DoorController")) (named (kind view) (name "lifecycle")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "DoorLifecycle")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle")))))
  )
  (relationships
    (relationship (kind initialState) (source (node (document "memory://snapshot/state_transition_view.md") (path (named (kind package) (name "DoorController")) (named (kind state-def) (name "DoorLifecycle")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::closed"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_transition_view.md") (path (named (kind package) (name "DoorController")) (named (kind state-def) (name "DoorLifecycle")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::open_door"))) (target (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::closed"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::open_door"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::open_door"))) (target (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::open"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::open_door"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::open_door"))) (target (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::OpenRequested"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::open_door"))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::retire"))) (target (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::open"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::retire"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::retire"))) (target (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::retired"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::retire"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::lifecycle"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::lifecycle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/state_transition_view.md") (path (named (kind package) (name "DoorController")) (named (kind view) (name "lifecycle")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_transition_view.md") (path (named (kind package) (name "DoorController")) (named (kind view) (name "lifecycle")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/state_transition_view.md") (path (named (kind package) (name "DoorController")) (named (kind state-def) (name "DoorLifecycle")) (anonymous (kind initial-state) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle")))
    )
    (declaration (id (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::closed")))
      (featured-by (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle")))
    )
    (declaration (id (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::open")))
      (featured-by (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle")))
    )
    (declaration (id (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::open_door")))
      (featured-by (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle")))
    )
    (declaration (id (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::retire")))
      (featured-by (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle")))
    )
    (declaration (id (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::retired")))
      (featured-by (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle")))
    )
    (declaration (id (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::lifecycle")))
      (type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/state_transition_view.md") (path (named (kind package) (name "DoorController")) (named (kind view) (name "lifecycle")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::lifecycle")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/state_transition_view.md") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/state_transition_view.md") (path (named (kind package) (name "DoorController")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/state_transition_view.md") (range (start 6 13) (end 6 19)) (probe (position 6 13))
    (reference (id (source (node (document "memory://snapshot/state_transition_view.md") (path (named (kind package) (name "DoorController")) (named (kind state-def) (name "DoorLifecycle")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0) (authored-target "closed")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::closed")))))
    )
  )
  (query (document "memory://snapshot/state_transition_view.md") (range (start 10 35) (end 10 41)) (probe (position 10 35))
    (reference (id (source (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::open_door"))) (kind transitionSource) (ordinal 0) (authored-target "closed")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::closed")))))
    )
  )
  (query (document "memory://snapshot/state_transition_view.md") (range (start 10 68) (end 10 72)) (probe (position 10 68))
    (reference (id (source (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::open_door"))) (kind transitionTarget) (ordinal 0) (authored-target "open")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::open")))))
    )
  )
  (query (document "memory://snapshot/state_transition_view.md") (range (start 10 49) (end 10 62)) (probe (position 10 49))
    (reference (id (source (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::open_door"))) (kind transitionTrigger) (ordinal 0) (authored-target "OpenRequested")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::OpenRequested")))))
    )
  )
  (query (document "memory://snapshot/state_transition_view.md") (range (start 11 32) (end 11 36)) (probe (position 11 32))
    (reference (id (source (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::retire"))) (kind transitionSource) (ordinal 0) (authored-target "open")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::open")))))
    )
  )
  (query (document "memory://snapshot/state_transition_view.md") (range (start 11 42) (end 11 49)) (probe (position 11 42))
    (reference (id (source (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::retire"))) (kind transitionTarget) (ordinal 0) (authored-target "retired")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle::retired")))))
    )
  )
  (query (document "memory://snapshot/state_transition_view.md") (range (start 14 21) (end 14 40)) (probe (position 14 21))
    (reference (id (source (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::lifecycle"))) (kind featureTyping) (ordinal 0) (authored-target "StateTransitionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView")))))
    )
  )
  (query (document "memory://snapshot/state_transition_view.md") (range (start 15 15) (end 15 28)) (probe (position 15 15))
    (reference (id (source (node (document "memory://snapshot/state_transition_view.md") (path (named (kind package) (name "DoorController")) (named (kind view) (name "lifecycle")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "DoorLifecycle")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_transition_view.md") (qualified-name "DoorController::DoorLifecycle")))))
    )
  )
)
~~~
