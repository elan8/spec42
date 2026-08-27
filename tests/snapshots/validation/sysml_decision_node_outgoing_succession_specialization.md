# META
~~~ini
description=SysML checkDecisionNodeOutgoingSuccessionSpecialization desired semantics
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.17.7:checkDecisionNodeOutgoingSuccessionSpecialization
libraries=standard
type=file
~~~
# SOURCE
~~~sysml
package Model {
    action def Act {
        action before;
        action after;
        decide choice;

        succession incoming first before then choice;
        succession outgoing first choice then after;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind subsetting) (source "Model::Act::outgoing") (target "ControlPerformances::DecisionPerformance::outgoingHBLink") (provenance implied) (outcome resolved))
  (specialization-check (rule_id "sysml-2.0:8.3.17.7:checkDecisionNodeOutgoingSuccessionSpecialization") (outcome satisfied)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:eeb1d5e65d1fe7ca792368e05695bd699f86da3fd83f2e004c5ffd016af83b4d") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::after"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::before"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::choice"))) (kind decide) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::incoming"))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "before")) (succession (reference "choice")))))
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::outgoing"))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "choice")) (succession (reference "after")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::incoming"))) (kind succession) (ordinal 0))
      (authored-target "before")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::before")))))
    (reference (id (source (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::incoming"))) (kind succession) (ordinal 1))
      (authored-target "choice")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::choice")))))
    (reference (id (source (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::outgoing"))) (kind succession) (ordinal 0))
      (authored-target "choice")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::choice")))))
    (reference (id (source (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::outgoing"))) (kind succession) (ordinal 1))
      (authored-target "after")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::after")))))
  )
  (relationships
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::incoming"))) (target (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::before"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::incoming"))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::incoming"))) (target (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::choice"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::incoming"))) (kind succession) (ordinal 1)))
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::outgoing"))) (target (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::choice"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::outgoing"))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::outgoing"))) (target (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::after"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::outgoing"))) (kind succession) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::after"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::after"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::after"))) (target (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::before"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::before"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::before"))) (target (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::choice"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::decisions"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::choice"))) (target (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::incoming"))) (target (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::outgoing"))) (target (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::DecisionPerformance::outgoingHBLink"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::outgoing"))) (target (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::after")))
      (featured-by (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act")))
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
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::before")))
      (featured-by (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act")))
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
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::choice")))
      (featured-by (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::ControlAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::controls"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::DecisionAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::decisions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::controls")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::decisions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action::subactions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::ControlAction")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::DecisionAction")) (scopes any))
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
    )
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::incoming")))
      (featured-by (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::outgoing")))
      (featured-by (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::HappensBefore")) (source inherited) (from (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::DecisionPerformance::outgoingHBLink"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/control_performances.md") (qualified-name "ControlPerformances::DecisionPerformance::outgoingHBLink")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::HappensBefore")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::HappensLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Without")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (range (start 6 34) (end 6 40)) (probe (position 6 34))
    (reference (id (source (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::incoming"))) (kind succession) (ordinal 0) (authored-target "before")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::before")))))
    )
  )
  (query (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (range (start 6 46) (end 6 52)) (probe (position 6 46))
    (reference (id (source (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::incoming"))) (kind succession) (ordinal 1) (authored-target "choice")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::choice")))))
    )
  )
  (query (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (range (start 7 34) (end 7 40)) (probe (position 7 34))
    (reference (id (source (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::outgoing"))) (kind succession) (ordinal 0) (authored-target "choice")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::choice")))))
    )
  )
  (query (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (range (start 7 46) (end 7 51)) (probe (position 7 46))
    (reference (id (source (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::outgoing"))) (kind succession) (ordinal 1) (authored-target "after")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_decision_node_outgoing_succession_specialization.md") (qualified-name "Model::Act::after")))))
    )
  )
)
~~~
