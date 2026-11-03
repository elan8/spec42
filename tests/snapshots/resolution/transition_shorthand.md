# META
~~~ini
description=Shorthand state transitions take their lexical state or entry action as source (S21); SysML 2.0 clause 7.18.3
type=generate
plugin=native:diagram
viewKind=state-transition-view
viewDocument=transition_shorthand.md
viewQualifiedName=Shorthand::states
libraries=standard
require_complete_publication=true
require_no_diagnostics=true
~~~
# SOURCE
~~~sysml
package Shorthand {
    occurrence def Go;
    state def Machine {
        entry; then a;
        state a;
        accept Go then b;
        accept Go then c;
        state b;
        accept Go then c;
        state c;
    }
    view states : StandardViewDefinitions::StateTransitionView { expose Machine; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/transition_shorthand.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:7d5a193d96b5ea2f7b313f59979405c222630385ce12c07a876db2480ba66ccd") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Go"))) (kind occurrence-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entry-action-binding) (membership (kind feature) (visibility default) (role entry)))
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "a")))))
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionTarget (reference "b")) (transitionTrigger (reference "Go")))))
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionTarget (reference "c")) (transitionTrigger (reference "Go")))))
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionTarget (reference "c")) (transitionTrigger (reference "Go")))))
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (succession (reference "b")))))
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind parameter) (ordinal 1))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (succession (reference "c")))))
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind parameter) (ordinal 1))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (succession (reference "c")))))
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind parameter) (ordinal 1))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction inout)))
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction inout)))
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction inout)))
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::a"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::b"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::c"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::states"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StandardViewDefinitions::StateTransitionView")))))
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind view) (name "states")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Machine")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0))
      (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::a")))))
    (reference (id (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0))
      (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::b")))))
    (reference (id (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTarget) (ordinal 0))
      (authored-target "c")
      (outcome (status resolved) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::c")))))
    (reference (id (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2))))) (kind transitionTarget) (ordinal 0))
      (authored-target "c")
      (outcome (status resolved) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::c")))))
    (reference (id (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTrigger) (ordinal 0))
      (authored-target "Go")
      (outcome (status resolved) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Go")))))
    (reference (id (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTrigger) (ordinal 0))
      (authored-target "Go")
      (outcome (status resolved) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Go")))))
    (reference (id (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2))))) (kind transitionTrigger) (ordinal 0))
      (authored-target "Go")
      (outcome (status resolved) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Go")))))
    (reference (id (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::b")))))
    (reference (id (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "c")
      (outcome (status resolved) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::c")))))
    (reference (id (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "c")
      (outcome (status resolved) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::c")))))
    (reference (id (source (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::states"))) (kind featureTyping) (ordinal 0))
      (authored-target "StandardViewDefinitions::StateTransitionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView")))))
    (reference (id (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind view) (name "states")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Machine")
      (outcome (status resolved) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine")))))
  )
  (relationships
    (relationship (kind initialState) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1))))) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::c"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2))))) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::c"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Go"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1))))) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Go"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind transitionTrigger) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2))))) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Go"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2))))) (kind transitionTrigger) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::c"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::c"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::states"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::states"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind view) (name "states")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind view) (name "states")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Go"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1))))) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2))))) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine"))) (provenance implied))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::a"))) (provenance implied))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1))))) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::a"))) (provenance implied))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2))))) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::b"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::entryAction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::stateTransitions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::stateTransitions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::stateTransitions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::accepter"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::accepter"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::accepter"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::transitionLinkSource"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::transitionLinkSource"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::transitionLinkSource"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::happensBeforeLinks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind succession) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::a"))) (provenance implied))
    (relationship (kind succession) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::a"))) (provenance implied))
    (relationship (kind succession) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::b"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::payload"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::payload"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind parameter) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::payload"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::transitionLinkSource"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::transitionLinkSource"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::transitionLinkSource"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind accept-action) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind accept-action) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::a"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::a"))) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::a"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::exclusiveStates"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::a"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::b"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::b"))) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::b"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::exclusiveStates"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::b"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::c"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::c"))) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::c"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::exclusiveStates"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::c"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::states"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind view) (name "states")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::states"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Go")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::DecisionPerformance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine")))
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
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine")))
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
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine")))
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
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine")))
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
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine")))
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
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind parameter) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::payload")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::payload")) (scopes any feature))
      (supertype (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind succession) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)))))
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
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)))))
    )
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)))))
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
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind parameter) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::payload")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::payload")) (scopes any feature))
      (supertype (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind succession) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2)))))
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
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2)))))
    )
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2)))))
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
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind parameter) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateTransitionAction::payload")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::payload")) (scopes any feature))
      (supertype (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::payload")) (scopes any feature))
      (subtype (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind parameter) (ordinal 1)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind accept-action) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::payload")) (scopes any feature))
      (subtype (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind parameter) (ordinal 1)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind accept-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind accept-action) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (path (named (kind library-package) (name "Actions")) (named (kind action-def) (name "AcceptMessageAction")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance::payload")) (scopes any feature))
      (subtype (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind parameter) (ordinal 1)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::a")))
      (featured-by (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine")))
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
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::b")))
      (featured-by (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine")))
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
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::c")))
      (featured-by (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine")))
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
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::states")))
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
    (declaration (id (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind view) (name "states")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::states")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/transition_shorthand.md") (range (start 3 20) (end 3 21)) (probe (position 3 20))
    (reference (id (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0) (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::a")))))
    )
  )
  (query (document "memory://snapshot/transition_shorthand.md") (range (start 5 23) (end 5 24)) (probe (position 5 23))
    (reference (id (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0) (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::b")))))
    )
  )
  (query (document "memory://snapshot/transition_shorthand.md") (range (start 6 23) (end 6 24)) (probe (position 6 23))
    (reference (id (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTarget) (ordinal 0) (authored-target "c")
      (outcome (status resolved) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::c")))))
    )
  )
  (query (document "memory://snapshot/transition_shorthand.md") (range (start 8 23) (end 8 24)) (probe (position 8 23))
    (reference (id (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2))))) (kind transitionTarget) (ordinal 0) (authored-target "c")
      (outcome (status resolved) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::c")))))
    )
  )
  (query (document "memory://snapshot/transition_shorthand.md") (range (start 5 15) (end 5 17)) (probe (position 5 15))
    (reference (id (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTrigger) (ordinal 0) (authored-target "Go")
      (outcome (status resolved) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Go")))))
    )
  )
  (query (document "memory://snapshot/transition_shorthand.md") (range (start 6 15) (end 6 17)) (probe (position 6 15))
    (reference (id (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1))))) (kind transitionTrigger) (ordinal 0) (authored-target "Go")
      (outcome (status resolved) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Go")))))
    )
  )
  (query (document "memory://snapshot/transition_shorthand.md") (range (start 8 15) (end 8 17)) (probe (position 8 15))
    (reference (id (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2))))) (kind transitionTrigger) (ordinal 0) (authored-target "Go")
      (outcome (status resolved) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Go")))))
    )
  )
  (query (document "memory://snapshot/transition_shorthand.md") (range (start 5 23) (end 5 24)) (probe (position 5 23))
    (reference (id (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::b")))))
    )
  )
  (query (document "memory://snapshot/transition_shorthand.md") (range (start 6 23) (end 6 24)) (probe (position 6 23))
    (reference (id (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 1)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "c")
      (outcome (status resolved) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::c")))))
    )
  )
  (query (document "memory://snapshot/transition_shorthand.md") (range (start 8 23) (end 8 24)) (probe (position 8 23))
    (reference (id (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 2)) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "c")
      (outcome (status resolved) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine::c")))))
    )
  )
  (query (document "memory://snapshot/transition_shorthand.md") (range (start 11 18) (end 11 62)) (probe (position 11 18))
    (reference (id (source (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::states"))) (kind featureTyping) (ordinal 0) (authored-target "StandardViewDefinitions::StateTransitionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::StateTransitionView")))))
    )
  )
  (query (document "memory://snapshot/transition_shorthand.md") (range (start 11 72) (end 11 79)) (probe (position 11 72))
    (reference (id (source (node (document "memory://snapshot/transition_shorthand.md") (path (named (kind package) (name "Shorthand")) (named (kind view) (name "states")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Machine")
      (outcome (status resolved) (target (node (document "memory://snapshot/transition_shorthand.md") (qualified-name "Shorthand::Machine")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 11,
  "modelDigest": "blake3:08a20f2209df182639dddfaa939403d2d1c4d8797c85abd905c5016e91d83a65",
  "documents": [
    {
      "uri": "memory://snapshot/transition_shorthand.md",
      "sourceDomain": "workspace"
    }
  ],
  "sources": [
    {
      "document": 0,
      "range": [
        2,
        14,
        2,
        21
      ]
    },
    {
      "document": 0,
      "range": [
        3,
        8,
        3,
        14
      ]
    },
    {
      "document": 0,
      "range": [
        3,
        15,
        3,
        22
      ]
    },
    {
      "document": 0,
      "range": [
        3,
        20,
        3,
        21
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        14,
        4,
        15
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        8,
        5,
        25
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        15,
        5,
        17
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        23,
        5,
        24
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        8,
        6,
        25
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        15,
        6,
        17
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        23,
        6,
        24
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        14,
        7,
        15
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        8,
        8,
        25
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        15,
        8,
        17
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        23,
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
        15
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        9,
        11,
        15
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Shorthand::Go"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Shorthand::Machine"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Shorthand::Machine::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Shorthand::Machine::::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Shorthand::Machine::a"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Shorthand::Machine::b"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Shorthand::Machine::c"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Shorthand::states"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ActionUsage",
      "ownerQualifiedName": "Shorthand::Machine",
      "source": 1,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Shorthand::Machine",
      "source": 2,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "TransitionUsage",
      "ownerQualifiedName": "Shorthand::Machine",
      "source": 5,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "TransitionUsage",
      "ownerQualifiedName": "Shorthand::Machine",
      "source": 8,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "TransitionUsage",
      "ownerQualifiedName": "Shorthand::Machine",
      "source": 12,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "AcceptActionUsage",
      "ownerQualifiedName": "Shorthand::Machine::",
      "source": 5,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "AcceptActionUsage",
      "ownerQualifiedName": "Shorthand::Machine::",
      "source": 8,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "AcceptActionUsage",
      "ownerQualifiedName": "Shorthand::Machine::",
      "source": 12,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Shorthand::Machine::",
      "source": 5,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Shorthand::Machine::",
      "source": 8,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Shorthand::Machine::",
      "source": 12,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Shorthand::Machine::",
      "source": 5,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Shorthand::Machine::",
      "source": 8,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "SuccessionAsUsage",
      "ownerQualifiedName": "Shorthand::Machine::",
      "source": 12,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Shorthand::Machine::::",
      "source": 5,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Shorthand::Machine::::",
      "source": 8,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Shorthand::Machine::::",
      "source": 12,
      "sourceDomain": "workspace"
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 3,
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
      "ordinal": 21,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 30,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 32,
      "relationshipKind": "containment",
      "source": 1
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
      "ordinal": 8,
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
      "ordinal": 14,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 15,
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
      "ordinal": 23,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 24,
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
      "ordinal": 26,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 28,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 31,
      "relationshipKind": "initialState",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 76,
      "relationshipKind": "initialState",
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
      "ordinal": 38,
      "relationshipKind": "transitionTarget",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 59,
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
      "ordinal": 39,
      "relationshipKind": "transitionTrigger",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 60,
      "relationshipKind": "transitionTrigger",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 11,
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
      "ordinal": 29,
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
      "ordinal": 49,
      "relationshipKind": "succession",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 70,
      "relationshipKind": "succession",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "transition",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "transition",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "succession",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 22,
      "relationshipKind": "transition",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 18,
      "relationshipKind": "succession",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 27,
      "relationshipKind": "succession",
      "source": 6
    }
  ],
  "selectedView": {
    "reference": 7,
    "kind": "state-transition-view",
    "name": "states",
    "source": 16
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "edges": [
      {
        "kind": "containment",
        "navigation": 4,
        "origin": 24,
        "provenance": "authored",
        "reference": 25,
        "source": 0,
        "target": 24
      },
      {
        "kind": "containment",
        "navigation": 11,
        "origin": 25,
        "provenance": "authored",
        "reference": 26,
        "source": 0,
        "target": 25
      },
      {
        "kind": "containment",
        "navigation": 15,
        "origin": 26,
        "provenance": "authored",
        "reference": 27,
        "source": 0,
        "target": 26
      },
      {
        "kind": "containment",
        "navigation": 5,
        "origin": 1,
        "provenance": "authored",
        "reference": 28,
        "source": 0,
        "target": 1
      },
      {
        "kind": "transition",
        "navigation": 7,
        "origin": 1,
        "provenance": "implied",
        "reference": 62,
        "source": 24,
        "target": 25
      },
      {
        "kind": "containment",
        "navigation": 5,
        "origin": 7,
        "provenance": "authored",
        "reference": 33,
        "source": 1,
        "target": 7
      },
      {
        "kind": "containment",
        "navigation": 5,
        "origin": 7,
        "provenance": "authored",
        "reference": 34,
        "source": 1,
        "target": 7
      },
      {
        "kind": "containment",
        "navigation": 5,
        "origin": 7,
        "provenance": "authored",
        "reference": 35,
        "source": 1,
        "target": 7
      },
      {
        "kind": "containment",
        "navigation": 5,
        "origin": 2,
        "provenance": "authored",
        "reference": 36,
        "source": 1,
        "target": 2
      },
      {
        "kind": "succession",
        "navigation": 7,
        "origin": 2,
        "provenance": "implied",
        "reference": 64,
        "source": 25,
        "target": 24
      },
      {
        "kind": "containment",
        "navigation": 5,
        "origin": 3,
        "provenance": "authored",
        "reference": 37,
        "source": 1,
        "target": 3
      },
      {
        "kind": "containment",
        "navigation": 5,
        "origin": 4,
        "provenance": "authored",
        "reference": 56,
        "source": 3,
        "target": 4
      },
      {
        "kind": "containment",
        "navigation": 8,
        "origin": 8,
        "provenance": "authored",
        "reference": 29,
        "source": 0,
        "target": 8
      },
      {
        "kind": "transition",
        "navigation": 10,
        "origin": 8,
        "provenance": "implied",
        "reference": 63,
        "source": 24,
        "target": 26
      },
      {
        "kind": "containment",
        "navigation": 8,
        "origin": 14,
        "provenance": "authored",
        "reference": 38,
        "source": 8,
        "target": 14
      },
      {
        "kind": "containment",
        "navigation": 8,
        "origin": 14,
        "provenance": "authored",
        "reference": 39,
        "source": 8,
        "target": 14
      },
      {
        "kind": "containment",
        "navigation": 8,
        "origin": 14,
        "provenance": "authored",
        "reference": 40,
        "source": 8,
        "target": 14
      },
      {
        "kind": "containment",
        "navigation": 8,
        "origin": 9,
        "provenance": "authored",
        "reference": 41,
        "source": 8,
        "target": 9
      },
      {
        "kind": "succession",
        "navigation": 10,
        "origin": 9,
        "provenance": "implied",
        "reference": 66,
        "source": 26,
        "target": 24
      },
      {
        "kind": "containment",
        "navigation": 8,
        "origin": 10,
        "provenance": "authored",
        "reference": 42,
        "source": 8,
        "target": 10
      },
      {
        "kind": "containment",
        "navigation": 8,
        "origin": 11,
        "provenance": "authored",
        "reference": 57,
        "source": 10,
        "target": 11
      },
      {
        "kind": "containment",
        "navigation": 12,
        "origin": 15,
        "provenance": "authored",
        "reference": 30,
        "source": 0,
        "target": 15
      },
      {
        "kind": "transition",
        "navigation": 14,
        "origin": 15,
        "provenance": "implied",
        "reference": 65,
        "source": 25,
        "target": 26
      },
      {
        "kind": "containment",
        "navigation": 12,
        "origin": 21,
        "provenance": "authored",
        "reference": 43,
        "source": 15,
        "target": 21
      },
      {
        "kind": "containment",
        "navigation": 12,
        "origin": 21,
        "provenance": "authored",
        "reference": 44,
        "source": 15,
        "target": 21
      },
      {
        "kind": "containment",
        "navigation": 12,
        "origin": 21,
        "provenance": "authored",
        "reference": 45,
        "source": 15,
        "target": 21
      },
      {
        "kind": "containment",
        "navigation": 12,
        "origin": 16,
        "provenance": "authored",
        "reference": 46,
        "source": 15,
        "target": 16
      },
      {
        "kind": "succession",
        "navigation": 14,
        "origin": 16,
        "provenance": "implied",
        "reference": 67,
        "source": 26,
        "target": 25
      },
      {
        "kind": "containment",
        "navigation": 12,
        "origin": 17,
        "provenance": "authored",
        "reference": 47,
        "source": 15,
        "target": 17
      },
      {
        "kind": "containment",
        "navigation": 12,
        "origin": 18,
        "provenance": "authored",
        "reference": 58,
        "source": 17,
        "target": 18
      },
      {
        "kind": "containment",
        "navigation": 2,
        "origin": 22,
        "provenance": "authored",
        "reference": 31,
        "source": 0,
        "target": 22
      },
      {
        "kind": "initial-state",
        "navigation": 3,
        "origin": 22,
        "provenance": "authored",
        "reference": 48,
        "source": 22,
        "target": 24
      },
      {
        "kind": "containment",
        "navigation": 1,
        "origin": 23,
        "provenance": "authored",
        "reference": 32,
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
        22
      ],
      "states": [
        0,
        24,
        25,
        26
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
              25,
              26
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
        "presentationKey": "p:f98d5449b02005574d598c75f1a72e8ffd2f027d2f512d3dc0eff45b1088a9c6",
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
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:23efc2fc276ff4c60b439c67bd31ee4dc204ecd6e475de8ee012000ae59d8f58",
        "reference": 10,
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
        "owner": 1,
        "presentationKey": "p:ddb9f35b32b57ffbc6872e1398a883dd55a76e88f4a5c783216cab78969c8284",
        "reference": 19,
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
        "presentationKey": "p:f448800cc84c2c1fbc207d56408552d6b7cd70319d1dfca71955008215284e91",
        "reference": 13,
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
        "direction": "inout",
        "featureDirection": null,
        "metaclass": "ReferenceUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "reference-usage",
        "owner": 3,
        "presentationKey": "p:cd8ac86f9a17ef442d063750413431a8daf42d5948a73836f9d54e59751f6471",
        "reference": 22,
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
        "metaclass": "ReferenceUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "reference-usage",
        "owner": 1,
        "presentationKey": "p:b257873dcbc7a225ceaa81ea162fb4ee88df8536d645a1aff473e8177c35850c",
        "reference": 16,
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
        "direction": "in",
        "featureDirection": null,
        "metaclass": "ReferenceUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "reference-usage",
        "owner": 1,
        "presentationKey": "p:08ffb33969d0ded6432f5e6baf9fd81c9137e54f7df8f561dcfa8497713cf490",
        "reference": 16,
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
        "direction": "in",
        "featureDirection": null,
        "metaclass": "ReferenceUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "reference-usage",
        "owner": 1,
        "presentationKey": "p:01b4a2681cf49f5d1179ea01d05fd5039ad8337c9e9be3435878c69251e28fa8",
        "reference": 16,
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
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:4d45fe68e9df14e9e3431fd9efd6084151577a3a449ca90eb80fc652f5d36da3",
        "reference": 11,
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
        "owner": 8,
        "presentationKey": "p:6ed9b4a82c81a3e199a8192749b9121271a1abeb2b22872a04ccc5fadc507cf5",
        "reference": 20,
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
        "presentationKey": "p:2b097519b4325c2abd1c1d8021426d9b870b368f1d59b01c0839af30eef6150e",
        "reference": 14,
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
        "featureDirection": null,
        "metaclass": "ReferenceUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "reference-usage",
        "owner": 10,
        "presentationKey": "p:da6c084c6de38f5afc76c3f7ab05703637c462e301351c992e6c1133a68ea187",
        "reference": 23,
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
        "owner": 8,
        "presentationKey": "p:b395869f399bd0e160a31a906f8d6e879cc6b5e1cbedd5398941e301507940fe",
        "reference": 17,
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
        "featureDirection": null,
        "metaclass": "ReferenceUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "reference-usage",
        "owner": 8,
        "presentationKey": "p:41153ab6aea0a4066e5abc0f875bcb41c6b065559377b2b70c8246ccc2fd8d9b",
        "reference": 17,
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
        "featureDirection": null,
        "metaclass": "ReferenceUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "reference-usage",
        "owner": 8,
        "presentationKey": "p:545c5c80993a6eb12770f704e4be1c3412511bfb527b673c4178ce75e769dfcd",
        "reference": 17,
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
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:3b0ee66dd3dad49dbbd8bf6b8fe46812a2bd7b16be930718a1cc6f0f279e9037",
        "reference": 12,
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
        "owner": 15,
        "presentationKey": "p:441822b1a2f99e6d39bbec705b02ab42126838825a9a0918a66a6c37b27fb7e1",
        "reference": 21,
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
        "presentationKey": "p:e04206c508357e2651030bf983820834de7762d72df33d580b68f7be2e2ca6e9",
        "reference": 15,
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
        "presentationKey": "p:1503e6e69ba55b151e875b2d001e2e932b8924bfc1888b3e13acb35c7145a71a",
        "reference": 24,
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
        "metaclass": "ReferenceUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "reference-usage",
        "owner": 15,
        "presentationKey": "p:64ceb8bf778371da47b5dd091971a4c45a217b8798f0d717963ebab28fb19f23",
        "reference": 18,
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
        "presentationKey": "p:83bb0a6ef9656180fe23173111a4277be8f5d21d98bad9dfe84a9a5043570e81",
        "reference": 18,
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
        "presentationKey": "p:8065c9caf34c29585da1b9b884622d3508a6f5e4f893d0c3afdbd35eea05bab1",
        "reference": 18,
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
        "presentationKey": "p:39f8bdf17c450bf564ee6bbe1f7f055d5b01950c4c7e447d8131b3c64d88dea3",
        "reference": 9,
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
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:391b926865aaa0dfd0a46f5b7623a133bda262bb1c2727ad6d4b19794a24df65",
        "reference": 8,
        "source": 1,
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
        "name": "a",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:e00806b610288db523bade6eb993ef460931c947fa4807a8007ac579958c971f",
        "reference": 4,
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
        "multiplicity": null,
        "name": "b",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:50e9c0dd55608b9a65069ff842ed00195c81b687af610133ab4dcc6815687e25",
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
        "name": "c",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:c663e468572b2115c86be47a6631b6b5a4dc3833d9926b9ca698cc4f479bb80c",
        "reference": 6,
        "source": 15,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      }
    ],
    "relationships": [
      {
        "kind": "transitionTarget",
        "navigation": 7,
        "provenance": "authored",
        "reference": 50,
        "source": 1,
        "target": {
          "node": 25,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTrigger",
        "navigation": 6,
        "provenance": "authored",
        "reference": 53,
        "source": 1,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 7,
        "provenance": "authored",
        "reference": 59,
        "source": 2,
        "target": {
          "node": 25,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTarget",
        "navigation": 10,
        "provenance": "authored",
        "reference": 51,
        "source": 8,
        "target": {
          "node": 26,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTrigger",
        "navigation": 9,
        "provenance": "authored",
        "reference": 54,
        "source": 8,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 10,
        "provenance": "authored",
        "reference": 60,
        "source": 9,
        "target": {
          "node": 26,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTarget",
        "navigation": 14,
        "provenance": "authored",
        "reference": 52,
        "source": 15,
        "target": {
          "node": 26,
          "status": "resolved"
        }
      },
      {
        "kind": "transitionTrigger",
        "navigation": 13,
        "provenance": "authored",
        "reference": 55,
        "source": 15,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "succession",
        "navigation": 14,
        "provenance": "authored",
        "reference": 61,
        "source": 16,
        "target": {
          "node": 26,
          "status": "resolved"
        }
      },
      {
        "kind": "initialState",
        "navigation": 3,
        "provenance": "authored",
        "reference": 49,
        "source": 22,
        "target": {
          "node": 24,
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
          "navigation": 7,
          "provenance": "implied",
          "source": 1,
          "target": 2,
          "trigger": {
            "label": "Go",
            "navigation": 6,
            "status": "accept",
            "target": {
              "id": "element/v141:memory://snapshot/transition_shorthand.md7:packagen9:Shorthand1:014:occurrence-defn2:Go1:0",
              "label": "Go"
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
          "navigation": 10,
          "provenance": "implied",
          "source": 1,
          "target": 3,
          "trigger": {
            "label": "Go",
            "navigation": 9,
            "status": "accept",
            "target": {
              "id": "element/v141:memory://snapshot/transition_shorthand.md7:packagen9:Shorthand1:014:occurrence-defn2:Go1:0",
              "label": "Go"
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
          "label": null,
          "navigation": 14,
          "provenance": "implied",
          "source": 2,
          "target": 3,
          "trigger": {
            "label": "Go",
            "navigation": 13,
            "status": "accept",
            "target": {
              "id": "element/v141:memory://snapshot/transition_shorthand.md7:packagen9:Shorthand1:014:occurrence-defn2:Go1:0",
              "label": "Go"
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
          "label": null,
          "navigation": 3,
          "provenance": "authored",
          "source": 0,
          "target": 1,
          "trigger": {
            "status": "absent"
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
          "label": "a",
          "navigation": 4
        },
        {
          "id": "state-2",
          "kind": "state",
          "label": "b",
          "navigation": 11
        },
        {
          "id": "state-3",
          "kind": "state",
          "label": "c",
          "navigation": 15
        }
      ]
    }
  }
}

~~~
