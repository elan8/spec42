# META
~~~ini
description=A transition publishes its trigger, its guard as written and the effect action it performs
type=generate
libraries=standard
plugin=native:diagram
viewKind=state-transition-view
viewDocument=model.sysml
viewQualifiedName=Door::cycle
~~~
# SOURCE
## model.sysml
~~~sysml
package Door {
    private import StandardViewDefinitions::*;
    private import ScalarValues::*;
    attribute def Interrupted;
    attribute def Closed;
    action def CountReopen;
    state def Cycle {
        attribute attempts : Integer;
        attribute limit : Integer;
        entry; then closing;
        state closing;
        state opening;
        state nudging;
        state closed;
        transition reopen first closing accept Interrupted
            if attempts < limit
            do action count : CountReopen
            then opening;
        transition nudge first closing accept Interrupted if attempts >= limit then nudging;
        transition shut first nudging accept Closed then closed;
    }
    view cycle : StateTransitionView { expose Cycle; }
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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:8bd46dc8f9a5600febf97d4ebd7a4f3c1beae3bc20d8cdcf36e6e781a4483945") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Door"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Closed"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Door::CountReopen"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entry-action-binding) (membership (kind feature) (visibility default) (role entry)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (anonymous (kind initial-state) (ordinal 0))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "closing")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::attempts"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Integer")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::closed"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::closing"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::limit"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Integer")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudge"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "closing")) (transitionTarget (reference "nudging")) (transitionTrigger (reference "Interrupted")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (succession (reference "closing")) (succession (reference "nudging")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (kind kerml-boolean-expression) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "attempts")) (expressionOperand (reference "limit")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind parameter) (ordinal 1))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction inout)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudging"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::opening"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::reopen"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "closing")) (transitionTarget (reference "opening")) (transitionTrigger (reference "Interrupted")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (succession (reference "closing")) (succession (reference "opening")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (kind kerml-boolean-expression) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "attempts")) (expressionOperand (reference "limit")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind parameter) (ordinal 1))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction inout)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::reopen::count"))) (kind perform-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility default)) (relationships (transitionEffect (reference "CountReopen")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::shut"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "nudging")) (transitionTarget (reference "closed")) (transitionTrigger (reference "Closed")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "shut")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (succession (reference "nudging")) (succession (reference "closed")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "shut")) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "shut")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "shut")) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "shut")) (anonymous (kind parameter) (ordinal 1))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "shut")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction inout)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Interrupted"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Door::cycle"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StateTransitionView")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind view) (name "cycle")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Cycle")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0))
      (authored-target "closing")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::closing")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::attempts"))) (kind featureTyping) (ordinal 0))
      (authored-target "Integer")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::limit"))) (kind featureTyping) (ordinal 0))
      (authored-target "Integer")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudge"))) (kind transitionSource) (ordinal 0))
      (authored-target "closing")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::closing")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudge"))) (kind transitionTarget) (ordinal 0))
      (authored-target "nudging")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudging")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudge"))) (kind transitionTrigger) (ordinal 0))
      (authored-target "Interrupted")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Interrupted")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "closing")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::closing")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "nudging")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudging")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "attempts")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::attempts")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 1))
      (authored-target "limit")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::limit")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::reopen"))) (kind transitionSource) (ordinal 0))
      (authored-target "closing")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::closing")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::reopen"))) (kind transitionTarget) (ordinal 0))
      (authored-target "opening")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::opening")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::reopen"))) (kind transitionTrigger) (ordinal 0))
      (authored-target "Interrupted")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Interrupted")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "closing")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::closing")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "opening")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::opening")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "attempts")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::attempts")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 1))
      (authored-target "limit")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::limit")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::reopen::count"))) (kind transitionEffect) (ordinal 0))
      (authored-target "CountReopen")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::CountReopen")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::shut"))) (kind transitionSource) (ordinal 0))
      (authored-target "nudging")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudging")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::shut"))) (kind transitionTarget) (ordinal 0))
      (authored-target "closed")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::closed")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::shut"))) (kind transitionTrigger) (ordinal 0))
      (authored-target "Closed")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Closed")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "shut")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "nudging")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudging")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "shut")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "closed")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::closed")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::cycle"))) (kind featureTyping) (ordinal 0))
      (authored-target "StateTransitionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind view) (name "cycle")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Cycle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle")))))
  )
  (relationships
    (relationship (kind initialState) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::closing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::attempts"))) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::attempts"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::limit"))) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::limit"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudge"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::closing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudge"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudge"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudging"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudge"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudge"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Interrupted"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudge"))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::closing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudging"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::attempts"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::limit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::reopen"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::closing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::reopen"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::reopen"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::opening"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::reopen"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::reopen"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Interrupted"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::reopen"))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::closing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::opening"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::attempts"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::limit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind transitionEffect) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::reopen::count"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::CountReopen"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::reopen::count"))) (kind transitionEffect) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::shut"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudging"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::shut"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::shut"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::closed"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::shut"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::shut"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Closed"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::shut"))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "shut")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudging"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "shut")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "shut")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::closed"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "shut")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::cycle"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::cycle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind view) (name "cycle")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind view) (name "cycle")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Closed"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::CountReopen"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::entryAction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::attempts"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::attempts"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::closed"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::closed"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::closed"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::exclusiveStates"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::closed"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::closing"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::closing"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::closing"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::exclusiveStates"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::closing"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::limit"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::limit"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudge"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudge"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudge"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudge"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::stateTransitions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::accepter"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::transitionLinkSource"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudge"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudge"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudge"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudge"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudge"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::payload"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::transitionLinkSource"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::guard"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind accept-action) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudging"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudging"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudging"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::exclusiveStates"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudging"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::opening"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::opening"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::opening"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::exclusiveStates"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::opening"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::reopen"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::reopen"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::reopen"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::reopen"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::stateTransitions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::accepter"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::transitionLinkSource"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::reopen"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::reopen"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::reopen"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::reopen"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::reopen"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::payload"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::transitionLinkSource"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::guard"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind accept-action) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::reopen::count"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::effect"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::reopen::count"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::reopen::count"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::reopen"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::shut"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::shut"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::shut"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::shut"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::stateTransitions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "shut")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::accepter"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "shut")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::transitionLinkSource"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "shut")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::shut"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "shut")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::shut"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "shut")) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::shut"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "shut")) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::shut"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "shut")) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "shut")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "shut")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "shut")) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::payload"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "shut")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::transitionLinkSource"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "shut")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "shut")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "shut")) (anonymous (kind accept-action) (ordinal 0))))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Interrupted"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::cycle"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind view) (name "cycle")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::cycle"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Closed")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Door::CountReopen")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::DecisionPerformance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (anonymous (kind entry-action-binding) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (anonymous (kind initial-state) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::attempts")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle")))
      (type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Complex")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Number")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Rational")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::closed")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::closing")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::limit")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle")))
      (type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Complex")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Number")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Rational")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudge")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind succession) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudge")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudge")))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind kerml-boolean-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudge")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudge")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind parameter) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudge")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::payload")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::payload")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind accept-action) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::payload")) (scopes any feature))
      (subtype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind parameter) (ordinal 1)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudging")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::opening")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::reopen")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind succession) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::reopen")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::reopen")))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind kerml-boolean-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::reopen")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::reopen")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind parameter) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::reopen")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::payload")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::payload")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind accept-action) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::payload")) (scopes any feature))
      (subtype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind parameter) (ordinal 1)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::reopen::count")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::reopen")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::shut")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "shut")) (anonymous (kind succession) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "shut")) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::shut")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "shut")) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::shut")))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "shut")) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::shut")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "shut")) (anonymous (kind parameter) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::shut")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "shut")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::payload")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::payload")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "shut")) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "shut")) (anonymous (kind accept-action) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::payload")) (scopes any feature))
      (subtype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "shut")) (anonymous (kind parameter) (ordinal 1)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Interrupted")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Door::cycle")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind view) (name "cycle")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Door::cycle")))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (outcome resolved) (operator ">=" (feature-reference "attempts" (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::attempts")))) (feature-reference "limit" (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::limit"))))))
  (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (outcome resolved) (operator "<" (feature-reference "attempts" (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::attempts")))) (feature-reference "limit" (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::limit"))))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/model.sysml") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 2 19) (end 2 34)) (probe (position 2 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 9 20) (end 9 27)) (probe (position 9 20))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0) (authored-target "closing")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::closing")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 7 29) (end 7 36)) (probe (position 7 29))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::attempts"))) (kind featureTyping) (ordinal 0) (authored-target "Integer")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 8 26) (end 8 33)) (probe (position 8 26))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::limit"))) (kind featureTyping) (ordinal 0) (authored-target "Integer")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 18 31) (end 18 38)) (probe (position 18 31))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudge"))) (kind transitionSource) (ordinal 0) (authored-target "closing")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::closing")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 18 84) (end 18 91)) (probe (position 18 84))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudge"))) (kind transitionTarget) (ordinal 0) (authored-target "nudging")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudging")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 18 46) (end 18 57)) (probe (position 18 46))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudge"))) (kind transitionTrigger) (ordinal 0) (authored-target "Interrupted")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Interrupted")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 18 31) (end 18 38)) (probe (position 18 31))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "closing")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::closing")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 18 84) (end 18 91)) (probe (position 18 84))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "nudging")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudging")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 18 61) (end 18 69)) (probe (position 18 61))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "attempts")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::attempts")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 18 73) (end 18 78)) (probe (position 18 73))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "nudge")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 1) (authored-target "limit")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::limit")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 14 32) (end 14 39)) (probe (position 14 32))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::reopen"))) (kind transitionSource) (ordinal 0) (authored-target "closing")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::closing")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 17 17) (end 17 24)) (probe (position 17 17))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::reopen"))) (kind transitionTarget) (ordinal 0) (authored-target "opening")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::opening")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 14 47) (end 14 58)) (probe (position 14 47))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::reopen"))) (kind transitionTrigger) (ordinal 0) (authored-target "Interrupted")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Interrupted")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 14 32) (end 14 39)) (probe (position 14 32))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "closing")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::closing")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 17 17) (end 17 24)) (probe (position 17 17))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "opening")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::opening")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 15 15) (end 15 23)) (probe (position 15 15))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "attempts")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::attempts")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 15 26) (end 15 31)) (probe (position 15 26))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "reopen")) (anonymous (kind kerml-boolean-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 1) (authored-target "limit")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::limit")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 16 30) (end 16 41)) (probe (position 16 30))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::reopen::count"))) (kind transitionEffect) (ordinal 0) (authored-target "CountReopen")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::CountReopen")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 19 30) (end 19 37)) (probe (position 19 30))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::shut"))) (kind transitionSource) (ordinal 0) (authored-target "nudging")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudging")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 19 57) (end 19 63)) (probe (position 19 57))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::shut"))) (kind transitionTarget) (ordinal 0) (authored-target "closed")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::closed")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 19 45) (end 19 51)) (probe (position 19 45))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::shut"))) (kind transitionTrigger) (ordinal 0) (authored-target "Closed")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Closed")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 19 30) (end 19 37)) (probe (position 19 30))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "shut")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "nudging")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::nudging")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 19 57) (end 19 63)) (probe (position 19 57))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind state-def) (name "Cycle")) (named (kind transition) (name "shut")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "closed")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle::closed")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 21 17) (end 21 36)) (probe (position 21 17))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Door::cycle"))) (kind featureTyping) (ordinal 0) (authored-target "StateTransitionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 21 46) (end 21 51)) (probe (position 21 46))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Door")) (named (kind view) (name "cycle")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Cycle")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Door::Cycle")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 11,
  "modelDigest": "blake3:1c47d051779ad2114fb92209904181e38e2b9961fd9a4fd1b21b5f94a1879c1f",
  "documents": [
    {
      "uri": "memory://snapshot/model.sysml",
      "sourceDomain": "workspace"
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
        6,
        14,
        6,
        19
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        18,
        7,
        26
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        29,
        7,
        36
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
        33
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        8,
        9,
        14
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        15,
        9,
        28
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        20,
        9,
        27
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        14,
        10,
        21
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        14,
        11,
        21
      ]
    },
    {
      "document": 0,
      "range": [
        12,
        14,
        12,
        21
      ]
    },
    {
      "document": 0,
      "range": [
        13,
        14,
        13,
        20
      ]
    },
    {
      "document": 0,
      "range": [
        14,
        8,
        17,
        25
      ]
    },
    {
      "document": 0,
      "range": [
        14,
        19,
        14,
        25
      ]
    },
    {
      "document": 0,
      "range": [
        14,
        32,
        14,
        39
      ]
    },
    {
      "document": 0,
      "range": [
        14,
        47,
        14,
        58
      ]
    },
    {
      "document": 0,
      "range": [
        16,
        22,
        16,
        27
      ]
    },
    {
      "document": 0,
      "range": [
        16,
        30,
        16,
        41
      ]
    },
    {
      "document": 0,
      "range": [
        17,
        17,
        17,
        24
      ]
    },
    {
      "document": 0,
      "range": [
        18,
        8,
        18,
        92
      ]
    },
    {
      "document": 0,
      "range": [
        18,
        19,
        18,
        24
      ]
    },
    {
      "document": 0,
      "range": [
        18,
        31,
        18,
        38
      ]
    },
    {
      "document": 0,
      "range": [
        18,
        46,
        18,
        57
      ]
    },
    {
      "document": 0,
      "range": [
        18,
        84,
        18,
        91
      ]
    },
    {
      "document": 0,
      "range": [
        19,
        8,
        19,
        64
      ]
    },
    {
      "document": 0,
      "range": [
        19,
        19,
        19,
        23
      ]
    },
    {
      "document": 0,
      "range": [
        19,
        30,
        19,
        37
      ]
    },
    {
      "document": 0,
      "range": [
        19,
        45,
        19,
        51
      ]
    },
    {
      "document": 0,
      "range": [
        19,
        57,
        19,
        63
      ]
    },
    {
      "document": 0,
      "range": [
        21,
        9,
        21,
        14
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Door::Closed"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Door::CountReopen"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Door::Cycle"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Door::Cycle::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Door::Cycle::attempts"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Door::Cycle::closed"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Door::Cycle::closing"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Door::Cycle::limit"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Door::Cycle::nudge"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Door::Cycle::nudge::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Door::Cycle::nudging"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Door::Cycle::opening"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Door::Cycle::reopen"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Door::Cycle::reopen::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Door::Cycle::reopen::count"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Door::Cycle::shut"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Door::Cycle::shut::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Door::Interrupted"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Door::cycle"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "ScalarValues::Integer"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ActionUsage",
      "ownerQualifiedName": "Door::Cycle",
      "source": 5,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Door::Cycle",
      "source": 6,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "AcceptActionUsage",
      "ownerQualifiedName": "Door::Cycle::nudge",
      "source": 19,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Door::Cycle::nudge",
      "source": 19,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Door::Cycle::nudge",
      "source": 19,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Door::Cycle::nudge::",
      "source": 19,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "AcceptActionUsage",
      "ownerQualifiedName": "Door::Cycle::reopen",
      "source": 12,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Door::Cycle::reopen",
      "source": 12,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Door::Cycle::reopen",
      "source": 12,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Door::Cycle::reopen::",
      "source": 12,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "AcceptActionUsage",
      "ownerQualifiedName": "Door::Cycle::shut",
      "source": 24,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Door::Cycle::shut",
      "source": 24,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Door::Cycle::shut",
      "source": 24,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Door::Cycle::shut::",
      "source": 24,
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
      "ordinal": 1,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "containment",
      "source": 2
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
      "ordinal": 16,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 25,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 35,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 36,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "initialState",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "initialState",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 90,
      "relationshipKind": "typing",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 22,
      "relationshipKind": "succession",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 31,
      "relationshipKind": "succession",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 17,
      "relationshipKind": "transition",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 26,
      "relationshipKind": "transition",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 20,
      "relationshipKind": "typing",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 18,
      "relationshipKind": "containment",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 19,
      "relationshipKind": "containment",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 20,
      "relationshipKind": "containment",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 21,
      "relationshipKind": "containment",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 23,
      "relationshipKind": "containment",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 47,
      "relationshipKind": "transitionSource",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 48,
      "relationshipKind": "transitionTarget",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 49,
      "relationshipKind": "transitionTrigger",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 24,
      "relationshipKind": "containment",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 59,
      "relationshipKind": "succession",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 60,
      "relationshipKind": "succession",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "succession",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "transition",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 27,
      "relationshipKind": "containment",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 28,
      "relationshipKind": "containment",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 29,
      "relationshipKind": "containment",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 30,
      "relationshipKind": "containment",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 32,
      "relationshipKind": "containment",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 34,
      "relationshipKind": "containment",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 68,
      "relationshipKind": "transitionSource",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 69,
      "relationshipKind": "transitionTarget",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 70,
      "relationshipKind": "transitionTrigger",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 33,
      "relationshipKind": "containment",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 80,
      "relationshipKind": "succession",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 81,
      "relationshipKind": "succession",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 88,
      "relationshipKind": "transitionEffect",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "containment",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "containment",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 11,
      "relationshipKind": "containment",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 12,
      "relationshipKind": "containment",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 14,
      "relationshipKind": "containment",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 26,
      "relationshipKind": "transitionSource",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 27,
      "relationshipKind": "transitionTarget",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 28,
      "relationshipKind": "transitionTrigger",
      "source": 15
    },
    {
      "kind": "relationship",
      "ordinal": 15,
      "relationshipKind": "containment",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 38,
      "relationshipKind": "succession",
      "source": 16
    },
    {
      "kind": "relationship",
      "ordinal": 39,
      "relationshipKind": "succession",
      "source": 16
    }
  ],
  "selectedView": {
    "reference": 18,
    "kind": "state-transition-view",
    "name": "cycle",
    "source": 29
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "edges": [
      {
        "kind": "containment",
        "navigation": 11,
        "origin": 25,
        "provenance": "authored",
        "reference": 34,
        "source": 0,
        "target": 25
      },
      {
        "kind": "containment",
        "navigation": 6,
        "origin": 23,
        "provenance": "authored",
        "reference": 35,
        "source": 0,
        "target": 23
      },
      {
        "kind": "initial-state",
        "navigation": 7,
        "origin": 23,
        "provenance": "authored",
        "reference": 45,
        "source": 23,
        "target": 26
      },
      {
        "kind": "containment",
        "navigation": 8,
        "origin": 26,
        "provenance": "authored",
        "reference": 36,
        "source": 0,
        "target": 26
      },
      {
        "kind": "containment",
        "navigation": 10,
        "origin": 27,
        "provenance": "authored",
        "reference": 37,
        "source": 0,
        "target": 27
      },
      {
        "kind": "containment",
        "navigation": 9,
        "origin": 28,
        "provenance": "authored",
        "reference": 38,
        "source": 0,
        "target": 28
      },
      {
        "kind": "containment",
        "navigation": 3,
        "origin": 29,
        "provenance": "authored",
        "reference": 39,
        "source": 0,
        "target": 29
      },
      {
        "kind": "containment",
        "navigation": 25,
        "origin": 1,
        "provenance": "authored",
        "reference": 40,
        "source": 0,
        "target": 1
      },
      {
        "kind": "transition",
        "navigation": 26,
        "origin": 1,
        "provenance": "implied",
        "reference": 65,
        "source": 27,
        "target": 25
      },
      {
        "kind": "containment",
        "navigation": 24,
        "origin": 7,
        "provenance": "authored",
        "reference": 79,
        "source": 1,
        "target": 7
      },
      {
        "kind": "containment",
        "navigation": 24,
        "origin": 7,
        "provenance": "authored",
        "reference": 80,
        "source": 1,
        "target": 7
      },
      {
        "kind": "containment",
        "navigation": 24,
        "origin": 7,
        "provenance": "authored",
        "reference": 81,
        "source": 1,
        "target": 7
      },
      {
        "kind": "containment",
        "navigation": 24,
        "origin": 2,
        "provenance": "authored",
        "reference": 82,
        "source": 1,
        "target": 2
      },
      {
        "kind": "succession",
        "navigation": 26,
        "origin": 2,
        "provenance": "implied",
        "reference": 64,
        "source": 27,
        "target": 25
      },
      {
        "kind": "containment",
        "navigation": 24,
        "origin": 3,
        "provenance": "authored",
        "reference": 83,
        "source": 1,
        "target": 3
      },
      {
        "kind": "containment",
        "navigation": 24,
        "origin": 4,
        "provenance": "authored",
        "reference": 87,
        "source": 3,
        "target": 4
      },
      {
        "kind": "containment",
        "navigation": 20,
        "origin": 8,
        "provenance": "authored",
        "reference": 41,
        "source": 0,
        "target": 8
      },
      {
        "kind": "transition",
        "navigation": 21,
        "origin": 8,
        "provenance": "implied",
        "reference": 50,
        "source": 26,
        "target": 27
      },
      {
        "kind": "containment",
        "navigation": 19,
        "origin": 14,
        "provenance": "authored",
        "reference": 53,
        "source": 8,
        "target": 14
      },
      {
        "kind": "containment",
        "navigation": 19,
        "origin": 14,
        "provenance": "authored",
        "reference": 54,
        "source": 8,
        "target": 14
      },
      {
        "kind": "containment",
        "navigation": 19,
        "origin": 14,
        "provenance": "authored",
        "reference": 55,
        "source": 8,
        "target": 14
      },
      {
        "kind": "containment",
        "navigation": 19,
        "origin": 9,
        "provenance": "authored",
        "reference": 56,
        "source": 8,
        "target": 9
      },
      {
        "kind": "succession",
        "navigation": 21,
        "origin": 9,
        "provenance": "implied",
        "reference": 48,
        "source": 26,
        "target": 27
      },
      {
        "kind": "containment",
        "navigation": 19,
        "origin": 10,
        "provenance": "authored",
        "reference": 57,
        "source": 8,
        "target": 10
      },
      {
        "kind": "containment",
        "navigation": 19,
        "origin": 11,
        "provenance": "authored",
        "reference": 61,
        "source": 10,
        "target": 11
      },
      {
        "kind": "containment",
        "navigation": 13,
        "origin": 15,
        "provenance": "authored",
        "reference": 42,
        "source": 0,
        "target": 15
      },
      {
        "kind": "transition",
        "navigation": 14,
        "origin": 15,
        "provenance": "implied",
        "reference": 51,
        "source": 26,
        "target": 28
      },
      {
        "kind": "containment",
        "navigation": 12,
        "origin": 22,
        "provenance": "authored",
        "reference": 66,
        "source": 15,
        "target": 22
      },
      {
        "kind": "containment",
        "navigation": 12,
        "origin": 22,
        "provenance": "authored",
        "reference": 67,
        "source": 15,
        "target": 22
      },
      {
        "kind": "containment",
        "navigation": 12,
        "origin": 22,
        "provenance": "authored",
        "reference": 68,
        "source": 15,
        "target": 22
      },
      {
        "kind": "containment",
        "navigation": 12,
        "origin": 16,
        "provenance": "authored",
        "reference": 69,
        "source": 15,
        "target": 16
      },
      {
        "kind": "succession",
        "navigation": 14,
        "origin": 16,
        "provenance": "implied",
        "reference": 49,
        "source": 26,
        "target": 28
      },
      {
        "kind": "containment",
        "navigation": 12,
        "origin": 17,
        "provenance": "authored",
        "reference": 70,
        "source": 15,
        "target": 17
      },
      {
        "kind": "containment",
        "navigation": 12,
        "origin": 18,
        "provenance": "authored",
        "reference": 75,
        "source": 17,
        "target": 18
      },
      {
        "kind": "containment",
        "navigation": 16,
        "origin": 19,
        "provenance": "authored",
        "reference": 71,
        "source": 15,
        "target": 19
      },
      {
        "kind": "containment",
        "navigation": 1,
        "origin": 30,
        "provenance": "authored",
        "reference": 43,
        "source": 0,
        "target": 30
      },
      {
        "kind": "containment",
        "navigation": 5,
        "origin": 24,
        "provenance": "authored",
        "reference": 44,
        "source": 0,
        "target": 24
      }
    ],
    "exposedRoots": [
      0
    ],
    "kind": "state-transition-view",
    "metadata": {
      "finalNodes": [],
      "initialNodes": [
        23
      ],
      "states": [
        0,
        25,
        26,
        27,
        28
      ]
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "attributes",
            "members": [
              29,
              30
            ],
            "provenance": "direct"
          },
          {
            "kind": "actions",
            "members": [
              24
            ],
            "provenance": "direct"
          },
          {
            "kind": "states",
            "members": [
              25,
              26,
              27,
              28
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "StateDefinition",
        "multiplicity": null,
        "name": "Cycle",
        "notationRole": "definition",
        "owner": null,
        "presentationKey": "p:4cef69571364abd3d9daae791b0b0f18e289bd789de96a1039101af6cf07903f",
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
        "multiplicity": null,
        "name": "shut",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:aba43fe3f78197a1fefcb5516a96ee5eb2a27849bafa073442706f6126eff84a",
        "reference": 15,
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
        "metaclass": "SuccessionAsUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 1,
        "presentationKey": "p:c4818a22daf4cd5b960b9bca158c5051a10e4f3e714538ca09d622512ffc7f5d",
        "reference": 32,
        "source": 24,
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
        "presentationKey": "p:f9b6822868315310b9020745a3744da47f8ea60ec25ce15e0fc94e59f08f4f52",
        "reference": 30,
        "source": 24,
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
        "owner": 3,
        "presentationKey": "p:72935cffa97dcb3ccd04939b850d36d3b08a0c222ee9c7d34c5f70e3e95dda22",
        "reference": 33,
        "source": 24,
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
        "presentationKey": "p:49218caa218f2dced1cea054ac2310dd683ed0826d4d77bfc589ee9fe84e11f5",
        "reference": 31,
        "source": 24,
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
        "owner": 1,
        "presentationKey": "p:3d61338af7687cb4d8bd7398922b4351ca557a5dba99124c48427ade956c0fd1",
        "reference": 31,
        "source": 24,
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
        "owner": 1,
        "presentationKey": "p:89615d14bd0395277a4832d4e627dee8cc1fcb324dba79d805782e0d2976160e",
        "reference": 31,
        "source": 24,
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
        "name": "nudge",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:92c8ed1454376c735e763fced666a4451ae2769058b6e05468180dbf2247477b",
        "reference": 8,
        "source": 20,
        "stateSubaction": null,
        "successionGuard": "attempts >= limit",
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
        "presentationKey": "p:42560cea18b9d8956f70fc939d78e4e95044758c27c9bc48a620f977a77f2119",
        "reference": 24,
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
        "presentationKey": "p:1fab1056279d0b9256f213ce56f932e29803b50fb91a72c119a52f5de81c39a3",
        "reference": 22,
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
        "multiplicity": null,
        "name": null,
        "notationRole": "reference-usage",
        "owner": 10,
        "presentationKey": "p:c6885d33e6689f385b9923bd0ca3712bbd281f5f9bae61dcaef582438c7fe699",
        "reference": 25,
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
        "multiplicity": null,
        "name": null,
        "notationRole": "reference-usage",
        "owner": 8,
        "presentationKey": "p:ec314cb91b4c411ab92fce82702b4d2db25debaf2a8ef25d781844c18badffc8",
        "reference": 23,
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
        "multiplicity": null,
        "name": null,
        "notationRole": "reference-usage",
        "owner": 8,
        "presentationKey": "p:ca1550aebb9a0f1f6d860cbed5276d39a50258ad2c5de30e77fbb3d816de1828",
        "reference": 23,
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
        "multiplicity": null,
        "name": null,
        "notationRole": "reference-usage",
        "owner": 8,
        "presentationKey": "p:2e1115d77d93360d08a6c2fde056665845c52f135289e193b7f9d3c7e212459a",
        "reference": 23,
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
              22,
              22,
              22
            ],
            "provenance": "direct"
          },
          {
            "kind": "actions",
            "members": [
              17,
              19
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "TransitionUsage",
        "multiplicity": null,
        "name": "reopen",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:5a7e5153fdce6eef349f9de9c4ffceeaf033e6d68ddc074dac139a33789f23c8",
        "reference": 12,
        "source": 13,
        "stateSubaction": null,
        "successionGuard": "attempts < limit",
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
        "presentationKey": "p:f97aa7729747c276f5b25614df0539092a01625b2dc421669208b23e73760b7f",
        "reference": 28,
        "source": 12,
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
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": 15,
        "presentationKey": "p:fe7349e27e5982041a2802a87de24cecea5f4ac60c3cfb82dbfb89c80749a781",
        "reference": 26,
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
        "direction": "inout",
        "featureDirection": null,
        "metaclass": "ReferenceUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "reference-usage",
        "owner": 17,
        "presentationKey": "p:fdf1a80fa3e2980bd6ce7e81511c8cb0d0ef6da5ede24786da2fb007d3f7dbb6",
        "reference": 29,
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
        "metaclass": "PerformActionUsage",
        "multiplicity": null,
        "name": "count",
        "notationRole": "usage",
        "owner": 15,
        "presentationKey": "p:9c436f146a060767eb86473b95796a1eb36392986f2151552feebcf3340c601c",
        "reference": 14,
        "source": 16,
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
        "presentationKey": "p:1dbbb47daf853e34aa822491be4ac83f9ce438cfb8a5b4a2c57beb10543a6a1f",
        "reference": 27,
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
        "direction": "in",
        "featureDirection": null,
        "metaclass": "ReferenceUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "reference-usage",
        "owner": 15,
        "presentationKey": "p:c86d6397b1b31b38348fa84507eb9cd810eabe5f24bf0131f77bb0dda1ab196d",
        "reference": 27,
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
        "direction": "in",
        "featureDirection": null,
        "metaclass": "ReferenceUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "reference-usage",
        "owner": 15,
        "presentationKey": "p:6f42693669e17cefd5e48c560a1fb21d9b1dcee1f6f4fda7630fd097e1354bf6",
        "reference": 27,
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
        "metaclass": "SuccessionAsUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 0,
        "presentationKey": "p:877e25b3a3d0e214cabc51f302294835aa63a96795f4b8e9b9c8b3030b9c0e8e",
        "reference": 21,
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
        "metaclass": "ActionUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:5292afc8683138335a691cba841207f841f8ff59dc426b19a95e9edc3deb85de",
        "reference": 20,
        "source": 5,
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
        "name": "closed",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:d291b98fd26319203934e28c00043a49ebb0776ab2be6bd90fa4f9c9cb7c2101",
        "reference": 5,
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
        "metaclass": "StateUsage",
        "multiplicity": null,
        "name": "closing",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:ae6c497f09034ce83fecbfb0a1385faed577dd385737daa2c81bdc6a4b882a52",
        "reference": 6,
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
        "metaclass": "StateUsage",
        "multiplicity": null,
        "name": "nudging",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:d89203c192943e8bcc1efd6e5a4015cbccd874c7e5ea3558d30645faabc57d27",
        "reference": 10,
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
        "multiplicity": null,
        "name": "opening",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:4b439def88e6f99ea52b221028576b4e09b62f9c8fd226c76e8e4b15ce2b5f5a",
        "reference": 11,
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
        "metaclass": "AttributeUsage",
        "multiplicity": null,
        "name": "limit",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:69f0c2b1e9a777606924520b912133591415d4f97e358646d9f32a29c936318a",
        "reference": 7,
        "source": 3,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Integer",
              "reference": 19
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
        "name": "attempts",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:9f3060dada84b8a197a9d7379c1f439b245e5cabce043f6888e2ca00723e3991",
        "reference": 4,
        "source": 1,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Integer",
              "reference": 19
            }
          ]
        }
      }
    ],
    "relationships": [
      {
        "kind": "initialState",
        "navigation": 7,
        "provenance": "authored",
        "reference": 46,
        "source": 23,
        "target": {
          "node": 26,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 4,
        "provenance": "authored",
        "reference": 52,
        "source": 29,
        "target": {
          "reference": 19,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionSource",
        "navigation": 26,
        "provenance": "authored",
        "reference": 84,
        "source": 1,
        "target": {
          "node": 27,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTarget",
        "navigation": 28,
        "provenance": "authored",
        "reference": 85,
        "source": 1,
        "target": {
          "node": 25,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTrigger",
        "navigation": 27,
        "provenance": "authored",
        "reference": 86,
        "source": 1,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 26,
        "provenance": "authored",
        "reference": 88,
        "source": 2,
        "target": {
          "node": 27,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 28,
        "provenance": "authored",
        "reference": 89,
        "source": 2,
        "target": {
          "node": 25,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionSource",
        "navigation": 21,
        "provenance": "authored",
        "reference": 58,
        "source": 8,
        "target": {
          "node": 26,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTarget",
        "navigation": 23,
        "provenance": "authored",
        "reference": 59,
        "source": 8,
        "target": {
          "node": 27,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTrigger",
        "navigation": 22,
        "provenance": "authored",
        "reference": 60,
        "source": 8,
        "target": {
          "reference": 17,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 21,
        "provenance": "authored",
        "reference": 62,
        "source": 9,
        "target": {
          "node": 26,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 23,
        "provenance": "authored",
        "reference": 63,
        "source": 9,
        "target": {
          "node": 27,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionSource",
        "navigation": 14,
        "provenance": "authored",
        "reference": 72,
        "source": 15,
        "target": {
          "node": 26,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTarget",
        "navigation": 18,
        "provenance": "authored",
        "reference": 73,
        "source": 15,
        "target": {
          "node": 28,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTrigger",
        "navigation": 15,
        "provenance": "authored",
        "reference": 74,
        "source": 15,
        "target": {
          "reference": 17,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 14,
        "provenance": "authored",
        "reference": 76,
        "source": 16,
        "target": {
          "node": 26,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 18,
        "provenance": "authored",
        "reference": 77,
        "source": 16,
        "target": {
          "node": 28,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionEffect",
        "navigation": 17,
        "provenance": "authored",
        "reference": 78,
        "source": 19,
        "target": {
          "reference": 1,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 2,
        "provenance": "authored",
        "reference": 47,
        "source": 30,
        "target": {
          "reference": 19,
          "status": "resolved"
        }
      }
    ],
    "scene": {
      "frame": {
        "id": "state-machine",
        "label": "Cycle",
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
          "navigation": 7,
          "provenance": "authored",
          "source": 0,
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
          "label": "shut",
          "navigation": 26,
          "provenance": "implied",
          "source": 3,
          "target": 1,
          "trigger": {
            "label": "Closed",
            "navigation": 27,
            "status": "accept",
            "target": {
              "id": "element/v129:memory://snapshot/model.sysml7:packagen4:Door1:013:attribute-defn6:Closed1:0",
              "label": "Closed"
            }
          }
        },
        {
          "effect": {
            "status": "absent"
          },
          "guard": {
            "label": "attempts >= limit",
            "navigation": 20,
            "status": "supported"
          },
          "id": "transition-2",
          "label": "nudge",
          "navigation": 21,
          "provenance": "implied",
          "source": 2,
          "target": 3,
          "trigger": {
            "label": "Interrupted",
            "navigation": 22,
            "status": "accept",
            "target": {
              "id": "element/v129:memory://snapshot/model.sysml7:packagen4:Door1:013:attribute-defn11:Interrupted1:0",
              "label": "Interrupted"
            }
          }
        },
        {
          "effect": {
            "label": "count",
            "navigation": 17,
            "status": "supported"
          },
          "guard": {
            "label": "attempts < limit",
            "navigation": 13,
            "status": "supported"
          },
          "id": "transition-3",
          "label": "reopen",
          "navigation": 14,
          "provenance": "implied",
          "source": 2,
          "target": 4,
          "trigger": {
            "label": "Interrupted",
            "navigation": 15,
            "status": "accept",
            "target": {
              "id": "element/v129:memory://snapshot/model.sysml7:packagen4:Door1:013:attribute-defn11:Interrupted1:0",
              "label": "Interrupted"
            }
          }
        }
      ],
      "vertices": [
        {
          "id": "state-0",
          "kind": "initial",
          "label": "",
          "navigation": 6
        },
        {
          "id": "state-1",
          "kind": "state",
          "label": "closed",
          "navigation": 11
        },
        {
          "id": "state-2",
          "kind": "state",
          "label": "closing",
          "navigation": 8
        },
        {
          "id": "state-3",
          "kind": "state",
          "label": "nudging",
          "navigation": 10
        },
        {
          "id": "state-4",
          "kind": "state",
          "label": "opening",
          "navigation": 9
        }
      ]
    }
  }
}

~~~
