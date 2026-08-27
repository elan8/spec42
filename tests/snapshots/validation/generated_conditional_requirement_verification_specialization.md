# META
~~~ini
description=Generated requirement-verification specialization uses the canonical RequirementVerificationMembership role
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.21.9:checkRequirementUsageRequirementVerificationSpecialization
type=file
libraries=standard
~~~
# SOURCE
~~~sysml
package RequirementVerificationSpecialization {
    requirement def Limit;
    verification def VerificationCase {
        objective {
            verify requirement limit : Limit;
        }
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind subsetting) (source "RequirementVerificationSpecialization::VerificationCase::objective::limit") (target "VerificationCases::VerificationCase::obj::requirementVerifications") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_conditional_requirement_verification_specialization.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:f1756d70be373e96029d343d6c0b26ada3c50aa43c53d9d4931405ed0c600cb1") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_conditional_requirement_verification_specialization.md") (qualified-name "RequirementVerificationSpecialization"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_requirement_verification_specialization.md") (qualified-name "RequirementVerificationSpecialization::Limit"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_requirement_verification_specialization.md") (qualified-name "RequirementVerificationSpecialization::VerificationCase"))) (kind verification-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_requirement_verification_specialization.md") (qualified-name "RequirementVerificationSpecialization::VerificationCase::objective"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_requirement_verification_specialization.md") (qualified-name "RequirementVerificationSpecialization::VerificationCase::objective::limit"))) (kind verify-requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Limit")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/generated_conditional_requirement_verification_specialization.md") (qualified-name "RequirementVerificationSpecialization::VerificationCase::objective::limit"))) (kind featureTyping) (ordinal 0))
      (authored-target "Limit")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_requirement_verification_specialization.md") (qualified-name "RequirementVerificationSpecialization::Limit")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/generated_conditional_requirement_verification_specialization.md") (qualified-name "RequirementVerificationSpecialization::VerificationCase::objective::limit"))) (target (node (document "memory://snapshot/generated_conditional_requirement_verification_specialization.md") (qualified-name "RequirementVerificationSpecialization::Limit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_requirement_verification_specialization.md") (qualified-name "RequirementVerificationSpecialization::VerificationCase::objective::limit"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_requirement_verification_specialization.md") (qualified-name "RequirementVerificationSpecialization::Limit"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_requirement_verification_specialization.md") (qualified-name "RequirementVerificationSpecialization::VerificationCase"))) (target (node (document "memory://snapshot/sysml.library/verification_cases.md") (qualified-name "VerificationCases::VerificationCase"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_requirement_verification_specialization.md") (qualified-name "RequirementVerificationSpecialization::VerificationCase::objective"))) (target (node (document "memory://snapshot/generated_conditional_requirement_verification_specialization.md") (qualified-name "RequirementVerificationSpecialization::VerificationCase"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_requirement_verification_specialization.md") (qualified-name "RequirementVerificationSpecialization::VerificationCase::objective"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_requirement_verification_specialization.md") (qualified-name "RequirementVerificationSpecialization::VerificationCase::objective::limit"))) (target (node (document "memory://snapshot/generated_conditional_requirement_verification_specialization.md") (qualified-name "RequirementVerificationSpecialization::VerificationCase::objective"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_requirement_verification_specialization.md") (qualified-name "RequirementVerificationSpecialization::VerificationCase::objective::limit"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_requirement_verification_specialization.md") (qualified-name "RequirementVerificationSpecialization::VerificationCase::objective::limit"))) (target (node (document "memory://snapshot/sysml.library/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::obj::requirementVerifications"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_conditional_requirement_verification_specialization.md") (qualified-name "RequirementVerificationSpecialization::Limit")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/generated_conditional_requirement_verification_specialization.md") (qualified-name "RequirementVerificationSpecialization::VerificationCase::objective::limit")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_requirement_verification_specialization.md") (qualified-name "RequirementVerificationSpecialization::VerificationCase")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/calculations.md") (qualified-name "Calculations::Calculation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/cases.md") (qualified-name "Cases::Case")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/verification_cases.md") (qualified-name "VerificationCases::VerificationCase")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_requirement_verification_specialization.md") (qualified-name "RequirementVerificationSpecialization::VerificationCase::objective")))
      (featured-by (node (document "memory://snapshot/generated_conditional_requirement_verification_specialization.md") (qualified-name "RequirementVerificationSpecialization::VerificationCase")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (source inherited) (from (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (source inherited) (from (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks"))))
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
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_requirement_verification_specialization.md") (qualified-name "RequirementVerificationSpecialization::VerificationCase::objective::limit")))
      (featured-by (node (document "memory://snapshot/generated_conditional_requirement_verification_specialization.md") (qualified-name "RequirementVerificationSpecialization::VerificationCase::objective")))
      (type (node (document "memory://snapshot/generated_conditional_requirement_verification_specialization.md") (qualified-name "RequirementVerificationSpecialization::Limit")) (provenance authored))
      (effective-type (node (document "memory://snapshot/generated_conditional_requirement_verification_specialization.md") (qualified-name "RequirementVerificationSpecialization::Limit")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (source inherited) (from (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (effective-type (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (source inherited) (from (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (source inherited) (from (node (document "memory://snapshot/sysml.library/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::obj::requirementVerifications"))))
      (supertype (node (document "memory://snapshot/generated_conditional_requirement_verification_specialization.md") (qualified-name "RequirementVerificationSpecialization::Limit")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::enclosedPerformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::subperformances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::constraints")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::subrequirements")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck::constraints")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/verification_cases.md") (qualified-name "VerificationCases::VerificationCase::obj::requirementVerifications")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/generated_conditional_requirement_verification_specialization.md") (range (start 4 39) (end 4 44)) (probe (position 4 39))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_requirement_verification_specialization.md") (qualified-name "RequirementVerificationSpecialization::VerificationCase::objective::limit"))) (kind featureTyping) (ordinal 0) (authored-target "Limit")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_requirement_verification_specialization.md") (qualified-name "RequirementVerificationSpecialization::Limit")))))
    )
  )
)
~~~
