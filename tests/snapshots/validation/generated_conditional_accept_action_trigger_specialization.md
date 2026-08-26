# META
~~~ini
description=Generated trigger accept-action specialization uses the canonical TransitionAccept lowering fact
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.17.2:checkAcceptActionUsageTriggerActionSpecialization
blocked_by=semantic-query-gap-anonymous-library-specialization-forms
type=file
libraries=standard
~~~
# SOURCE
~~~sysml
package AcceptActionTriggerSpecialization {
    state def Machine {
        state source;
        state target;
        transition first source accept when true then target;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind specialization) (source "AcceptActionTriggerSpecialization::Machine::<anonymous>::<anonymous>") (target "Actions::TransitionAction::accepter") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_final_state")
        (source "semantic")
        (range (start 1 4) (end 5 5))
      )
      (diagnostic
        (severity information)
        (code "missing_initial_state")
        (source "semantic")
        (range (start 1 4) (end 5 5))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:97241291c79c6611f6cbbabaee0d924fbd1f0fd845630f45961f07b34750609b") (contract-version "feature-chain-expression-result-v10") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (qualified-name "AcceptActionTriggerSpecialization"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (qualified-name "AcceptActionTriggerSpecialization::Machine"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (path (named (kind package) (name "AcceptActionTriggerSpecialization")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "source")) (transitionTarget (reference "target")))))
    (declaration (id (node (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (path (named (kind package) (name "AcceptActionTriggerSpecialization")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (kind accept-action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (qualified-name "AcceptActionTriggerSpecialization::Machine::source"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (qualified-name "AcceptActionTriggerSpecialization::Machine::target"))) (kind state) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (path (named (kind package) (name "AcceptActionTriggerSpecialization")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0))
      (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (qualified-name "AcceptActionTriggerSpecialization::Machine::source")))))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (path (named (kind package) (name "AcceptActionTriggerSpecialization")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0))
      (authored-target "target")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (qualified-name "AcceptActionTriggerSpecialization::Machine::target")))))
  )
  (relationships
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (path (named (kind package) (name "AcceptActionTriggerSpecialization")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (qualified-name "AcceptActionTriggerSpecialization::Machine::source"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (path (named (kind package) (name "AcceptActionTriggerSpecialization")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (path (named (kind package) (name "AcceptActionTriggerSpecialization")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (qualified-name "AcceptActionTriggerSpecialization::Machine::target"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (path (named (kind package) (name "AcceptActionTriggerSpecialization")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (qualified-name "AcceptActionTriggerSpecialization::Machine"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (path (named (kind package) (name "AcceptActionTriggerSpecialization")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (qualified-name "AcceptActionTriggerSpecialization::Machine"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (path (named (kind package) (name "AcceptActionTriggerSpecialization")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (path (named (kind package) (name "AcceptActionTriggerSpecialization")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (path (named (kind package) (name "AcceptActionTriggerSpecialization")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (path (named (kind package) (name "AcceptActionTriggerSpecialization")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::accepter"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (qualified-name "AcceptActionTriggerSpecialization::Machine::source"))) (target (node (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (qualified-name "AcceptActionTriggerSpecialization::Machine"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (qualified-name "AcceptActionTriggerSpecialization::Machine::source"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (qualified-name "AcceptActionTriggerSpecialization::Machine::target"))) (target (node (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (qualified-name "AcceptActionTriggerSpecialization::Machine"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (qualified-name "AcceptActionTriggerSpecialization::Machine::target"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (qualified-name "AcceptActionTriggerSpecialization::Machine")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::DecisionPerformance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (path (named (kind package) (name "AcceptActionTriggerSpecialization")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (qualified-name "AcceptActionTriggerSpecialization::Machine")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::transitionActions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (path (named (kind package) (name "AcceptActionTriggerSpecialization")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)) (anonymous (kind accept-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (path (named (kind package) (name "AcceptActionTriggerSpecialization")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::AcceptMessageAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::TransitionAction::accepter")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::AcceptPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transfers.md") (qualified-name "Transfers::acceptPerformances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/transition_performances.md") (qualified-name "TransitionPerformances::TransitionPerformance::accept")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (qualified-name "AcceptActionTriggerSpecialization::Machine::source")))
      (featured-by (node (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (qualified-name "AcceptActionTriggerSpecialization::Machine")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::DecisionPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (qualified-name "AcceptActionTriggerSpecialization::Machine::target")))
      (featured-by (node (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (qualified-name "AcceptActionTriggerSpecialization::Machine")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::DecisionPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (range (start 4 25) (end 4 31)) (probe (position 4 25))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (path (named (kind package) (name "AcceptActionTriggerSpecialization")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionSource) (ordinal 0) (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (qualified-name "AcceptActionTriggerSpecialization::Machine::source")))))
    )
  )
  (query (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (range (start 4 54) (end 4 60)) (probe (position 4 54))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (path (named (kind package) (name "AcceptActionTriggerSpecialization")) (named (kind state-def) (name "Machine")) (anonymous (kind transition) (ordinal 0))))) (kind transitionTarget) (ordinal 0) (authored-target "target")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_accept_action_trigger_specialization.md") (qualified-name "AcceptActionTriggerSpecialization::Machine::target")))))
    )
  )
)
~~~
