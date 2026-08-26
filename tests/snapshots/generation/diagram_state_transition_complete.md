# META
~~~ini
description=State transition view projects states initial final and transitions
type=generate
libraries=standard
plugin=repository:diagram
viewKind=state-transition-view
viewDocument=diagram_state_transition_complete.md
viewQualifiedName=StateExample::selected
~~~
# SOURCE
~~~sysml
package StateExample {
    private import StandardViewDefinitions::*;
    item def Start;
    state def Machine {
        then idle;
        state idle;
        final done;
        transition finish first idle accept Start then done;
    }
    view selected : StateTransitionView { expose Machine; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagram_state_transition_complete.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:aab42d851e2e58d57f22ee31c4d1530c5657248959ede0fdc4f57c110ca50024") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "idle")))))
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::done"))) (kind final-state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "idle")) (transitionTarget (reference "done")) (transitionTrigger (reference "Start")))))
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "finish")) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::idle"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Start"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::selected"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StateTransitionView")))))
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Machine")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (kind transitionSource) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (kind transitionTarget) (ordinal 0))
      (authored-target "done")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::done")))))
    (reference (id (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (kind transitionTrigger) (ordinal 0))
      (authored-target "Start")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Start")))))
    (reference (id (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::selected"))) (kind featureTyping) (ordinal 0))
      (authored-target "StateTransitionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView")))))
    (reference (id (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Machine")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine")))))
  )
  (relationships
    (relationship (kind initialState) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::done"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Start"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::selected"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::selected"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::done"))) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "finish")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::accepter"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "finish")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::idle"))) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::idle"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Start"))) (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::selected"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::selected"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::DecisionPerformance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::done")))
      (featured-by (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish")))
      (featured-by (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "finish")) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::AcceptMessageAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::accepter"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::acceptPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::accept"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::AcceptMessageAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::accepter")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::acceptPerformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::accept")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::idle")))
      (featured-by (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::DecisionPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Start")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::selected")))
      (type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (source inherited) (from (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::selected")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/diagram_state_transition_complete.md") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/diagram_state_transition_complete.md") (range (start 4 13) (end 4 17)) (probe (position 4 13))
    (reference (id (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/diagram_state_transition_complete.md") (range (start 7 32) (end 7 36)) (probe (position 7 32))
    (reference (id (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (kind transitionSource) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/diagram_state_transition_complete.md") (range (start 7 55) (end 7 59)) (probe (position 7 55))
    (reference (id (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (kind transitionTarget) (ordinal 0) (authored-target "done")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::done")))))
    )
  )
  (query (document "memory://snapshot/diagram_state_transition_complete.md") (range (start 7 44) (end 7 49)) (probe (position 7 44))
    (reference (id (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (kind transitionTrigger) (ordinal 0) (authored-target "Start")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Start")))))
    )
  )
  (query (document "memory://snapshot/diagram_state_transition_complete.md") (range (start 9 20) (end 9 39)) (probe (position 9 20))
    (reference (id (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::selected"))) (kind featureTyping) (ordinal 0) (authored-target "StateTransitionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView")))))
    )
  )
  (query (document "memory://snapshot/diagram_state_transition_complete.md") (range (start 9 49) (end 9 56)) (probe (position 9 49))
    (reference (id (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Machine")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 5,
  "modelDigest": "blake3:640bb9efa9690d28ba4c8c1c97805bdf83a4eed82ddfb3ae2a7316a7d95ec16c",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_state_transition_complete.md",
      "sourceDomain": "workspace"
    },
    {
      "uri": "memory://snapshot/sysml.library/actions.md",
      "sourceDomain": "standard-library"
    },
    {
      "uri": "memory://snapshot/sysml.library/states.md",
      "sourceDomain": "standard-library"
    }
  ],
  "sources": [
    {
      "document": 0,
      "range": [
        3,
        14,
        3,
        21
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        8,
        4,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        13,
        4,
        17
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        14,
        5,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        14,
        6,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        8,
        7,
        60
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        19,
        7,
        25
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        32,
        7,
        36
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        44,
        7,
        49
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        55,
        7,
        59
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        9,
        9,
        17
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "StateExample::Machine"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "StateExample::Machine::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "StateExample::Machine::done"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "StateExample::Machine::finish"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "StateExample::Machine::finish::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "StateExample::Machine::idle"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "StateExample::Start"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "StateExample::selected"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Actions::TransitionAction::accepter"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Actions::transitionActions"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "States::StateAction"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "States::stateActions"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "StateExample::Machine",
      "source": 1,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "AcceptActionUsage",
      "ownerQualifiedName": "StateExample::Machine::finish",
      "source": 5,
      "sourceDomain": "workspace"
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "specializes",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "initialState",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "initialState",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "typeFeaturing",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "typeFeaturing",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "containment",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "subsetting",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "transitionSource",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "transitionTarget",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "transitionTrigger",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "typeFeaturing",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 11,
      "relationshipKind": "subsetting",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 12,
      "relationshipKind": "typeFeaturing",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "subsetting",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "transition",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "typeFeaturing",
      "source": 5
    }
  ],
  "selectedView": {
    "reference": 7,
    "kind": "state-transition-view",
    "name": "selected",
    "source": 10
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "edges": [
      {
        "kind": "containment",
        "navigation": 3,
        "provenance": "authored",
        "reference": 14,
        "source": 0,
        "target": 5
      },
      {
        "kind": "containment",
        "navigation": 1,
        "provenance": "authored",
        "reference": 15,
        "source": 0,
        "target": 4
      },
      {
        "kind": "initial-state",
        "navigation": 2,
        "provenance": "authored",
        "reference": 19,
        "source": 4,
        "target": 5
      },
      {
        "kind": "containment",
        "navigation": 4,
        "provenance": "authored",
        "reference": 16,
        "source": 0,
        "target": 3
      },
      {
        "kind": "containment",
        "navigation": 6,
        "provenance": "authored",
        "reference": 17,
        "source": 0,
        "target": 1
      },
      {
        "kind": "transition",
        "navigation": 7,
        "provenance": "implied",
        "reference": 32,
        "source": 5,
        "target": 3
      },
      {
        "kind": "containment",
        "navigation": 5,
        "provenance": "authored",
        "reference": 23,
        "source": 1,
        "target": 2
      }
    ],
    "exposedRoots": [
      0
    ],
    "kind": "state-transition-view",
    "metadata": {
      "finalNodes": [
        3
      ],
      "initialNodes": [
        4
      ],
      "states": [
        0,
        5
      ]
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "states",
            "members": [
              3,
              5
            ],
            "provenance": "direct"
          }
        ],
        "metaclass": "StateDefinition",
        "name": "Machine",
        "notationRole": "definition",
        "owner": null,
        "reference": 0,
        "source": 0,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [
          {
            "kind": "actions",
            "members": [
              2
            ],
            "provenance": "direct"
          }
        ],
        "metaclass": "TransitionUsage",
        "name": "finish",
        "notationRole": "usage",
        "owner": 0,
        "reference": 3,
        "source": 6,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "AcceptActionUsage",
        "name": null,
        "notationRole": "unsupported",
        "owner": 1,
        "reference": 13,
        "source": 5,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "FinalState",
        "name": "done",
        "notationRole": "usage",
        "owner": 0,
        "reference": 2,
        "source": 4,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "SuccessionAsUsage",
        "name": null,
        "notationRole": "unsupported",
        "owner": 0,
        "reference": 12,
        "source": 1,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "StateUsage",
        "name": "idle",
        "notationRole": "usage",
        "owner": 0,
        "reference": 5,
        "source": 3,
        "typing": {
          "status": "absent"
        }
      }
    ],
    "relationships": [
      {
        "kind": "specializes",
        "navigation": null,
        "provenance": "implied",
        "reference": 18,
        "source": 0,
        "target": {
          "reference": 10,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 31,
        "source": 5,
        "target": {
          "reference": 11,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 33,
        "source": 5,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "initialState",
        "navigation": 2,
        "provenance": "authored",
        "reference": 20,
        "source": 4,
        "target": {
          "node": 5,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 21,
        "source": 4,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 22,
        "source": 3,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 24,
        "source": 1,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionSource",
        "navigation": 7,
        "provenance": "authored",
        "reference": 25,
        "source": 1,
        "target": {
          "node": 5,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTarget",
        "navigation": 9,
        "provenance": "authored",
        "reference": 26,
        "source": 1,
        "target": {
          "node": 3,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTrigger",
        "navigation": 8,
        "provenance": "authored",
        "reference": 27,
        "source": 1,
        "target": {
          "reference": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 28,
        "source": 1,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 29,
        "source": 2,
        "target": {
          "reference": 8,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 30,
        "source": 2,
        "target": {
          "node": 1,
          "status": "resolved"
        }
      }
    ],
    "scene": {
      "frame": {
        "id": "state-machine",
        "label": "Machine",
        "navigation": 0
      },
      "kind": "state-transition",
      "transitions": [
        {
          "effect": {
            "status": "absent"
          },
          "guard": {
            "status": "absent"
          },
          "id": "transition-0",
          "label": null,
          "navigation": 2,
          "provenance": "authored",
          "source": 1,
          "target": 2,
          "trigger": {
            "status": "absent"
          }
        },
        {
          "effect": {
            "status": "absent"
          },
          "guard": {
            "status": "absent"
          },
          "id": "transition-1",
          "label": "finish",
          "navigation": 7,
          "provenance": "implied",
          "source": 2,
          "target": 0,
          "trigger": {
            "label": "Start",
            "navigation": 8,
            "status": "accept",
            "target": {
              "id": "element/v154:memory://snapshot/diagram_state_transition_complete.md7:packagen12:StateExample1:08:item-defn5:Start1:0",
              "label": "Start"
            }
          }
        }
      ],
      "vertices": [
        {
          "id": "state-0",
          "kind": "final",
          "label": "done",
          "navigation": 4
        },
        {
          "id": "state-1",
          "kind": "initial",
          "label": "",
          "navigation": 1
        },
        {
          "id": "state-2",
          "kind": "state",
          "label": "idle",
          "navigation": 3
        }
      ]
    }
  }
}

~~~
