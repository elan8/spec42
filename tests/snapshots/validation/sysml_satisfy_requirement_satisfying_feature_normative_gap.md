# META
~~~ini
description=SysML 8.3.21.10 deriveSatisfyRequirementUsageSatisfyingFeature calls an operation OCL does not define in its pinned body
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=sysml-2.0:8.3.21.10:deriveSatisfyRequirementUsageSatisfyingFeature
blocked_by=normative-specification-gap-satisfy-requirement-satisfying-feature-body
libraries=standard
type=file
~~~
# SOURCE
~~~sysml
package Satisfaction {
    part def Vehicle;
    requirement def MassLimit {
        subject vehicle : Vehicle;
    }
    requirement massLimit : MassLimit;
    part car : Vehicle;
    satisfy requirement carMass : MassLimit by car;
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (action-derived-fact (rule_id "sysml-2.0:8.3.21.10:deriveSatisfyRequirementUsageSatisfyingFeature") (source "Satisfaction::carMass") (target "Satisfaction::car") (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:31ba205d2038f7d4b4fc2a6f0ab0bd54623f395362473c6de4d32ab3c78a01a8") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::MassLimit"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::MassLimit::vehicle"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::car"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::carMass"))) (kind satisfy) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassLimit")) (satisfyTarget (reference "car")))))
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::massLimit"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassLimit")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::MassLimit::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::car"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::carMass"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassLimit")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::MassLimit")))))
    (reference (id (source (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::carMass"))) (kind satisfyTarget) (ordinal 0))
      (authored-target "car")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::car")))))
    (reference (id (source (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::massLimit"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassLimit")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::MassLimit")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::MassLimit::vehicle"))) (target (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::MassLimit::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::car"))) (target (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::car"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::carMass"))) (target (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::MassLimit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::carMass"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind satisfyTarget) (source (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::carMass"))) (target (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::car"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::carMass"))) (kind satisfyTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::massLimit"))) (target (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::MassLimit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::massLimit"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::MassLimit"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::MassLimit::vehicle"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::subj"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::MassLimit::vehicle"))) (target (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::MassLimit"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::Vehicle"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::car"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::carMass"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::carMass"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::satisfiedRequirementChecks"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::massLimit"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::MassLimit")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::carMass")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::massLimit")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::MassLimit::vehicle")))
      (featured-by (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::MassLimit")))
      (type (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::subj"))))
      (effective-type (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::subj")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::Vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::Vehicle")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::MassLimit::vehicle")) (scopes any))
      (subtype (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::car")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::car")))
      (type (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::Vehicle")) (source direct))
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
      (supertype (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::Vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::carMass")))
      (type (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::MassLimit")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (source inherited) (from (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (source inherited) (from (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks"))))
      (effective-type (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::MassLimit")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::assertedConstraintChecks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::trueEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::satisfiedRequirementChecks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::MassLimit")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::massLimit")))
      (type (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::MassLimit")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (source inherited) (from (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (source inherited) (from (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks"))))
      (effective-type (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::MassLimit")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::MassLimit")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (range (start 3 26) (end 3 33)) (probe (position 3 26))
    (reference (id (source (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::MassLimit::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (range (start 6 15) (end 6 22)) (probe (position 6 15))
    (reference (id (source (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::car"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (range (start 7 34) (end 7 43)) (probe (position 7 34))
    (reference (id (source (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::carMass"))) (kind featureTyping) (ordinal 0) (authored-target "MassLimit")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::MassLimit")))))
    )
  )
  (query (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (range (start 7 47) (end 7 50)) (probe (position 7 47))
    (reference (id (source (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::carMass"))) (kind satisfyTarget) (ordinal 0) (authored-target "car")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::car")))))
    )
  )
  (query (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (range (start 5 28) (end 5 37)) (probe (position 5 28))
    (reference (id (source (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::massLimit"))) (kind featureTyping) (ordinal 0) (authored-target "MassLimit")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_satisfy_requirement_satisfying_feature_normative_gap.md") (qualified-name "Satisfaction::MassLimit")))))
    )
  )
)
~~~
