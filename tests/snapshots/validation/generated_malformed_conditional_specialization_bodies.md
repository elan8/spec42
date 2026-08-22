# META
~~~ini
description=Malformed pinned specialization bodies remain visible normative-specification gaps without inferred predicate contracts
specification=OMG SysML 2.0 Language and KerML 1.0 (formal/26-03)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.25.2:checkIncludeUseCaseSpecialization
blocked_by=normative-specification-gap-include-use-case-specialization-body
type=file
libraries=standard
~~~
# SOURCE
~~~sysml
package MalformedConditionalSpecializationBodies {
    use case def Library {
        use case operate;
    }
    use case def Main {
        include Library::operate;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind specialization) (source "MalformedConditionalSpecializationBodies::Main") (target "UseCases::UseCase::includedUseCases") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_malformed_conditional_specialization_bodies.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:6ca4f87f3922aab6deb094e60212090f973f918f966f7bf18641d03ae860e62e") (contract-version "parser-owned-resolution-v1") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_malformed_conditional_specialization_bodies.md") (qualified-name "MalformedConditionalSpecializationBodies"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_malformed_conditional_specialization_bodies.md") (qualified-name "MalformedConditionalSpecializationBodies::Library"))) (kind use-case-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_malformed_conditional_specialization_bodies.md") (qualified-name "MalformedConditionalSpecializationBodies::Library::operate"))) (kind use-case) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_malformed_conditional_specialization_bodies.md") (qualified-name "MalformedConditionalSpecializationBodies::Main"))) (kind use-case-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (includeUseCase (reference "Library::operate")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/generated_malformed_conditional_specialization_bodies.md") (qualified-name "MalformedConditionalSpecializationBodies::Main"))) (kind includeUseCase) (ordinal 0))
      (authored-target "Library::operate")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_malformed_conditional_specialization_bodies.md") (qualified-name "MalformedConditionalSpecializationBodies::Library::operate")))))
  )
  (relationships
    (relationship (kind includeUseCase) (source (node (document "memory://snapshot/generated_malformed_conditional_specialization_bodies.md") (qualified-name "MalformedConditionalSpecializationBodies::Main"))) (target (node (document "memory://snapshot/generated_malformed_conditional_specialization_bodies.md") (qualified-name "MalformedConditionalSpecializationBodies::Library::operate"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_malformed_conditional_specialization_bodies.md") (qualified-name "MalformedConditionalSpecializationBodies::Main"))) (kind includeUseCase) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_malformed_conditional_specialization_bodies.md") (qualified-name "MalformedConditionalSpecializationBodies::Library"))) (target (node (document "memory://snapshot/sysml.library/use_cases.md") (qualified-name "UseCases::UseCase"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_malformed_conditional_specialization_bodies.md") (qualified-name "MalformedConditionalSpecializationBodies::Library::operate"))) (target (node (document "memory://snapshot/generated_malformed_conditional_specialization_bodies.md") (qualified-name "MalformedConditionalSpecializationBodies::Library"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_malformed_conditional_specialization_bodies.md") (qualified-name "MalformedConditionalSpecializationBodies::Library::operate"))) (target (node (document "memory://snapshot/sysml.library/use_cases.md") (qualified-name "UseCases::useCases"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_malformed_conditional_specialization_bodies.md") (qualified-name "MalformedConditionalSpecializationBodies::Main"))) (target (node (document "memory://snapshot/sysml.library/use_cases.md") (qualified-name "UseCases::UseCase"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_malformed_conditional_specialization_bodies.md") (qualified-name "MalformedConditionalSpecializationBodies::Library")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/calculations.md") (qualified-name "Calculations::Calculation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/cases.md") (qualified-name "Cases::Case")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/use_cases.md") (qualified-name "UseCases::UseCase")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_malformed_conditional_specialization_bodies.md") (qualified-name "MalformedConditionalSpecializationBodies::Library::operate")))
      (featured-by (node (document "memory://snapshot/generated_malformed_conditional_specialization_bodies.md") (qualified-name "MalformedConditionalSpecializationBodies::Library")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/calculations.md") (qualified-name "Calculations::Calculation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/calculations.md") (qualified-name "Calculations::calculations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/cases.md") (qualified-name "Cases::Case")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/cases.md") (qualified-name "Cases::cases")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/use_cases.md") (qualified-name "UseCases::UseCase")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/use_cases.md") (qualified-name "UseCases::useCases")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_malformed_conditional_specialization_bodies.md") (qualified-name "MalformedConditionalSpecializationBodies::Main")))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/calculations.md") (qualified-name "Calculations::Calculation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/cases.md") (qualified-name "Cases::Case")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/use_cases.md") (qualified-name "UseCases::UseCase")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/generated_malformed_conditional_specialization_bodies.md") (range (start 5 16) (end 5 32)) (probe (position 5 16))
    (reference (id (source (node (document "memory://snapshot/generated_malformed_conditional_specialization_bodies.md") (qualified-name "MalformedConditionalSpecializationBodies::Main"))) (kind includeUseCase) (ordinal 0) (authored-target "Library::operate")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_malformed_conditional_specialization_bodies.md") (qualified-name "MalformedConditionalSpecializationBodies::Library::operate")))))
    )
  )
)
~~~
