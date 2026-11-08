# META
~~~ini
description=A transition whose effect computes with a quantity is drawn: the unit reference has no diagram notation and is not a diagram relationship
type=generate
libraries=standard
plugin=native:diagram
viewKind=state-transition-view
viewDocument=model.sysml
viewQualifiedName=Timer::countdown
~~~
# SOURCE
## model.sysml
~~~sysml
package Timer {
    private import StandardViewDefinitions::*;
    private import ISQ::*;
    private import SI::*;
    attribute def Start;
    state def Countdown {
        attribute remaining : DurationValue := 0 [s];
        entry; then idle;
        state idle;
        state running;
        transition start first idle accept Start then running;
        transition bump first idle accept Start do action { assign remaining := remaining + 60 [s]; } then idle;
    }
    view countdown : StateTransitionView { expose Countdown; }
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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:fa02abd1d99f34626ba1e72e0c08bbf4c376c4b36b6c7b89d401bc08bee17903") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Timer"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQ") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "SI") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entry-action-binding) (membership (kind feature) (visibility default) (role entry)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (anonymous (kind initial-state) (ordinal 0))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "idle")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "idle")) (transitionTarget (reference "idle")) (transitionTrigger (reference "Start")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (succession (reference "idle")) (succession (reference "idle")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind parameter) (ordinal 1))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0))))) (kind perform-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction inout)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (kind assign) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "remaining")) (quantityUnit (reference "s")) (assignTarget (reference "remaining")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::remaining"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind assign) (value (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DurationValue")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (quantityUnit (reference "s")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::running"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "idle")) (transitionTarget (reference "running")) (transitionTrigger (reference "Start")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (succession (reference "idle")) (succession (reference "running")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind parameter) (ordinal 1))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction inout)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Start"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::countdown"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StateTransitionView")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind view) (name "countdown")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Countdown")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQ")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq.md") (qualified-name "ISQ")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "SI")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/si.md") (qualified-name "SI")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump"))) (kind transitionSource) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump"))) (kind transitionTarget) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump"))) (kind transitionTrigger) (ordinal 0))
      (authored-target "Start")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Start")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "remaining")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::remaining")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (kind quantityUnit) (ordinal 0))
      (authored-target "s")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/si.md") (qualified-name "SI::second")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (kind assignTarget) (ordinal 0))
      (authored-target "remaining")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::remaining")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::remaining"))) (kind featureTyping) (ordinal 0))
      (authored-target "DurationValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind quantityUnit) (ordinal 0))
      (authored-target "s")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/si.md") (qualified-name "SI::second")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start"))) (kind transitionSource) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start"))) (kind transitionTarget) (ordinal 0))
      (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::running")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start"))) (kind transitionTrigger) (ordinal 0))
      (authored-target "Start")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Start")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::running")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::countdown"))) (kind featureTyping) (ordinal 0))
      (authored-target "StateTransitionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind view) (name "countdown")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Countdown")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown")))))
  )
  (relationships
    (relationship (kind initialState) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Start"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump"))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::remaining"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind assignTarget) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::remaining"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (kind assignTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::remaining"))) (target (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::remaining"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::running"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Start"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start"))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::running"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::countdown"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::countdown"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind view) (name "countdown")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind view) (name "countdown")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::entryAction"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::stateTransitions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown"))) (provenance implied))
    (relationship (kind transitionEffect) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump"))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::accepter"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::effect"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::transitionLinkSource"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::payload"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::transitionLinkSource"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::assignments"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::assignmentActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind accept-action) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::assignments::target"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::assignmentActions::target"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalIntegerEvaluations"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance::onOccurrence::startingAt"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance::onOccurrence::startingAt::accessedFeature"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::remaining"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::exclusiveStates"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::remaining"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::remaining"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation::result"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalIntegerEvaluations"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::running"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::running"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::exclusiveStates"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::running"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::running"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::stateTransitions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::accepter"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::transitionLinkSource"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::payload"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::transitionLinkSource"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0))))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Start"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::countdown"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind view) (name "countdown")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::countdown"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (state evaluated) (value (kind quantity) (magnitude (value (kind integer) (integer 60))) (unit "s")))
    (evaluated (declaration (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0))))) (state literal) (value (kind quantity) (magnitude (value (kind integer) (integer 0))) (unit "s")))
    (unit (declaration (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (ordinal 0) (authored "s") (start 11 96) (end 11 97) (outcome (status resolved) (unit (node (document "memory://snapshot/sysml.library/si.md") (qualified-name "SI::second"))) (dimension (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationUnit")))))
    (unit (declaration (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0))))) (ordinal 0) (authored "s") (start 6 50) (end 6 51) (outcome (status resolved) (unit (node (document "memory://snapshot/sysml.library/si.md") (qualified-name "SI::second"))) (dimension (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationUnit")))))
    (measurement (declaration (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0))))) (status required) (dimension (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationUnit"))))
    (measurement (declaration (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::remaining"))) (status required) (dimension (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationUnit"))))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::DecisionPerformance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (anonymous (kind entry-action-binding) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (anonymous (kind initial-state) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind succession) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump")))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind parameter) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::payload")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::payload")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::effect"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::effect")) (scopes any feature))
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
      (supertype (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::effect")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind accept-action) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::payload")) (scopes any feature))
      (subtype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind parameter) (ordinal 1)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::AssignmentAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::assignments"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::AssignmentAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::assignmentActions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::assignments")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::AssignmentAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::assignmentActions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureReferencingPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureWritePerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::AssignmentAction::target"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::assignmentActions::target"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureReferencingPerformance::onOccurrence"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance::onOccurrence"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureWritePerformance::onOccurrence"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::assignments::target")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::AssignmentAction::target")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::assignmentActions::target")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance::onOccurrence")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureReferencingPerformance::onOccurrence")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureWritePerformance::onOccurrence")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance::onOccurrence::startingAt"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSlices"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance::onOccurrence::startingAt")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::spaceTimeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeSlices")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance::onOccurrence::startingAt::accessedFeature"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (effective-type (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")) (source inherited) (from (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::remaining"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::remaining")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Array")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Collection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::OrderedCollection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance::onOccurrence::startingAt::accessedFeature")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::ScalarQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::VectorQuantityValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/vector_values.md") (qualified-name "VectorValues::VectorValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::remaining")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown")))
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
      (subtype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation::result"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation::result")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::running")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind succession) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start")))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind parameter) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::payload")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::payload")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::payload")) (scopes any feature))
      (subtype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind parameter) (ordinal 1)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Start")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::countdown")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind view) (name "countdown")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::countdown")))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (outcome resolved) (result-typing (status unresolved)) (operator "+" (feature-reference "remaining" (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::remaining")))) (literal (value (kind quantity) (magnitude (value (kind integer) (integer 60))) (unit "s")))))
  (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (result-typing (status unsupported)) (literal (value (kind quantity) (magnitude (value (kind integer) (integer 0))) (unit "s"))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation evaluated) (value (kind boolean) (boolean true)) (feature-reference "this::isRunToCompletion" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "this::runToCompletionScope" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown"))) (element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation evaluated) (value (kind boolean) (boolean true)) (feature-reference "this::isRunToCompletion" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown"))) (element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "this::runToCompletionScope" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown"))) (element (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::incomingTransitionTrigger"))) (effective-element (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::incomingTransitionTrigger"))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::incomingTransitionTrigger"))) (outcome unsupported) (result-typing (status unsupported)) (evaluation unsupported))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown"))) (element (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::isTriggerDuring"))) (effective-element (node (document "memory://snapshot/sysml.library/states.md") (path (named (kind library-package) (name "States")) (named (kind state-def) (name "StateAction")) (anonymous (kind attribute) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::isTriggerDuring"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown"))) (element (node (document "memory://snapshot/sysml.library/states.md") (path (named (kind library-package) (name "States")) (named (kind state-def) (name "StateAction")) (anonymous (kind attribute) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/states.md") (path (named (kind library-package) (name "States")) (named (kind state-def) (name "StateAction")) (anonymous (kind attribute) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::isTriggerDuring"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (anonymous (kind entry-action-binding) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (anonymous (kind entry-action-binding) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (anonymous (kind entry-action-binding) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (anonymous (kind entry-action-binding) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (anonymous (kind entry-action-binding) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (anonymous (kind entry-action-binding) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (anonymous (kind entry-action-binding) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (anonymous (kind entry-action-binding) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (anonymous (kind entry-action-binding) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (anonymous (kind entry-action-binding) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (anonymous (kind entry-action-binding) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (anonymous (kind entry-action-binding) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (anonymous (kind entry-action-binding) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (anonymous (kind entry-action-binding) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (anonymous (kind entry-action-binding) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump"))) (element (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::receiver"))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::receiver"))) (authored-element (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::triggerTarget"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump"))) (element (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::accNum"))) (effective-element (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::accNum"))) (authored-element (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::accNum"))) (outcome unsupported) (result-typing (status unsupported)) (evaluation unsupported))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump"))) (element (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::triggerTarget"))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::receiver"))) (authored-element (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::triggerTarget"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "that::this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1)))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "that::incomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::Clock")))) (evaluation non-constant) (feature-reference "that::localClock" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind accept-action) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind accept-action) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::Clock")))) (evaluation non-constant) (feature-reference "that::localClock" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "that::incomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "that::this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1)))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "that::this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "that::thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "that::thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::receiver"))) (effective-element (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::receiver"))) (authored-element (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::receiver"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "this" (target (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0)))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/transition_performances.md") (path (named (kind library-package) (name "TransitionPerformances")) (named (kind kerml-behavior) (name "TransitionPerformance")) (named (kind kerml-step) (name "accept")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/transition_performances.md") (path (named (kind library-package) (name "TransitionPerformances")) (named (kind kerml-behavior) (name "TransitionPerformance")) (named (kind kerml-step) (name "accept")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/transition_performances.md") (path (named (kind library-package) (name "TransitionPerformances")) (named (kind kerml-behavior) (name "TransitionPerformance")) (named (kind kerml-step) (name "accept")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::MessageTransfer")))) (evaluation non-constant) (feature-reference "trigger" (target (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::trigger")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::assignments::target"))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::assignments::target"))) (authored-element (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::assignmentActions::target"))) (outcome resolved) (result-typing (status unsupported)) (evaluation non-constant) (unsupported (feature-reference "that" (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things::that"))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "that::this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::assignmentActions::target"))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::assignments::target"))) (authored-element (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::assignmentActions::target"))) (outcome resolved) (result-typing (status unsupported)) (evaluation non-constant) (unsupported (feature-reference "that" (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things::that"))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1)))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "that::incomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::Clock")))) (evaluation non-constant) (feature-reference "that::localClock" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::Clock")))) (evaluation non-constant) (feature-reference "that::localClock" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "that::incomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "that::this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1)))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "that::this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "that::thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "that::thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation evaluated) (value (kind boolean) (boolean true)) (feature-reference "this::isRunToCompletion" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "this::runToCompletionScope" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle"))) (element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation evaluated) (value (kind boolean) (boolean true)) (feature-reference "this::isRunToCompletion" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle"))) (element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "this::runToCompletionScope" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle"))) (element (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::incomingTransitionTrigger"))) (effective-element (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::incomingTransitionTrigger"))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::incomingTransitionTrigger"))) (outcome unsupported) (result-typing (status unsupported)) (evaluation unsupported))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle"))) (element (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::isTriggerDuring"))) (effective-element (node (document "memory://snapshot/sysml.library/states.md") (path (named (kind library-package) (name "States")) (named (kind state-def) (name "StateAction")) (anonymous (kind attribute) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::isTriggerDuring"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle"))) (element (node (document "memory://snapshot/sysml.library/states.md") (path (named (kind library-package) (name "States")) (named (kind state-def) (name "StateAction")) (anonymous (kind attribute) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/states.md") (path (named (kind library-package) (name "States")) (named (kind state-def) (name "StateAction")) (anonymous (kind attribute) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::isTriggerDuring"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::remaining"))) (element (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Array::flattenedSize"))) (effective-element (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Array::flattenedSize"))) (authored-element (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Array::flattenedSize"))) (outcome unsupported) (result-typing (status unsupported)) (evaluation unsupported))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::remaining"))) (element (node (document "memory://snapshot/sysml.library/collections.md") (qualified-name "Collections::Array::rank"))) (outcome ambiguous) (candidates (node (document "memory://snapshot/sysml.library/quantities.md") (path (named (kind library-package) (name "Quantities")) (named (kind attribute-def) (name "TensorQuantityValue")) (anonymous (kind attribute) (ordinal 0)))) (node (document "memory://snapshot/sysml.library/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue::dimension"))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::remaining"))) (element (node (document "memory://snapshot/sysml.library/quantities.md") (path (named (kind library-package) (name "Quantities")) (named (kind attribute-def) (name "TensorQuantityValue")) (anonymous (kind attribute) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/quantities.md") (path (named (kind library-package) (name "Quantities")) (named (kind attribute-def) (name "TensorQuantityValue")) (anonymous (kind attribute) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/quantities.md") (path (named (kind library-package) (name "Quantities")) (named (kind attribute-def) (name "TensorQuantityValue")) (anonymous (kind attribute) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Positive")))) (evaluation non-constant) (feature-reference "mRef::dimensions" (target (node (document "memory://snapshot/sysml.library/measurement_references.md") (path (named (kind library-package) (name "MeasurementReferences")) (named (kind attribute-def) (name "ScalarMeasurementReference")) (anonymous (kind attribute) (ordinal 0)))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::remaining"))) (element (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue::boundMatch"))) (effective-element (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue::boundMatch"))) (authored-element (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue::boundMatch"))) (outcome resolved) (result-typing (status unsupported)) (evaluation non-constant) (operator "or" (operator "==" (feature-reference "isBound" (target (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue::isBound")))) (feature-reference "mRef::isBound" (target (node (document "memory://snapshot/sysml.library/measurement_references.md") (path (named (kind library-package) (name "MeasurementReferences")) (named (kind attribute-def) (name "MeasurementUnit")) (anonymous (kind attribute) (ordinal 0))))))) (operator "and" (operator "not" (feature-reference "isBound" (target (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue::isBound"))))) (feature-reference "mRef::isBound" (target (node (document "memory://snapshot/sysml.library/measurement_references.md") (path (named (kind library-package) (name "MeasurementReferences")) (named (kind attribute-def) (name "MeasurementUnit")) (anonymous (kind attribute) (ordinal 0)))))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::remaining"))) (element (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue::order"))) (outcome ambiguous) (candidates (node (document "memory://snapshot/sysml.library/quantities.md") (path (named (kind library-package) (name "Quantities")) (named (kind attribute-def) (name "TensorQuantityValue")) (anonymous (kind attribute) (ordinal 0)))) (node (document "memory://snapshot/sysml.library/vector_values.md") (qualified-name "VectorValues::NumericalVectorValue::dimension"))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::remaining"))) (element (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue::orderSum"))) (effective-element (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue::orderSum"))) (authored-element (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue::orderSum"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation non-constant) (operator "==" (operator "+" (feature-reference "contravariantOrder" (target (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue::contravariantOrder")))) (feature-reference "covariantOrder" (target (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue::covariantOrder"))))) (feature-reference "order" (target (node (document "memory://snapshot/sysml.library/quantities.md") (qualified-name "Quantities::TensorQuantityValue::order"))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::running"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::running"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::running"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::running"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation evaluated) (value (kind boolean) (boolean true)) (feature-reference "this::isRunToCompletion" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::running"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::running"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::running")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::running"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "this::runToCompletionScope" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::running"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::running")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::running"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::running"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::running"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::running")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::running"))) (element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation evaluated) (value (kind boolean) (boolean true)) (feature-reference "this::isRunToCompletion" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::running"))) (element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (path (named (kind library-package) (name "StatePerformances")) (named (kind kerml-behavior) (name "StatePerformance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "this::runToCompletionScope" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::running"))) (element (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::incomingTransitionTrigger"))) (effective-element (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::incomingTransitionTrigger"))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::incomingTransitionTrigger"))) (outcome unsupported) (result-typing (status unsupported)) (evaluation unsupported))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::running"))) (element (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::isTriggerDuring"))) (effective-element (node (document "memory://snapshot/sysml.library/states.md") (path (named (kind library-package) (name "States")) (named (kind state-def) (name "StateAction")) (anonymous (kind attribute) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::isTriggerDuring"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::running"))) (element (node (document "memory://snapshot/sysml.library/states.md") (path (named (kind library-package) (name "States")) (named (kind state-def) (name "StateAction")) (anonymous (kind attribute) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/states.md") (path (named (kind library-package) (name "States")) (named (kind state-def) (name "StateAction")) (anonymous (kind attribute) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::isTriggerDuring"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start"))) (element (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::receiver"))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::receiver"))) (authored-element (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::triggerTarget"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start"))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start"))) (element (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::accNum"))) (effective-element (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::accNum"))) (authored-element (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::accNum"))) (outcome unsupported) (result-typing (status unsupported)) (evaluation unsupported))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start"))) (element (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::triggerTarget"))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::receiver"))) (authored-element (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::triggerTarget"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "that::this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1)))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "that::incomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::Clock")))) (evaluation non-constant) (feature-reference "that::localClock" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::Clock")))) (evaluation non-constant) (feature-reference "that::localClock" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (path (named (kind library-package) (name "Occurrences")) (named (kind class-def) (name "Occurrence")) (named (kind kerml-feature) (name "suboccurrences")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "that::incomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "that::this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1)))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "that::this" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "that::thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (named (kind kerml-step) (name "subperformances")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "that::thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::receiver"))) (effective-element (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::receiver"))) (authored-element (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::receiver"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")))) (evaluation non-constant) (feature-reference "this" (target (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "Action")) (named (kind action) (name "subactions")) (anonymous (kind occurrence) (ordinal 0)))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind accept-action) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/transition_performances.md") (path (named (kind library-package) (name "TransitionPerformances")) (named (kind kerml-behavior) (name "TransitionPerformance")) (named (kind kerml-step) (name "accept")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/transition_performances.md") (path (named (kind library-package) (name "TransitionPerformances")) (named (kind kerml-behavior) (name "TransitionPerformance")) (named (kind kerml-step) (name "accept")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/transition_performances.md") (path (named (kind library-package) (name "TransitionPerformances")) (named (kind kerml-behavior) (name "TransitionPerformance")) (named (kind kerml-step) (name "accept")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::MessageTransfer")))) (evaluation non-constant) (feature-reference "trigger" (target (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::trigger")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::countdown"))) (element (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item::isSolid"))) (effective-element (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item::isSolid"))) (authored-element (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item::isSolid"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation non-constant) (invocation (callee (feature-reference "isEmpty" (target (node (document "memory://snapshot/sysml.library/sequence_functions.md") (qualified-name "SequenceFunctions::isEmpty"))))) (arguments (feature-reference "voids" (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item::voids")))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::countdown"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::countdown")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::countdown"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::countdown"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean false)) (literal (value (kind boolean) (boolean false))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::countdown"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::countdown"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::countdown"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::countdown")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::countdown"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::countdown")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::countdown"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::countdown")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/model.sysml") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 2 19) (end 2 25)) (probe (position 2 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQ")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq.md") (qualified-name "ISQ")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 3 19) (end 3 24)) (probe (position 3 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "SI")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/si.md") (qualified-name "SI")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 7 20) (end 7 24)) (probe (position 7 20))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 11 30) (end 11 34)) (probe (position 11 30))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump"))) (kind transitionSource) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 11 107) (end 11 111)) (probe (position 11 107))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump"))) (kind transitionTarget) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 11 42) (end 11 47)) (probe (position 11 42))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::bump"))) (kind transitionTrigger) (ordinal 0) (authored-target "Start")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Start")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 11 30) (end 11 34)) (probe (position 11 30))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 11 107) (end 11 111)) (probe (position 11 107))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 11 80) (end 11 89)) (probe (position 11 80))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "remaining")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::remaining")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 11 96) (end 11 97)) (probe (position 11 96))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (kind quantityUnit) (ordinal 0) (authored-target "s")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/si.md") (qualified-name "SI::second")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 11 67) (end 11 76)) (probe (position 11 67))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "bump")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (kind assignTarget) (ordinal 0) (authored-target "remaining")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::remaining")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 6 30) (end 6 43)) (probe (position 6 30))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::remaining"))) (kind featureTyping) (ordinal 0) (authored-target "DurationValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/isq_base.md") (qualified-name "ISQBase::DurationValue")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 6 50) (end 6 51)) (probe (position 6 50))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind attribute) (name "remaining")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind quantityUnit) (ordinal 0) (authored-target "s")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/si.md") (qualified-name "SI::second")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 10 31) (end 10 35)) (probe (position 10 31))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start"))) (kind transitionSource) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 10 54) (end 10 61)) (probe (position 10 54))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start"))) (kind transitionTarget) (ordinal 0) (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::running")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 10 43) (end 10 48)) (probe (position 10 43))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::start"))) (kind transitionTrigger) (ordinal 0) (authored-target "Start")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Start")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 10 31) (end 10 35)) (probe (position 10 31))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "idle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::idle")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 10 54) (end 10 61)) (probe (position 10 54))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind state-def) (name "Countdown")) (named (kind transition) (name "start")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "running")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown::running")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 13 21) (end 13 40)) (probe (position 13 21))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::countdown"))) (kind featureTyping) (ordinal 0) (authored-target "StateTransitionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 13 50) (end 13 59)) (probe (position 13 50))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Timer")) (named (kind view) (name "countdown")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Countdown")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Timer::Countdown")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 12,
  "modelDigest": "blake3:a66aef1f1284bfecfbdf175b2bd216d102f75203fceb2f4e3605e70de920ef59",
  "documents": [
    {
      "uri": "memory://snapshot/model.sysml",
      "sourceDomain": "workspace"
    },
    {
      "uri": "memory://snapshot/sysml.library/isq_base.md",
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
        23
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
        43
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        8,
        7,
        14
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        15,
        7,
        25
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        20,
        7,
        24
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        14,
        8,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        14,
        9,
        21
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        8,
        10,
        62
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        19,
        10,
        24
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        31,
        10,
        35
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        43,
        10,
        48
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        54,
        10,
        61
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        8,
        11,
        112
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        19,
        11,
        23
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        30,
        11,
        34
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
        11,
        60,
        11,
        99
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        67,
        11,
        76
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        80,
        11,
        89
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        107,
        11,
        111
      ]
    },
    {
      "document": 0,
      "range": [
        13,
        9,
        13,
        18
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Timer::Countdown"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Timer::Countdown::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Timer::Countdown::bump"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Timer::Countdown::bump::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Timer::Countdown::bump::::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Timer::Countdown::bump::::::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Timer::Countdown::bump::::::::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Timer::Countdown::idle"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Timer::Countdown::remaining"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Timer::Countdown::running"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Timer::Countdown::start"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Timer::Countdown::start::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Timer::Start"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Timer::countdown"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "ISQBase::DurationValue"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ActionUsage",
      "ownerQualifiedName": "Timer::Countdown",
      "source": 3,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Timer::Countdown",
      "source": 4,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "AcceptActionUsage",
      "ownerQualifiedName": "Timer::Countdown::bump",
      "source": 13,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "PerformActionUsage",
      "ownerQualifiedName": "Timer::Countdown::bump",
      "source": 13,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Timer::Countdown::bump",
      "source": 13,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Timer::Countdown::bump",
      "source": 13,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "AssignmentActionUsage",
      "ownerQualifiedName": "Timer::Countdown::bump::",
      "source": 17,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Timer::Countdown::bump::",
      "source": 13,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Timer::Countdown::bump::::",
      "source": 18,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Feature",
      "ownerQualifiedName": "Timer::Countdown::bump::::::",
      "source": 18,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Timer::Countdown::bump::::::::",
      "source": 18,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "AcceptActionUsage",
      "ownerQualifiedName": "Timer::Countdown::start",
      "source": 8,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Timer::Countdown::start",
      "source": 8,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Timer::Countdown::start",
      "source": 8,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Timer::Countdown::start::",
      "source": 8,
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
      "ordinal": 18,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 27,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 28,
      "relationshipKind": "containment",
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
      "ordinal": 5,
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
      "ordinal": 13,
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
      "ordinal": 14,
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
      "ordinal": 37,
      "relationshipKind": "assignTarget",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 15,
      "relationshipKind": "containment",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 38,
      "relationshipKind": "expressionOperand",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 16,
      "relationshipKind": "containment",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 17,
      "relationshipKind": "containment",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "succession",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 24,
      "relationshipKind": "succession",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "transition",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 19,
      "relationshipKind": "transition",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 72,
      "relationshipKind": "typing",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 20,
      "relationshipKind": "containment",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 21,
      "relationshipKind": "containment",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 22,
      "relationshipKind": "containment",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 23,
      "relationshipKind": "containment",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 25,
      "relationshipKind": "containment",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 54,
      "relationshipKind": "transitionSource",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 55,
      "relationshipKind": "transitionTarget",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 56,
      "relationshipKind": "transitionTrigger",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 26,
      "relationshipKind": "containment",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 66,
      "relationshipKind": "succession",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 67,
      "relationshipKind": "succession",
      "source": 11
    }
  ],
  "selectedView": {
    "reference": 13,
    "kind": "state-transition-view",
    "name": "countdown",
    "source": 21
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "edges": [
      {
        "kind": "containment",
        "navigation": 6,
        "origin": 22,
        "provenance": "authored",
        "reference": 30,
        "source": 0,
        "target": 22
      },
      {
        "kind": "containment",
        "navigation": 4,
        "origin": 20,
        "provenance": "authored",
        "reference": 31,
        "source": 0,
        "target": 20
      },
      {
        "kind": "initial-state",
        "navigation": 5,
        "origin": 20,
        "provenance": "authored",
        "reference": 37,
        "source": 20,
        "target": 22
      },
      {
        "kind": "containment",
        "navigation": 7,
        "origin": 23,
        "provenance": "authored",
        "reference": 32,
        "source": 0,
        "target": 23
      },
      {
        "kind": "containment",
        "navigation": 14,
        "origin": 1,
        "provenance": "authored",
        "reference": 33,
        "source": 0,
        "target": 1
      },
      {
        "kind": "transition",
        "navigation": 15,
        "origin": 1,
        "provenance": "implied",
        "reference": 59,
        "source": 22,
        "target": 22
      },
      {
        "kind": "containment",
        "navigation": 13,
        "origin": 12,
        "provenance": "authored",
        "reference": 39,
        "source": 1,
        "target": 12
      },
      {
        "kind": "containment",
        "navigation": 13,
        "origin": 12,
        "provenance": "authored",
        "reference": 40,
        "source": 1,
        "target": 12
      },
      {
        "kind": "containment",
        "navigation": 13,
        "origin": 12,
        "provenance": "authored",
        "reference": 41,
        "source": 1,
        "target": 12
      },
      {
        "kind": "containment",
        "navigation": 13,
        "origin": 2,
        "provenance": "authored",
        "reference": 42,
        "source": 1,
        "target": 2
      },
      {
        "kind": "succession",
        "navigation": 15,
        "origin": 2,
        "provenance": "implied",
        "reference": 57,
        "source": 22,
        "target": 22
      },
      {
        "kind": "containment",
        "navigation": 13,
        "origin": 3,
        "provenance": "authored",
        "reference": 43,
        "source": 1,
        "target": 3
      },
      {
        "kind": "containment",
        "navigation": 13,
        "origin": 4,
        "provenance": "authored",
        "reference": 48,
        "source": 3,
        "target": 4
      },
      {
        "kind": "containment",
        "navigation": 13,
        "origin": 5,
        "provenance": "authored",
        "reference": 44,
        "source": 1,
        "target": 5
      },
      {
        "kind": "containment",
        "navigation": 17,
        "origin": 6,
        "provenance": "authored",
        "reference": 49,
        "source": 5,
        "target": 6
      },
      {
        "kind": "containment",
        "navigation": 18,
        "origin": 7,
        "provenance": "authored",
        "reference": 53,
        "source": 6,
        "target": 7
      },
      {
        "kind": "containment",
        "navigation": 18,
        "origin": 8,
        "provenance": "authored",
        "reference": 55,
        "source": 7,
        "target": 8
      },
      {
        "kind": "containment",
        "navigation": 18,
        "origin": 9,
        "provenance": "authored",
        "reference": 56,
        "source": 8,
        "target": 9
      },
      {
        "kind": "containment",
        "navigation": 9,
        "origin": 13,
        "provenance": "authored",
        "reference": 34,
        "source": 0,
        "target": 13
      },
      {
        "kind": "transition",
        "navigation": 10,
        "origin": 13,
        "provenance": "implied",
        "reference": 60,
        "source": 22,
        "target": 23
      },
      {
        "kind": "containment",
        "navigation": 8,
        "origin": 19,
        "provenance": "authored",
        "reference": 62,
        "source": 13,
        "target": 19
      },
      {
        "kind": "containment",
        "navigation": 8,
        "origin": 19,
        "provenance": "authored",
        "reference": 63,
        "source": 13,
        "target": 19
      },
      {
        "kind": "containment",
        "navigation": 8,
        "origin": 19,
        "provenance": "authored",
        "reference": 64,
        "source": 13,
        "target": 19
      },
      {
        "kind": "containment",
        "navigation": 8,
        "origin": 14,
        "provenance": "authored",
        "reference": 65,
        "source": 13,
        "target": 14
      },
      {
        "kind": "succession",
        "navigation": 10,
        "origin": 14,
        "provenance": "implied",
        "reference": 58,
        "source": 22,
        "target": 23
      },
      {
        "kind": "containment",
        "navigation": 8,
        "origin": 15,
        "provenance": "authored",
        "reference": 66,
        "source": 13,
        "target": 15
      },
      {
        "kind": "containment",
        "navigation": 8,
        "origin": 16,
        "provenance": "authored",
        "reference": 70,
        "source": 15,
        "target": 16
      },
      {
        "kind": "containment",
        "navigation": 1,
        "origin": 24,
        "provenance": "authored",
        "reference": 35,
        "source": 0,
        "target": 24
      },
      {
        "kind": "containment",
        "navigation": 3,
        "origin": 21,
        "provenance": "authored",
        "reference": 36,
        "source": 0,
        "target": 21
      }
    ],
    "exposedRoots": [
      0
    ],
    "kind": "state-transition-view",
    "metadata": {
      "finalNodes": [],
      "initialNodes": [
        20
      ],
      "states": [
        0,
        22,
        23
      ]
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "attributes",
            "members": [
              24
            ],
            "provenance": "direct"
          },
          {
            "kind": "actions",
            "members": [
              21
            ],
            "provenance": "direct"
          },
          {
            "kind": "states",
            "members": [
              22,
              23
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "StateDefinition",
        "multiplicity": null,
        "name": "Countdown",
        "notationRole": "definition",
        "owner": null,
        "presentationKey": "p:b6ed722ca78d68beb0cd1dabe3da6249aefeaa7907e6495ab904e3c1bf802518",
        "reference": 0,
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
              12,
              12,
              12
            ],
            "provenance": "direct"
          },
          {
            "kind": "actions",
            "members": [
              3,
              5
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "TransitionUsage",
        "multiplicity": null,
        "name": "bump",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:61b7f18f56d0d7a7a1bfa89154bb304d0fc19541ea53ec9da75d28475e86407c",
        "reference": 2,
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
        "metaclass": "SuccessionAsUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 1,
        "presentationKey": "p:b16b753eae599e560962f4c766aed1353aedb02179402705ef887ec270f4457c",
        "reference": 20,
        "source": 13,
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
        "presentationKey": "p:0fce954946f98cb9c9c404926a590b87d14800357eb3f194579d2dbfe1ec68e9",
        "reference": 17,
        "source": 13,
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
        "presentationKey": "p:263f0eb0800906deb808df94c8bac4f10dbabe467ddb6db9940f11f443b6ab35",
        "reference": 22,
        "source": 13,
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
              6
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PerformActionUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": 1,
        "presentationKey": "p:ad709bb08e138684a6aa6a0f38094f933f02512cf3389e766fe29e8ae11f2509",
        "reference": 18,
        "source": 13,
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
              7
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "AssignmentActionUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": 5,
        "presentationKey": "p:8f127832c41fcf27a3e7cf60241e2b8fd8e8d3a85b15fba7a52e65e979686687",
        "reference": 21,
        "source": 17,
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
        "owner": 6,
        "presentationKey": "p:15578ae71af71a1acfefd89f9619fd2b10816c6e14f6b0831c7ff2e4e2efbe49",
        "reference": 23,
        "source": 18,
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
              9
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "Feature",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 7,
        "presentationKey": "p:aa92e8dcffbe562c6b0e55968ef7eb8cd4a1b8fe326e4c16f4add435e2a387ae",
        "reference": 24,
        "source": 18,
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
        "presentationKey": "p:710e8cc232c01b62b03c429e0b3c1747df18acb9e7b8d9b900cc5025caebcb17",
        "reference": 25,
        "source": 18,
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
        "owner": 1,
        "presentationKey": "p:78b653ab70872729a2d0f8005cc8b1f5e69baf8b8ccc98e308abfd405087637c",
        "reference": 19,
        "source": 13,
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
        "presentationKey": "p:0a0aa1ddbcca19bb35f16af41c8b07be4686d0e1bf1e08be923e0e4e8aca582e",
        "reference": 19,
        "source": 13,
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
        "presentationKey": "p:086ea06bf189156dca56d3998ba8504894682d952d0659af4f7e82c5badf2327",
        "reference": 19,
        "source": 13,
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
              19,
              19,
              19
            ],
            "provenance": "direct"
          },
          {
            "kind": "actions",
            "members": [
              15
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "TransitionUsage",
        "multiplicity": null,
        "name": "start",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:ee304fb4b899271aea0f2e994ade156f79b1a6eae4658ec162046c93aac8f65e",
        "reference": 10,
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
        "metaclass": "SuccessionAsUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 13,
        "presentationKey": "p:97d317985dc848882fe1e4803a0e89c63cb8f2fad8f77dd96bad0cc8fffcb706",
        "reference": 28,
        "source": 8,
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
              16
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
        "owner": 13,
        "presentationKey": "p:5e8421736764a9f9ca9e14212f34a8292447a2c1d9b16c935e5f97542de34613",
        "reference": 26,
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
        "direction": "inout",
        "featureDirection": "inout",
        "metaclass": "ReferenceUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "reference-usage",
        "owner": 15,
        "presentationKey": "p:2ba82772b7a709370291e598d0b146b40baab2aa1c2cf451e388769427085673",
        "reference": 29,
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
        "metaclass": "ReferenceUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "reference-usage",
        "owner": 13,
        "presentationKey": "p:de29eb095c2603eb45ee98a5fd3d1608548600ea7772b4db114fd99126716503",
        "reference": 27,
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
        "direction": "in",
        "featureDirection": "in",
        "metaclass": "ReferenceUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "reference-usage",
        "owner": 13,
        "presentationKey": "p:f7c206e1aee929d75a3038c9b21660c14d993ad1768b92ce43e501286f025afc",
        "reference": 27,
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
        "direction": "in",
        "featureDirection": "in",
        "metaclass": "ReferenceUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "reference-usage",
        "owner": 13,
        "presentationKey": "p:2ffcabf76b26a70c4c8fae8ad59d536f93b192465472b93649204aba8c8f65d9",
        "reference": 27,
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
        "metaclass": "SuccessionAsUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 0,
        "presentationKey": "p:cc5777aa54193be2f0a353893c59ae23a63bc234dc0dfe62f98b14a11d48abb8",
        "reference": 16,
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
        "metaclass": "ActionUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:b961f144b49ec0bfe4d4d3734436d317944d08c175d44b3000e27b93228f7a69",
        "reference": 15,
        "source": 3,
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
        "presentationKey": "p:bc0e313d6987b2a31aa6f3b65f4ef8bf3ccba794b340306fd017875bd694e110",
        "reference": 7,
        "source": 6,
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
        "name": "running",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:d6b474978dcf79302ce8e635ec22a6af0d4196edfba8a6e82242a39b29c4628e",
        "reference": 9,
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
        "metaclass": "AttributeUsage",
        "multiplicity": null,
        "name": "remaining",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:6f8b3f79fdce18e142f6e064e8168ca6dc23b0e032217288dfa85ee6be40d94c",
        "reference": 8,
        "source": 1,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "DurationValue",
              "reference": 14
            }
          ]
        }
      }
    ],
    "relationships": [
      {
        "kind": "initialState",
        "navigation": 5,
        "provenance": "authored",
        "reference": 38,
        "source": 20,
        "target": {
          "node": 22,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionSource",
        "navigation": 15,
        "provenance": "authored",
        "reference": 45,
        "source": 1,
        "target": {
          "node": 22,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTarget",
        "navigation": 20,
        "provenance": "authored",
        "reference": 46,
        "source": 1,
        "target": {
          "node": 22,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTrigger",
        "navigation": 16,
        "provenance": "authored",
        "reference": 47,
        "source": 1,
        "target": {
          "reference": 12,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 15,
        "provenance": "authored",
        "reference": 50,
        "source": 2,
        "target": {
          "node": 22,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 20,
        "provenance": "authored",
        "reference": 51,
        "source": 2,
        "target": {
          "node": 22,
          "status": "resolved"
        }
      },
      {
        "kind": "assignTarget",
        "navigation": 18,
        "provenance": "authored",
        "reference": 52,
        "source": 6,
        "target": {
          "node": 24,
          "status": "resolved"
        }
      },
      {
        "kind": "expressionOperand",
        "navigation": 19,
        "provenance": "authored",
        "reference": 54,
        "source": 6,
        "target": {
          "node": 24,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionSource",
        "navigation": 10,
        "provenance": "authored",
        "reference": 67,
        "source": 13,
        "target": {
          "node": 22,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTarget",
        "navigation": 12,
        "provenance": "authored",
        "reference": 68,
        "source": 13,
        "target": {
          "node": 23,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTrigger",
        "navigation": 11,
        "provenance": "authored",
        "reference": 69,
        "source": 13,
        "target": {
          "reference": 12,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 10,
        "provenance": "authored",
        "reference": 71,
        "source": 14,
        "target": {
          "node": 22,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 12,
        "provenance": "authored",
        "reference": 72,
        "source": 14,
        "target": {
          "node": 23,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 2,
        "provenance": "authored",
        "reference": 61,
        "source": 24,
        "target": {
          "reference": 14,
          "status": "resolved"
        }
      }
    ],
    "scene": {
      "frame": {
        "id": "state-machine",
        "label": "Countdown",
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
          "navigation": 5,
          "provenance": "authored",
          "source": 0,
          "target": 1,
          "trigger": {
            "status": "absent"
          }
        },
        {
          "effect": {
            "label": "",
            "navigation": 14,
            "status": "supported"
          },
          "guard": {
            "status": "absent"
          },
          "id": "transition-1",
          "label": "bump",
          "navigation": 15,
          "provenance": "implied",
          "source": 1,
          "target": 1,
          "trigger": {
            "label": "Start",
            "navigation": 16,
            "status": "accept",
            "target": {
              "id": "element/v129:memory://snapshot/model.sysml7:packagen5:Timer1:013:attribute-defn5:Start1:0",
              "label": "Start"
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
          "id": "transition-2",
          "label": "start",
          "navigation": 10,
          "provenance": "implied",
          "source": 1,
          "target": 2,
          "trigger": {
            "label": "Start",
            "navigation": 11,
            "status": "accept",
            "target": {
              "id": "element/v129:memory://snapshot/model.sysml7:packagen5:Timer1:013:attribute-defn5:Start1:0",
              "label": "Start"
            }
          }
        }
      ],
      "vertices": [
        {
          "id": "state-0",
          "kind": "initial",
          "label": "",
          "navigation": 4
        },
        {
          "id": "state-1",
          "kind": "state",
          "label": "idle",
          "navigation": 6
        },
        {
          "id": "state-2",
          "kind": "state",
          "label": "running",
          "navigation": 7
        }
      ]
    }
  }
}

~~~
