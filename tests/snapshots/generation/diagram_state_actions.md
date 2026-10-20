# META
~~~ini
description=The entry, do and exit actions of a state are published with their role
type=generate
libraries=standard
plugin=native:diagram
viewKind=state-transition-view
viewDocument=model.sysml
viewQualifiedName=Oven::machine
~~~
# SOURCE
## model.sysml
~~~sysml
package Oven {
    private import StandardViewDefinitions::*;
    attribute def Go;
    attribute def Halt;
    action def Heat;
    action def Cool;
    action def Log;
    state def Machine {
        entry; then idle;
        state idle {
            entry action warmUp : Log;
            do action hold : Heat;
            exit action coolDown : Cool;
        }
        state baking {
            do action bake : Heat;
        }
        transition start first idle accept Go then baking;
        transition stop first baking accept Halt then idle;
    }
    view machine : StateTransitionView { expose Machine; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/model.sysml"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:bd27673c02184e1bb13ef8498896e2a7fe52232dc63e457646dddcae3284dda8") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Oven"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Cool"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Go"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Halt"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Heat"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Log"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entry-action-binding) (membership (kind feature) (visibility default) (role entry)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "idle")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::baking"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::baking::bake"))) (kind do-action-binding) (membership (kind feature) (visibility default) (role do)) (authored (membership (kind feature) (visibility default) (role do)) (relationships (featureTyping (reference "Heat")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle::coolDown"))) (kind exit-action-binding) (membership (kind feature) (visibility default) (role exit)) (authored (membership (kind feature) (visibility default) (role exit)) (relationships (featureTyping (reference "Cool")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle::hold"))) (kind do-action-binding) (membership (kind feature) (visibility default) (role do)) (authored (membership (kind feature) (visibility default) (role do)) (relationships (featureTyping (reference "Heat")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle::warmUp"))) (kind entry-action-binding) (membership (kind feature) (visibility default) (role entry)) (authored (membership (kind feature) (visibility default) (role entry)) (relationships (featureTyping (reference "Log")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::start"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "idle")) (transitionTarget (reference "baking")) (transitionTrigger (reference "Go")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "start")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (succession (reference "idle")) (succession (reference "baking")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "start")) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "start")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "start")) (anonymous (kind parameter) (ordinal 1))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction inout)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::stop"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "baking")) (transitionTarget (reference "idle")) (transitionTrigger (reference "Halt")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "stop")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (succession (reference "baking")) (succession (reference "idle")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "stop")) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "stop")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "stop")) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "stop")) (anonymous (kind parameter) (ordinal 1))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "stop")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction inout)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::machine"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StateTransitionView")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind view) (name "machine")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Machine")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::baking::bake"))) (kind featureTyping) (ordinal 0))
      (authored-target "Heat")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Heat")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle::coolDown"))) (kind featureTyping) (ordinal 0))
      (authored-target "Cool")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Cool")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle::hold"))) (kind featureTyping) (ordinal 0))
      (authored-target "Heat")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Heat")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle::warmUp"))) (kind featureTyping) (ordinal 0))
      (authored-target "Log")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Log")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::start"))) (kind transitionSource) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::start"))) (kind transitionTarget) (ordinal 0))
      (authored-target "baking")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::baking")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::start"))) (kind transitionTrigger) (ordinal 0))
      (authored-target "Go")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Go")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "start")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "start")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "baking")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::baking")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::stop"))) (kind transitionSource) (ordinal 0))
      (authored-target "baking")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::baking")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::stop"))) (kind transitionTarget) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::stop"))) (kind transitionTrigger) (ordinal 0))
      (authored-target "Halt")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Halt")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "stop")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "baking")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::baking")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "stop")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::machine"))) (kind featureTyping) (ordinal 0))
      (authored-target "StateTransitionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind view) (name "machine")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Machine")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine")))))
  )
  (relationships
    (relationship (kind initialState) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::baking::bake"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Heat"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::baking::bake"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle::coolDown"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Cool"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle::coolDown"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle::hold"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Heat"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle::hold"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle::warmUp"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Log"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle::warmUp"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::start"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::start"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::start"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::baking"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::start"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::start"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Go"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::start"))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "start")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "start")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "start")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::baking"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "start")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::stop"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::baking"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::stop"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::stop"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::stop"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::stop"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Halt"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::stop"))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "stop")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::baking"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "stop")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "stop")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "stop")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::machine"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::machine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind view) (name "machine")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind view) (name "machine")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Cool"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Go"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Halt"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Heat"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Log"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::entryAction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::baking"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::baking"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::baking"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::exclusiveStates"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::baking"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::baking::bake"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::baking::bake"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::baking::bake"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::baking"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::baking::bake"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::doAction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::exclusiveStates"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle::coolDown"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle::coolDown"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle::coolDown"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle::coolDown"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::exitAction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle::hold"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle::hold"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle::hold"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle::hold"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::doAction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle::warmUp"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle::warmUp"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle::warmUp"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle::warmUp"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::entryAction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::start"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::start"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::start"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::start"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::stateTransitions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::accepter"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "start")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::transitionLinkSource"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "start")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "start")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::start"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "start")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::start"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::start"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "start")) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::start"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "start")) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "start")) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::payload"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "start")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::transitionLinkSource"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::stop"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::stop"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::stop"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::stop"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::stateTransitions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "stop")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::accepter"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "stop")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::transitionLinkSource"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "stop")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "stop")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::stop"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "stop")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::stop"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "stop")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::stop"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "stop")) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::stop"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "stop")) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "stop")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "stop")) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::payload"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "stop")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::transitionLinkSource"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "stop")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "stop")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "stop")) (anonymous (kind accept-action) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::machine"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind view) (name "machine")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::machine"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Cool")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle::coolDown")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Go")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Halt")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Heat")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::baking::bake")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle::hold")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Log")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle::warmUp")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::DecisionPerformance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::entry")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::entryAction")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::baking")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::baking::bake")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::baking")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Heat")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Heat")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::doAction"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Heat")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
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
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::do")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::middle")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::doAction")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle::coolDown")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Cool")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Cool")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::exitAction"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Cool")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::exit")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::exitAction")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle::hold")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Heat")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Heat")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::doAction"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Heat")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
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
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::do")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::middle")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::doAction")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle::warmUp")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Log")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Log")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Log")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::entry")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::entryAction")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::start")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "start")) (anonymous (kind succession) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "start")) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::start")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "start")) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::start")))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::start")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "start")) (anonymous (kind parameter) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::start")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::payload")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::payload")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::payload")) (scopes any feature))
      (subtype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "start")) (anonymous (kind parameter) (ordinal 1)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::stop")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "stop")) (anonymous (kind succession) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "stop")) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::stop")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "stop")) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::stop")))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "stop")) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::stop")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "stop")) (anonymous (kind parameter) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::stop")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "stop")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::payload")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::payload")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "stop")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "stop")) (anonymous (kind accept-action) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::payload")) (scopes any feature))
      (subtype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "stop")) (anonymous (kind parameter) (ordinal 1)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::machine")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind view) (name "machine")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::machine")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/model.sysml") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 8 20) (end 8 24)) (probe (position 8 20))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 15 29) (end 15 33)) (probe (position 15 29))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::baking::bake"))) (kind featureTyping) (ordinal 0) (authored-target "Heat")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Heat")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 12 35) (end 12 39)) (probe (position 12 35))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle::coolDown"))) (kind featureTyping) (ordinal 0) (authored-target "Cool")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Cool")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 11 29) (end 11 33)) (probe (position 11 29))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle::hold"))) (kind featureTyping) (ordinal 0) (authored-target "Heat")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Heat")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 10 34) (end 10 37)) (probe (position 10 34))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle::warmUp"))) (kind featureTyping) (ordinal 0) (authored-target "Log")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Log")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 17 31) (end 17 35)) (probe (position 17 31))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::start"))) (kind transitionSource) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 17 51) (end 17 57)) (probe (position 17 51))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::start"))) (kind transitionTarget) (ordinal 0) (authored-target "baking")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::baking")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 17 43) (end 17 45)) (probe (position 17 43))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::start"))) (kind transitionTrigger) (ordinal 0) (authored-target "Go")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Go")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 17 31) (end 17 35)) (probe (position 17 31))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "start")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 17 51) (end 17 57)) (probe (position 17 51))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "start")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "baking")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::baking")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 18 30) (end 18 36)) (probe (position 18 30))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::stop"))) (kind transitionSource) (ordinal 0) (authored-target "baking")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::baking")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 18 54) (end 18 58)) (probe (position 18 54))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::stop"))) (kind transitionTarget) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 18 44) (end 18 48)) (probe (position 18 44))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::stop"))) (kind transitionTrigger) (ordinal 0) (authored-target "Halt")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Halt")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 18 30) (end 18 36)) (probe (position 18 30))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "stop")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "baking")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::baking")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 18 54) (end 18 58)) (probe (position 18 54))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "stop")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 20 19) (end 20 38)) (probe (position 20 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::machine"))) (kind featureTyping) (ordinal 0) (authored-target "StateTransitionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 20 48) (end 20 55)) (probe (position 20 48))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Oven")) (named (kind view) (name "machine")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Machine")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Oven::Machine")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 6,
  "modelDigest": "blake3:ea7e8f387eaae408ac808f6f91b31a78b699a65423b6fbb8882c89dce5819adf",
  "documents": [
    {
      "uri": "memory://snapshot/model.sysml",
      "sourceDomain": "workspace"
    }
  ],
  "sources": [
    {
      "document": 0,
      "range": [
        7,
        14,
        7,
        21
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        8,
        8,
        14
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        15,
        8,
        25
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        20,
        8,
        24
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        14,
        9,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        25,
        10,
        31
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        34,
        10,
        37
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        22,
        11,
        26
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        29,
        11,
        33
      ]
    },
    {
      "document": 0,
      "range": [
        12,
        24,
        12,
        32
      ]
    },
    {
      "document": 0,
      "range": [
        12,
        35,
        12,
        39
      ]
    },
    {
      "document": 0,
      "range": [
        14,
        14,
        14,
        20
      ]
    },
    {
      "document": 0,
      "range": [
        15,
        22,
        15,
        26
      ]
    },
    {
      "document": 0,
      "range": [
        15,
        29,
        15,
        33
      ]
    },
    {
      "document": 0,
      "range": [
        17,
        8,
        17,
        58
      ]
    },
    {
      "document": 0,
      "range": [
        17,
        19,
        17,
        24
      ]
    },
    {
      "document": 0,
      "range": [
        17,
        31,
        17,
        35
      ]
    },
    {
      "document": 0,
      "range": [
        17,
        43,
        17,
        45
      ]
    },
    {
      "document": 0,
      "range": [
        17,
        51,
        17,
        57
      ]
    },
    {
      "document": 0,
      "range": [
        18,
        8,
        18,
        59
      ]
    },
    {
      "document": 0,
      "range": [
        18,
        19,
        18,
        23
      ]
    },
    {
      "document": 0,
      "range": [
        18,
        30,
        18,
        36
      ]
    },
    {
      "document": 0,
      "range": [
        18,
        44,
        18,
        48
      ]
    },
    {
      "document": 0,
      "range": [
        18,
        54,
        18,
        58
      ]
    },
    {
      "document": 0,
      "range": [
        20,
        9,
        20,
        16
      ]
    }
  ],
  "references": [
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Oven::Cool"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Oven::Go"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Oven::Halt"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Oven::Heat"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Oven::Log"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Oven::Machine"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Oven::Machine::"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Oven::Machine::baking"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Oven::Machine::baking::bake"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Oven::Machine::idle"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Oven::Machine::idle::coolDown"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Oven::Machine::idle::hold"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Oven::Machine::idle::warmUp"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Oven::Machine::start"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Oven::Machine::start::"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Oven::Machine::stop"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Oven::Machine::stop::"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Oven::machine"
    },
    {
      "kind": "source-anchor",
      "source": 1,
      "ownerQualifiedName": "Oven::Machine",
      "metaclass": "ActionUsage",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 2,
      "ownerQualifiedName": "Oven::Machine",
      "metaclass": "SuccessionAsUsage",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 14,
      "ownerQualifiedName": "Oven::Machine::start",
      "metaclass": "AcceptActionUsage",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 14,
      "ownerQualifiedName": "Oven::Machine::start",
      "metaclass": "ReferenceUsage",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 14,
      "ownerQualifiedName": "Oven::Machine::start",
      "metaclass": "SuccessionAsUsage",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 14,
      "ownerQualifiedName": "Oven::Machine::start::",
      "metaclass": "ReferenceUsage",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 19,
      "ownerQualifiedName": "Oven::Machine::stop",
      "metaclass": "AcceptActionUsage",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 19,
      "ownerQualifiedName": "Oven::Machine::stop",
      "metaclass": "ReferenceUsage",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 19,
      "ownerQualifiedName": "Oven::Machine::stop",
      "metaclass": "SuccessionAsUsage",
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "source": 19,
      "ownerQualifiedName": "Oven::Machine::stop::",
      "metaclass": "ReferenceUsage",
      "sourceDomain": "workspace"
    },
    {
      "kind": "relationship",
      "source": 5,
      "relationshipKind": "containment",
      "ordinal": 0
    },
    {
      "kind": "relationship",
      "source": 5,
      "relationshipKind": "containment",
      "ordinal": 4
    },
    {
      "kind": "relationship",
      "source": 5,
      "relationshipKind": "containment",
      "ordinal": 6
    },
    {
      "kind": "relationship",
      "source": 5,
      "relationshipKind": "containment",
      "ordinal": 8
    },
    {
      "kind": "relationship",
      "source": 5,
      "relationshipKind": "containment",
      "ordinal": 17
    },
    {
      "kind": "relationship",
      "source": 5,
      "relationshipKind": "containment",
      "ordinal": 26
    },
    {
      "kind": "relationship",
      "source": 6,
      "relationshipKind": "initialState",
      "ordinal": 7
    },
    {
      "kind": "relationship",
      "source": 6,
      "relationshipKind": "initialState",
      "ordinal": 29
    },
    {
      "kind": "relationship",
      "source": 7,
      "relationshipKind": "containment",
      "ordinal": 5
    },
    {
      "kind": "relationship",
      "source": 7,
      "relationshipKind": "succession",
      "ordinal": 14
    },
    {
      "kind": "relationship",
      "source": 7,
      "relationshipKind": "transition",
      "ordinal": 9
    },
    {
      "kind": "relationship",
      "source": 8,
      "relationshipKind": "typing",
      "ordinal": 24
    },
    {
      "kind": "relationship",
      "source": 9,
      "relationshipKind": "containment",
      "ordinal": 1
    },
    {
      "kind": "relationship",
      "source": 9,
      "relationshipKind": "containment",
      "ordinal": 2
    },
    {
      "kind": "relationship",
      "source": 9,
      "relationshipKind": "containment",
      "ordinal": 3
    },
    {
      "kind": "relationship",
      "source": 9,
      "relationshipKind": "succession",
      "ordinal": 23
    },
    {
      "kind": "relationship",
      "source": 9,
      "relationshipKind": "transition",
      "ordinal": 18
    },
    {
      "kind": "relationship",
      "source": 10,
      "relationshipKind": "typing",
      "ordinal": 15
    },
    {
      "kind": "relationship",
      "source": 11,
      "relationshipKind": "typing",
      "ordinal": 5
    },
    {
      "kind": "relationship",
      "source": 12,
      "relationshipKind": "typing",
      "ordinal": 10
    },
    {
      "kind": "relationship",
      "source": 13,
      "relationshipKind": "containment",
      "ordinal": 19
    },
    {
      "kind": "relationship",
      "source": 13,
      "relationshipKind": "containment",
      "ordinal": 20
    },
    {
      "kind": "relationship",
      "source": 13,
      "relationshipKind": "containment",
      "ordinal": 21
    },
    {
      "kind": "relationship",
      "source": 13,
      "relationshipKind": "containment",
      "ordinal": 22
    },
    {
      "kind": "relationship",
      "source": 13,
      "relationshipKind": "containment",
      "ordinal": 24
    },
    {
      "kind": "relationship",
      "source": 13,
      "relationshipKind": "transitionSource",
      "ordinal": 56
    },
    {
      "kind": "relationship",
      "source": 13,
      "relationshipKind": "transitionTarget",
      "ordinal": 57
    },
    {
      "kind": "relationship",
      "source": 13,
      "relationshipKind": "transitionTrigger",
      "ordinal": 58
    },
    {
      "kind": "relationship",
      "source": 14,
      "relationshipKind": "containment",
      "ordinal": 25
    },
    {
      "kind": "relationship",
      "source": 14,
      "relationshipKind": "succession",
      "ordinal": 68
    },
    {
      "kind": "relationship",
      "source": 14,
      "relationshipKind": "succession",
      "ordinal": 69
    },
    {
      "kind": "relationship",
      "source": 15,
      "relationshipKind": "containment",
      "ordinal": 10
    },
    {
      "kind": "relationship",
      "source": 15,
      "relationshipKind": "containment",
      "ordinal": 11
    },
    {
      "kind": "relationship",
      "source": 15,
      "relationshipKind": "containment",
      "ordinal": 12
    },
    {
      "kind": "relationship",
      "source": 15,
      "relationshipKind": "containment",
      "ordinal": 13
    },
    {
      "kind": "relationship",
      "source": 15,
      "relationshipKind": "containment",
      "ordinal": 15
    },
    {
      "kind": "relationship",
      "source": 15,
      "relationshipKind": "transitionSource",
      "ordinal": 35
    },
    {
      "kind": "relationship",
      "source": 15,
      "relationshipKind": "transitionTarget",
      "ordinal": 36
    },
    {
      "kind": "relationship",
      "source": 15,
      "relationshipKind": "transitionTrigger",
      "ordinal": 37
    },
    {
      "kind": "relationship",
      "source": 16,
      "relationshipKind": "containment",
      "ordinal": 16
    },
    {
      "kind": "relationship",
      "source": 16,
      "relationshipKind": "succession",
      "ordinal": 47
    },
    {
      "kind": "relationship",
      "source": 16,
      "relationshipKind": "succession",
      "ordinal": 48
    }
  ],
  "selectedView": {
    "reference": 17,
    "kind": "state-transition-view",
    "name": "machine",
    "source": 24
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "kind": "state-transition-view",
    "exposedRoots": [
      0
    ],
    "nodes": [
      {
        "reference": 5,
        "metaclass": "StateDefinition",
        "notationRole": "definition",
        "name": "Machine",
        "typing": {
          "status": "absent"
        },
        "owner": null,
        "source": 0,
        "compartments": [
          {
            "kind": "actions",
            "provenance": "direct",
            "members": [
              16
            ]
          },
          {
            "kind": "states",
            "provenance": "direct",
            "members": [
              17,
              21
            ]
          }
        ],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 15,
        "metaclass": "TransitionUsage",
        "notationRole": "usage",
        "name": "stop",
        "typing": {
          "status": "absent"
        },
        "owner": 0,
        "source": 20,
        "compartments": [
          {
            "kind": "attributes",
            "provenance": "direct",
            "members": [
              7,
              7,
              7
            ]
          },
          {
            "kind": "actions",
            "provenance": "direct",
            "members": [
              3
            ]
          }
        ],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 26,
        "metaclass": "SuccessionAsUsage",
        "notationRole": "unsupported",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 1,
        "source": 19,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 24,
        "metaclass": "AcceptActionUsage",
        "notationRole": "usage",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 1,
        "source": 19,
        "compartments": [
          {
            "kind": "attributes",
            "provenance": "direct",
            "members": [
              4
            ]
          }
        ],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 27,
        "metaclass": "ReferenceUsage",
        "notationRole": "reference-usage",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 3,
        "source": 19,
        "compartments": [],
        "direction": "inout",
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 25,
        "metaclass": "ReferenceUsage",
        "notationRole": "reference-usage",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 1,
        "source": 19,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 25,
        "metaclass": "ReferenceUsage",
        "notationRole": "reference-usage",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 1,
        "source": 19,
        "compartments": [],
        "direction": "in",
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 25,
        "metaclass": "ReferenceUsage",
        "notationRole": "reference-usage",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 1,
        "source": 19,
        "compartments": [],
        "direction": "in",
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 13,
        "metaclass": "TransitionUsage",
        "notationRole": "usage",
        "name": "start",
        "typing": {
          "status": "absent"
        },
        "owner": 0,
        "source": 15,
        "compartments": [
          {
            "kind": "attributes",
            "provenance": "direct",
            "members": [
              14,
              14,
              14
            ]
          },
          {
            "kind": "actions",
            "provenance": "direct",
            "members": [
              10
            ]
          }
        ],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 22,
        "metaclass": "SuccessionAsUsage",
        "notationRole": "unsupported",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 8,
        "source": 14,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 20,
        "metaclass": "AcceptActionUsage",
        "notationRole": "usage",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 8,
        "source": 14,
        "compartments": [
          {
            "kind": "attributes",
            "provenance": "direct",
            "members": [
              11
            ]
          }
        ],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 23,
        "metaclass": "ReferenceUsage",
        "notationRole": "reference-usage",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 10,
        "source": 14,
        "compartments": [],
        "direction": "inout",
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 21,
        "metaclass": "ReferenceUsage",
        "notationRole": "reference-usage",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 8,
        "source": 14,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 21,
        "metaclass": "ReferenceUsage",
        "notationRole": "reference-usage",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 8,
        "source": 14,
        "compartments": [],
        "direction": "in",
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 21,
        "metaclass": "ReferenceUsage",
        "notationRole": "reference-usage",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 8,
        "source": 14,
        "compartments": [],
        "direction": "in",
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 19,
        "metaclass": "SuccessionAsUsage",
        "notationRole": "unsupported",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 0,
        "source": 2,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 18,
        "metaclass": "ActionUsage",
        "notationRole": "usage",
        "name": null,
        "typing": {
          "status": "absent"
        },
        "owner": 0,
        "source": 1,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": "entry",
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 9,
        "metaclass": "StateUsage",
        "notationRole": "usage",
        "name": "idle",
        "typing": {
          "status": "absent"
        },
        "owner": 0,
        "source": 4,
        "compartments": [
          {
            "kind": "actions",
            "provenance": "direct",
            "members": [
              18,
              19,
              20
            ]
          }
        ],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 11,
        "metaclass": "ActionUsage",
        "notationRole": "usage",
        "name": "hold",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 3,
              "label": "Heat"
            }
          ]
        },
        "owner": 17,
        "source": 7,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": "do",
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 10,
        "metaclass": "ActionUsage",
        "notationRole": "usage",
        "name": "coolDown",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 0,
              "label": "Cool"
            }
          ]
        },
        "owner": 17,
        "source": 9,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": "exit",
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 12,
        "metaclass": "ActionUsage",
        "notationRole": "usage",
        "name": "warmUp",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 4,
              "label": "Log"
            }
          ]
        },
        "owner": 17,
        "source": 5,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": "entry",
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 7,
        "metaclass": "StateUsage",
        "notationRole": "usage",
        "name": "baking",
        "typing": {
          "status": "absent"
        },
        "owner": 0,
        "source": 11,
        "compartments": [
          {
            "kind": "actions",
            "provenance": "direct",
            "members": [
              22
            ]
          }
        ],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 8,
        "metaclass": "ActionUsage",
        "notationRole": "usage",
        "name": "bake",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 3,
              "label": "Heat"
            }
          ]
        },
        "owner": 21,
        "source": 12,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": "do",
        "successionGuard": null,
        "featureDirection": null
      }
    ],
    "relationships": [
      {
        "reference": 46,
        "source": 18,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 3
        },
        "provenance": "authored",
        "navigation": 8
      },
      {
        "reference": 47,
        "source": 20,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 4
        },
        "provenance": "authored",
        "navigation": 6
      },
      {
        "reference": 45,
        "source": 19,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 0
        },
        "provenance": "authored",
        "navigation": 10
      },
      {
        "reference": 39,
        "source": 22,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 3
        },
        "provenance": "authored",
        "navigation": 13
      },
      {
        "reference": 35,
        "source": 15,
        "kind": "initialState",
        "target": {
          "status": "resolved",
          "node": 17
        },
        "provenance": "authored",
        "navigation": 3
      },
      {
        "reference": 64,
        "source": 1,
        "kind": "transitionSource",
        "target": {
          "status": "resolved",
          "node": 21
        },
        "provenance": "authored",
        "navigation": 21
      },
      {
        "reference": 65,
        "source": 1,
        "kind": "transitionTarget",
        "target": {
          "status": "resolved",
          "node": 17
        },
        "provenance": "authored",
        "navigation": 23
      },
      {
        "reference": 66,
        "source": 1,
        "kind": "transitionTrigger",
        "target": {
          "status": "resolved",
          "reference": 2
        },
        "provenance": "authored",
        "navigation": 22
      },
      {
        "reference": 68,
        "source": 2,
        "kind": "succession",
        "target": {
          "status": "resolved",
          "node": 21
        },
        "provenance": "authored",
        "navigation": 21
      },
      {
        "reference": 69,
        "source": 2,
        "kind": "succession",
        "target": {
          "status": "resolved",
          "node": 17
        },
        "provenance": "authored",
        "navigation": 23
      },
      {
        "reference": 53,
        "source": 8,
        "kind": "transitionSource",
        "target": {
          "status": "resolved",
          "node": 17
        },
        "provenance": "authored",
        "navigation": 16
      },
      {
        "reference": 54,
        "source": 8,
        "kind": "transitionTarget",
        "target": {
          "status": "resolved",
          "node": 21
        },
        "provenance": "authored",
        "navigation": 18
      },
      {
        "reference": 55,
        "source": 8,
        "kind": "transitionTrigger",
        "target": {
          "status": "resolved",
          "reference": 1
        },
        "provenance": "authored",
        "navigation": 17
      },
      {
        "reference": 57,
        "source": 9,
        "kind": "succession",
        "target": {
          "status": "resolved",
          "node": 17
        },
        "provenance": "authored",
        "navigation": 16
      },
      {
        "reference": 58,
        "source": 9,
        "kind": "succession",
        "target": {
          "status": "resolved",
          "node": 21
        },
        "provenance": "authored",
        "navigation": 18
      }
    ],
    "edges": [
      {
        "reference": 28,
        "source": 0,
        "target": 17,
        "origin": 17,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 4
      },
      {
        "reference": 40,
        "source": 17,
        "target": 18,
        "origin": 18,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 7
      },
      {
        "reference": 41,
        "source": 17,
        "target": 20,
        "origin": 20,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 5
      },
      {
        "reference": 42,
        "source": 17,
        "target": 19,
        "origin": 19,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 9
      },
      {
        "reference": 29,
        "source": 0,
        "target": 21,
        "origin": 21,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 11
      },
      {
        "reference": 36,
        "source": 21,
        "target": 22,
        "origin": 22,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 12
      },
      {
        "reference": 30,
        "source": 0,
        "target": 15,
        "origin": 15,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 2
      },
      {
        "reference": 34,
        "source": 15,
        "target": 17,
        "origin": 15,
        "kind": "initial-state",
        "provenance": "authored",
        "navigation": 3
      },
      {
        "reference": 31,
        "source": 0,
        "target": 1,
        "origin": 1,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 20
      },
      {
        "reference": 38,
        "source": 21,
        "target": 17,
        "origin": 1,
        "kind": "transition",
        "provenance": "implied",
        "navigation": 21
      },
      {
        "reference": 59,
        "source": 1,
        "target": 7,
        "origin": 7,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 19
      },
      {
        "reference": 60,
        "source": 1,
        "target": 7,
        "origin": 7,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 19
      },
      {
        "reference": 61,
        "source": 1,
        "target": 7,
        "origin": 7,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 19
      },
      {
        "reference": 62,
        "source": 1,
        "target": 2,
        "origin": 2,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 19
      },
      {
        "reference": 37,
        "source": 21,
        "target": 17,
        "origin": 2,
        "kind": "succession",
        "provenance": "implied",
        "navigation": 21
      },
      {
        "reference": 63,
        "source": 1,
        "target": 3,
        "origin": 3,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 19
      },
      {
        "reference": 67,
        "source": 3,
        "target": 4,
        "origin": 4,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 19
      },
      {
        "reference": 32,
        "source": 0,
        "target": 8,
        "origin": 8,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 15
      },
      {
        "reference": 44,
        "source": 17,
        "target": 21,
        "origin": 8,
        "kind": "transition",
        "provenance": "implied",
        "navigation": 16
      },
      {
        "reference": 48,
        "source": 8,
        "target": 14,
        "origin": 14,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 14
      },
      {
        "reference": 49,
        "source": 8,
        "target": 14,
        "origin": 14,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 14
      },
      {
        "reference": 50,
        "source": 8,
        "target": 14,
        "origin": 14,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 14
      },
      {
        "reference": 51,
        "source": 8,
        "target": 9,
        "origin": 9,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 14
      },
      {
        "reference": 43,
        "source": 17,
        "target": 21,
        "origin": 9,
        "kind": "succession",
        "provenance": "implied",
        "navigation": 16
      },
      {
        "reference": 52,
        "source": 8,
        "target": 10,
        "origin": 10,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 14
      },
      {
        "reference": 56,
        "source": 10,
        "target": 11,
        "origin": 11,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 14
      },
      {
        "reference": 33,
        "source": 0,
        "target": 16,
        "origin": 16,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 1
      }
    ],
    "metadata": {
      "states": [
        0,
        17,
        21
      ],
      "initialNodes": [
        15
      ],
      "finalNodes": []
    },
    "scene": {
      "kind": "state-transition",
      "frame": {
        "id": "state-machine",
        "label": "Machine",
        "navigation": 0
      },
      "vertices": [
        {
          "id": "state-0",
          "label": "",
          "kind": "initial",
          "navigation": 2
        },
        {
          "id": "state-1",
          "label": "idle",
          "kind": "state",
          "navigation": 4
        },
        {
          "id": "state-2",
          "label": "baking",
          "kind": "state",
          "navigation": 11
        }
      ],
      "transitions": [
        {
          "id": "transition-0",
          "label": null,
          "source": 0,
          "target": 1,
          "trigger": {
            "status": "absent"
          },
          "guard": {
            "status": "absent"
          },
          "effect": {
            "status": "absent"
          },
          "provenance": "authored",
          "navigation": 3
        },
        {
          "id": "transition-1",
          "label": "stop",
          "source": 2,
          "target": 1,
          "trigger": {
            "status": "accept",
            "label": "Halt",
            "target": {
              "id": "element/v129:memory://snapshot/model.sysml7:packagen4:Oven1:013:attribute-defn4:Halt1:0",
              "label": "Halt"
            },
            "navigation": 22
          },
          "guard": {
            "status": "absent"
          },
          "effect": {
            "status": "absent"
          },
          "provenance": "implied",
          "navigation": 21
        },
        {
          "id": "transition-2",
          "label": "start",
          "source": 1,
          "target": 2,
          "trigger": {
            "status": "accept",
            "label": "Go",
            "target": {
              "id": "element/v129:memory://snapshot/model.sysml7:packagen4:Oven1:013:attribute-defn2:Go1:0",
              "label": "Go"
            },
            "navigation": 17
          },
          "guard": {
            "status": "absent"
          },
          "effect": {
            "status": "absent"
          },
          "provenance": "implied",
          "navigation": 16
        }
      ]
    }
  }
}

~~~
