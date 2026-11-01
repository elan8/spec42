# META
~~~ini
description=State transition view projects states initial final and transitions
type=generate
libraries=standard
plugin=native:diagram
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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:5f12a1cb741c26e06b3f7ae1acbf0dcb81fe94c14201856942d1048f7e354bf3") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "idle")))))
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "idle")) (transitionTarget (reference "done")) (transitionTrigger (reference "Start")))))
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "finish")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (succession (reference "idle")) (succession (reference "done")))))
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "finish")) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "finish")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "finish")) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "finish")) (anonymous (kind parameter) (ordinal 1))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "finish")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction inout)))
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
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::done")))))
    (reference (id (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (kind transitionTrigger) (ordinal 0))
      (authored-target "Start")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Start")))))
    (reference (id (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "finish")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "finish")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "done")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::done")))))
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
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::done"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Start"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "finish")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "finish")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "finish")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::done"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "finish")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::selected"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::selected"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::stateTransitions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "finish")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::accepter"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "finish")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::transitionLinkSource"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "finish")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "finish")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "finish")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "finish")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "finish")) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "finish")) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "finish")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "finish")) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::payload"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "finish")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::transitionLinkSource"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "finish")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "finish")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "finish")) (anonymous (kind accept-action) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::idle"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::idle"))) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::idle"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::exclusiveStates"))) (provenance implied))
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
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::HappensBefore")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::HappensBefore")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::HappensLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Without")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish")))
      (featured-by (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::transitions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::stateTransitions"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::transitions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions")) (scopes any feature))
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
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StateTransitionPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::stateTransitions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "finish")) (anonymous (kind succession) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::HappensBefore")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::HappensBefore")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::HappensLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Without")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "finish")) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::transitionLinkSource"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::transitionLinkSource"))))
      (effective-type (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StateTransitionPerformance::transitionLinkSource"))))
      (effective-type (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::transitionLinkSource"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::transitionLinkSource")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::DecisionPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StateTransitionPerformance::transitionLinkSource")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::transitionLinkSource")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::transitionLinkSource")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "finish")) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish")))
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
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "finish")) (anonymous (kind parameter) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "finish")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::payload")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::payload")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "finish")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "finish")) (anonymous (kind accept-action) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::payload")) (scopes any feature))
      (subtype (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "finish")) (anonymous (kind parameter) (ordinal 1)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::idle")))
      (featured-by (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::substates"))))
      (effective-type (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::exclusiveStates"))))
      (effective-type (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::DecisionPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::middle")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (path (named (kind library-package) (name "States")) (named (kind state-def) (name "StateAction")) (anonymous (kind action) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::exclusiveStates")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::substates")) (scopes any feature))
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
  (query (document "memory://snapshot/diagram_state_transition_complete.md") (range (start 6 32) (end 6 36)) (probe (position 6 32))
    (reference (id (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (kind transitionSource) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/diagram_state_transition_complete.md") (range (start 6 55) (end 6 59)) (probe (position 6 55))
    (reference (id (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (kind transitionTarget) (ordinal 0) (authored-target "done")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::done")))))
    )
  )
  (query (document "memory://snapshot/diagram_state_transition_complete.md") (range (start 6 44) (end 6 49)) (probe (position 6 44))
    (reference (id (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::finish"))) (kind transitionTrigger) (ordinal 0) (authored-target "Start")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Start")))))
    )
  )
  (query (document "memory://snapshot/diagram_state_transition_complete.md") (range (start 6 32) (end 6 36)) (probe (position 6 32))
    (reference (id (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "finish")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/diagram_state_transition_complete.md") (range (start 6 55) (end 6 59)) (probe (position 6 55))
    (reference (id (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (path (named (kind package) (name "StateExample")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "finish")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "done")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::done")))))
    )
  )
  (query (document "memory://snapshot/diagram_state_transition_complete.md") (range (start 8 20) (end 8 39)) (probe (position 8 20))
    (reference (id (source (node (document "memory://snapshot/diagram_state_transition_complete.md") (qualified-name "StateExample::selected"))) (kind featureTyping) (ordinal 0) (authored-target "StateTransitionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView")))))
    )
  )
  (query (document "memory://snapshot/diagram_state_transition_complete.md") (range (start 8 49) (end 8 56)) (probe (position 8 49))
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
  "schemaVersion": 11,
  "modelDigest": "blake3:869bdba229ae1b558be91e9824d0c561c69d9957bf1b71c758853519c3351b24",
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
      "uri": "memory://snapshot/sysml.library/occurrences.md",
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
        8,
        6,
        60
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        19,
        6,
        25
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        32,
        6,
        36
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        44,
        6,
        49
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        55,
        6,
        59
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        9,
        8,
        17
      ]
    },
    {
      "document": 3,
      "range": [
        33,
        12,
        33,
        16
      ]
    },
    {
      "document": 3,
      "range": [
        33,
        18,
        33,
        29
      ]
    },
    {
      "document": 3,
      "range": [
        33,
        34,
        33,
        46
      ]
    },
    {
      "document": 3,
      "range": [
        33,
        48,
        33,
        73
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
      "qualifiedName": "Actions::Action::done"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "Occurrences::Occurrence::endShot"
    },
    {
      "document": 3,
      "kind": "qualified-name",
      "qualifiedName": "States::StateAction"
    },
    {
      "document": 3,
      "kind": "qualified-name",
      "qualifiedName": "States::StateAction::done"
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
      "source": 4,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "StateExample::Machine::finish",
      "source": 4,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "StateExample::Machine::finish",
      "source": 4,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "StateExample::Machine::finish::",
      "source": 4,
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
      "ordinal": 2,
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
      "ordinal": 3,
      "relationshipKind": "initialState",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "initialState",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 11,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 16,
      "relationshipKind": "transitionSource",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 17,
      "relationshipKind": "transitionTarget",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 18,
      "relationshipKind": "transitionTrigger",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 12,
      "relationshipKind": "containment",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 28,
      "relationshipKind": "succession",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 29,
      "relationshipKind": "succession",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "succession",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "transition",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "redefinition",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "redefinition",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "typing",
      "source": 10
    }
  ],
  "selectedView": {
    "reference": 6,
    "kind": "state-transition-view",
    "name": "selected",
    "source": 9
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "edges": [
      {
        "kind": "containment",
        "navigation": 10,
        "origin": 1,
        "provenance": "implied",
        "reference": 16,
        "source": 0,
        "target": 1
      },
      {
        "kind": "containment",
        "navigation": 3,
        "origin": 10,
        "provenance": "authored",
        "reference": 17,
        "source": 0,
        "target": 10
      },
      {
        "kind": "containment",
        "navigation": 1,
        "origin": 9,
        "provenance": "authored",
        "reference": 18,
        "source": 0,
        "target": 9
      },
      {
        "kind": "initial-state",
        "navigation": 2,
        "origin": 9,
        "provenance": "authored",
        "reference": 20,
        "source": 9,
        "target": 10
      },
      {
        "kind": "containment",
        "navigation": 5,
        "origin": 2,
        "provenance": "authored",
        "reference": 19,
        "source": 0,
        "target": 2
      },
      {
        "kind": "transition",
        "navigation": 6,
        "origin": 2,
        "provenance": "implied",
        "reference": 34,
        "source": 10,
        "target": 1
      },
      {
        "kind": "containment",
        "navigation": 4,
        "origin": 8,
        "provenance": "authored",
        "reference": 22,
        "source": 2,
        "target": 8
      },
      {
        "kind": "containment",
        "navigation": 4,
        "origin": 8,
        "provenance": "authored",
        "reference": 23,
        "source": 2,
        "target": 8
      },
      {
        "kind": "containment",
        "navigation": 4,
        "origin": 8,
        "provenance": "authored",
        "reference": 24,
        "source": 2,
        "target": 8
      },
      {
        "kind": "containment",
        "navigation": 4,
        "origin": 3,
        "provenance": "authored",
        "reference": 25,
        "source": 2,
        "target": 3
      },
      {
        "kind": "succession",
        "navigation": 6,
        "origin": 3,
        "provenance": "implied",
        "reference": 33,
        "source": 10,
        "target": 1
      },
      {
        "kind": "containment",
        "navigation": 4,
        "origin": 4,
        "provenance": "authored",
        "reference": 26,
        "source": 2,
        "target": 4
      },
      {
        "kind": "containment",
        "navigation": 4,
        "origin": 5,
        "provenance": "authored",
        "reference": 30,
        "source": 4,
        "target": 5
      }
    ],
    "exposedRoots": [
      0
    ],
    "kind": "state-transition-view",
    "metadata": {
      "finalNodes": [
        1
      ],
      "initialNodes": [
        9
      ],
      "states": [
        0,
        10
      ]
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "states",
            "members": [
              10
            ],
            "provenance": "direct"
          },
          {
            "kind": "states",
            "members": [
              1
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "StateDefinition",
        "multiplicity": null,
        "name": "Machine",
        "notationRole": "definition",
        "owner": null,
        "presentationKey": "p:0639116c7b7a460ba4d3a6e7d3100baa2769253aae9301d7b9fca16da16ae433",
        "reference": 0,
        "source": 0,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "StateUsage",
        "multiplicity": null,
        "name": "done",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:726625c7ba5ce945a049494710faa11e8fb65cd850c3b64e7af1bbe0006584f0",
        "reference": 10,
        "source": 10,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "StateAction",
              "reference": 9
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "attributes",
            "members": [
              8,
              8,
              8
            ],
            "provenance": "direct"
          },
          {
            "kind": "actions",
            "members": [
              4
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "TransitionUsage",
        "multiplicity": null,
        "name": "finish",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:b70a2e8f4cc56eff46e9cafb905634e17f78e1e88803dc3023dabc753d58d285",
        "reference": 2,
        "source": 5,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "SuccessionAsUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 2,
        "presentationKey": "p:e40a78cb467aa18e06602ccadcac6c45a4f5d8a7e4afbe0fabc7c82db2442aac",
        "reference": 14,
        "source": 4,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [
          {
            "kind": "attributes",
            "members": [
              5
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "AcceptActionUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": 2,
        "presentationKey": "p:00c9667ee160fe3993cc9924ecfa72ff3a1c194694ad6a60180cd812df71a436",
        "reference": 12,
        "source": 4,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": "inout",
        "featureDirection": null,
        "metaclass": "ReferenceUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "reference-usage",
        "owner": 4,
        "presentationKey": "p:ae9fa08e4379f64d998a541b9d9a1d94744b82bbb51da3ef9270a6c2cceeba54",
        "reference": 15,
        "source": 4,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "ReferenceUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "reference-usage",
        "owner": 2,
        "presentationKey": "p:afd0d0bad44119c8b6f011823be45b21019cc81a6769d14ffff75473a5c25c3e",
        "reference": 13,
        "source": 4,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": "in",
        "featureDirection": null,
        "metaclass": "ReferenceUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "reference-usage",
        "owner": 2,
        "presentationKey": "p:96b82f2000726e01c26f01c4cc91e55062914a0e1c00f6d7bddfa46ec4d9afe3",
        "reference": 13,
        "source": 4,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": "in",
        "featureDirection": null,
        "metaclass": "ReferenceUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "reference-usage",
        "owner": 2,
        "presentationKey": "p:229d3cde721c795cd1515a42e85a453b745ab178bb42500c2407f2a6495a7ff4",
        "reference": 13,
        "source": 4,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "SuccessionAsUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 0,
        "presentationKey": "p:6df3a93f3cb39dd569e18f1fc9d15ec126aba166712cfcfa75a93868f4c5da66",
        "reference": 11,
        "source": 1,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "StateUsage",
        "multiplicity": null,
        "name": "idle",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:ae04bebfc5e16b3a664eb809aab8b4f6ea81280a4c1853f2f7a9f36e4eeca440",
        "reference": 4,
        "source": 3,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      }
    ],
    "relationships": [
      {
        "kind": "typing",
        "navigation": 11,
        "provenance": "authored",
        "reference": 37,
        "source": 1,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": 12,
        "provenance": "authored",
        "reference": 35,
        "source": 1,
        "target": {
          "reference": 7,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": 13,
        "provenance": "authored",
        "reference": 36,
        "source": 1,
        "target": {
          "reference": 8,
          "status": "resolved"
        }
      },
      {
        "kind": "initialState",
        "navigation": 2,
        "provenance": "authored",
        "reference": 21,
        "source": 9,
        "target": {
          "node": 10,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionSource",
        "navigation": 6,
        "provenance": "authored",
        "reference": 27,
        "source": 2,
        "target": {
          "node": 10,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTarget",
        "navigation": 8,
        "provenance": "authored",
        "reference": 28,
        "source": 2,
        "target": {
          "node": 1,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTrigger",
        "navigation": 7,
        "provenance": "authored",
        "reference": 29,
        "source": 2,
        "target": {
          "reference": 5,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 6,
        "provenance": "authored",
        "reference": 31,
        "source": 3,
        "target": {
          "node": 10,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 8,
        "provenance": "authored",
        "reference": 32,
        "source": 3,
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
          "navigation": 6,
          "provenance": "implied",
          "source": 2,
          "target": 0,
          "trigger": {
            "label": "Start",
            "navigation": 7,
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
          "navigation": 10
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
