# META
~~~ini
description=State diagrams retain temporal and typed accepting triggers and referenced state actions
type=generate
libraries=standard
plugin=native:diagram
viewKind=state-transition-view
viewDocument=model.sysml
viewQualifiedName=Behavior::machine
require_no_diagnostics=true
require_complete_publication=true
~~~
# SOURCE
## model.sysml
~~~sysml
package Behavior {
    private import StandardViewDefinitions::*;
    attribute def InvokeReset;
    attribute def CommandFinished;
    action externalStart;
    state def Machine {
        attribute dwellTime : ISQ::DurationValue = 5 [SI::s];
        attribute engagementDiscrepancyTime : ISQ::DurationValue = 0.1 [SI::s];
        attribute ready : ScalarValues::Boolean;
        action startRun;
        action stopRun;
        action rememberLanding;
        action fullForce;
        entry; then idle;
        state idle;
        state moving {
            entry startRun;
            do rememberLanding;
            exit stopRun;
            state running { entry externalStart; do fullForce; }
        }
        transition begin first idle accept c : InvokeReset then moving;
        transition dwellExpired first moving accept after dwellTime if ready then idle;
        transition brakeStuck first moving accept after engagementDiscrepancyTime then idle;
        transition first idle accept d : CommandFinished then moving;
        transition completion first moving then idle;
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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:cae641981affdcff2240a8d3d68e0fde843c83bc664270ea8d784164231ca9cd") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::CommandFinished"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::InvokeReset"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entry-action-binding) (membership (kind feature) (visibility default) (role entry)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "idle")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "idle")) (transitionTarget (reference "moving")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (succession (reference "idle")) (succession (reference "moving")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction inout)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CommandFinished") (direction inout)))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (named (kind parameter) (name "d"))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "idle")) (transitionTarget (reference "moving")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (succession (reference "idle")) (succession (reference "moving")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction inout)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "InvokeReset") (direction inout)))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin::c"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::brakeStuck"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "moving")) (transitionTarget (reference "idle")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (succession (reference "moving")) (succession (reference "idle")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind parameter) (ordinal 1))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction inout)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (trigger-kind after) (expression-result (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "engagementDiscrepancyTime")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::completion"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "moving")) (transitionTarget (reference "idle")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "completion")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (succession (reference "moving")) (succession (reference "idle")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "completion")) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "completion")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellExpired"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "moving")) (transitionTarget (reference "idle")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (succession (reference "moving")) (succession (reference "idle")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (kind kerml-boolean-expression) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "ready")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind parameter) (ordinal 1))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction inout)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (trigger-kind after) (expression-result (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "dwellTime")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellTime"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ISQ::DurationValue")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (quantityUnit (reference "SI::s")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::engagementDiscrepancyTime"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ISQ::DurationValue")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (quantityUnit (reference "SI::s")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-rational) (ordinal 0))))) (kind kerml-literal-rational) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::fullForce"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 0))))) (kind perform-action) (membership (kind feature) (visibility default) (role entry)) (authored (membership (kind feature) (visibility default) (role entry)) (relationships (entryActionBinding (reference "startRun")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 1))))) (kind perform-action) (membership (kind feature) (visibility default) (role do)) (authored (membership (kind feature) (visibility default) (role do)) (relationships (doActionBinding (reference "rememberLanding")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 2))))) (kind perform-action) (membership (kind feature) (visibility default) (role exit)) (authored (membership (kind feature) (visibility default) (role exit)) (relationships (exitActionBinding (reference "stopRun")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving::running"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 0))))) (kind perform-action) (membership (kind feature) (visibility default) (role entry)) (authored (membership (kind feature) (visibility default) (role entry)) (relationships (entryActionBinding (reference "externalStart")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 1))))) (kind perform-action) (membership (kind feature) (visibility default) (role do)) (authored (membership (kind feature) (visibility default) (role do)) (relationships (doActionBinding (reference "fullForce")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::ready"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Boolean")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::rememberLanding"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::startRun"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::stopRun"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::externalStart"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::machine"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StateTransitionView")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind view) (name "machine")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Machine")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0))
      (authored-target "moving")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "moving")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "CommandFinished")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::CommandFinished")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin"))) (kind transitionSource) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin"))) (kind transitionTarget) (ordinal 0))
      (authored-target "moving")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "moving")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "InvokeReset")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::InvokeReset")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::brakeStuck"))) (kind transitionSource) (ordinal 0))
      (authored-target "moving")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::brakeStuck"))) (kind transitionTarget) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "moving")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "engagementDiscrepancyTime")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::engagementDiscrepancyTime")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::completion"))) (kind transitionSource) (ordinal 0))
      (authored-target "moving")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::completion"))) (kind transitionTarget) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "completion")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "moving")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "completion")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellExpired"))) (kind transitionSource) (ordinal 0))
      (authored-target "moving")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellExpired"))) (kind transitionTarget) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "moving")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "ready")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::ready")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "dwellTime")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellTime")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellTime"))) (kind featureTyping) (ordinal 0))
      (authored-target "ISQ::DurationValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind quantityUnit) (ordinal 0))
      (authored-target "SI::s")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/si.md") (qualified-name "SI::second")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::engagementDiscrepancyTime"))) (kind featureTyping) (ordinal 0))
      (authored-target "ISQ::DurationValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind quantityUnit) (ordinal 0))
      (authored-target "SI::s")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/si.md") (qualified-name "SI::second")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 0))))) (kind entryActionBinding) (ordinal 0))
      (authored-target "startRun")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::startRun")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 1))))) (kind doActionBinding) (ordinal 0))
      (authored-target "rememberLanding")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::rememberLanding")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 2))))) (kind exitActionBinding) (ordinal 0))
      (authored-target "stopRun")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::stopRun")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 0))))) (kind entryActionBinding) (ordinal 0))
      (authored-target "externalStart")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::externalStart")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 1))))) (kind doActionBinding) (ordinal 0))
      (authored-target "fullForce")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::fullForce")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::ready"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::Boolean")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::machine"))) (kind featureTyping) (ordinal 0))
      (authored-target "StateTransitionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind view) (name "machine")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Machine")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine")))))
  )
  (relationships
    (relationship (kind initialState) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind typing) (direction inout) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::CommandFinished"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind typing) (direction inout) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::InvokeReset"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::brakeStuck"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::brakeStuck"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::brakeStuck"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::brakeStuck"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::engagementDiscrepancyTime"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::completion"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::completion"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::completion"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::completion"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "completion")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "completion")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "completion")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "completion")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellExpired"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellExpired"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellExpired"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellExpired"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::ready"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellTime"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellTime"))) (target (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellTime"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::engagementDiscrepancyTime"))) (target (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::engagementDiscrepancyTime"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind entryActionBinding) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::startRun"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 0))))) (kind entryActionBinding) (ordinal 0)))
    (relationship (kind doActionBinding) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::rememberLanding"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 1))))) (kind doActionBinding) (ordinal 0)))
    (relationship (kind exitActionBinding) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 2))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::stopRun"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 2))))) (kind exitActionBinding) (ordinal 0)))
    (relationship (kind entryActionBinding) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::externalStart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 0))))) (kind entryActionBinding) (ordinal 0)))
    (relationship (kind doActionBinding) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::fullForce"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 1))))) (kind doActionBinding) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::ready"))) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::ready"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::machine"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::machine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind view) (name "machine")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind view) (name "machine")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::CommandFinished"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::InvokeReset"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::entryAction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::stateTransitions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::accepter"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::transitionLinkSource"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::transitionLinkSource"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (named (kind parameter) (name "d"))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (named (kind parameter) (name "d"))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (named (kind parameter) (name "d"))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::payload"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::stateTransitions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::accepter"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::transitionLinkSource"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::transitionLinkSource"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind accept-action) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin::c"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin::c"))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin::c"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::payload"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::brakeStuck"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::brakeStuck"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::brakeStuck"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::brakeStuck"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::stateTransitions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::accepter"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::transitionLinkSource"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::brakeStuck"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::brakeStuck"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::brakeStuck"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::brakeStuck"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::payload"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::transitionLinkSource"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation::result"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::completion"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::completion"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::completion"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::completion"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::stateTransitions"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "completion")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::transitionLinkSource"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "completion")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::completion"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "completion")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::completion"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "completion")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "completion")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::transitionLinkSource"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellExpired"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellExpired"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellExpired"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellExpired"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::stateTransitions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::accepter"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::transitionLinkSource"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellExpired"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellExpired"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellExpired"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellExpired"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellExpired"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::payload"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::transitionLinkSource"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::guard"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation::result"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellTime"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellTime"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation::result"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalIntegerEvaluations"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::engagementDiscrepancyTime"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::engagementDiscrepancyTime"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation::result"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-rational) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalRationalEvaluations"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::fullForce"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::fullForce"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::fullForce"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::exclusiveStates"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::exclusiveStates"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 2))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 2))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::doAction"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::entryAction"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 2))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::exitAction"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving::running"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving::running"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving::running"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::exclusiveStates"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving::running"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving::running"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving::running"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::doAction"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::entryAction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::ready"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::ready"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::rememberLanding"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::rememberLanding"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::rememberLanding"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::startRun"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::startRun"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::startRun"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::stopRun"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::stopRun"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::stopRun"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::externalStart"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::machine"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind view) (name "machine")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::machine"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (state evaluated) (value (kind quantity) (magnitude (value (kind real) (real 0.1))) (unit "SI::s")))
    (evaluated (declaration (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (state evaluated) (value (kind quantity) (magnitude (value (kind integer) (integer 5))) (unit "SI::s")))
    (evaluated (declaration (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0))))) (state literal) (value (kind quantity) (magnitude (value (kind integer) (integer 5))) (unit "SI::s")))
    (evaluated (declaration (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0))))) (state literal) (value (kind quantity) (magnitude (value (kind real) (real 0.1))) (unit "SI::s")))
    (unit (declaration (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0))))) (ordinal 0) (authored "SI::s") (start 6 54) (end 6 59) (outcome (status resolved) (unit (node (document "memory://snapshot/sysml.library/si.md") (qualified-name "SI::second"))) (dimension (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationUnit")))))
    (unit (declaration (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0))))) (ordinal 0) (authored "SI::s") (start 7 72) (end 7 77) (outcome (status resolved) (unit (node (document "memory://snapshot/sysml.library/si.md") (qualified-name "SI::second"))) (dimension (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationUnit")))))
    (measurement (declaration (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellTime"))) (status required) (dimension (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationUnit"))))
    (measurement (declaration (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::engagementDiscrepancyTime"))) (status required) (dimension (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationUnit"))))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::CommandFinished")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::InvokeReset")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::DecisionPerformance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)))))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::CommandFinished")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::CommandFinished")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::CommandFinished")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::payload")) (scopes any feature))
      (subtype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (named (kind parameter) (name "d")))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (named (kind parameter) (name "d")))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::CommandFinished")) (source inherited) (from (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::CommandFinished")) (scopes any))
      (supertype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::payload")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::payload")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind succession) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin")))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind accept-action) (ordinal 0)))))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::InvokeReset")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::InvokeReset")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::InvokeReset")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::payload")) (scopes any feature))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin::c")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin::c")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin")))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::InvokeReset")) (source inherited) (from (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::InvokeReset")) (scopes any))
      (supertype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::payload")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::payload")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::brakeStuck")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind succession) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::brakeStuck")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::brakeStuck")))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::brakeStuck")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind parameter) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::brakeStuck")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::payload")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::payload")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::payload")) (scopes any feature))
      (subtype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind parameter) (ordinal 1)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation::result"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation::result")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::completion")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "completion")) (anonymous (kind succession) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "completion")) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::completion")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "completion")) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::completion")))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellExpired")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind succession) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellExpired")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellExpired")))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind kerml-boolean-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellExpired")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::guard")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellExpired")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind parameter) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellExpired")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::payload")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::payload")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::payload")) (scopes any feature))
      (subtype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind parameter) (ordinal 1)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation::result"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation::result")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellTime")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine")))
      (type (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (effective-type (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Array")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Collection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::OrderedCollection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::ScalarQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::VectorQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/vector_values.md") (qualified-name "VectorValues::VectorValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation::result"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation::result")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralIntegerEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalIntegerEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralIntegerEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalIntegerEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::engagementDiscrepancyTime")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine")))
      (type (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (effective-type (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Array")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Collection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::OrderedCollection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::ScalarQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::VectorQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/vector_values.md") (qualified-name "VectorValues::VectorValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation::result"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation::result")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-rational) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralRationalEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalRationalEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralRationalEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalRationalEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::fullForce")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
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
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::entry")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::entryAction")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::doAction"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::do")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::middle")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::doAction")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::exitAction"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::exit")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::exitAction")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving::running")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving::running")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::entry")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::entryAction")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving::running")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::doAction"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::do")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::middle")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::doAction")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::ready")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine")))
      (type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::rememberLanding")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
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
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::startRun")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
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
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::stopRun")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
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
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::externalStart")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::machine")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind view) (name "machine")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::machine")))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")))) (feature-reference "engagementDiscrepancyTime" (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::engagementDiscrepancyTime")))))
  (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (feature-reference "ready" (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::ready")))))
  (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")))) (feature-reference "dwellTime" (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellTime")))))
  (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (result-typing (status unsupported)) (literal (value (kind quantity) (magnitude (value (kind integer) (integer 5))) (unit "SI::s"))))
  (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (result-typing (status unsupported)) (literal (value (kind quantity) (magnitude (value (kind real) (real 0.1))) (unit "SI::s"))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation evaluated) (value (kind boolean) (boolean true)) (feature-reference "this::isRunToCompletion" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "this::runToCompletionScope" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine"))) (element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation evaluated) (value (kind boolean) (boolean true)) (feature-reference "this::isRunToCompletion" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine"))) (element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "this::runToCompletionScope" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine"))) (element (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::incomingTransitionTrigger"))) (effective-element (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::incomingTransitionTrigger"))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::incomingTransitionTrigger"))) (outcome unsupported) (result-typing (status unsupported)) (evaluation unsupported))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine"))) (element (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::isTriggerDuring"))) (effective-element (node (document "memory://snapshot/sysml.library/states.md") (path (named (kind library-package) (name "States")) (named (kind state-def) (name "StateAction")) (anonymous (kind attribute) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::isTriggerDuring"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine"))) (element (node (document "memory://snapshot/sysml.library/states.md") (path (named (kind library-package) (name "States")) (named (kind state-def) (name "StateAction")) (anonymous (kind attribute) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/states.md") (path (named (kind library-package) (name "States")) (named (kind state-def) (name "StateAction")) (anonymous (kind attribute) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::isTriggerDuring"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::receiver"))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::receiver"))) (authored-element (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::triggerTarget"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::accNum"))) (effective-element (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::accNum"))) (authored-element (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::accNum"))) (outcome unsupported) (result-typing (status unsupported)) (evaluation unsupported))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::triggerTarget"))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::receiver"))) (authored-element (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::triggerTarget"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "that::this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1)))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "that::incomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::Clock")))) (evaluation non-constant) (feature-reference "that::localClock" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::Clock")))) (evaluation non-constant) (feature-reference "that::localClock" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "that::incomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "that::this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1)))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "that::this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "that::thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "that::thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::receiver"))) (effective-element (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::receiver"))) (authored-element (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::receiver"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "this" (target (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0)))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/transition_performances.md") (path (named (kind library-package) (name "TransitionPerformances")) (named (kind kerml-behavior) (name "TransitionPerformance")) (named (kind kerml-step) (name "accept")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/transition_performances.md") (path (named (kind library-package) (name "TransitionPerformances")) (named (kind kerml-behavior) (name "TransitionPerformance")) (named (kind kerml-step) (name "accept")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/transition_performances.md") (path (named (kind library-package) (name "TransitionPerformances")) (named (kind kerml-behavior) (name "TransitionPerformance")) (named (kind kerml-step) (name "accept")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::MessageTransfer")))) (evaluation non-constant) (feature-reference "trigger" (target (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::trigger")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin"))) (element (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::receiver"))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::receiver"))) (authored-element (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::triggerTarget"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin"))) (element (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::accNum"))) (effective-element (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::accNum"))) (authored-element (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::accNum"))) (outcome unsupported) (result-typing (status unsupported)) (evaluation unsupported))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin"))) (element (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::triggerTarget"))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::receiver"))) (authored-element (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::triggerTarget"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "that::this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1)))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "that::incomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::Clock")))) (evaluation non-constant) (feature-reference "that::localClock" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind accept-action) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind accept-action) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::Clock")))) (evaluation non-constant) (feature-reference "that::localClock" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "that::incomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "that::this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1)))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "that::this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "that::thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "that::thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::receiver"))) (effective-element (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::receiver"))) (authored-element (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::receiver"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "this" (target (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0)))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/transition_performances.md") (path (named (kind library-package) (name "TransitionPerformances")) (named (kind kerml-behavior) (name "TransitionPerformance")) (named (kind kerml-step) (name "accept")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/transition_performances.md") (path (named (kind library-package) (name "TransitionPerformances")) (named (kind kerml-behavior) (name "TransitionPerformance")) (named (kind kerml-step) (name "accept")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/transition_performances.md") (path (named (kind library-package) (name "TransitionPerformances")) (named (kind kerml-behavior) (name "TransitionPerformance")) (named (kind kerml-step) (name "accept")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::MessageTransfer")))) (evaluation non-constant) (feature-reference "trigger" (target (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::trigger")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::brakeStuck"))) (element (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::receiver"))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::receiver"))) (authored-element (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::triggerTarget"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::brakeStuck"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::brakeStuck"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::brakeStuck"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::brakeStuck"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::brakeStuck"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::brakeStuck"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::brakeStuck")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::brakeStuck"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::brakeStuck")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::brakeStuck"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::brakeStuck")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::brakeStuck"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::brakeStuck"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::brakeStuck"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::brakeStuck")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::brakeStuck"))) (element (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::accNum"))) (effective-element (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::accNum"))) (authored-element (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::accNum"))) (outcome unsupported) (result-typing (status unsupported)) (evaluation unsupported))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::brakeStuck"))) (element (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::triggerTarget"))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::receiver"))) (authored-element (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::triggerTarget"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "that::this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1)))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "that::incomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::Clock")))) (evaluation non-constant) (feature-reference "that::localClock" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::Clock")))) (evaluation non-constant) (feature-reference "that::localClock" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "that::incomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "that::this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1)))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "that::this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "that::thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "that::thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::receiver"))) (effective-element (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::receiver"))) (authored-element (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::receiver"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "this" (target (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0)))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/transition_performances.md") (path (named (kind library-package) (name "TransitionPerformances")) (named (kind kerml-behavior) (name "TransitionPerformance")) (named (kind kerml-step) (name "accept")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/transition_performances.md") (path (named (kind library-package) (name "TransitionPerformances")) (named (kind kerml-behavior) (name "TransitionPerformance")) (named (kind kerml-step) (name "accept")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/transition_performances.md") (path (named (kind library-package) (name "TransitionPerformances")) (named (kind kerml-behavior) (name "TransitionPerformance")) (named (kind kerml-step) (name "accept")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::MessageTransfer")))) (evaluation non-constant) (feature-reference "trigger" (target (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::trigger")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::completion"))) (element (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::receiver"))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::receiver"))) (authored-element (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::triggerTarget"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::completion"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::completion"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::completion"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::completion"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::completion"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::completion"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::completion")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::completion"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::completion")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::completion"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::completion")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::completion"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::completion"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::completion"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::completion")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::completion"))) (element (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::accNum"))) (effective-element (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::accNum"))) (authored-element (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::accNum"))) (outcome unsupported) (result-typing (status unsupported)) (evaluation unsupported))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::completion"))) (element (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::triggerTarget"))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::receiver"))) (authored-element (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::triggerTarget"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellExpired"))) (element (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::receiver"))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::receiver"))) (authored-element (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::triggerTarget"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellExpired"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellExpired"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellExpired"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellExpired"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellExpired"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellExpired"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellExpired")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellExpired"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellExpired")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellExpired"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellExpired")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellExpired"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellExpired"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellExpired"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellExpired")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellExpired"))) (element (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::accNum"))) (effective-element (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::accNum"))) (authored-element (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::accNum"))) (outcome unsupported) (result-typing (status unsupported)) (evaluation unsupported))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellExpired"))) (element (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::triggerTarget"))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::receiver"))) (authored-element (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::triggerTarget"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind kerml-boolean-expression) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind kerml-boolean-expression) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind kerml-boolean-expression) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind kerml-boolean-expression) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "that::this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1)))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "that::incomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::Clock")))) (evaluation non-constant) (feature-reference "that::localClock" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::Clock")))) (evaluation non-constant) (feature-reference "that::localClock" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "that::incomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "that::this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1)))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "that::this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "that::thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "that::thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::receiver"))) (effective-element (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::receiver"))) (authored-element (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::receiver"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "this" (target (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0)))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/transition_performances.md") (path (named (kind library-package) (name "TransitionPerformances")) (named (kind kerml-behavior) (name "TransitionPerformance")) (named (kind kerml-step) (name "accept")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/transition_performances.md") (path (named (kind library-package) (name "TransitionPerformances")) (named (kind kerml-behavior) (name "TransitionPerformance")) (named (kind kerml-step) (name "accept")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/transition_performances.md") (path (named (kind library-package) (name "TransitionPerformances")) (named (kind kerml-behavior) (name "TransitionPerformance")) (named (kind kerml-step) (name "accept")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::MessageTransfer")))) (evaluation non-constant) (feature-reference "trigger" (target (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::trigger")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellTime"))) (element (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Array::flattenedSize"))) (effective-element (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Array::flattenedSize"))) (authored-element (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Array::flattenedSize"))) (outcome unsupported) (result-typing (status unsupported)) (evaluation unsupported))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellTime"))) (element (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Array::rank"))) (outcome ambiguous) (candidates (node (document "memory://snapshot/sysml.library/quantities.md") (path (named (kind library-package) (name "Quantities")) (named (kind attribute-def) (name "TensorQuantityValue")) (anonymous (kind attribute) (ordinal 0)))) (node (document "memory://snapshot/sysml.library/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue::dimension"))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellTime"))) (element (node (document "memory://snapshot/sysml.library/quantities.md") (path (named (kind library-package) (name "Quantities")) (named (kind attribute-def) (name "TensorQuantityValue")) (anonymous (kind attribute) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/quantities.md") (path (named (kind library-package) (name "Quantities")) (named (kind attribute-def) (name "TensorQuantityValue")) (anonymous (kind attribute) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/quantities.md") (path (named (kind library-package) (name "Quantities")) (named (kind attribute-def) (name "TensorQuantityValue")) (anonymous (kind attribute) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Positive")))) (evaluation non-constant) (feature-reference "mRef::dimensions" (target (node (document "memory://snapshot/sysml.library/measurement_references.md") (path (named (kind library-package) (name "MeasurementReferences")) (named (kind attribute-def) (name "ScalarMeasurementReference")) (anonymous (kind attribute) (ordinal 0)))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellTime"))) (element (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue::boundMatch"))) (effective-element (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue::boundMatch"))) (authored-element (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue::boundMatch"))) (outcome resolved) (result-typing (status unsupported)) (evaluation non-constant) (operator "or" (operator "==" (feature-reference "isBound" (target (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue::isBound")))) (feature-reference "mRef::isBound" (target (node (document "memory://snapshot/sysml.library/measurement_references.md") (path (named (kind library-package) (name "MeasurementReferences")) (named (kind attribute-def) (name "MeasurementUnit")) (anonymous (kind attribute) (ordinal 0))))))) (operator "and" (operator "not" (feature-reference "isBound" (target (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue::isBound"))))) (feature-reference "mRef::isBound" (target (node (document "memory://snapshot/sysml.library/measurement_references.md") (path (named (kind library-package) (name "MeasurementReferences")) (named (kind attribute-def) (name "MeasurementUnit")) (anonymous (kind attribute) (ordinal 0)))))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellTime"))) (element (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue::order"))) (outcome ambiguous) (candidates (node (document "memory://snapshot/sysml.library/quantities.md") (path (named (kind library-package) (name "Quantities")) (named (kind attribute-def) (name "TensorQuantityValue")) (anonymous (kind attribute) (ordinal 0)))) (node (document "memory://snapshot/sysml.library/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue::dimension"))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellTime"))) (element (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue::orderSum"))) (effective-element (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue::orderSum"))) (authored-element (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue::orderSum"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation non-constant) (operator "==" (operator "+" (feature-reference "contravariantOrder" (target (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue::contravariantOrder")))) (feature-reference "covariantOrder" (target (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue::covariantOrder"))))) (feature-reference "order" (target (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue::order"))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::engagementDiscrepancyTime"))) (element (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Array::flattenedSize"))) (effective-element (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Array::flattenedSize"))) (authored-element (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Array::flattenedSize"))) (outcome unsupported) (result-typing (status unsupported)) (evaluation unsupported))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::engagementDiscrepancyTime"))) (element (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Array::rank"))) (outcome ambiguous) (candidates (node (document "memory://snapshot/sysml.library/quantities.md") (path (named (kind library-package) (name "Quantities")) (named (kind attribute-def) (name "TensorQuantityValue")) (anonymous (kind attribute) (ordinal 0)))) (node (document "memory://snapshot/sysml.library/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue::dimension"))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::engagementDiscrepancyTime"))) (element (node (document "memory://snapshot/sysml.library/quantities.md") (path (named (kind library-package) (name "Quantities")) (named (kind attribute-def) (name "TensorQuantityValue")) (anonymous (kind attribute) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/quantities.md") (path (named (kind library-package) (name "Quantities")) (named (kind attribute-def) (name "TensorQuantityValue")) (anonymous (kind attribute) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/quantities.md") (path (named (kind library-package) (name "Quantities")) (named (kind attribute-def) (name "TensorQuantityValue")) (anonymous (kind attribute) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Positive")))) (evaluation non-constant) (feature-reference "mRef::dimensions" (target (node (document "memory://snapshot/sysml.library/measurement_references.md") (path (named (kind library-package) (name "MeasurementReferences")) (named (kind attribute-def) (name "ScalarMeasurementReference")) (anonymous (kind attribute) (ordinal 0)))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::engagementDiscrepancyTime"))) (element (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue::boundMatch"))) (effective-element (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue::boundMatch"))) (authored-element (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue::boundMatch"))) (outcome resolved) (result-typing (status unsupported)) (evaluation non-constant) (operator "or" (operator "==" (feature-reference "isBound" (target (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue::isBound")))) (feature-reference "mRef::isBound" (target (node (document "memory://snapshot/sysml.library/measurement_references.md") (path (named (kind library-package) (name "MeasurementReferences")) (named (kind attribute-def) (name "MeasurementUnit")) (anonymous (kind attribute) (ordinal 0))))))) (operator "and" (operator "not" (feature-reference "isBound" (target (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue::isBound"))))) (feature-reference "mRef::isBound" (target (node (document "memory://snapshot/sysml.library/measurement_references.md") (path (named (kind library-package) (name "MeasurementReferences")) (named (kind attribute-def) (name "MeasurementUnit")) (anonymous (kind attribute) (ordinal 0)))))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::engagementDiscrepancyTime"))) (element (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue::order"))) (outcome ambiguous) (candidates (node (document "memory://snapshot/sysml.library/quantities.md") (path (named (kind library-package) (name "Quantities")) (named (kind attribute-def) (name "TensorQuantityValue")) (anonymous (kind attribute) (ordinal 0)))) (node (document "memory://snapshot/sysml.library/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue::dimension"))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::engagementDiscrepancyTime"))) (element (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue::orderSum"))) (effective-element (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue::orderSum"))) (authored-element (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue::orderSum"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation non-constant) (operator "==" (operator "+" (feature-reference "contravariantOrder" (target (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue::contravariantOrder")))) (feature-reference "covariantOrder" (target (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue::covariantOrder"))))) (feature-reference "order" (target (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue::order"))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-rational) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-rational) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-rational) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-rational) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-rational) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-rational) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-rational) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-rational) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-rational) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-rational) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-rational) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-rational) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-rational) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-rational) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-rational) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::fullForce"))) (element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "that::this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::fullForce"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1)))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::fullForce"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "that::incomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::fullForce"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::fullForce"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::fullForce"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::Clock")))) (evaluation non-constant) (feature-reference "that::localClock" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::fullForce"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::fullForce")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::fullForce"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::fullForce")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::fullForce"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::Clock")))) (evaluation non-constant) (feature-reference "that::localClock" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::fullForce"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "that::incomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::fullForce"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "that::this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::fullForce"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::fullForce"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1)))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::fullForce"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "that::this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::fullForce"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "that::thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::fullForce"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "that::thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation evaluated) (value (kind boolean) (boolean true)) (feature-reference "this::isRunToCompletion" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "this::runToCompletionScope" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle"))) (element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation evaluated) (value (kind boolean) (boolean true)) (feature-reference "this::isRunToCompletion" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle"))) (element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "this::runToCompletionScope" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle"))) (element (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::incomingTransitionTrigger"))) (effective-element (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::incomingTransitionTrigger"))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::incomingTransitionTrigger"))) (outcome unsupported) (result-typing (status unsupported)) (evaluation unsupported))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle"))) (element (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::isTriggerDuring"))) (effective-element (node (document "memory://snapshot/sysml.library/states.md") (path (named (kind library-package) (name "States")) (named (kind state-def) (name "StateAction")) (anonymous (kind attribute) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::isTriggerDuring"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle"))) (element (node (document "memory://snapshot/sysml.library/states.md") (path (named (kind library-package) (name "States")) (named (kind state-def) (name "StateAction")) (anonymous (kind attribute) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/states.md") (path (named (kind library-package) (name "States")) (named (kind state-def) (name "StateAction")) (anonymous (kind attribute) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::isTriggerDuring"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation evaluated) (value (kind boolean) (boolean true)) (feature-reference "this::isRunToCompletion" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "this::runToCompletionScope" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving"))) (element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation evaluated) (value (kind boolean) (boolean true)) (feature-reference "this::isRunToCompletion" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving"))) (element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "this::runToCompletionScope" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving"))) (element (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::incomingTransitionTrigger"))) (effective-element (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::incomingTransitionTrigger"))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::incomingTransitionTrigger"))) (outcome unsupported) (result-typing (status unsupported)) (evaluation unsupported))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving"))) (element (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::isTriggerDuring"))) (effective-element (node (document "memory://snapshot/sysml.library/states.md") (path (named (kind library-package) (name "States")) (named (kind state-def) (name "StateAction")) (anonymous (kind attribute) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::isTriggerDuring"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving"))) (element (node (document "memory://snapshot/sysml.library/states.md") (path (named (kind library-package) (name "States")) (named (kind state-def) (name "StateAction")) (anonymous (kind attribute) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/states.md") (path (named (kind library-package) (name "States")) (named (kind state-def) (name "StateAction")) (anonymous (kind attribute) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::isTriggerDuring"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 1)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 1)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 1)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 1)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 2))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 2))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 2))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 2))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 2))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 2))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 2)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 2))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 2)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 2))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 2)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 2))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 2))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 2))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 2)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving::running"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving::running"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving::running"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving::running"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation evaluated) (value (kind boolean) (boolean true)) (feature-reference "this::isRunToCompletion" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving::running"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving::running"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving::running")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving::running"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "this::runToCompletionScope" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving::running"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving::running")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving::running"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving::running"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving::running"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving::running")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving::running"))) (element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation evaluated) (value (kind boolean) (boolean true)) (feature-reference "this::isRunToCompletion" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving::running"))) (element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "this::runToCompletionScope" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving::running"))) (element (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::incomingTransitionTrigger"))) (effective-element (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::incomingTransitionTrigger"))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::incomingTransitionTrigger"))) (outcome unsupported) (result-typing (status unsupported)) (evaluation unsupported))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving::running"))) (element (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::isTriggerDuring"))) (effective-element (node (document "memory://snapshot/sysml.library/states.md") (path (named (kind library-package) (name "States")) (named (kind state-def) (name "StateAction")) (anonymous (kind attribute) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::isTriggerDuring"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving::running"))) (element (node (document "memory://snapshot/sysml.library/states.md") (path (named (kind library-package) (name "States")) (named (kind state-def) (name "StateAction")) (anonymous (kind attribute) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/states.md") (path (named (kind library-package) (name "States")) (named (kind state-def) (name "StateAction")) (anonymous (kind attribute) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::isTriggerDuring"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 1)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 1)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 1)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 1)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::rememberLanding"))) (element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "that::this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::rememberLanding"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1)))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::rememberLanding"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "that::incomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::rememberLanding"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::rememberLanding"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::rememberLanding"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::Clock")))) (evaluation non-constant) (feature-reference "that::localClock" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::rememberLanding"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::rememberLanding")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::rememberLanding"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::rememberLanding")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::rememberLanding"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::Clock")))) (evaluation non-constant) (feature-reference "that::localClock" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::rememberLanding"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "that::incomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::rememberLanding"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "that::this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::rememberLanding"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::rememberLanding"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1)))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::rememberLanding"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "that::this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::rememberLanding"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "that::thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::rememberLanding"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "that::thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::startRun"))) (element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "that::this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::startRun"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1)))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::startRun"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "that::incomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::startRun"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::startRun"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::startRun"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::Clock")))) (evaluation non-constant) (feature-reference "that::localClock" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::startRun"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::startRun")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::startRun"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::startRun")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::startRun"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::Clock")))) (evaluation non-constant) (feature-reference "that::localClock" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::startRun"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "that::incomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::startRun"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "that::this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::startRun"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::startRun"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1)))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::startRun"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "that::this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::startRun"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "that::thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::startRun"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "that::thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::stopRun"))) (element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "that::this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::stopRun"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1)))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::stopRun"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "that::incomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::stopRun"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::stopRun"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::stopRun"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::Clock")))) (evaluation non-constant) (feature-reference "that::localClock" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::stopRun"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::stopRun")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::stopRun"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::stopRun")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::stopRun"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::Clock")))) (evaluation non-constant) (feature-reference "that::localClock" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::stopRun"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "that::incomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::stopRun"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "that::this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::stopRun"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::stopRun"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1)))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::stopRun"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "that::this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::stopRun"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "that::thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::stopRun"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "that::thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::externalStart"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::externalStart"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::externalStart"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::externalStart"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::externalStart"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::externalStart"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::externalStart")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::externalStart"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::externalStart")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::externalStart"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::externalStart")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::externalStart"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::externalStart"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::externalStart"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::externalStart")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::machine"))) (element (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item::isSolid"))) (effective-element (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item::isSolid"))) (authored-element (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item::isSolid"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation non-constant) (invocation (callee (feature-reference "isEmpty" (target (node (document "memory://snapshot/sysml.library/sequence_functions.md") (qualified-name "SequenceFunctions::isEmpty"))))) (arguments (feature-reference "voids" (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item::voids")))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::machine"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::machine")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::machine"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::machine"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean false)) (literal (value (kind boolean) (boolean false))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::machine"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::machine"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::machine"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::machine")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::machine"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::machine")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::machine"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::machine")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/model.sysml") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 13 20) (end 13 24)) (probe (position 13 20))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 24 25) (end 24 29)) (probe (position 24 25))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 24 62) (end 24 68)) (probe (position 24 62))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0) (authored-target "moving")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 24 25) (end 24 29)) (probe (position 24 25))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 24 62) (end 24 68)) (probe (position 24 62))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "moving")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 24 41) (end 24 56)) (probe (position 24 41))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "CommandFinished")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::CommandFinished")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 21 31) (end 21 35)) (probe (position 21 31))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin"))) (kind transitionSource) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 21 64) (end 21 70)) (probe (position 21 64))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::begin"))) (kind transitionTarget) (ordinal 0) (authored-target "moving")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 21 31) (end 21 35)) (probe (position 21 31))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 21 64) (end 21 70)) (probe (position 21 64))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "moving")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 21 47) (end 21 58)) (probe (position 21 47))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "begin")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "InvokeReset")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::InvokeReset")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 23 36) (end 23 42)) (probe (position 23 36))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::brakeStuck"))) (kind transitionSource) (ordinal 0) (authored-target "moving")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 23 87) (end 23 91)) (probe (position 23 87))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::brakeStuck"))) (kind transitionTarget) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 23 36) (end 23 42)) (probe (position 23 36))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "moving")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 23 87) (end 23 91)) (probe (position 23 87))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 23 56) (end 23 81)) (probe (position 23 56))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "brakeStuck")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "engagementDiscrepancyTime")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::engagementDiscrepancyTime")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 25 36) (end 25 42)) (probe (position 25 36))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::completion"))) (kind transitionSource) (ordinal 0) (authored-target "moving")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 25 48) (end 25 52)) (probe (position 25 48))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::completion"))) (kind transitionTarget) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 25 36) (end 25 42)) (probe (position 25 36))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "completion")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "moving")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 25 48) (end 25 52)) (probe (position 25 48))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "completion")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 22 38) (end 22 44)) (probe (position 22 38))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellExpired"))) (kind transitionSource) (ordinal 0) (authored-target "moving")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 22 82) (end 22 86)) (probe (position 22 82))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellExpired"))) (kind transitionTarget) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 22 38) (end 22 44)) (probe (position 22 38))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "moving")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::moving")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 22 82) (end 22 86)) (probe (position 22 82))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::idle")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 22 71) (end 22 76)) (probe (position 22 71))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "ready")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::ready")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 22 58) (end 22 67)) (probe (position 22 58))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind transition) (name "dwellExpired")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "dwellTime")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellTime")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 6 30) (end 6 48)) (probe (position 6 30))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::dwellTime"))) (kind featureTyping) (ordinal 0) (authored-target "ISQ::DurationValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 6 54) (end 6 59)) (probe (position 6 54))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "dwellTime")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind quantityUnit) (ordinal 0) (authored-target "SI::s")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/si.md") (qualified-name "SI::second")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 7 46) (end 7 64)) (probe (position 7 46))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::engagementDiscrepancyTime"))) (kind featureTyping) (ordinal 0) (authored-target "ISQ::DurationValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 7 72) (end 7 77)) (probe (position 7 72))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind attribute) (name "engagementDiscrepancyTime")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind quantityUnit) (ordinal 0) (authored-target "SI::s")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/si.md") (qualified-name "SI::second")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 16 18) (end 16 26)) (probe (position 16 18))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 0))))) (kind entryActionBinding) (ordinal 0) (authored-target "startRun")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::startRun")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 17 15) (end 17 30)) (probe (position 17 15))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 1))))) (kind doActionBinding) (ordinal 0) (authored-target "rememberLanding")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::rememberLanding")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 18 17) (end 18 24)) (probe (position 18 17))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (anonymous (kind perform-action) (ordinal 2))))) (kind exitActionBinding) (ordinal 0) (authored-target "stopRun")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::stopRun")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 19 34) (end 19 47)) (probe (position 19 34))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 0))))) (kind entryActionBinding) (ordinal 0) (authored-target "externalStart")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::externalStart")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 19 52) (end 19 61)) (probe (position 19 52))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind state-def) (name "Machine")) (named (kind state) (name "moving")) (named (kind state) (name "running")) (anonymous (kind perform-action) (ordinal 1))))) (kind doActionBinding) (ordinal 0) (authored-target "fullForce")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::fullForce")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 8 26) (end 8 47)) (probe (position 8 26))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine::ready"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::Boolean")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 27 19) (end 27 38)) (probe (position 27 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::machine"))) (kind featureTyping) (ordinal 0) (authored-target "StateTransitionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 27 48) (end 27 55)) (probe (position 27 48))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Behavior")) (named (kind view) (name "machine")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Machine")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Behavior::Machine")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 12,
  "modelDigest": "blake3:bf9c35578d64c7687d538798f62a0c78cc4358eebdb133bb9d76c19034665ac0",
  "documents": [
    {
      "uri": "memory://snapshot/model.sysml",
      "sourceDomain": "workspace"
    },
    {
      "uri": "memory://snapshot/sysml.library/isq_base.md",
      "sourceDomain": "standard-library"
    },
    {
      "uri": "memory://snapshot/sysml.library/scalar_values.md",
      "sourceDomain": "standard-library"
    }
  ],
  "sources": [
    {
      "document": 0,
      "range": [
        5,
        14,
        5,
        21
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        18,
        6,
        27
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        30,
        6,
        48
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        18,
        7,
        43
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        46,
        7,
        64
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        18,
        8,
        23
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        26,
        8,
        47
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        15,
        9,
        23
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        15,
        10,
        22
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        15,
        11,
        30
      ]
    },
    {
      "document": 0,
      "range": [
        12,
        15,
        12,
        24
      ]
    },
    {
      "document": 0,
      "range": [
        13,
        8,
        13,
        14
      ]
    },
    {
      "document": 0,
      "range": [
        13,
        15,
        13,
        25
      ]
    },
    {
      "document": 0,
      "range": [
        13,
        20,
        13,
        24
      ]
    },
    {
      "document": 0,
      "range": [
        14,
        14,
        14,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        15,
        14,
        15,
        20
      ]
    },
    {
      "document": 0,
      "range": [
        16,
        12,
        16,
        27
      ]
    },
    {
      "document": 0,
      "range": [
        16,
        18,
        16,
        26
      ]
    },
    {
      "document": 0,
      "range": [
        17,
        12,
        17,
        31
      ]
    },
    {
      "document": 0,
      "range": [
        17,
        15,
        17,
        30
      ]
    },
    {
      "document": 0,
      "range": [
        18,
        12,
        18,
        25
      ]
    },
    {
      "document": 0,
      "range": [
        18,
        17,
        18,
        24
      ]
    },
    {
      "document": 0,
      "range": [
        19,
        18,
        19,
        25
      ]
    },
    {
      "document": 0,
      "range": [
        19,
        28,
        19,
        48
      ]
    },
    {
      "document": 0,
      "range": [
        19,
        34,
        19,
        47
      ]
    },
    {
      "document": 0,
      "range": [
        19,
        49,
        19,
        62
      ]
    },
    {
      "document": 0,
      "range": [
        19,
        52,
        19,
        61
      ]
    },
    {
      "document": 0,
      "range": [
        21,
        8,
        21,
        71
      ]
    },
    {
      "document": 0,
      "range": [
        21,
        19,
        21,
        24
      ]
    },
    {
      "document": 0,
      "range": [
        21,
        31,
        21,
        35
      ]
    },
    {
      "document": 0,
      "range": [
        21,
        43,
        21,
        44
      ]
    },
    {
      "document": 0,
      "range": [
        21,
        47,
        21,
        58
      ]
    },
    {
      "document": 0,
      "range": [
        21,
        64,
        21,
        70
      ]
    },
    {
      "document": 0,
      "range": [
        22,
        8,
        22,
        87
      ]
    },
    {
      "document": 0,
      "range": [
        22,
        19,
        22,
        31
      ]
    },
    {
      "document": 0,
      "range": [
        22,
        38,
        22,
        44
      ]
    },
    {
      "document": 0,
      "range": [
        22,
        82,
        22,
        86
      ]
    },
    {
      "document": 0,
      "range": [
        23,
        8,
        23,
        92
      ]
    },
    {
      "document": 0,
      "range": [
        23,
        19,
        23,
        29
      ]
    },
    {
      "document": 0,
      "range": [
        23,
        36,
        23,
        42
      ]
    },
    {
      "document": 0,
      "range": [
        23,
        87,
        23,
        91
      ]
    },
    {
      "document": 0,
      "range": [
        24,
        8,
        24,
        69
      ]
    },
    {
      "document": 0,
      "range": [
        24,
        25,
        24,
        29
      ]
    },
    {
      "document": 0,
      "range": [
        24,
        37,
        24,
        38
      ]
    },
    {
      "document": 0,
      "range": [
        24,
        41,
        24,
        56
      ]
    },
    {
      "document": 0,
      "range": [
        24,
        62,
        24,
        68
      ]
    },
    {
      "document": 0,
      "range": [
        25,
        8,
        25,
        53
      ]
    },
    {
      "document": 0,
      "range": [
        25,
        19,
        25,
        29
      ]
    },
    {
      "document": 0,
      "range": [
        25,
        36,
        25,
        42
      ]
    },
    {
      "document": 0,
      "range": [
        25,
        48,
        25,
        52
      ]
    },
    {
      "document": 0,
      "range": [
        27,
        9,
        27,
        16
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Behavior::CommandFinished"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Behavior::InvokeReset"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Behavior::Machine"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Behavior::Machine::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Behavior::Machine::::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Behavior::Machine::::::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Behavior::Machine::::d"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Behavior::Machine::begin"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Behavior::Machine::begin::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Behavior::Machine::begin::::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Behavior::Machine::begin::c"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Behavior::Machine::brakeStuck"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Behavior::Machine::brakeStuck::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Behavior::Machine::completion"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Behavior::Machine::completion::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Behavior::Machine::dwellExpired"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Behavior::Machine::dwellExpired::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Behavior::Machine::dwellTime"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Behavior::Machine::engagementDiscrepancyTime"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Behavior::Machine::fullForce"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Behavior::Machine::idle"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Behavior::Machine::moving"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Behavior::Machine::moving::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Behavior::Machine::moving::running"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Behavior::Machine::moving::running::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Behavior::Machine::ready"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Behavior::Machine::rememberLanding"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Behavior::Machine::startRun"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Behavior::Machine::stopRun"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Behavior::externalStart"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Behavior::machine"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "ISQBase::DurationValue"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "ScalarValues::Boolean"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ActionUsage",
      "ownerQualifiedName": "Behavior::Machine",
      "source": 11,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Behavior::Machine",
      "source": 12,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "TransitionUsage",
      "ownerQualifiedName": "Behavior::Machine",
      "source": 41,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "AcceptActionUsage",
      "ownerQualifiedName": "Behavior::Machine::",
      "source": 41,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Behavior::Machine::",
      "source": 41,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Behavior::Machine::",
      "source": 41,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Behavior::Machine::::",
      "source": 41,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "AcceptActionUsage",
      "ownerQualifiedName": "Behavior::Machine::begin",
      "source": 27,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Behavior::Machine::begin",
      "source": 27,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Behavior::Machine::begin",
      "source": 27,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Behavior::Machine::begin::",
      "source": 27,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "AcceptActionUsage",
      "ownerQualifiedName": "Behavior::Machine::brakeStuck",
      "source": 37,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Behavior::Machine::brakeStuck",
      "source": 37,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Behavior::Machine::brakeStuck",
      "source": 37,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Behavior::Machine::brakeStuck::",
      "source": 37,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Behavior::Machine::completion",
      "source": 46,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Behavior::Machine::completion",
      "source": 46,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "AcceptActionUsage",
      "ownerQualifiedName": "Behavior::Machine::dwellExpired",
      "source": 33,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Behavior::Machine::dwellExpired",
      "source": 33,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Behavior::Machine::dwellExpired",
      "source": 33,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Behavior::Machine::dwellExpired::",
      "source": 33,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "PerformActionUsage",
      "ownerQualifiedName": "Behavior::Machine::moving",
      "source": 16,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "PerformActionUsage",
      "ownerQualifiedName": "Behavior::Machine::moving",
      "source": 18,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "PerformActionUsage",
      "ownerQualifiedName": "Behavior::Machine::moving",
      "source": 20,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "PerformActionUsage",
      "ownerQualifiedName": "Behavior::Machine::moving::running",
      "source": 23,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "PerformActionUsage",
      "ownerQualifiedName": "Behavior::Machine::moving::running",
      "source": 25,
      "sourceDomain": "workspace"
    },
    {
      "kind": "relationship",
      "ordinal": 0,
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
      "ordinal": 10,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 17,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 19,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 20,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 21,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 22,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 23,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 32,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 33,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 34,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 43,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 49,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 50,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 59,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "containment",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "containment",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "containment",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "containment",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "containment",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 18,
      "relationshipKind": "initialState",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 54,
      "relationshipKind": "initialState",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "transitionSource",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "transitionTarget",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "containment",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 12,
      "relationshipKind": "succession",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "succession",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 19,
      "relationshipKind": "typing",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 25,
      "relationshipKind": "containment",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 26,
      "relationshipKind": "containment",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 27,
      "relationshipKind": "containment",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 29,
      "relationshipKind": "containment",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 30,
      "relationshipKind": "containment",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 72,
      "relationshipKind": "transitionSource",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 73,
      "relationshipKind": "transitionTarget",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 31,
      "relationshipKind": "containment",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 80,
      "relationshipKind": "succession",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 81,
      "relationshipKind": "succession",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 87,
      "relationshipKind": "typing",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 36,
      "relationshipKind": "containment",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 37,
      "relationshipKind": "containment",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 38,
      "relationshipKind": "containment",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 39,
      "relationshipKind": "containment",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 41,
      "relationshipKind": "containment",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 100,
      "relationshipKind": "transitionSource",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 101,
      "relationshipKind": "transitionTarget",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 42,
      "relationshipKind": "containment",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 111,
      "relationshipKind": "succession",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 112,
      "relationshipKind": "succession",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 45,
      "relationshipKind": "containment",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 46,
      "relationshipKind": "containment",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 47,
      "relationshipKind": "containment",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 120,
      "relationshipKind": "transitionSource",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 121,
      "relationshipKind": "transitionTarget",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 128,
      "relationshipKind": "succession",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 129,
      "relationshipKind": "succession",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 52,
      "relationshipKind": "containment",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 53,
      "relationshipKind": "containment",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 54,
      "relationshipKind": "containment",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 55,
      "relationshipKind": "containment",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 57,
      "relationshipKind": "containment",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 136,
      "relationshipKind": "transitionSource",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 137,
      "relationshipKind": "transitionTarget",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 58,
      "relationshipKind": "containment",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 147,
      "relationshipKind": "succession",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 148,
      "relationshipKind": "succession",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 90,
      "relationshipKind": "typing",
      "source": 17
    },
    {
      "kind": "relationship",
      "ordinal": 153,
      "relationshipKind": "typing",
      "source": 18
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "succession",
      "source": 20
    },
    {
      "kind": "relationship",
      "ordinal": 28,
      "relationshipKind": "succession",
      "source": 20
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "transition",
      "source": 20
    },
    {
      "kind": "relationship",
      "ordinal": 24,
      "relationshipKind": "transition",
      "source": 20
    },
    {
      "kind": "relationship",
      "ordinal": 11,
      "relationshipKind": "containment",
      "source": 21
    },
    {
      "kind": "relationship",
      "ordinal": 14,
      "relationshipKind": "containment",
      "source": 21
    },
    {
      "kind": "relationship",
      "ordinal": 15,
      "relationshipKind": "containment",
      "source": 21
    },
    {
      "kind": "relationship",
      "ordinal": 16,
      "relationshipKind": "containment",
      "source": 21
    },
    {
      "kind": "relationship",
      "ordinal": 40,
      "relationshipKind": "succession",
      "source": 21
    },
    {
      "kind": "relationship",
      "ordinal": 48,
      "relationshipKind": "succession",
      "source": 21
    },
    {
      "kind": "relationship",
      "ordinal": 56,
      "relationshipKind": "succession",
      "source": 21
    },
    {
      "kind": "relationship",
      "ordinal": 35,
      "relationshipKind": "transition",
      "source": 21
    },
    {
      "kind": "relationship",
      "ordinal": 44,
      "relationshipKind": "transition",
      "source": 21
    },
    {
      "kind": "relationship",
      "ordinal": 51,
      "relationshipKind": "transition",
      "source": 21
    },
    {
      "kind": "relationship",
      "ordinal": 46,
      "relationshipKind": "doActionBinding",
      "source": 22
    },
    {
      "kind": "relationship",
      "ordinal": 42,
      "relationshipKind": "entryActionBinding",
      "source": 22
    },
    {
      "kind": "relationship",
      "ordinal": 50,
      "relationshipKind": "exitActionBinding",
      "source": 22
    },
    {
      "kind": "relationship",
      "ordinal": 12,
      "relationshipKind": "containment",
      "source": 23
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "containment",
      "source": 23
    },
    {
      "kind": "relationship",
      "ordinal": 38,
      "relationshipKind": "doActionBinding",
      "source": 24
    },
    {
      "kind": "relationship",
      "ordinal": 34,
      "relationshipKind": "entryActionBinding",
      "source": 24
    },
    {
      "kind": "relationship",
      "ordinal": 63,
      "relationshipKind": "typing",
      "source": 25
    }
  ],
  "selectedView": {
    "reference": 30,
    "kind": "state-transition-view",
    "name": "machine",
    "source": 50
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "edges": [
      {
        "kind": "containment",
        "navigation": 41,
        "origin": 1,
        "provenance": "authored",
        "reference": 59,
        "source": 0,
        "target": 1
      },
      {
        "kind": "transition",
        "navigation": 42,
        "origin": 1,
        "provenance": "implied",
        "reference": 130,
        "source": 35,
        "target": 36
      },
      {
        "kind": "containment",
        "navigation": 41,
        "origin": 6,
        "provenance": "authored",
        "reference": 75,
        "source": 1,
        "target": 6
      },
      {
        "kind": "containment",
        "navigation": 41,
        "origin": 6,
        "provenance": "authored",
        "reference": 76,
        "source": 1,
        "target": 6
      },
      {
        "kind": "containment",
        "navigation": 41,
        "origin": 2,
        "provenance": "authored",
        "reference": 77,
        "source": 1,
        "target": 2
      },
      {
        "kind": "succession",
        "navigation": 42,
        "origin": 2,
        "provenance": "implied",
        "reference": 128,
        "source": 35,
        "target": 36
      },
      {
        "kind": "containment",
        "navigation": 43,
        "origin": 7,
        "provenance": "authored",
        "reference": 78,
        "source": 1,
        "target": 7
      },
      {
        "kind": "containment",
        "navigation": 41,
        "origin": 3,
        "provenance": "authored",
        "reference": 79,
        "source": 1,
        "target": 3
      },
      {
        "kind": "containment",
        "navigation": 41,
        "origin": 4,
        "provenance": "authored",
        "reference": 84,
        "source": 3,
        "target": 4
      },
      {
        "kind": "containment",
        "navigation": 14,
        "origin": 35,
        "provenance": "authored",
        "reference": 60,
        "source": 0,
        "target": 35
      },
      {
        "kind": "containment",
        "navigation": 15,
        "origin": 36,
        "provenance": "authored",
        "reference": 61,
        "source": 0,
        "target": 36
      },
      {
        "kind": "containment",
        "navigation": 22,
        "origin": 40,
        "provenance": "authored",
        "reference": 132,
        "source": 36,
        "target": 40
      },
      {
        "kind": "containment",
        "navigation": 23,
        "origin": 41,
        "provenance": "authored",
        "reference": 145,
        "source": 40,
        "target": 41
      },
      {
        "kind": "containment",
        "navigation": 25,
        "origin": 42,
        "provenance": "authored",
        "reference": 146,
        "source": 40,
        "target": 42
      },
      {
        "kind": "containment",
        "navigation": 16,
        "origin": 37,
        "provenance": "authored",
        "reference": 133,
        "source": 36,
        "target": 37
      },
      {
        "kind": "containment",
        "navigation": 18,
        "origin": 38,
        "provenance": "authored",
        "reference": 134,
        "source": 36,
        "target": 38
      },
      {
        "kind": "containment",
        "navigation": 20,
        "origin": 39,
        "provenance": "authored",
        "reference": 135,
        "source": 36,
        "target": 39
      },
      {
        "kind": "containment",
        "navigation": 12,
        "origin": 33,
        "provenance": "authored",
        "reference": 62,
        "source": 0,
        "target": 33
      },
      {
        "kind": "initial-state",
        "navigation": 13,
        "origin": 33,
        "provenance": "authored",
        "reference": 80,
        "source": 33,
        "target": 35
      },
      {
        "kind": "containment",
        "navigation": 8,
        "origin": 44,
        "provenance": "authored",
        "reference": 63,
        "source": 0,
        "target": 44
      },
      {
        "kind": "containment",
        "navigation": 7,
        "origin": 45,
        "provenance": "authored",
        "reference": 64,
        "source": 0,
        "target": 45
      },
      {
        "kind": "containment",
        "navigation": 5,
        "origin": 48,
        "provenance": "authored",
        "reference": 65,
        "source": 0,
        "target": 48
      },
      {
        "kind": "containment",
        "navigation": 10,
        "origin": 46,
        "provenance": "authored",
        "reference": 66,
        "source": 0,
        "target": 46
      },
      {
        "kind": "containment",
        "navigation": 28,
        "origin": 26,
        "provenance": "authored",
        "reference": 67,
        "source": 0,
        "target": 26
      },
      {
        "kind": "transition",
        "navigation": 29,
        "origin": 26,
        "provenance": "implied",
        "reference": 131,
        "source": 35,
        "target": 36
      },
      {
        "kind": "containment",
        "navigation": 27,
        "origin": 31,
        "provenance": "authored",
        "reference": 88,
        "source": 26,
        "target": 31
      },
      {
        "kind": "containment",
        "navigation": 27,
        "origin": 31,
        "provenance": "authored",
        "reference": 89,
        "source": 26,
        "target": 31
      },
      {
        "kind": "containment",
        "navigation": 27,
        "origin": 27,
        "provenance": "authored",
        "reference": 90,
        "source": 26,
        "target": 27
      },
      {
        "kind": "succession",
        "navigation": 29,
        "origin": 27,
        "provenance": "implied",
        "reference": 129,
        "source": 35,
        "target": 36
      },
      {
        "kind": "containment",
        "navigation": 30,
        "origin": 32,
        "provenance": "authored",
        "reference": 91,
        "source": 26,
        "target": 32
      },
      {
        "kind": "containment",
        "navigation": 27,
        "origin": 28,
        "provenance": "authored",
        "reference": 92,
        "source": 26,
        "target": 28
      },
      {
        "kind": "containment",
        "navigation": 27,
        "origin": 29,
        "provenance": "authored",
        "reference": 95,
        "source": 28,
        "target": 29
      },
      {
        "kind": "containment",
        "navigation": 1,
        "origin": 49,
        "provenance": "authored",
        "reference": 68,
        "source": 0,
        "target": 49
      },
      {
        "kind": "containment",
        "navigation": 11,
        "origin": 34,
        "provenance": "authored",
        "reference": 69,
        "source": 0,
        "target": 34
      },
      {
        "kind": "containment",
        "navigation": 38,
        "origin": 8,
        "provenance": "authored",
        "reference": 70,
        "source": 0,
        "target": 8
      },
      {
        "kind": "transition",
        "navigation": 39,
        "origin": 8,
        "provenance": "implied",
        "reference": 139,
        "source": 36,
        "target": 35
      },
      {
        "kind": "containment",
        "navigation": 37,
        "origin": 14,
        "provenance": "authored",
        "reference": 99,
        "source": 8,
        "target": 14
      },
      {
        "kind": "containment",
        "navigation": 37,
        "origin": 14,
        "provenance": "authored",
        "reference": 100,
        "source": 8,
        "target": 14
      },
      {
        "kind": "containment",
        "navigation": 37,
        "origin": 14,
        "provenance": "authored",
        "reference": 101,
        "source": 8,
        "target": 14
      },
      {
        "kind": "containment",
        "navigation": 37,
        "origin": 9,
        "provenance": "authored",
        "reference": 102,
        "source": 8,
        "target": 9
      },
      {
        "kind": "succession",
        "navigation": 39,
        "origin": 9,
        "provenance": "implied",
        "reference": 136,
        "source": 36,
        "target": 35
      },
      {
        "kind": "containment",
        "navigation": 37,
        "origin": 10,
        "provenance": "authored",
        "reference": 103,
        "source": 8,
        "target": 10
      },
      {
        "kind": "containment",
        "navigation": 37,
        "origin": 11,
        "provenance": "authored",
        "reference": 106,
        "source": 10,
        "target": 11
      },
      {
        "kind": "containment",
        "navigation": 47,
        "origin": 15,
        "provenance": "authored",
        "reference": 71,
        "source": 0,
        "target": 15
      },
      {
        "kind": "transition",
        "navigation": 48,
        "origin": 15,
        "provenance": "implied",
        "reference": 140,
        "source": 36,
        "target": 35
      },
      {
        "kind": "containment",
        "navigation": 46,
        "origin": 18,
        "provenance": "authored",
        "reference": 109,
        "source": 15,
        "target": 18
      },
      {
        "kind": "containment",
        "navigation": 46,
        "origin": 18,
        "provenance": "authored",
        "reference": 110,
        "source": 15,
        "target": 18
      },
      {
        "kind": "containment",
        "navigation": 46,
        "origin": 16,
        "provenance": "authored",
        "reference": 111,
        "source": 15,
        "target": 16
      },
      {
        "kind": "succession",
        "navigation": 48,
        "origin": 16,
        "provenance": "implied",
        "reference": 137,
        "source": 36,
        "target": 35
      },
      {
        "kind": "containment",
        "navigation": 9,
        "origin": 43,
        "provenance": "authored",
        "reference": 72,
        "source": 0,
        "target": 43
      },
      {
        "kind": "containment",
        "navigation": 34,
        "origin": 19,
        "provenance": "authored",
        "reference": 73,
        "source": 0,
        "target": 19
      },
      {
        "kind": "transition",
        "navigation": 35,
        "origin": 19,
        "provenance": "implied",
        "reference": 141,
        "source": 36,
        "target": 35
      },
      {
        "kind": "containment",
        "navigation": 33,
        "origin": 25,
        "provenance": "authored",
        "reference": 116,
        "source": 19,
        "target": 25
      },
      {
        "kind": "containment",
        "navigation": 33,
        "origin": 25,
        "provenance": "authored",
        "reference": 117,
        "source": 19,
        "target": 25
      },
      {
        "kind": "containment",
        "navigation": 33,
        "origin": 25,
        "provenance": "authored",
        "reference": 118,
        "source": 19,
        "target": 25
      },
      {
        "kind": "containment",
        "navigation": 33,
        "origin": 20,
        "provenance": "authored",
        "reference": 119,
        "source": 19,
        "target": 20
      },
      {
        "kind": "succession",
        "navigation": 35,
        "origin": 20,
        "provenance": "implied",
        "reference": 138,
        "source": 36,
        "target": 35
      },
      {
        "kind": "containment",
        "navigation": 33,
        "origin": 21,
        "provenance": "authored",
        "reference": 120,
        "source": 19,
        "target": 21
      },
      {
        "kind": "containment",
        "navigation": 33,
        "origin": 22,
        "provenance": "authored",
        "reference": 123,
        "source": 21,
        "target": 22
      },
      {
        "kind": "containment",
        "navigation": 3,
        "origin": 47,
        "provenance": "authored",
        "reference": 74,
        "source": 0,
        "target": 47
      }
    ],
    "exposedRoots": [
      0
    ],
    "kind": "state-transition-view",
    "metadata": {
      "finalNodes": [],
      "initialNodes": [
        33
      ],
      "states": [
        0,
        35,
        36,
        40
      ]
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "attributes",
            "members": [
              47,
              48,
              49
            ],
            "provenance": "direct"
          },
          {
            "kind": "actions",
            "members": [
              34,
              43,
              44,
              45,
              46
            ],
            "provenance": "direct"
          },
          {
            "kind": "states",
            "members": [
              35,
              36
            ],
            "provenance": "direct"
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
        "presentationKey": "p:696a71580fbd6d4c0cc8fcfdac451a337a837240e8c53b58b1972a4c29886b6b",
        "reference": 2,
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
              6,
              6,
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
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:1b8ae0e082d0276c6ad976e95a03cc8ba7000e41186e7836081c40d4461b5629",
        "reference": 35,
        "source": 41,
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
        "owner": 1,
        "presentationKey": "p:7180d7c08cf581a7cdc6483b9739d5f7cf0b10d03fea67e2d469e41de7f5ed65",
        "reference": 38,
        "source": 41,
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
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": 1,
        "presentationKey": "p:84fda8930df4c27dc6430ad7fdef131e2d63b1ad391d0f3bbe2f1546c7c38e25",
        "reference": 36,
        "source": 41,
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
        "featureDirection": "inout",
        "metaclass": "ReferenceUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "reference-usage",
        "owner": 3,
        "presentationKey": "p:7b3fbd8c46e3a46dccfae53dcb02b61585acec5bc4d3ffd50366211ecc10943f",
        "reference": 39,
        "source": 41,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "CommandFinished",
              "reference": 0
            }
          ]
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
        "owner": 1,
        "presentationKey": "p:2bc194d019810ab50c3e9189ccd013df7e0d760d35bf729ea72c95c19ecbff20",
        "reference": 37,
        "source": 41,
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
        "featureDirection": "in",
        "metaclass": "ReferenceUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "reference-usage",
        "owner": 1,
        "presentationKey": "p:b0fabac9d2c2b69035abf42e90024942057a0d2c2ffbd93807cd1977194a7f29",
        "reference": 37,
        "source": 41,
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
        "featureDirection": "in",
        "metaclass": "ReferenceUsage",
        "multiplicity": null,
        "name": "d",
        "notationRole": "reference-usage",
        "owner": 1,
        "presentationKey": "p:a0fbdf7c6b38940a016eea56ee229dd6e4d46f17955c17ae00dd44577e28c9b4",
        "reference": 6,
        "source": 43,
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
        "multiplicity": null,
        "name": "brakeStuck",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:f6ff00ce9b2858a67cac62e98b276a6af5e0c8f50db002fc463d90b14361560e",
        "reference": 11,
        "source": 38,
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
        "owner": 8,
        "presentationKey": "p:efa71c9a79312835e23cc77488f2b75e33ac09ba3af258cb42e6be189eef7c08",
        "reference": 46,
        "source": 37,
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
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": 8,
        "presentationKey": "p:506e3e138187f88379e04a8b8af963d2cb91b2a2d1f33d89037239a3da8d0c9d",
        "reference": 44,
        "source": 37,
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
        "featureDirection": "inout",
        "metaclass": "ReferenceUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "reference-usage",
        "owner": 10,
        "presentationKey": "p:6bebdc9cd675421a142e36a168f473378006c997e446439a33af2739e9af8cbd",
        "reference": 47,
        "source": 37,
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
        "owner": 8,
        "presentationKey": "p:3ecb785539c1de4b4d844a64524208c4f3ec3f0a20f91bd2978c3c2ff79f5d84",
        "reference": 45,
        "source": 37,
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
        "featureDirection": "in",
        "metaclass": "ReferenceUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "reference-usage",
        "owner": 8,
        "presentationKey": "p:6840a15315f6b8731a50e052209bbfb35cd01da7f2facb819c9723f769a86236",
        "reference": 45,
        "source": 37,
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
        "featureDirection": "in",
        "metaclass": "ReferenceUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "reference-usage",
        "owner": 8,
        "presentationKey": "p:9856d32ff36751fc2634b10157575c3cc3d5feda606e6b7af7bf9c6ac58aa996",
        "reference": 45,
        "source": 37,
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
              18,
              18
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "TransitionUsage",
        "multiplicity": null,
        "name": "completion",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:f0011bdbbeb6f38e13ade320f6d8b209172dfd1f3d6d876b08593e0497f180fc",
        "reference": 13,
        "source": 47,
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
        "owner": 15,
        "presentationKey": "p:41556dfbbf55a95bb097ce46d72567e94bc64dc5607ea74df3aadfc0088d6110",
        "reference": 49,
        "source": 46,
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
        "owner": 15,
        "presentationKey": "p:5d562882830eec37645beb98ea3e9e82f417b55c8fafb18c7c8655cb3eeb8948",
        "reference": 48,
        "source": 46,
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
        "featureDirection": "in",
        "metaclass": "ReferenceUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "reference-usage",
        "owner": 15,
        "presentationKey": "p:5326ac49034cc04c740966c3d91db9eb1c85a1b3ad55f750aed029ef73b07fe4",
        "reference": 48,
        "source": 46,
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
              25,
              25,
              25
            ],
            "provenance": "direct"
          },
          {
            "kind": "actions",
            "members": [
              21
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "TransitionUsage",
        "multiplicity": null,
        "name": "dwellExpired",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:11ae52cad96e7f14f32ad64e6db747dcc5120850a6fb81ebc8467bdbcf05742e",
        "reference": 15,
        "source": 34,
        "stateSubaction": null,
        "successionGuard": "ready",
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
        "owner": 19,
        "presentationKey": "p:2f8dd83aa8b745bdbd3410bed5bbbd286e72dd996cd076908b5102c12add6e90",
        "reference": 52,
        "source": 33,
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
              22
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
        "owner": 19,
        "presentationKey": "p:36eb9a139ab578d1dad0f13a497c309a7e292d880ec862cc32a7bc24016f3de9",
        "reference": 50,
        "source": 33,
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
        "featureDirection": "inout",
        "metaclass": "ReferenceUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "reference-usage",
        "owner": 21,
        "presentationKey": "p:77a7291f38ddbf311ba55bc40ddfa7270c8e46d79ef4e28b4a8f1c60c6ecebcc",
        "reference": 53,
        "source": 33,
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
        "owner": 19,
        "presentationKey": "p:16c646d4cb839f806121f6a58d2064c3b21a4147c4f5488c66519a132665ff24",
        "reference": 51,
        "source": 33,
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
        "featureDirection": "in",
        "metaclass": "ReferenceUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "reference-usage",
        "owner": 19,
        "presentationKey": "p:5536750b69c5a2c0c5fdafd1f45b538db7669c1cfd0b2cf4f0c1ca06684855ea",
        "reference": 51,
        "source": 33,
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
        "featureDirection": "in",
        "metaclass": "ReferenceUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "reference-usage",
        "owner": 19,
        "presentationKey": "p:a880e588ec561fec0eca7397bd446f08c045d1c9934a3f563532843b99551b93",
        "reference": 51,
        "source": 33,
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
              31,
              31,
              32
            ],
            "provenance": "direct"
          },
          {
            "kind": "actions",
            "members": [
              28
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "TransitionUsage",
        "multiplicity": null,
        "name": "begin",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:af0177a6e4e743c71d24d3190d81c3e8cf7504cbc8d3272967383076859e9ade",
        "reference": 7,
        "source": 28,
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
        "owner": 26,
        "presentationKey": "p:b9ba3a11a2d4bc6270a289c4a98e0e05f95493b00c73a61bc28b417a3a783650",
        "reference": 42,
        "source": 27,
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
              29
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
        "owner": 26,
        "presentationKey": "p:b00070a7309579be9b07785bba3b2eb9b51e4e73655f7adcb22cbca108499a66",
        "reference": 40,
        "source": 27,
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
        "featureDirection": "inout",
        "metaclass": "ReferenceUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "reference-usage",
        "owner": 28,
        "presentationKey": "p:46adc6b696af5811937dc2024b24e36d0955c5205e7514a28cfd478469b8dec9",
        "reference": 43,
        "source": 27,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "InvokeReset",
              "reference": 1
            }
          ]
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
        "owner": 26,
        "presentationKey": "p:5c31d808e570f156839be77f45e9482aad4901f4c37e8b0929df85c9e94fea5c",
        "reference": 41,
        "source": 27,
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
        "featureDirection": "in",
        "metaclass": "ReferenceUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "reference-usage",
        "owner": 26,
        "presentationKey": "p:cf2a7c03ae4e3f99bb306e74f2cf8d6230df722e442d4d8307d5da714f9ea678",
        "reference": 41,
        "source": 27,
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
        "featureDirection": "in",
        "metaclass": "ReferenceUsage",
        "multiplicity": null,
        "name": "c",
        "notationRole": "reference-usage",
        "owner": 26,
        "presentationKey": "p:fb4f57c105382229aa250c01b16a2fd60b8d1e8d4ad6817e825b33449e7dfd32",
        "reference": 10,
        "source": 30,
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
        "presentationKey": "p:fb7e3d225f834f98de5f45f6dce2efafc76d08fa3765a204f49fef291de44360",
        "reference": 34,
        "source": 12,
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
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:267a2cfd4267b0e78e2e8085365329bcdd8b7fe3a3b2cd1f444b19283f32a58d",
        "reference": 33,
        "source": 11,
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
        "multiplicity": null,
        "name": "idle",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:3624b5e190f8784957c20d65ccd896a8783cf39f125d0d5000e7d7918dd7b421",
        "reference": 20,
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
            "kind": "actions",
            "members": [
              37,
              38,
              39
            ],
            "provenance": "direct"
          },
          {
            "kind": "states",
            "members": [
              40
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "StateUsage",
        "multiplicity": null,
        "name": "moving",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:49c3465a605ac9663e5e7ab565787ed8f8e41ea4429fd4817ea61fff5d4ab66b",
        "reference": 21,
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
        "metaclass": "PerformActionUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": 36,
        "presentationKey": "p:7bc8746fcc65f70761a3ef6050734f4cd3b6cd661cae924d47be0e569e6ef4eb",
        "reference": 54,
        "source": 16,
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
        "metaclass": "PerformActionUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": 36,
        "presentationKey": "p:5f73c758d3b92fb0b57039cb957f1df68c7331e56694dcd2fcd42fe114123638",
        "reference": 55,
        "source": 18,
        "stateSubaction": "do",
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
        "metaclass": "PerformActionUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": 36,
        "presentationKey": "p:e798d91c195537405946f184f30a76c898fb058d751b4753c3e317dc69106372",
        "reference": 56,
        "source": 20,
        "stateSubaction": "exit",
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
              41,
              42
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "StateUsage",
        "multiplicity": null,
        "name": "running",
        "notationRole": "usage",
        "owner": 36,
        "presentationKey": "p:8c5aff5d3e71ebcf260afe6e3de2adf66f071949124d9f103d52358e5f2ee737",
        "reference": 23,
        "source": 22,
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
        "metaclass": "PerformActionUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": 40,
        "presentationKey": "p:6af1f435573a12c835094d0c7a912cd2ed9d06cd32b8e46a24834ec7e4f18550",
        "reference": 57,
        "source": 23,
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
        "metaclass": "PerformActionUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": 40,
        "presentationKey": "p:65c4541fc4cdbf26ec64e7390b411c50671304399b339a1674f63382a1715826",
        "reference": 58,
        "source": 25,
        "stateSubaction": "do",
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
        "multiplicity": null,
        "name": "rememberLanding",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:ab70f98e2bc1285358e1df4fe289a237e87c9da6dc65de68badaa2a7e6b96a69",
        "reference": 26,
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
        "metaclass": "ActionUsage",
        "multiplicity": null,
        "name": "stopRun",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:fe4690e44898a6a3b579c3164fb31d266178a7c0739bf36e336e960509e1143b",
        "reference": 28,
        "source": 8,
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
        "multiplicity": null,
        "name": "startRun",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:0e87163f4f61b50e086f314147ed7fbe4604d4042a6947a7fdfc0a2916c277c9",
        "reference": 27,
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
        "multiplicity": null,
        "name": "fullForce",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:879337adb17ac17d08b16e0c92574f0303341481e92a5b708d922b497fa74b46",
        "reference": 19,
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
        "metaclass": "AttributeUsage",
        "multiplicity": null,
        "name": "engagementDiscrepancyTime",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:8c1a739155c25d5f5ed10b72c2869244e53f9dba50991bee1501b70f449af5a0",
        "reference": 18,
        "source": 3,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "DurationValue",
              "reference": 31
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "AttributeUsage",
        "multiplicity": null,
        "name": "ready",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:a6e7ae5dedaee8b6935973dc6d46f5744fc85a6b42e20a2d597fc59bec420f51",
        "reference": 25,
        "source": 5,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Boolean",
              "reference": 32
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "AttributeUsage",
        "multiplicity": null,
        "name": "dwellTime",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:ba02bb6bbac85fa07baddcc55162db49eaa6f401dee26e1a9ae56e14ab01411e",
        "reference": 17,
        "source": 1,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "DurationValue",
              "reference": 31
            }
          ]
        }
      }
    ],
    "relationships": [
      {
        "kind": "transitionSource",
        "navigation": 42,
        "provenance": "authored",
        "reference": 82,
        "source": 1,
        "target": {
          "node": 35,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTarget",
        "navigation": 45,
        "provenance": "authored",
        "reference": 83,
        "source": 1,
        "target": {
          "node": 36,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 42,
        "provenance": "authored",
        "reference": 85,
        "source": 2,
        "target": {
          "node": 35,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 45,
        "provenance": "authored",
        "reference": 86,
        "source": 2,
        "target": {
          "node": 36,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 44,
        "provenance": "authored",
        "reference": 87,
        "source": 4,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "entryActionBinding",
        "navigation": 24,
        "provenance": "authored",
        "reference": 148,
        "source": 41,
        "target": {
          "reference": 29,
          "status": "resolved"
        }
      },
      {
        "kind": "doActionBinding",
        "navigation": 26,
        "provenance": "authored",
        "reference": 147,
        "source": 42,
        "target": {
          "node": 46,
          "status": "resolved"
        }
      },
      {
        "kind": "entryActionBinding",
        "navigation": 17,
        "provenance": "authored",
        "reference": 143,
        "source": 37,
        "target": {
          "node": 45,
          "status": "resolved"
        }
      },
      {
        "kind": "doActionBinding",
        "navigation": 19,
        "provenance": "authored",
        "reference": 142,
        "source": 38,
        "target": {
          "node": 43,
          "status": "resolved"
        }
      },
      {
        "kind": "exitActionBinding",
        "navigation": 21,
        "provenance": "authored",
        "reference": 144,
        "source": 39,
        "target": {
          "node": 44,
          "status": "resolved"
        }
      },
      {
        "kind": "initialState",
        "navigation": 13,
        "provenance": "authored",
        "reference": 81,
        "source": 33,
        "target": {
          "node": 35,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 6,
        "provenance": "authored",
        "reference": 149,
        "source": 48,
        "target": {
          "reference": 32,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionSource",
        "navigation": 29,
        "provenance": "authored",
        "reference": 93,
        "source": 26,
        "target": {
          "node": 35,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTarget",
        "navigation": 32,
        "provenance": "authored",
        "reference": 94,
        "source": 26,
        "target": {
          "node": 36,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 29,
        "provenance": "authored",
        "reference": 96,
        "source": 27,
        "target": {
          "node": 35,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 32,
        "provenance": "authored",
        "reference": 97,
        "source": 27,
        "target": {
          "node": 36,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 31,
        "provenance": "authored",
        "reference": 98,
        "source": 29,
        "target": {
          "reference": 1,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 2,
        "provenance": "authored",
        "reference": 126,
        "source": 49,
        "target": {
          "reference": 31,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionSource",
        "navigation": 39,
        "provenance": "authored",
        "reference": 104,
        "source": 8,
        "target": {
          "node": 36,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTarget",
        "navigation": 40,
        "provenance": "authored",
        "reference": 105,
        "source": 8,
        "target": {
          "node": 35,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 39,
        "provenance": "authored",
        "reference": 107,
        "source": 9,
        "target": {
          "node": 36,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 40,
        "provenance": "authored",
        "reference": 108,
        "source": 9,
        "target": {
          "node": 35,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionSource",
        "navigation": 48,
        "provenance": "authored",
        "reference": 112,
        "source": 15,
        "target": {
          "node": 36,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTarget",
        "navigation": 49,
        "provenance": "authored",
        "reference": 113,
        "source": 15,
        "target": {
          "node": 35,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 48,
        "provenance": "authored",
        "reference": 114,
        "source": 16,
        "target": {
          "node": 36,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 49,
        "provenance": "authored",
        "reference": 115,
        "source": 16,
        "target": {
          "node": 35,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionSource",
        "navigation": 35,
        "provenance": "authored",
        "reference": 121,
        "source": 19,
        "target": {
          "node": 36,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTarget",
        "navigation": 36,
        "provenance": "authored",
        "reference": 122,
        "source": 19,
        "target": {
          "node": 35,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 35,
        "provenance": "authored",
        "reference": 124,
        "source": 20,
        "target": {
          "node": 36,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 36,
        "provenance": "authored",
        "reference": 125,
        "source": 20,
        "target": {
          "node": 35,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 4,
        "provenance": "authored",
        "reference": 127,
        "source": 47,
        "target": {
          "reference": 31,
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
          "navigation": 42,
          "provenance": "implied",
          "source": 1,
          "target": 2,
          "trigger": {
            "label": "CommandFinished",
            "navigation": 41,
            "status": "accept",
            "target": {
              "id": "element/v129:memory://snapshot/model.sysml7:packagen8:Behavior1:013:attribute-defn15:CommandFinished1:0",
              "label": "CommandFinished"
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
          "id": "transition-1",
          "label": null,
          "navigation": 13,
          "provenance": "authored",
          "source": 0,
          "target": 1,
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
          "label": "begin",
          "navigation": 29,
          "provenance": "implied",
          "source": 1,
          "target": 2,
          "trigger": {
            "label": "InvokeReset",
            "navigation": 27,
            "status": "accept",
            "target": {
              "id": "element/v129:memory://snapshot/model.sysml7:packagen8:Behavior1:013:attribute-defn11:InvokeReset1:0",
              "label": "InvokeReset"
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
          "id": "transition-3",
          "label": "brakeStuck",
          "navigation": 39,
          "provenance": "implied",
          "source": 2,
          "target": 1,
          "trigger": {
            "label": "after engagementDiscrepancyTime",
            "navigation": 37,
            "status": "accept",
            "target": {
              "id": "element/v129:memory://snapshot/model.sysml7:packagen8:Behavior1:09:state-defn7:Machine1:010:transitionn10:brakeStuck1:013:accept-actiona1:016:kerml-expressiona1:0",
              "label": "after engagementDiscrepancyTime"
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
          "label": "completion",
          "navigation": 48,
          "provenance": "implied",
          "source": 2,
          "target": 1,
          "trigger": {
            "status": "absent"
          }
        },
        {
          "effect": {
            "status": "absent"
          },
          "guard": {
            "label": "ready",
            "navigation": 34,
            "status": "supported"
          },
          "id": "transition-5",
          "label": "dwellExpired",
          "navigation": 35,
          "provenance": "implied",
          "source": 2,
          "target": 1,
          "trigger": {
            "label": "after dwellTime",
            "navigation": 33,
            "status": "accept",
            "target": {
              "id": "element/v129:memory://snapshot/model.sysml7:packagen8:Behavior1:09:state-defn7:Machine1:010:transitionn12:dwellExpired1:013:accept-actiona1:016:kerml-expressiona1:0",
              "label": "after dwellTime"
            }
          }
        }
      ],
      "vertices": [
        {
          "id": "state-0",
          "kind": "initial",
          "label": "",
          "navigation": 12
        },
        {
          "id": "state-1",
          "kind": "state",
          "label": "idle",
          "navigation": 14
        },
        {
          "id": "state-2",
          "kind": "state",
          "label": "moving",
          "navigation": 15
        },
        {
          "id": "state-3",
          "kind": "state",
          "label": "running",
          "navigation": 22
        }
      ]
    }
  }
}

~~~
