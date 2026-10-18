# META
~~~ini
description=A state that owns states is published with them, so that it can show its own state transition view inside it; a transition whose source is written as a feature chain into it is composed from that end
type=generate
libraries=standard
plugin=native:diagram
viewKind=state-transition-view
viewDocument=model.sysml
viewQualifiedName=Lamp::machine
~~~
# SOURCE
## model.sysml
~~~sysml
package Lamp {
    private import StandardViewDefinitions::*;
    attribute def Switch;
    attribute def Fault;
    state def Machine {
        entry; then off;
        state off;
        state on {
            entry; then warming;
            state warming;
            state ready;
            transition first warming then ready;
        }
        transition turnOn first off accept Switch then on;
        transition turnOff first on accept Switch then off;
        state fault;
        transition fail first on.ready accept Fault then fault;
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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:bd3d024429e4ee1b9867fda3ca826a1938d1156e65594dad56b4478edfb577e7") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Fault"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entry-action-binding) (membership (kind feature) (visibility default) (role entry)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "off")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::fail"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionTarget (reference "fault")) (transitionTrigger (reference "Fault")) (memberAccessOperand (reference "on::ready")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "fail")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (succession (reference "on::ready")) (succession (reference "fault")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "fail")) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "fail")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "fail")) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "fail")) (anonymous (kind parameter) (ordinal 1))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "fail")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction inout)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::fault"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::off"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entry-action-binding) (membership (kind feature) (visibility default) (role entry)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind initial-state) (ordinal 0))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "warming")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind transition) (ordinal 0))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "warming")) (transitionTarget (reference "ready")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (succession (reference "warming")) (succession (reference "ready")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on::ready"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on::warming"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOff"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "on")) (transitionTarget (reference "off")) (transitionTrigger (reference "Switch")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOff")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (succession (reference "on")) (succession (reference "off")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOff")) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOff")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOff")) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOff")) (anonymous (kind parameter) (ordinal 1))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOff")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction inout)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOn"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "off")) (transitionTarget (reference "on")) (transitionTrigger (reference "Switch")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOn")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (succession (reference "off")) (succession (reference "on")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOn")) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOn")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOn")) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOn")) (anonymous (kind parameter) (ordinal 1))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOn")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction inout)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Switch"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::machine"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StateTransitionView")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind view) (name "machine")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Machine")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0))
      (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::off")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::fail"))) (kind transitionTarget) (ordinal 0))
      (authored-target "fault")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::fault")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::fail"))) (kind transitionTrigger) (ordinal 0))
      (authored-target "Fault")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Fault")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::fail"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "on::ready")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on::ready")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "fail")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "on::ready")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on::ready")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "fail")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "fault")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::fault")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0))
      (authored-target "warming")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on::warming")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0))
      (authored-target "warming")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on::warming")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0))
      (authored-target "ready")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on::ready")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "warming")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on::warming")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "ready")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on::ready")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOff"))) (kind transitionSource) (ordinal 0))
      (authored-target "on")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOff"))) (kind transitionTarget) (ordinal 0))
      (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::off")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOff"))) (kind transitionTrigger) (ordinal 0))
      (authored-target "Switch")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Switch")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOff")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "on")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOff")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::off")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOn"))) (kind transitionSource) (ordinal 0))
      (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::off")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOn"))) (kind transitionTarget) (ordinal 0))
      (authored-target "on")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOn"))) (kind transitionTrigger) (ordinal 0))
      (authored-target "Switch")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Switch")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOn")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::off")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOn")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "on")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::machine"))) (kind featureTyping) (ordinal 0))
      (authored-target "StateTransitionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind view) (name "machine")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Machine")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine")))))
  )
  (relationships
    (relationship (kind initialState) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::off"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::fail"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::fault"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::fail"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::fail"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Fault"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::fail"))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::fail"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on::ready"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::fail"))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "fail")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on::ready"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "fail")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "fail")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::fault"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "fail")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind initialState) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on::warming"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on::warming"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on::ready"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on::warming"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on::ready"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOff"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOff"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOff"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::off"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOff"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOff"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Switch"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOff"))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOff")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOff")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOff")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::off"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOff")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOn"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::off"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOn"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOn"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOn"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOn"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Switch"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOn"))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOn")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::off"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOn")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOn")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOn")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::machine"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::machine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind view) (name "machine")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind view) (name "machine")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Fault"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::entryAction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::fail"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::fail"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::fail"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "fail")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::accepter"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "fail")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::transitionLinkSource"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "fail")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::fail"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "fail")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::fail"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "fail")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::fail"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "fail")) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::fail"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "fail")) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "fail")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "fail")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "fail")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "fail")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "fail")) (anonymous (kind accept-action) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::fault"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::fault"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::fault"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::exclusiveStates"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::fault"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::off"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::off"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::off"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::exclusiveStates"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::off"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::exclusiveStates"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::entryAction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::stateTransitions"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::transitionLinkSource"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind transition) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind transition) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::transitionLinkSource"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on::ready"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on::ready"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on::ready"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::exclusiveStates"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on::ready"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on::warming"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on::warming"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on::warming"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::exclusiveStates"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on::warming"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOff"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOff"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOff"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOff"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::stateTransitions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOff")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::accepter"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOff")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::transitionLinkSource"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOff")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOff"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOff")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOff"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOff")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOff"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOff")) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOff"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOff")) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOff")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOff")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOff")) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::payload"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOff")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::transitionLinkSource"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOff")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOff")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOff")) (anonymous (kind accept-action) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOn"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOn"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOn"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOn"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::stateTransitions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOn")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::accepter"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOn")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::transitionLinkSource"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOn")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOn"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOn")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOn"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOn")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOn"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOn")) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOn"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOn")) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOn")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOn")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOn")) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::payload"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOn")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::transitionLinkSource"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOn")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOn")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOn")) (anonymous (kind accept-action) (ordinal 0))))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Switch"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::machine"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind view) (name "machine")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::machine"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Fault")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::DecisionPerformance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::fail")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "fail")) (anonymous (kind succession) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "fail")) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::fail")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::transitionLinkSource"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::transitionLinkSource"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::transitionLinkSource")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::transitionLinkSource")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "fail")) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::fail")))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "fail")) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::fail")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "fail")) (anonymous (kind parameter) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::fail")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "fail")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::payload")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "fail")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "fail")) (anonymous (kind accept-action) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::payload")) (scopes any feature))
      (subtype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "fail")) (anonymous (kind parameter) (ordinal 1)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::fault")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::off")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind entry-action-binding) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind initial-state) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind transition) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind transition) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind transition) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on::ready")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on::warming")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOff")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOff")) (anonymous (kind succession) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOff")) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOff")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOff")) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOff")))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOff")) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOff")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOff")) (anonymous (kind parameter) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOff")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOff")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::payload")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::payload")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOff")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOff")) (anonymous (kind accept-action) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::payload")) (scopes any feature))
      (subtype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOff")) (anonymous (kind parameter) (ordinal 1)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOn")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOn")) (anonymous (kind succession) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOn")) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOn")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOn")) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOn")))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOn")) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOn")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOn")) (anonymous (kind parameter) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOn")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOn")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::payload")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::payload")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOn")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOn")) (anonymous (kind accept-action) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::payload")) (scopes any feature))
      (subtype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOn")) (anonymous (kind parameter) (ordinal 1)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Switch")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::machine")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind view) (name "machine")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::machine")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/model.sysml") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 5 20) (end 5 23)) (probe (position 5 20))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0) (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::off")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 16 57) (end 16 62)) (probe (position 16 57))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::fail"))) (kind transitionTarget) (ordinal 0) (authored-target "fault")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::fault")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 16 46) (end 16 51)) (probe (position 16 46))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::fail"))) (kind transitionTrigger) (ordinal 0) (authored-target "Fault")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Fault")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 16 30) (end 16 38)) (probe (position 16 30))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::fail"))) (kind memberAccessOperand) (ordinal 0) (authored-target "on::ready")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on::ready")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 16 30) (end 16 38)) (probe (position 16 30))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "fail")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "on::ready")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on::ready")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 16 57) (end 16 62)) (probe (position 16 57))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "fail")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "fault")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::fault")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 8 24) (end 8 31)) (probe (position 8 24))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0) (authored-target "warming")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on::warming")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 11 29) (end 11 36)) (probe (position 11 29))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0) (authored-target "warming")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on::warming")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 11 42) (end 11 47)) (probe (position 11 42))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0) (authored-target "ready")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on::ready")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 11 29) (end 11 36)) (probe (position 11 29))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "warming")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on::warming")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 11 42) (end 11 47)) (probe (position 11 42))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind state) (name "on")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "ready")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on::ready")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 14 33) (end 14 35)) (probe (position 14 33))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOff"))) (kind transitionSource) (ordinal 0) (authored-target "on")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 14 55) (end 14 58)) (probe (position 14 55))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOff"))) (kind transitionTarget) (ordinal 0) (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::off")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 14 43) (end 14 49)) (probe (position 14 43))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOff"))) (kind transitionTrigger) (ordinal 0) (authored-target "Switch")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Switch")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 14 33) (end 14 35)) (probe (position 14 33))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOff")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "on")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 14 55) (end 14 58)) (probe (position 14 55))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOff")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::off")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 13 32) (end 13 35)) (probe (position 13 32))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOn"))) (kind transitionSource) (ordinal 0) (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::off")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 13 55) (end 13 57)) (probe (position 13 55))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOn"))) (kind transitionTarget) (ordinal 0) (authored-target "on")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 13 43) (end 13 49)) (probe (position 13 43))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::turnOn"))) (kind transitionTrigger) (ordinal 0) (authored-target "Switch")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Switch")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 13 32) (end 13 35)) (probe (position 13 32))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOn")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "off")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::off")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 13 55) (end 13 57)) (probe (position 13 55))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "turnOn")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "on")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine::on")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 18 19) (end 18 38)) (probe (position 18 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::machine"))) (kind featureTyping) (ordinal 0) (authored-target "StateTransitionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 18 48) (end 18 55)) (probe (position 18 48))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Lamp")) (named (kind view) (name "machine")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Machine")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Lamp::Machine")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 5,
  "modelDigest": "blake3:ecd33e59644a87cf56e53df707897b54243fa8118a241e5e28ddb42e98fcf99e",
  "documents": [
    {
      "uri": "memory://snapshot/model.sysml",
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
        4,
        14,
        4,
        21
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        8,
        5,
        14
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        15,
        5,
        24
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        20,
        5,
        23
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        14,
        6,
        17
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        14,
        7,
        16
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        12,
        8,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        19,
        8,
        32
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        24,
        8,
        31
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        18,
        9,
        25
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        18,
        10,
        23
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        12,
        11,
        48
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        29,
        11,
        36
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        42,
        11,
        47
      ]
    },
    {
      "document": 0,
      "range": [
        13,
        8,
        13,
        58
      ]
    },
    {
      "document": 0,
      "range": [
        13,
        19,
        13,
        25
      ]
    },
    {
      "document": 0,
      "range": [
        13,
        32,
        13,
        35
      ]
    },
    {
      "document": 0,
      "range": [
        13,
        43,
        13,
        49
      ]
    },
    {
      "document": 0,
      "range": [
        13,
        55,
        13,
        57
      ]
    },
    {
      "document": 0,
      "range": [
        14,
        8,
        14,
        59
      ]
    },
    {
      "document": 0,
      "range": [
        14,
        19,
        14,
        26
      ]
    },
    {
      "document": 0,
      "range": [
        14,
        33,
        14,
        35
      ]
    },
    {
      "document": 0,
      "range": [
        14,
        43,
        14,
        49
      ]
    },
    {
      "document": 0,
      "range": [
        14,
        55,
        14,
        58
      ]
    },
    {
      "document": 0,
      "range": [
        15,
        14,
        15,
        19
      ]
    },
    {
      "document": 0,
      "range": [
        16,
        8,
        16,
        63
      ]
    },
    {
      "document": 0,
      "range": [
        16,
        19,
        16,
        23
      ]
    },
    {
      "document": 0,
      "range": [
        16,
        30,
        16,
        38
      ]
    },
    {
      "document": 0,
      "range": [
        16,
        46,
        16,
        51
      ]
    },
    {
      "document": 0,
      "range": [
        16,
        57,
        16,
        62
      ]
    },
    {
      "document": 0,
      "range": [
        18,
        9,
        18,
        16
      ]
    },
    {
      "document": 1,
      "range": [
        212,
        2,
        212,
        20
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Lamp::Fault"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Lamp::Machine"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Lamp::Machine::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Lamp::Machine::fail"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Lamp::Machine::fail::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Lamp::Machine::fail::::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Lamp::Machine::fault"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Lamp::Machine::off"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Lamp::Machine::on"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Lamp::Machine::on::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Lamp::Machine::on::::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Lamp::Machine::on::ready"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Lamp::Machine::on::warming"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Lamp::Machine::turnOff"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Lamp::Machine::turnOff::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Lamp::Machine::turnOff::::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Lamp::Machine::turnOn"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Lamp::Machine::turnOn::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Lamp::Machine::turnOn::::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Lamp::Switch"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Lamp::machine"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Actions::TransitionAction::accepter"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Actions::TransitionAction::transitionLinkSource"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Actions::actions"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Actions::transitionActions"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "Occurrences::Occurrence::suboccurrences"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "Occurrences::happensBeforeLinks"
    },
    {
      "document": 3,
      "kind": "qualified-name",
      "qualifiedName": "States::StateAction"
    },
    {
      "document": 3,
      "kind": "qualified-name",
      "qualifiedName": "States::StateAction::entryAction"
    },
    {
      "document": 3,
      "kind": "qualified-name",
      "qualifiedName": "States::StateAction::exclusiveStates"
    },
    {
      "document": 3,
      "kind": "qualified-name",
      "qualifiedName": "States::StateAction::stateTransitions"
    },
    {
      "document": 3,
      "kind": "qualified-name",
      "qualifiedName": "States::StateTransitionAction::payload"
    },
    {
      "document": 3,
      "kind": "qualified-name",
      "qualifiedName": "States::StateTransitionAction::transitionLinkSource"
    },
    {
      "document": 3,
      "kind": "qualified-name",
      "qualifiedName": "States::stateActions"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ActionUsage",
      "ownerQualifiedName": "Lamp::Machine",
      "source": 1,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Lamp::Machine",
      "source": 2,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "AcceptActionUsage",
      "ownerQualifiedName": "Lamp::Machine::fail",
      "source": 25,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Lamp::Machine::fail",
      "source": 25,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Lamp::Machine::fail",
      "source": 25,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Lamp::Machine::fail::",
      "source": 25,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ActionUsage",
      "ownerQualifiedName": "Lamp::Machine::on",
      "source": 6,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Lamp::Machine::on",
      "source": 7,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "TransitionUsage",
      "ownerQualifiedName": "Lamp::Machine::on",
      "source": 11,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Lamp::Machine::on::",
      "source": 11,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Lamp::Machine::on::",
      "source": 11,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "AcceptActionUsage",
      "ownerQualifiedName": "Lamp::Machine::turnOff",
      "source": 19,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Lamp::Machine::turnOff",
      "source": 19,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Lamp::Machine::turnOff",
      "source": 19,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Lamp::Machine::turnOff::",
      "source": 19,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "AcceptActionUsage",
      "ownerQualifiedName": "Lamp::Machine::turnOn",
      "source": 14,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Lamp::Machine::turnOn",
      "source": 14,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Lamp::Machine::turnOn",
      "source": 14,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Lamp::Machine::turnOn::",
      "source": 14,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Actions::AcceptMessageAction",
      "source": 31,
      "sourceDomain": "standard-library"
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 12,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 14,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 16,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 25,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 34,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 43,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "specializes",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 15,
      "relationshipKind": "initialState",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 41,
      "relationshipKind": "initialState",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 104,
      "relationshipKind": "redefinition",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 42,
      "relationshipKind": "subsetting",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 105,
      "relationshipKind": "subsetting",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 106,
      "relationshipKind": "subsetting",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 43,
      "relationshipKind": "typeFeaturing",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 107,
      "relationshipKind": "typeFeaturing",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 18,
      "relationshipKind": "containment",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 19,
      "relationshipKind": "containment",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 20,
      "relationshipKind": "containment",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 21,
      "relationshipKind": "containment",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 23,
      "relationshipKind": "containment",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 44,
      "relationshipKind": "memberAccessOperand",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 45,
      "relationshipKind": "subsetting",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 46,
      "relationshipKind": "subsetting",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 47,
      "relationshipKind": "transitionTarget",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 48,
      "relationshipKind": "transitionTrigger",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 49,
      "relationshipKind": "typeFeaturing",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 24,
      "relationshipKind": "containment",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 51,
      "relationshipKind": "redefinition",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 53,
      "relationshipKind": "subsetting",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 55,
      "relationshipKind": "subsetting",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 58,
      "relationshipKind": "subsetting",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 56,
      "relationshipKind": "succession",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 57,
      "relationshipKind": "succession",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 50,
      "relationshipKind": "typeFeaturing",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 52,
      "relationshipKind": "typeFeaturing",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 54,
      "relationshipKind": "typeFeaturing",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 59,
      "relationshipKind": "typeFeaturing",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 60,
      "relationshipKind": "redefinition",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 61,
      "relationshipKind": "typeFeaturing",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 37,
      "relationshipKind": "subsetting",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 38,
      "relationshipKind": "subsetting",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 39,
      "relationshipKind": "subsetting",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 40,
      "relationshipKind": "typeFeaturing",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 33,
      "relationshipKind": "subsetting",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 34,
      "relationshipKind": "subsetting",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 35,
      "relationshipKind": "subsetting",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 31,
      "relationshipKind": "succession",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 26,
      "relationshipKind": "transition",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 36,
      "relationshipKind": "typeFeaturing",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "containment",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "containment",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "containment",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "containment",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 11,
      "relationshipKind": "containment",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "subsetting",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "subsetting",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "subsetting",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 40,
      "relationshipKind": "succession",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 35,
      "relationshipKind": "transition",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "typeFeaturing",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "containment",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "containment",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "containment",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "initialState",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 22,
      "relationshipKind": "initialState",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 29,
      "relationshipKind": "redefinition",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "subsetting",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "subsetting",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "subsetting",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 23,
      "relationshipKind": "subsetting",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 30,
      "relationshipKind": "subsetting",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 31,
      "relationshipKind": "subsetting",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "transitionSource",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "transitionTarget",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "typeFeaturing",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 24,
      "relationshipKind": "typeFeaturing",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 32,
      "relationshipKind": "typeFeaturing",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 12,
      "relationshipKind": "redefinition",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "redefinition",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 15,
      "relationshipKind": "subsetting",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 16,
      "relationshipKind": "succession",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 17,
      "relationshipKind": "succession",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 11,
      "relationshipKind": "typeFeaturing",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 14,
      "relationshipKind": "typeFeaturing",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 18,
      "relationshipKind": "subsetting",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 19,
      "relationshipKind": "subsetting",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 20,
      "relationshipKind": "subsetting",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 22,
      "relationshipKind": "succession",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 17,
      "relationshipKind": "transition",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 21,
      "relationshipKind": "typeFeaturing",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 25,
      "relationshipKind": "subsetting",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 26,
      "relationshipKind": "subsetting",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 27,
      "relationshipKind": "subsetting",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "succession",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "transition",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 28,
      "relationshipKind": "typeFeaturing",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 36,
      "relationshipKind": "containment",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 37,
      "relationshipKind": "containment",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 38,
      "relationshipKind": "containment",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 39,
      "relationshipKind": "containment",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 41,
      "relationshipKind": "containment",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 83,
      "relationshipKind": "subsetting",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 84,
      "relationshipKind": "subsetting",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 85,
      "relationshipKind": "subsetting",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 86,
      "relationshipKind": "transitionSource",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 87,
      "relationshipKind": "transitionTarget",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 88,
      "relationshipKind": "transitionTrigger",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 89,
      "relationshipKind": "typeFeaturing",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 42,
      "relationshipKind": "containment",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 91,
      "relationshipKind": "redefinition",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 92,
      "relationshipKind": "redefinition",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 94,
      "relationshipKind": "redefinition",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 95,
      "relationshipKind": "subsetting",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 97,
      "relationshipKind": "subsetting",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 100,
      "relationshipKind": "subsetting",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 98,
      "relationshipKind": "succession",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 99,
      "relationshipKind": "succession",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 90,
      "relationshipKind": "typeFeaturing",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 93,
      "relationshipKind": "typeFeaturing",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 96,
      "relationshipKind": "typeFeaturing",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 101,
      "relationshipKind": "typeFeaturing",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 102,
      "relationshipKind": "redefinition",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 103,
      "relationshipKind": "typeFeaturing",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 27,
      "relationshipKind": "containment",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 28,
      "relationshipKind": "containment",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 29,
      "relationshipKind": "containment",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 30,
      "relationshipKind": "containment",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 32,
      "relationshipKind": "containment",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 62,
      "relationshipKind": "subsetting",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 63,
      "relationshipKind": "subsetting",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 64,
      "relationshipKind": "subsetting",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 65,
      "relationshipKind": "transitionSource",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 66,
      "relationshipKind": "transitionTarget",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 67,
      "relationshipKind": "transitionTrigger",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 68,
      "relationshipKind": "typeFeaturing",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 33,
      "relationshipKind": "containment",
      "source": 17
    },
    {
      "kind": "relationship",
      "ordinal": 70,
      "relationshipKind": "redefinition",
      "source": 17
    },
    {
      "kind": "relationship",
      "ordinal": 71,
      "relationshipKind": "redefinition",
      "source": 17
    },
    {
      "kind": "relationship",
      "ordinal": 73,
      "relationshipKind": "redefinition",
      "source": 17
    },
    {
      "kind": "relationship",
      "ordinal": 74,
      "relationshipKind": "subsetting",
      "source": 17
    },
    {
      "kind": "relationship",
      "ordinal": 76,
      "relationshipKind": "subsetting",
      "source": 17
    },
    {
      "kind": "relationship",
      "ordinal": 79,
      "relationshipKind": "subsetting",
      "source": 17
    },
    {
      "kind": "relationship",
      "ordinal": 77,
      "relationshipKind": "succession",
      "source": 17
    },
    {
      "kind": "relationship",
      "ordinal": 78,
      "relationshipKind": "succession",
      "source": 17
    },
    {
      "kind": "relationship",
      "ordinal": 69,
      "relationshipKind": "typeFeaturing",
      "source": 17
    },
    {
      "kind": "relationship",
      "ordinal": 72,
      "relationshipKind": "typeFeaturing",
      "source": 17
    },
    {
      "kind": "relationship",
      "ordinal": 75,
      "relationshipKind": "typeFeaturing",
      "source": 17
    },
    {
      "kind": "relationship",
      "ordinal": 80,
      "relationshipKind": "typeFeaturing",
      "source": 17
    },
    {
      "kind": "relationship",
      "ordinal": 81,
      "relationshipKind": "redefinition",
      "source": 18
    },
    {
      "kind": "relationship",
      "ordinal": 82,
      "relationshipKind": "typeFeaturing",
      "source": 18
    }
  ],
  "selectedView": {
    "reference": 20,
    "kind": "state-transition-view",
    "name": "machine",
    "source": 30
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "edges": [
      {
        "kind": "containment",
        "navigation": 5,
        "origin": 24,
        "provenance": "authored",
        "reference": 54,
        "source": 0,
        "target": 24
      },
      {
        "kind": "containment",
        "navigation": 11,
        "origin": 25,
        "provenance": "authored",
        "reference": 105,
        "source": 24,
        "target": 25
      },
      {
        "kind": "transition",
        "navigation": 12,
        "origin": 25,
        "provenance": "implied",
        "reference": 150,
        "source": 32,
        "target": 31
      },
      {
        "kind": "containment",
        "navigation": 11,
        "origin": 28,
        "provenance": "authored",
        "reference": 116,
        "source": 25,
        "target": 28
      },
      {
        "kind": "containment",
        "navigation": 11,
        "origin": 28,
        "provenance": "authored",
        "reference": 117,
        "source": 25,
        "target": 28
      },
      {
        "kind": "containment",
        "navigation": 11,
        "origin": 26,
        "provenance": "authored",
        "reference": 118,
        "source": 25,
        "target": 26
      },
      {
        "kind": "succession",
        "navigation": 12,
        "origin": 26,
        "provenance": "implied",
        "reference": 149,
        "source": 32,
        "target": 31
      },
      {
        "kind": "containment",
        "navigation": 10,
        "origin": 31,
        "provenance": "authored",
        "reference": 106,
        "source": 24,
        "target": 31
      },
      {
        "kind": "containment",
        "navigation": 7,
        "origin": 29,
        "provenance": "authored",
        "reference": 107,
        "source": 24,
        "target": 29
      },
      {
        "kind": "initial-state",
        "navigation": 8,
        "origin": 29,
        "provenance": "authored",
        "reference": 119,
        "source": 29,
        "target": 32
      },
      {
        "kind": "containment",
        "navigation": 9,
        "origin": 32,
        "provenance": "authored",
        "reference": 108,
        "source": 24,
        "target": 32
      },
      {
        "kind": "containment",
        "navigation": 6,
        "origin": 30,
        "provenance": "authored",
        "reference": 109,
        "source": 24,
        "target": 30
      },
      {
        "kind": "containment",
        "navigation": 4,
        "origin": 33,
        "provenance": "authored",
        "reference": 55,
        "source": 0,
        "target": 33
      },
      {
        "kind": "containment",
        "navigation": 24,
        "origin": 34,
        "provenance": "authored",
        "reference": 56,
        "source": 0,
        "target": 34
      },
      {
        "kind": "containment",
        "navigation": 2,
        "origin": 22,
        "provenance": "authored",
        "reference": 57,
        "source": 0,
        "target": 22
      },
      {
        "kind": "initial-state",
        "navigation": 3,
        "origin": 22,
        "provenance": "authored",
        "reference": 63,
        "source": 22,
        "target": 33
      },
      {
        "kind": "containment",
        "navigation": 26,
        "origin": 1,
        "provenance": "authored",
        "reference": 58,
        "source": 0,
        "target": 1
      },
      {
        "kind": "transition",
        "navigation": 27,
        "origin": 1,
        "provenance": "implied",
        "reference": 144,
        "source": 31,
        "target": 34
      },
      {
        "kind": "containment",
        "navigation": 25,
        "origin": 7,
        "provenance": "authored",
        "reference": 71,
        "source": 1,
        "target": 7
      },
      {
        "kind": "containment",
        "navigation": 25,
        "origin": 7,
        "provenance": "authored",
        "reference": 72,
        "source": 1,
        "target": 7
      },
      {
        "kind": "containment",
        "navigation": 25,
        "origin": 7,
        "provenance": "authored",
        "reference": 73,
        "source": 1,
        "target": 7
      },
      {
        "kind": "containment",
        "navigation": 25,
        "origin": 2,
        "provenance": "authored",
        "reference": 74,
        "source": 1,
        "target": 2
      },
      {
        "kind": "succession",
        "navigation": 27,
        "origin": 2,
        "provenance": "implied",
        "reference": 143,
        "source": 31,
        "target": 34
      },
      {
        "kind": "containment",
        "navigation": 25,
        "origin": 3,
        "provenance": "authored",
        "reference": 75,
        "source": 1,
        "target": 3
      },
      {
        "kind": "containment",
        "navigation": 25,
        "origin": 4,
        "provenance": "authored",
        "reference": 82,
        "source": 3,
        "target": 4
      },
      {
        "kind": "containment",
        "navigation": 15,
        "origin": 8,
        "provenance": "authored",
        "reference": 59,
        "source": 0,
        "target": 8
      },
      {
        "kind": "transition",
        "navigation": 16,
        "origin": 8,
        "provenance": "implied",
        "reference": 103,
        "source": 33,
        "target": 24
      },
      {
        "kind": "containment",
        "navigation": 14,
        "origin": 14,
        "provenance": "authored",
        "reference": 179,
        "source": 8,
        "target": 14
      },
      {
        "kind": "containment",
        "navigation": 14,
        "origin": 14,
        "provenance": "authored",
        "reference": 180,
        "source": 8,
        "target": 14
      },
      {
        "kind": "containment",
        "navigation": 14,
        "origin": 14,
        "provenance": "authored",
        "reference": 181,
        "source": 8,
        "target": 14
      },
      {
        "kind": "containment",
        "navigation": 14,
        "origin": 9,
        "provenance": "authored",
        "reference": 182,
        "source": 8,
        "target": 9
      },
      {
        "kind": "succession",
        "navigation": 16,
        "origin": 9,
        "provenance": "implied",
        "reference": 102,
        "source": 33,
        "target": 24
      },
      {
        "kind": "containment",
        "navigation": 14,
        "origin": 10,
        "provenance": "authored",
        "reference": 183,
        "source": 8,
        "target": 10
      },
      {
        "kind": "containment",
        "navigation": 14,
        "origin": 11,
        "provenance": "authored",
        "reference": 191,
        "source": 10,
        "target": 11
      },
      {
        "kind": "containment",
        "navigation": 20,
        "origin": 15,
        "provenance": "authored",
        "reference": 60,
        "source": 0,
        "target": 15
      },
      {
        "kind": "transition",
        "navigation": 21,
        "origin": 15,
        "provenance": "implied",
        "reference": 114,
        "source": 24,
        "target": 33
      },
      {
        "kind": "containment",
        "navigation": 19,
        "origin": 21,
        "provenance": "authored",
        "reference": 152,
        "source": 15,
        "target": 21
      },
      {
        "kind": "containment",
        "navigation": 19,
        "origin": 21,
        "provenance": "authored",
        "reference": 153,
        "source": 15,
        "target": 21
      },
      {
        "kind": "containment",
        "navigation": 19,
        "origin": 21,
        "provenance": "authored",
        "reference": 154,
        "source": 15,
        "target": 21
      },
      {
        "kind": "containment",
        "navigation": 19,
        "origin": 16,
        "provenance": "authored",
        "reference": 155,
        "source": 15,
        "target": 16
      },
      {
        "kind": "succession",
        "navigation": 21,
        "origin": 16,
        "provenance": "implied",
        "reference": 113,
        "source": 24,
        "target": 33
      },
      {
        "kind": "containment",
        "navigation": 19,
        "origin": 17,
        "provenance": "authored",
        "reference": 156,
        "source": 15,
        "target": 17
      },
      {
        "kind": "containment",
        "navigation": 19,
        "origin": 18,
        "provenance": "authored",
        "reference": 164,
        "source": 17,
        "target": 18
      },
      {
        "kind": "containment",
        "navigation": 1,
        "origin": 23,
        "provenance": "authored",
        "reference": 61,
        "source": 0,
        "target": 23
      }
    ],
    "exposedRoots": [
      0
    ],
    "kind": "state-transition-view",
    "metadata": {
      "finalNodes": [],
      "initialNodes": [
        29,
        22
      ],
      "states": [
        0,
        24,
        31,
        32,
        33,
        34
      ]
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "actions",
            "members": [
              23
            ],
            "provenance": "direct"
          },
          {
            "kind": "states",
            "members": [
              24,
              33,
              34
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "StateDefinition",
        "name": "Machine",
        "notationRole": "definition",
        "owner": null,
        "reference": 1,
        "source": 0,
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
              7,
              7,
              7
            ],
            "provenance": "direct"
          },
          {
            "kind": "actions",
            "members": [
              3
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "TransitionUsage",
        "name": "fail",
        "notationRole": "usage",
        "owner": 0,
        "reference": 3,
        "source": 26,
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
        "name": null,
        "notationRole": "unsupported",
        "owner": 1,
        "reference": 38,
        "source": 25,
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
              4
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "AcceptActionUsage",
        "name": null,
        "notationRole": "usage",
        "owner": 1,
        "reference": 36,
        "source": 25,
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
        "name": null,
        "notationRole": "reference-usage",
        "owner": 3,
        "reference": 39,
        "source": 25,
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
        "name": null,
        "notationRole": "reference-usage",
        "owner": 1,
        "reference": 37,
        "source": 25,
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
        "name": null,
        "notationRole": "reference-usage",
        "owner": 1,
        "reference": 37,
        "source": 25,
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
        "name": null,
        "notationRole": "reference-usage",
        "owner": 1,
        "reference": 37,
        "source": 25,
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
              14,
              14,
              14
            ],
            "provenance": "direct"
          },
          {
            "kind": "actions",
            "members": [
              10
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "TransitionUsage",
        "name": "turnOn",
        "notationRole": "usage",
        "owner": 0,
        "reference": 16,
        "source": 15,
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
        "name": null,
        "notationRole": "unsupported",
        "owner": 8,
        "reference": 51,
        "source": 14,
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
              11
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "AcceptActionUsage",
        "name": null,
        "notationRole": "usage",
        "owner": 8,
        "reference": 49,
        "source": 14,
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
        "name": null,
        "notationRole": "reference-usage",
        "owner": 10,
        "reference": 52,
        "source": 14,
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
        "name": null,
        "notationRole": "reference-usage",
        "owner": 8,
        "reference": 50,
        "source": 14,
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
        "name": null,
        "notationRole": "reference-usage",
        "owner": 8,
        "reference": 50,
        "source": 14,
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
        "name": null,
        "notationRole": "reference-usage",
        "owner": 8,
        "reference": 50,
        "source": 14,
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
              21,
              21,
              21
            ],
            "provenance": "direct"
          },
          {
            "kind": "actions",
            "members": [
              17
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "TransitionUsage",
        "name": "turnOff",
        "notationRole": "usage",
        "owner": 0,
        "reference": 13,
        "source": 20,
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
        "name": null,
        "notationRole": "unsupported",
        "owner": 15,
        "reference": 47,
        "source": 19,
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
              18
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "AcceptActionUsage",
        "name": null,
        "notationRole": "usage",
        "owner": 15,
        "reference": 45,
        "source": 19,
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
        "name": null,
        "notationRole": "reference-usage",
        "owner": 17,
        "reference": 48,
        "source": 19,
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
        "name": null,
        "notationRole": "reference-usage",
        "owner": 15,
        "reference": 46,
        "source": 19,
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
        "name": null,
        "notationRole": "reference-usage",
        "owner": 15,
        "reference": 46,
        "source": 19,
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
        "name": null,
        "notationRole": "reference-usage",
        "owner": 15,
        "reference": 46,
        "source": 19,
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
        "name": null,
        "notationRole": "unsupported",
        "owner": 0,
        "reference": 35,
        "source": 2,
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
        "metaclass": "ActionUsage",
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "reference": 34,
        "source": 1,
        "stateSubaction": "entry",
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [
          {
            "kind": "actions",
            "members": [
              30
            ],
            "provenance": "direct"
          },
          {
            "kind": "states",
            "members": [
              31,
              32
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "StateUsage",
        "name": "on",
        "notationRole": "usage",
        "owner": 0,
        "reference": 8,
        "source": 5,
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
              28,
              28
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "TransitionUsage",
        "name": null,
        "notationRole": "usage",
        "owner": 24,
        "reference": 42,
        "source": 11,
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
        "name": null,
        "notationRole": "unsupported",
        "owner": 25,
        "reference": 44,
        "source": 11,
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
        "name": null,
        "notationRole": "reference-usage",
        "owner": 25,
        "reference": 43,
        "source": 11,
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
        "name": null,
        "notationRole": "reference-usage",
        "owner": 25,
        "reference": 43,
        "source": 11,
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
        "name": null,
        "notationRole": "unsupported",
        "owner": 24,
        "reference": 41,
        "source": 7,
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
        "metaclass": "ActionUsage",
        "name": null,
        "notationRole": "usage",
        "owner": 24,
        "reference": 40,
        "source": 6,
        "stateSubaction": "entry",
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
        "name": "ready",
        "notationRole": "usage",
        "owner": 24,
        "reference": 11,
        "source": 10,
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
        "name": "warming",
        "notationRole": "usage",
        "owner": 24,
        "reference": 12,
        "source": 9,
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
        "name": "off",
        "notationRole": "usage",
        "owner": 0,
        "reference": 7,
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
        "metaclass": "StateUsage",
        "name": "fault",
        "notationRole": "usage",
        "owner": 0,
        "reference": 6,
        "source": 24,
        "stateSubaction": null,
        "successionGuard": null,
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
        "reference": 62,
        "source": 0,
        "target": {
          "reference": 27,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 110,
        "source": 24,
        "target": {
          "reference": 25,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 111,
        "source": 24,
        "target": {
          "reference": 29,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 112,
        "source": 24,
        "target": {
          "reference": 33,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 115,
        "source": 24,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 122,
        "source": 25,
        "target": {
          "reference": 24,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 123,
        "source": 25,
        "target": {
          "reference": 25,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 124,
        "source": 25,
        "target": {
          "reference": 30,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionSource",
        "navigation": 12,
        "provenance": "authored",
        "reference": 128,
        "source": 25,
        "target": {
          "node": 32,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTarget",
        "navigation": 13,
        "provenance": "authored",
        "reference": 129,
        "source": 25,
        "target": {
          "node": 31,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 130,
        "source": 25,
        "target": {
          "node": 24,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 138,
        "source": 28,
        "target": {
          "node": 25,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": null,
        "provenance": "implied",
        "reference": 133,
        "source": 28,
        "target": {
          "reference": 22,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": null,
        "provenance": "implied",
        "reference": 134,
        "source": 28,
        "target": {
          "reference": 32,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 139,
        "source": 28,
        "target": {
          "node": 25,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 135,
        "source": 26,
        "target": {
          "reference": 26,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 12,
        "provenance": "authored",
        "reference": 136,
        "source": 26,
        "target": {
          "node": 32,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 13,
        "provenance": "authored",
        "reference": 137,
        "source": 26,
        "target": {
          "node": 31,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 140,
        "source": 31,
        "target": {
          "reference": 25,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 141,
        "source": 31,
        "target": {
          "reference": 29,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 142,
        "source": 31,
        "target": {
          "reference": 33,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 145,
        "source": 31,
        "target": {
          "node": 24,
          "status": "resolved"
        }
      },
      {
        "kind": "initialState",
        "navigation": 8,
        "provenance": "authored",
        "reference": 120,
        "source": 29,
        "target": {
          "node": 32,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 125,
        "source": 29,
        "target": {
          "reference": 26,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 131,
        "source": 29,
        "target": {
          "node": 24,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 146,
        "source": 32,
        "target": {
          "reference": 25,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 147,
        "source": 32,
        "target": {
          "reference": 29,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 148,
        "source": 32,
        "target": {
          "reference": 33,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 151,
        "source": 32,
        "target": {
          "node": 24,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": null,
        "provenance": "implied",
        "reference": 121,
        "source": 30,
        "target": {
          "reference": 28,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 126,
        "source": 30,
        "target": {
          "reference": 23,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 127,
        "source": 30,
        "target": {
          "reference": 25,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 132,
        "source": 30,
        "target": {
          "node": 24,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 99,
        "source": 33,
        "target": {
          "reference": 25,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 100,
        "source": 33,
        "target": {
          "reference": 29,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 101,
        "source": 33,
        "target": {
          "reference": 33,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 104,
        "source": 33,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 95,
        "source": 34,
        "target": {
          "reference": 25,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 96,
        "source": 34,
        "target": {
          "reference": 29,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 97,
        "source": 34,
        "target": {
          "reference": 33,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 98,
        "source": 34,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "initialState",
        "navigation": 3,
        "provenance": "authored",
        "reference": 64,
        "source": 22,
        "target": {
          "node": 33,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 66,
        "source": 22,
        "target": {
          "reference": 26,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 69,
        "source": 22,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "memberAccessOperand",
        "navigation": 27,
        "provenance": "authored",
        "reference": 76,
        "source": 1,
        "target": {
          "node": 31,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 77,
        "source": 1,
        "target": {
          "reference": 24,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 78,
        "source": 1,
        "target": {
          "reference": 25,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTarget",
        "navigation": 29,
        "provenance": "authored",
        "reference": 79,
        "source": 1,
        "target": {
          "node": 34,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTrigger",
        "navigation": 28,
        "provenance": "authored",
        "reference": 80,
        "source": 1,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 81,
        "source": 1,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 89,
        "source": 7,
        "target": {
          "node": 1,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": null,
        "provenance": "implied",
        "reference": 83,
        "source": 7,
        "target": {
          "reference": 22,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 90,
        "source": 7,
        "target": {
          "node": 1,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 84,
        "source": 7,
        "target": {
          "node": 4,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 91,
        "source": 7,
        "target": {
          "node": 1,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 85,
        "source": 2,
        "target": {
          "reference": 26,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 27,
        "provenance": "authored",
        "reference": 87,
        "source": 2,
        "target": {
          "node": 31,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 29,
        "provenance": "authored",
        "reference": 88,
        "source": 2,
        "target": {
          "node": 34,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 86,
        "source": 3,
        "target": {
          "reference": 21,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 92,
        "source": 3,
        "target": {
          "node": 1,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": null,
        "provenance": "implied",
        "reference": 93,
        "source": 4,
        "target": {
          "reference": 53,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 94,
        "source": 4,
        "target": {
          "node": 3,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 184,
        "source": 8,
        "target": {
          "reference": 24,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 185,
        "source": 8,
        "target": {
          "reference": 25,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 186,
        "source": 8,
        "target": {
          "reference": 30,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionSource",
        "navigation": 16,
        "provenance": "authored",
        "reference": 187,
        "source": 8,
        "target": {
          "node": 33,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTarget",
        "navigation": 18,
        "provenance": "authored",
        "reference": 188,
        "source": 8,
        "target": {
          "node": 24,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTrigger",
        "navigation": 17,
        "provenance": "authored",
        "reference": 189,
        "source": 8,
        "target": {
          "reference": 19,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 190,
        "source": 8,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 200,
        "source": 14,
        "target": {
          "node": 8,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": null,
        "provenance": "implied",
        "reference": 192,
        "source": 14,
        "target": {
          "reference": 22,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": null,
        "provenance": "implied",
        "reference": 193,
        "source": 14,
        "target": {
          "reference": 32,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 201,
        "source": 14,
        "target": {
          "node": 8,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": null,
        "provenance": "implied",
        "reference": 194,
        "source": 14,
        "target": {
          "reference": 31,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 195,
        "source": 14,
        "target": {
          "node": 11,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 202,
        "source": 14,
        "target": {
          "node": 8,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 196,
        "source": 9,
        "target": {
          "reference": 26,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 16,
        "provenance": "authored",
        "reference": 198,
        "source": 9,
        "target": {
          "node": 33,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 18,
        "provenance": "authored",
        "reference": 199,
        "source": 9,
        "target": {
          "node": 24,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 197,
        "source": 10,
        "target": {
          "reference": 21,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 203,
        "source": 10,
        "target": {
          "node": 8,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": null,
        "provenance": "implied",
        "reference": 204,
        "source": 11,
        "target": {
          "reference": 53,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 205,
        "source": 11,
        "target": {
          "node": 10,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 157,
        "source": 15,
        "target": {
          "reference": 24,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 158,
        "source": 15,
        "target": {
          "reference": 25,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 159,
        "source": 15,
        "target": {
          "reference": 30,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionSource",
        "navigation": 21,
        "provenance": "authored",
        "reference": 160,
        "source": 15,
        "target": {
          "node": 24,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTarget",
        "navigation": 23,
        "provenance": "authored",
        "reference": 161,
        "source": 15,
        "target": {
          "node": 33,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTrigger",
        "navigation": 22,
        "provenance": "authored",
        "reference": 162,
        "source": 15,
        "target": {
          "reference": 19,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 163,
        "source": 15,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 173,
        "source": 21,
        "target": {
          "node": 15,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": null,
        "provenance": "implied",
        "reference": 165,
        "source": 21,
        "target": {
          "reference": 22,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": null,
        "provenance": "implied",
        "reference": 166,
        "source": 21,
        "target": {
          "reference": 32,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 174,
        "source": 21,
        "target": {
          "node": 15,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": null,
        "provenance": "implied",
        "reference": 167,
        "source": 21,
        "target": {
          "reference": 31,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 168,
        "source": 21,
        "target": {
          "node": 18,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 175,
        "source": 21,
        "target": {
          "node": 15,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 169,
        "source": 16,
        "target": {
          "reference": 26,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 21,
        "provenance": "authored",
        "reference": 171,
        "source": 16,
        "target": {
          "node": 24,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 23,
        "provenance": "authored",
        "reference": 172,
        "source": 16,
        "target": {
          "node": 33,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 170,
        "source": 17,
        "target": {
          "reference": 21,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 176,
        "source": 17,
        "target": {
          "node": 15,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": null,
        "provenance": "implied",
        "reference": 177,
        "source": 18,
        "target": {
          "reference": 53,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 178,
        "source": 18,
        "target": {
          "node": 17,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": null,
        "provenance": "implied",
        "reference": 65,
        "source": 23,
        "target": {
          "reference": 28,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 67,
        "source": 23,
        "target": {
          "reference": 23,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 68,
        "source": 23,
        "target": {
          "reference": 25,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 70,
        "source": 23,
        "target": {
          "node": 0,
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
          "navigation": 12,
          "provenance": "implied",
          "source": 4,
          "target": 3,
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
          "label": null,
          "navigation": 8,
          "provenance": "authored",
          "source": 2,
          "target": 4,
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
          "id": "transition-2",
          "label": null,
          "navigation": 3,
          "provenance": "authored",
          "source": 0,
          "target": 5,
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
          "id": "transition-3",
          "label": "fail",
          "navigation": 27,
          "provenance": "implied",
          "source": 3,
          "target": 6,
          "trigger": {
            "label": "Fault",
            "navigation": 28,
            "status": "accept",
            "target": {
              "id": "element/v129:memory://snapshot/model.sysml7:packagen4:Lamp1:013:attribute-defn5:Fault1:0",
              "label": "Fault"
            }
          }
        },
        {
          "effect": {
            "status": "absent"
          },
          "guard": {
            "status": "absent"
          },
          "id": "transition-4",
          "label": "turnOn",
          "navigation": 16,
          "provenance": "implied",
          "source": 5,
          "target": 1,
          "trigger": {
            "label": "Switch",
            "navigation": 17,
            "status": "accept",
            "target": {
              "id": "element/v129:memory://snapshot/model.sysml7:packagen4:Lamp1:013:attribute-defn6:Switch1:0",
              "label": "Switch"
            }
          }
        },
        {
          "effect": {
            "status": "absent"
          },
          "guard": {
            "status": "absent"
          },
          "id": "transition-5",
          "label": "turnOff",
          "navigation": 21,
          "provenance": "implied",
          "source": 1,
          "target": 5,
          "trigger": {
            "label": "Switch",
            "navigation": 22,
            "status": "accept",
            "target": {
              "id": "element/v129:memory://snapshot/model.sysml7:packagen4:Lamp1:013:attribute-defn6:Switch1:0",
              "label": "Switch"
            }
          }
        }
      ],
      "vertices": [
        {
          "id": "state-0",
          "kind": "initial",
          "label": "",
          "navigation": 2
        },
        {
          "id": "state-1",
          "kind": "state",
          "label": "on",
          "navigation": 5
        },
        {
          "id": "state-2",
          "kind": "initial",
          "label": "",
          "navigation": 7
        },
        {
          "id": "state-3",
          "kind": "state",
          "label": "ready",
          "navigation": 10
        },
        {
          "id": "state-4",
          "kind": "state",
          "label": "warming",
          "navigation": 9
        },
        {
          "id": "state-5",
          "kind": "state",
          "label": "off",
          "navigation": 4
        },
        {
          "id": "state-6",
          "kind": "state",
          "label": "fault",
          "navigation": 24
        }
      ]
    }
  }
}

~~~
