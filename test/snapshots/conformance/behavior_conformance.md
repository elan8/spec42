# META
~~~ini
description=Performs, transitions and state machines report their settled endpoints
type=file
~~~
# SOURCE
~~~sysml
package Behavior {
    action def Step;
    part def Structure;

    part def ConformingPerform {
        perform action ok : Step;
    }

    part def PerformsSomethingThatIsNotAnAction {
        perform action wrong : Structure;
    }

    state def ConformingMachine {
        then running;
        state running;
        final stopped;
        transition first running then stopped;
    }

    state def TransitionLeavesAState {
        then armed;
        state armed;
        final disarmed;
        transition first armed then Structure;
    }

    action def ConformingSuccession {
        action first : Step;
        action second : Step;
        first first then second;
    }

    action def SuccessionOverSomethingThatIsNotAnAction {
        part structure : Structure;
        action step : Step;
        first structure then step;
    }

    action def AcceptsAnIncompatiblePayload {
        accept incoming : Step;
    }

    state def NoInitialOrFinalMarker {
        state opening;
        state closing;
        transition first opening then closing;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/behavior_conformance.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "perform_target_invalid_kind")
        (source "semantic")
        (range (start 9 31) (end 9 40))
        (related-information
          (related
            (uri "memory://snapshot/behavior_conformance.md")
            (range (start 2 4) (end 2 23))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "transition_endpoint_invalid_state")
        (source "semantic")
        (range (start 23 8) (end 23 46))
        (related-information
          (related
            (uri "memory://snapshot/behavior_conformance.md")
            (range (start 21 8) (end 21 20))
          )
          (related
            (uri "memory://snapshot/behavior_conformance.md")
            (range (start 2 4) (end 2 23))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "succession_endpoint_invalid")
        (source "semantic")
        (range (start 35 14) (end 35 23))
        (related-information
          (related
            (uri "memory://snapshot/behavior_conformance.md")
            (range (start 33 8) (end 33 35))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "accept_payload_incompatible")
        (source "semantic")
        (range (start 39 26) (end 39 30))
        (related-information
          (related
            (uri "memory://snapshot/behavior_conformance.md")
            (range (start 1 4) (end 1 20))
          )
        )
      )
      (diagnostic
        (severity information)
        (code "missing_final_state")
        (source "semantic")
        (range (start 42 4) (end 46 5))
      )
      (diagnostic
        (severity information)
        (code "missing_initial_state")
        (source "semantic")
        (range (start 42 4) (end 46 5))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:39aaa8d2f87b6729b3a6a98cdefab1758af92ed95dcb18833372ed5ac2be218b") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::AcceptsAnIncompatiblePayload"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::AcceptsAnIncompatiblePayload::accept"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (acceptPayloadType (reference "Step")))))
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingMachine"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "ConformingMachine")) (anonymous (kind initial-state) (ordinal 0))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "running")))))
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "ConformingMachine")) (anonymous (kind transition) (ordinal 0))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "running")) (transitionTarget (reference "stopped")))))
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingMachine::running"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingMachine::stopped"))) (kind final-state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingPerform"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingPerform::ok"))) (kind perform-action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Step")))))
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingSuccession"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind action-def) (name "ConformingSuccession")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "first")) (succession (reference "second")))))
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingSuccession::first"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Step")))))
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingSuccession::second"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Step")))))
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::NoInitialOrFinalMarker"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "NoInitialOrFinalMarker")) (anonymous (kind transition) (ordinal 0))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "opening")) (transitionTarget (reference "closing")))))
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::NoInitialOrFinalMarker::closing"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::NoInitialOrFinalMarker::opening"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::PerformsSomethingThatIsNotAnAction"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::PerformsSomethingThatIsNotAnAction::wrong"))) (kind perform-action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Structure")))))
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Step"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Structure"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::SuccessionOverSomethingThatIsNotAnAction"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind action-def) (name "SuccessionOverSomethingThatIsNotAnAction")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "structure")) (succession (reference "step")))))
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::SuccessionOverSomethingThatIsNotAnAction::step"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Step")))))
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::SuccessionOverSomethingThatIsNotAnAction::structure"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Structure")))))
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::TransitionLeavesAState"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "TransitionLeavesAState")) (anonymous (kind initial-state) (ordinal 0))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "armed")))))
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "TransitionLeavesAState")) (anonymous (kind transition) (ordinal 0))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "armed")) (transitionTarget (reference "Structure")))))
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::TransitionLeavesAState::armed"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::TransitionLeavesAState::disarmed"))) (kind final-state) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::AcceptsAnIncompatiblePayload::accept"))) (kind acceptPayloadType) (ordinal 0))
      (authored-target "Step")
      (outcome (status resolved) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Step")))))
    (reference (id (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "ConformingMachine")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0))
      (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingMachine::running")))))
    (reference (id (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "ConformingMachine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0))
      (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingMachine::running")))))
    (reference (id (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "ConformingMachine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0))
      (authored-target "stopped")
      (outcome (status resolved) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingMachine::stopped")))))
    (reference (id (source (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingPerform::ok"))) (kind featureTyping) (ordinal 0))
      (authored-target "Step")
      (outcome (status resolved) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Step")))))
    (reference (id (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind action-def) (name "ConformingSuccession")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "first")
      (outcome (status resolved) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingSuccession::first")))))
    (reference (id (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind action-def) (name "ConformingSuccession")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "second")
      (outcome (status resolved) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingSuccession::second")))))
    (reference (id (source (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingSuccession::first"))) (kind featureTyping) (ordinal 0))
      (authored-target "Step")
      (outcome (status resolved) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Step")))))
    (reference (id (source (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingSuccession::second"))) (kind featureTyping) (ordinal 0))
      (authored-target "Step")
      (outcome (status resolved) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Step")))))
    (reference (id (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "NoInitialOrFinalMarker")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0))
      (authored-target "opening")
      (outcome (status resolved) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::NoInitialOrFinalMarker::opening")))))
    (reference (id (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "NoInitialOrFinalMarker")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0))
      (authored-target "closing")
      (outcome (status resolved) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::NoInitialOrFinalMarker::closing")))))
    (reference (id (source (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::PerformsSomethingThatIsNotAnAction::wrong"))) (kind featureTyping) (ordinal 0))
      (authored-target "Structure")
      (outcome (status resolved) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Structure")))))
    (reference (id (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind action-def) (name "SuccessionOverSomethingThatIsNotAnAction")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "structure")
      (outcome (status resolved) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::SuccessionOverSomethingThatIsNotAnAction::structure")))))
    (reference (id (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind action-def) (name "SuccessionOverSomethingThatIsNotAnAction")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "step")
      (outcome (status resolved) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::SuccessionOverSomethingThatIsNotAnAction::step")))))
    (reference (id (source (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::SuccessionOverSomethingThatIsNotAnAction::step"))) (kind featureTyping) (ordinal 0))
      (authored-target "Step")
      (outcome (status resolved) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Step")))))
    (reference (id (source (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::SuccessionOverSomethingThatIsNotAnAction::structure"))) (kind featureTyping) (ordinal 0))
      (authored-target "Structure")
      (outcome (status resolved) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Structure")))))
    (reference (id (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "TransitionLeavesAState")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0))
      (authored-target "armed")
      (outcome (status resolved) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::TransitionLeavesAState::armed")))))
    (reference (id (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "TransitionLeavesAState")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0))
      (authored-target "armed")
      (outcome (status resolved) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::TransitionLeavesAState::armed")))))
    (reference (id (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "TransitionLeavesAState")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0))
      (authored-target "Structure")
      (outcome (status resolved) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Structure")))))
  )
  (relationships
    (relationship (kind acceptPayloadType) (source (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::AcceptsAnIncompatiblePayload::accept"))) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Step"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::AcceptsAnIncompatiblePayload::accept"))) (kind acceptPayloadType) (ordinal 0)))
    (relationship (kind initialState) (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "ConformingMachine")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingMachine::running"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "ConformingMachine")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "ConformingMachine")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingMachine::running"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "ConformingMachine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "ConformingMachine")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingMachine::stopped"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "ConformingMachine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingPerform::ok"))) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Step"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingPerform::ok"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind action-def) (name "ConformingSuccession")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingSuccession::first"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind action-def) (name "ConformingSuccession")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind action-def) (name "ConformingSuccession")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingSuccession::second"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind action-def) (name "ConformingSuccession")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingSuccession::first"))) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Step"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingSuccession::first"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingSuccession::second"))) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Step"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingSuccession::second"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "NoInitialOrFinalMarker")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::NoInitialOrFinalMarker::opening"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "NoInitialOrFinalMarker")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "NoInitialOrFinalMarker")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::NoInitialOrFinalMarker::closing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "NoInitialOrFinalMarker")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::PerformsSomethingThatIsNotAnAction::wrong"))) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Structure"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::PerformsSomethingThatIsNotAnAction::wrong"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind action-def) (name "SuccessionOverSomethingThatIsNotAnAction")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::SuccessionOverSomethingThatIsNotAnAction::structure"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind action-def) (name "SuccessionOverSomethingThatIsNotAnAction")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind action-def) (name "SuccessionOverSomethingThatIsNotAnAction")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::SuccessionOverSomethingThatIsNotAnAction::step"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind action-def) (name "SuccessionOverSomethingThatIsNotAnAction")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::SuccessionOverSomethingThatIsNotAnAction::step"))) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Step"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::SuccessionOverSomethingThatIsNotAnAction::step"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::SuccessionOverSomethingThatIsNotAnAction::structure"))) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Structure"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::SuccessionOverSomethingThatIsNotAnAction::structure"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind initialState) (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "TransitionLeavesAState")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::TransitionLeavesAState::armed"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "TransitionLeavesAState")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "TransitionLeavesAState")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::TransitionLeavesAState::armed"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "TransitionLeavesAState")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "TransitionLeavesAState")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Structure"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "TransitionLeavesAState")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::AcceptsAnIncompatiblePayload::accept")))
      (featured-by (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::AcceptsAnIncompatiblePayload")))
    )
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "ConformingMachine")) (anonymous (kind initial-state) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingMachine")))
    )
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "ConformingMachine")) (anonymous (kind transition) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingMachine")))
    )
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingMachine::running")))
      (featured-by (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingMachine")))
    )
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingMachine::stopped")))
      (featured-by (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingMachine")))
    )
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingPerform::ok")))
      (featured-by (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingPerform")))
      (type (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Step")) (provenance authored))
      (effective-type (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Step")) (source direct))
      (supertype (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Step")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind action-def) (name "ConformingSuccession")) (anonymous (kind succession) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingSuccession")))
    )
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingSuccession::first")))
      (featured-by (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingSuccession")))
      (type (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Step")) (provenance authored))
      (effective-type (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Step")) (source direct))
      (supertype (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Step")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingSuccession::second")))
      (featured-by (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingSuccession")))
      (type (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Step")) (provenance authored))
      (effective-type (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Step")) (source direct))
      (supertype (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Step")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "NoInitialOrFinalMarker")) (anonymous (kind transition) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::NoInitialOrFinalMarker")))
    )
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::NoInitialOrFinalMarker::closing")))
      (featured-by (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::NoInitialOrFinalMarker")))
    )
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::NoInitialOrFinalMarker::opening")))
      (featured-by (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::NoInitialOrFinalMarker")))
    )
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::PerformsSomethingThatIsNotAnAction::wrong")))
      (featured-by (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::PerformsSomethingThatIsNotAnAction")))
      (type (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Structure")) (provenance authored))
      (effective-type (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Structure")) (source direct))
      (supertype (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Structure")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Step")))
      (subtype (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingPerform::ok")) (scopes any))
      (subtype (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingSuccession::first")) (scopes any))
      (subtype (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingSuccession::second")) (scopes any))
      (subtype (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::SuccessionOverSomethingThatIsNotAnAction::step")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Structure")))
      (subtype (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::PerformsSomethingThatIsNotAnAction::wrong")) (scopes any))
      (subtype (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::SuccessionOverSomethingThatIsNotAnAction::structure")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind action-def) (name "SuccessionOverSomethingThatIsNotAnAction")) (anonymous (kind succession) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::SuccessionOverSomethingThatIsNotAnAction")))
    )
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::SuccessionOverSomethingThatIsNotAnAction::step")))
      (featured-by (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::SuccessionOverSomethingThatIsNotAnAction")))
      (type (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Step")) (provenance authored))
      (effective-type (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Step")) (source direct))
      (supertype (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Step")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::SuccessionOverSomethingThatIsNotAnAction::structure")))
      (featured-by (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::SuccessionOverSomethingThatIsNotAnAction")))
      (type (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Structure")) (provenance authored))
      (effective-type (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Structure")) (source direct))
      (supertype (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Structure")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "TransitionLeavesAState")) (anonymous (kind initial-state) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::TransitionLeavesAState")))
    )
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "TransitionLeavesAState")) (anonymous (kind transition) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::TransitionLeavesAState")))
    )
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::TransitionLeavesAState::armed")))
      (featured-by (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::TransitionLeavesAState")))
    )
    (declaration (id (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::TransitionLeavesAState::disarmed")))
      (featured-by (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::TransitionLeavesAState")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/behavior_conformance.md") (range (start 39 26) (end 39 30)) (probe (position 39 26))
    (reference (id (source (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::AcceptsAnIncompatiblePayload::accept"))) (kind acceptPayloadType) (ordinal 0) (authored-target "Step")
      (outcome (status resolved) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Step")))))
    )
  )
  (query (document "memory://snapshot/behavior_conformance.md") (range (start 13 13) (end 13 20)) (probe (position 13 13))
    (reference (id (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "ConformingMachine")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0) (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingMachine::running")))))
    )
  )
  (query (document "memory://snapshot/behavior_conformance.md") (range (start 16 25) (end 16 32)) (probe (position 16 25))
    (reference (id (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "ConformingMachine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0) (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingMachine::running")))))
    )
  )
  (query (document "memory://snapshot/behavior_conformance.md") (range (start 16 38) (end 16 45)) (probe (position 16 38))
    (reference (id (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "ConformingMachine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0) (authored-target "stopped")
      (outcome (status resolved) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingMachine::stopped")))))
    )
  )
  (query (document "memory://snapshot/behavior_conformance.md") (range (start 5 28) (end 5 32)) (probe (position 5 28))
    (reference (id (source (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingPerform::ok"))) (kind featureTyping) (ordinal 0) (authored-target "Step")
      (outcome (status resolved) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Step")))))
    )
  )
  (query (document "memory://snapshot/behavior_conformance.md") (range (start 29 14) (end 29 19)) (probe (position 29 14))
    (reference (id (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind action-def) (name "ConformingSuccession")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "first")
      (outcome (status resolved) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingSuccession::first")))))
    )
  )
  (query (document "memory://snapshot/behavior_conformance.md") (range (start 29 25) (end 29 31)) (probe (position 29 25))
    (reference (id (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind action-def) (name "ConformingSuccession")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "second")
      (outcome (status resolved) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingSuccession::second")))))
    )
  )
  (query (document "memory://snapshot/behavior_conformance.md") (range (start 27 23) (end 27 27)) (probe (position 27 23))
    (reference (id (source (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingSuccession::first"))) (kind featureTyping) (ordinal 0) (authored-target "Step")
      (outcome (status resolved) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Step")))))
    )
  )
  (query (document "memory://snapshot/behavior_conformance.md") (range (start 28 24) (end 28 28)) (probe (position 28 24))
    (reference (id (source (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::ConformingSuccession::second"))) (kind featureTyping) (ordinal 0) (authored-target "Step")
      (outcome (status resolved) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Step")))))
    )
  )
  (query (document "memory://snapshot/behavior_conformance.md") (range (start 45 25) (end 45 32)) (probe (position 45 25))
    (reference (id (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "NoInitialOrFinalMarker")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0) (authored-target "opening")
      (outcome (status resolved) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::NoInitialOrFinalMarker::opening")))))
    )
  )
  (query (document "memory://snapshot/behavior_conformance.md") (range (start 45 38) (end 45 45)) (probe (position 45 38))
    (reference (id (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "NoInitialOrFinalMarker")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0) (authored-target "closing")
      (outcome (status resolved) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::NoInitialOrFinalMarker::closing")))))
    )
  )
  (query (document "memory://snapshot/behavior_conformance.md") (range (start 9 31) (end 9 40)) (probe (position 9 31))
    (reference (id (source (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::PerformsSomethingThatIsNotAnAction::wrong"))) (kind featureTyping) (ordinal 0) (authored-target "Structure")
      (outcome (status resolved) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Structure")))))
    )
  )
  (query (document "memory://snapshot/behavior_conformance.md") (range (start 35 14) (end 35 23)) (probe (position 35 14))
    (reference (id (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind action-def) (name "SuccessionOverSomethingThatIsNotAnAction")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "structure")
      (outcome (status resolved) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::SuccessionOverSomethingThatIsNotAnAction::structure")))))
    )
  )
  (query (document "memory://snapshot/behavior_conformance.md") (range (start 35 29) (end 35 33)) (probe (position 35 29))
    (reference (id (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind action-def) (name "SuccessionOverSomethingThatIsNotAnAction")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "step")
      (outcome (status resolved) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::SuccessionOverSomethingThatIsNotAnAction::step")))))
    )
  )
  (query (document "memory://snapshot/behavior_conformance.md") (range (start 34 22) (end 34 26)) (probe (position 34 22))
    (reference (id (source (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::SuccessionOverSomethingThatIsNotAnAction::step"))) (kind featureTyping) (ordinal 0) (authored-target "Step")
      (outcome (status resolved) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Step")))))
    )
  )
  (query (document "memory://snapshot/behavior_conformance.md") (range (start 33 25) (end 33 34)) (probe (position 33 25))
    (reference (id (source (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::SuccessionOverSomethingThatIsNotAnAction::structure"))) (kind featureTyping) (ordinal 0) (authored-target "Structure")
      (outcome (status resolved) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Structure")))))
    )
  )
  (query (document "memory://snapshot/behavior_conformance.md") (range (start 20 13) (end 20 18)) (probe (position 20 13))
    (reference (id (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "TransitionLeavesAState")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0) (authored-target "armed")
      (outcome (status resolved) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::TransitionLeavesAState::armed")))))
    )
  )
  (query (document "memory://snapshot/behavior_conformance.md") (range (start 23 25) (end 23 30)) (probe (position 23 25))
    (reference (id (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "TransitionLeavesAState")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0) (authored-target "armed")
      (outcome (status resolved) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::TransitionLeavesAState::armed")))))
    )
  )
  (query (document "memory://snapshot/behavior_conformance.md") (range (start 23 36) (end 23 45)) (probe (position 23 36))
    (reference (id (source (node (document "memory://snapshot/behavior_conformance.md") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "TransitionLeavesAState")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0) (authored-target "Structure")
      (outcome (status resolved) (target (node (document "memory://snapshot/behavior_conformance.md") (qualified-name "Behavior::Structure")))))
    )
  )
)
~~~
