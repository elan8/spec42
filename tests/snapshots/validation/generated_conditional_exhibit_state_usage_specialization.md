# META
~~~ini
description=An exhibited state of a part definition specializes Parts::Part::exhibitedStates and keeps the StateUsage default States::stateActions
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.18.2:checkExhibitStateUsageSpecialization
type=file
libraries=standard
~~~
# SOURCE
~~~sysml
package ExhibitStateSpecialization {
    state def Operating;
    part def Vehicle {
        exhibit state operating : Operating;
    }
    // Not owned by a part definition or usage: only the StateUsage default applies.
    exhibit state standalone : Operating;
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind subsetting) (source "ExhibitStateSpecialization::Vehicle::operating") (target "Parts::Part::exhibitedStates") (provenance implied) (outcome resolved))
  (relationship (kind subsetting) (source "ExhibitStateSpecialization::Vehicle::operating") (target "States::stateActions") (provenance implied) (outcome resolved))
  (relationship (kind subsetting) (source "ExhibitStateSpecialization::standalone") (target "States::stateActions") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_conditional_exhibit_state_usage_specialization.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:3d9b590f381ff1247fbd011ff2834834a2251361bd6ccb27c02438e1e416cd15") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_conditional_exhibit_state_usage_specialization.md") (qualified-name "ExhibitStateSpecialization"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_exhibit_state_usage_specialization.md") (qualified-name "ExhibitStateSpecialization::Operating"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_exhibit_state_usage_specialization.md") (qualified-name "ExhibitStateSpecialization::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_exhibit_state_usage_specialization.md") (qualified-name "ExhibitStateSpecialization::Vehicle::operating"))) (kind exhibit-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Operating")))))
    (declaration (id (node (document "memory://snapshot/generated_conditional_exhibit_state_usage_specialization.md") (qualified-name "ExhibitStateSpecialization::standalone"))) (kind exhibit-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Operating")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/generated_conditional_exhibit_state_usage_specialization.md") (qualified-name "ExhibitStateSpecialization::Vehicle::operating"))) (kind featureTyping) (ordinal 0))
      (authored-target "Operating")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_exhibit_state_usage_specialization.md") (qualified-name "ExhibitStateSpecialization::Operating")))))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_exhibit_state_usage_specialization.md") (qualified-name "ExhibitStateSpecialization::standalone"))) (kind featureTyping) (ordinal 0))
      (authored-target "Operating")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_exhibit_state_usage_specialization.md") (qualified-name "ExhibitStateSpecialization::Operating")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/generated_conditional_exhibit_state_usage_specialization.md") (qualified-name "ExhibitStateSpecialization::Vehicle::operating"))) (target (node (document "memory://snapshot/generated_conditional_exhibit_state_usage_specialization.md") (qualified-name "ExhibitStateSpecialization::Operating"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_exhibit_state_usage_specialization.md") (qualified-name "ExhibitStateSpecialization::Vehicle::operating"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/generated_conditional_exhibit_state_usage_specialization.md") (qualified-name "ExhibitStateSpecialization::standalone"))) (target (node (document "memory://snapshot/generated_conditional_exhibit_state_usage_specialization.md") (qualified-name "ExhibitStateSpecialization::Operating"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_exhibit_state_usage_specialization.md") (qualified-name "ExhibitStateSpecialization::standalone"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_exhibit_state_usage_specialization.md") (qualified-name "ExhibitStateSpecialization::Operating"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_exhibit_state_usage_specialization.md") (qualified-name "ExhibitStateSpecialization::Vehicle"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_exhibit_state_usage_specialization.md") (qualified-name "ExhibitStateSpecialization::Vehicle::operating"))) (target (node (document "memory://snapshot/generated_conditional_exhibit_state_usage_specialization.md") (qualified-name "ExhibitStateSpecialization::Vehicle"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_exhibit_state_usage_specialization.md") (qualified-name "ExhibitStateSpecialization::Vehicle::operating"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::exhibitedStates"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_exhibit_state_usage_specialization.md") (qualified-name "ExhibitStateSpecialization::Vehicle::operating"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_exhibit_state_usage_specialization.md") (qualified-name "ExhibitStateSpecialization::standalone"))) (target (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_conditional_exhibit_state_usage_specialization.md") (qualified-name "ExhibitStateSpecialization::Operating")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::DecisionPerformance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/generated_conditional_exhibit_state_usage_specialization.md") (qualified-name "ExhibitStateSpecialization::Vehicle::operating")) (scopes any))
      (subtype (node (document "memory://snapshot/generated_conditional_exhibit_state_usage_specialization.md") (qualified-name "ExhibitStateSpecialization::standalone")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_exhibit_state_usage_specialization.md") (qualified-name "ExhibitStateSpecialization::Vehicle")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_exhibit_state_usage_specialization.md") (qualified-name "ExhibitStateSpecialization::Vehicle::operating")))
      (featured-by (node (document "memory://snapshot/generated_conditional_exhibit_state_usage_specialization.md") (qualified-name "ExhibitStateSpecialization::Vehicle")))
      (type (node (document "memory://snapshot/generated_conditional_exhibit_state_usage_specialization.md") (qualified-name "ExhibitStateSpecialization::Operating")) (provenance authored))
      (effective-type (node (document "memory://snapshot/generated_conditional_exhibit_state_usage_specialization.md") (qualified-name "ExhibitStateSpecialization::Operating")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::performedActions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::involvingPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::enactedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::exhibitedStates"))))
      (effective-type (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions"))))
      (supertype (node (document "memory://snapshot/generated_conditional_exhibit_state_usage_specialization.md") (qualified-name "ExhibitStateSpecialization::Operating")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::DecisionPerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::enactedPerformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::involvingPerformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::exhibitedStates")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::performedActions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/state_performances.md") (qualified-name "StatePerformances::StatePerformance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_exhibit_state_usage_specialization.md") (qualified-name "ExhibitStateSpecialization::standalone")))
      (type (node (document "memory://snapshot/generated_conditional_exhibit_state_usage_specialization.md") (qualified-name "ExhibitStateSpecialization::Operating")) (provenance authored))
      (effective-type (node (document "memory://snapshot/generated_conditional_exhibit_state_usage_specialization.md") (qualified-name "ExhibitStateSpecialization::Operating")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::StateAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/states.md") (qualified-name "States::stateActions"))))
      (supertype (node (document "memory://snapshot/generated_conditional_exhibit_state_usage_specialization.md") (qualified-name "ExhibitStateSpecialization::Operating")) (scopes any))
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
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/generated_conditional_exhibit_state_usage_specialization.md") (range (start 3 34) (end 3 43)) (probe (position 3 34))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_exhibit_state_usage_specialization.md") (qualified-name "ExhibitStateSpecialization::Vehicle::operating"))) (kind featureTyping) (ordinal 0) (authored-target "Operating")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_exhibit_state_usage_specialization.md") (qualified-name "ExhibitStateSpecialization::Operating")))))
    )
  )
  (query (document "memory://snapshot/generated_conditional_exhibit_state_usage_specialization.md") (range (start 6 31) (end 6 40)) (probe (position 6 31))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_exhibit_state_usage_specialization.md") (qualified-name "ExhibitStateSpecialization::standalone"))) (kind featureTyping) (ordinal 0) (authored-target "Operating")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_exhibit_state_usage_specialization.md") (qualified-name "ExhibitStateSpecialization::Operating")))))
    )
  )
)
~~~
