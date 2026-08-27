# META
~~~ini
description=Generated library-specialization checks publish implied anchors for requirement and case definitions
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.21.3:checkConcernDefinitionSpecialization
rule_id=sysml-2.0:8.3.21.8:checkRequirementDefinitionSpecialization
rule_id=sysml-2.0:8.3.22.2:checkCaseDefinitionSpecialization
rule_id=sysml-2.0:8.3.23.2:checkAnalysisCaseDefinitionSpecialization
rule_id=sysml-2.0:8.3.24.3:checkVerificationCaseSpecialization
rule_id=sysml-2.0:8.3.25.3:checkUseCaseDefinitionSpecialization
type=file
libraries=standard
~~~
# SOURCE
~~~sysml
package GeneratedCaseDefinitions {
    concern def ConcernDefinition;
    requirement def RequirementDefinition;
    case def CaseDefinition;
    analysis def AnalysisCaseDefinition;
    verification def VerificationCaseDefinition;
    use case def UseCaseDefinition;
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind specialization) (source "GeneratedCaseDefinitions::ConcernDefinition") (target "Requirements::ConcernCheck") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "GeneratedCaseDefinitions::RequirementDefinition") (target "Requirements::RequirementCheck") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "GeneratedCaseDefinitions::CaseDefinition") (target "Cases::Case") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "GeneratedCaseDefinitions::AnalysisCaseDefinition") (target "AnalysisCases::AnalysisCase") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "GeneratedCaseDefinitions::VerificationCaseDefinition") (target "VerificationCases::VerificationCase") (provenance implied) (outcome resolved))
  (relationship (kind specialization) (source "GeneratedCaseDefinitions::UseCaseDefinition") (target "UseCases::UseCase") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_library_specialization_case_definitions.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:73b2efda406fa599ce472b7acc106b37617626014b219d684f91d923147b6a0b") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_case_definitions.md") (qualified-name "GeneratedCaseDefinitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_case_definitions.md") (qualified-name "GeneratedCaseDefinitions::AnalysisCaseDefinition"))) (kind analysis-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_case_definitions.md") (qualified-name "GeneratedCaseDefinitions::CaseDefinition"))) (kind case-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_case_definitions.md") (qualified-name "GeneratedCaseDefinitions::ConcernDefinition"))) (kind concern-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_case_definitions.md") (qualified-name "GeneratedCaseDefinitions::RequirementDefinition"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_case_definitions.md") (qualified-name "GeneratedCaseDefinitions::UseCaseDefinition"))) (kind use-case-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_case_definitions.md") (qualified-name "GeneratedCaseDefinitions::VerificationCaseDefinition"))) (kind verification-def) (membership (kind owning) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_case_definitions.md") (qualified-name "GeneratedCaseDefinitions::AnalysisCaseDefinition"))) (target (node (document "memory://snapshot/sysml.library/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_case_definitions.md") (qualified-name "GeneratedCaseDefinitions::CaseDefinition"))) (target (node (document "memory://snapshot/sysml.library/cases.md") (qualified-name "Cases::Case"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_case_definitions.md") (qualified-name "GeneratedCaseDefinitions::ConcernDefinition"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::ConcernCheck"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_case_definitions.md") (qualified-name "GeneratedCaseDefinitions::RequirementDefinition"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_case_definitions.md") (qualified-name "GeneratedCaseDefinitions::UseCaseDefinition"))) (target (node (document "memory://snapshot/sysml.library/use_cases.md") (qualified-name "UseCases::UseCase"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_library_specialization_case_definitions.md") (qualified-name "GeneratedCaseDefinitions::VerificationCaseDefinition"))) (target (node (document "memory://snapshot/sysml.library/verification_cases.md") (qualified-name "VerificationCases::VerificationCase"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_case_definitions.md") (qualified-name "GeneratedCaseDefinitions::AnalysisCaseDefinition")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/analysis_cases.md") (qualified-name "AnalysisCases::AnalysisCase")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/calculations.md") (qualified-name "Calculations::Calculation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/cases.md") (qualified-name "Cases::Case")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_case_definitions.md") (qualified-name "GeneratedCaseDefinitions::CaseDefinition")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/calculations.md") (qualified-name "Calculations::Calculation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/cases.md") (qualified-name "Cases::Case")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_case_definitions.md") (qualified-name "GeneratedCaseDefinitions::ConcernDefinition")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::ConcernCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_case_definitions.md") (qualified-name "GeneratedCaseDefinitions::RequirementDefinition")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_case_definitions.md") (qualified-name "GeneratedCaseDefinitions::UseCaseDefinition")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/calculations.md") (qualified-name "Calculations::Calculation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/cases.md") (qualified-name "Cases::Case")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/use_cases.md") (qualified-name "UseCases::UseCase")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_library_specialization_case_definitions.md") (qualified-name "GeneratedCaseDefinitions::VerificationCaseDefinition")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/calculations.md") (qualified-name "Calculations::Calculation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/cases.md") (qualified-name "Cases::Case")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/verification_cases.md") (qualified-name "VerificationCases::VerificationCase")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
