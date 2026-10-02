# META
~~~ini
description=SysML 8.3.17.5 checkAssignmentActionUsageReferentRedefinition is unresolved, not satisfied, while an assignment referent does not settle
specification=OMG SysML 2.0 (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.17.5:checkAssignmentActionUsageReferentRedefinition
coverage_role=secondary
type=file
libraries=standard
~~~
# SOURCE
~~~sysml
package Redefinition {
    action def Work {
        // The referent names nothing, so what the accessed feature must redefine is unknown.
        assign missing := 1;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics (redefinition-check (rule_id "sysml-2.0:8.3.17.5:checkAssignmentActionUsageReferentRedefinition") (outcome unresolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_assignment_action_usage_referent_redefinition_unresolved.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 3 15) (end 3 22))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:745e755d702f466c4c36027ffd1aa21307eeec6c57db3ea9ff007b2406a65bf5") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage_referent_redefinition_unresolved.md") (qualified-name "Redefinition"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage_referent_redefinition_unresolved.md") (qualified-name "Redefinition::Work"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage_referent_redefinition_unresolved.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind assign) (ordinal 0))))) (kind assign) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (assignTarget (reference "missing")))))
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage_referent_redefinition_unresolved.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage_referent_redefinition_unresolved.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage_referent_redefinition_unresolved.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_assignment_action_usage_referent_redefinition_unresolved.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind assign) (ordinal 0))))) (kind assignTarget) (ordinal 0))
      (authored-target "missing")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_assignment_action_usage_referent_redefinition_unresolved.md") (qualified-name "Redefinition::Work"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_assignment_action_usage_referent_redefinition_unresolved.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind assign) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::assignmentActions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_assignment_action_usage_referent_redefinition_unresolved.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind assign) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_assignment_action_usage_referent_redefinition_unresolved.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind assign) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_assignment_action_usage_referent_redefinition_unresolved.md") (qualified-name "Redefinition::Work"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/sysml_assignment_action_usage_referent_redefinition_unresolved.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::assignmentActions::target"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_assignment_action_usage_referent_redefinition_unresolved.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_assignment_action_usage_referent_redefinition_unresolved.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind assign) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_assignment_action_usage_referent_redefinition_unresolved.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/sysml_assignment_action_usage_referent_redefinition_unresolved.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance::onOccurrence::startingAt"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_assignment_action_usage_referent_redefinition_unresolved.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_assignment_action_usage_referent_redefinition_unresolved.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/sysml_assignment_action_usage_referent_redefinition_unresolved.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance::onOccurrence::startingAt::accessedFeature"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_assignment_action_usage_referent_redefinition_unresolved.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_assignment_action_usage_referent_redefinition_unresolved.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/sysml_assignment_action_usage_referent_redefinition_unresolved.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind assign) (ordinal 0))))) (state literal) (value (kind integer) (integer 1)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage_referent_redefinition_unresolved.md") (qualified-name "Redefinition::Work")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage_referent_redefinition_unresolved.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind assign) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_assignment_action_usage_referent_redefinition_unresolved.md") (qualified-name "Redefinition::Work")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::AssignmentAction")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::assignmentActions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
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
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage_referent_redefinition_unresolved.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_assignment_action_usage_referent_redefinition_unresolved.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind assign) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::AssignmentAction::target"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::assignmentActions::target"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureReferencingPerformance::onOccurrence"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance::onOccurrence"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureWritePerformance::onOccurrence"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
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
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage_referent_redefinition_unresolved.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_assignment_action_usage_referent_redefinition_unresolved.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage_referent_redefinition_unresolved.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_assignment_action_usage_referent_redefinition_unresolved.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind assign) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance::onOccurrence::startingAt::accessedFeature"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/feature_referencing_performances.md") (qualified-name "FeatureReferencingPerformances::FeatureAccessPerformance::onOccurrence::startingAt::accessedFeature")) (scopes any feature))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/sysml_assignment_action_usage_referent_redefinition_unresolved.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind assign) (ordinal 0))))) (outcome resolved) (literal (value (kind integer) (integer 1))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_assignment_action_usage_referent_redefinition_unresolved.md") (range (start 3 15) (end 3 22)) (probe (position 3 15))
    (reference (id (source (node (document "memory://snapshot/sysml_assignment_action_usage_referent_redefinition_unresolved.md") (path (named (kind package) (name "Redefinition")) (named (kind action-def) (name "Work")) (anonymous (kind assign) (ordinal 0))))) (kind assignTarget) (ordinal 0) (authored-target "missing")
      (outcome (status unresolved)))
    )
  )
)
~~~
