# META
~~~ini
description=Declared and referencing entry/do/exit actions imply their States::StateAction redefinition alongside any authored one
specification=OMG SysML 2.0 (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.17.4:checkActionUsageStateActionRedefinition
coverage_role=secondary
libraries=standard
~~~
# SOURCE
~~~sysml
package StateActions {
    action def Work;
    action work : Work;
    state def Machine {
        entry work;
        do action operate : Work;
        exit action leave : Work :>> work;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship
    (kind redefinition)
    (source (anonymous (owner "StateActions::Machine") (kind ActionUsage) (ordinal 0)))
    (target "States::StateAction::entryAction")
    (provenance implied)
    (outcome resolved))
  (relationship
    (kind redefinition)
    (source "StateActions::Machine::operate")
    (target "States::StateAction::doAction")
    (provenance implied)
    (outcome resolved))
  (relationship
    (kind redefinition)
    (source "StateActions::Machine::leave")
    (target "States::StateAction::exitAction")
    (provenance implied)
    (outcome resolved))
  (relationship
    (kind redefinition)
    (source "StateActions::Machine::leave")
    (target "StateActions::work")
    (provenance authored)
    (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/state_action_library_redefinitions.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:3442caf5ab437b8a02a1eaa9339009c47198791e80c7263e1b5bba2605f6d21c") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Machine"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_action_library_redefinitions.md") (path (named (kind package) (name "StateActions")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entry-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (entryActionBinding (reference "work")))))
    (declaration (id (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Machine::leave"))) (kind exit-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Work")) (redefinition (reference "work")))))
    (declaration (id (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Machine::operate"))) (kind do-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Work")))))
    (declaration (id (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Work"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::work"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Work")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/state_action_library_redefinitions.md") (path (named (kind package) (name "StateActions")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0))
      (authored-target "work")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::work")))))
    (reference (id (source (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Machine::leave"))) (kind featureTyping) (ordinal 0))
      (authored-target "Work")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Work")))))
    (reference (id (source (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Machine::leave"))) (kind redefinition) (ordinal 0))
      (authored-target "work")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::work")))))
    (reference (id (source (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Machine::operate"))) (kind featureTyping) (ordinal 0))
      (authored-target "Work")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Work")))))
    (reference (id (source (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::work"))) (kind featureTyping) (ordinal 0))
      (authored-target "Work")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Work")))))
  )
  (relationships
    (relationship (kind entryActionBinding) (source (node (document "memory://snapshot/state_action_library_redefinitions.md") (path (named (kind package) (name "StateActions")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::work"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_action_library_redefinitions.md") (path (named (kind package) (name "StateActions")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Machine::leave"))) (target (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Work"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Machine::leave"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Machine::leave"))) (target (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::work"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Machine::leave"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Machine::operate"))) (target (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Work"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Machine::operate"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::work"))) (target (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Work"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::work"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Machine"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/state_action_library_redefinitions.md") (path (named (kind package) (name "StateActions")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/state_action_library_redefinitions.md") (path (named (kind package) (name "StateActions")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/state_action_library_redefinitions.md") (path (named (kind package) (name "StateActions")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Machine"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/state_action_library_redefinitions.md") (path (named (kind package) (name "StateActions")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::entryAction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Machine::leave"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Machine::leave"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Machine::leave"))) (target (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Machine"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Machine::leave"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::exitAction"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Machine::operate"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Machine::operate"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Machine::operate"))) (target (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Machine"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Machine::operate"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::doAction"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Work"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::work"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Machine")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::DecisionPerformance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/state_action_library_redefinitions.md") (path (named (kind package) (name "StateActions")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Machine")))
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
    (declaration (id (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Machine::leave")))
      (featured-by (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Machine")))
      (type (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Work")) (provenance authored))
      (effective-type (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Work")) (source direct))
      (effective-type (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Work")) (source inherited) (from (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::work"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::exitAction"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Work")) (scopes any))
      (supertype (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::work")) (scopes any feature))
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
    (declaration (id (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Machine::operate")))
      (featured-by (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Machine")))
      (type (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Work")) (provenance authored))
      (effective-type (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Work")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::doAction"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Work")) (scopes any))
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
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::do")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance::middle")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction::doAction")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Work")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Machine::leave")) (scopes any))
      (subtype (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Machine::operate")) (scopes any))
      (subtype (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::work")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::work")))
      (type (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Work")) (provenance authored))
      (effective-type (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Work")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Work")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (subtype (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Machine::leave")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/state_action_library_redefinitions.md") (range (start 4 14) (end 4 18)) (probe (position 4 14))
    (reference (id (source (node (document "memory://snapshot/state_action_library_redefinitions.md") (path (named (kind package) (name "StateActions")) (named (kind state-def) (name "Machine")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0) (authored-target "work")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::work")))))
    )
  )
  (query (document "memory://snapshot/state_action_library_redefinitions.md") (range (start 6 28) (end 6 32)) (probe (position 6 28))
    (reference (id (source (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Machine::leave"))) (kind featureTyping) (ordinal 0) (authored-target "Work")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Work")))))
    )
  )
  (query (document "memory://snapshot/state_action_library_redefinitions.md") (range (start 6 37) (end 6 41)) (probe (position 6 37))
    (reference (id (source (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Machine::leave"))) (kind redefinition) (ordinal 0) (authored-target "work")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::work")))))
    )
  )
  (query (document "memory://snapshot/state_action_library_redefinitions.md") (range (start 5 28) (end 5 32)) (probe (position 5 28))
    (reference (id (source (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Machine::operate"))) (kind featureTyping) (ordinal 0) (authored-target "Work")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Work")))))
    )
  )
  (query (document "memory://snapshot/state_action_library_redefinitions.md") (range (start 2 18) (end 2 22)) (probe (position 2 18))
    (reference (id (source (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::work"))) (kind featureTyping) (ordinal 0) (authored-target "Work")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_action_library_redefinitions.md") (qualified-name "StateActions::Work")))))
    )
  )
)
~~~
