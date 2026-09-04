# META
~~~ini
description=Generated polarity specialization chooses the exact isNegated branch anchor from authored assert and invariant syntax
specification=OMG SysML 2.0 and KerML 1.0 (formal/26-03)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.20.2:checkAssertConstraintUsageSpecialization
rule_id=kerml-1.0:8.3.4.7.5:checkInvariantSpecialization
type=file
libraries=standard
~~~
# SOURCE
~~~sysml
package PolaritySpecializations {
    inv PositiveInvariant { true }
    inv not NegativeInvariant { true }
    assert constraint PositiveAssert { true; }
    assert not constraint NegativeAssert { true; }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind subsetting) (source "PolaritySpecializations::PositiveInvariant") (target "Performances::trueEvaluations") (provenance implied) (outcome resolved))
  (relationship (kind subsetting) (source "PolaritySpecializations::NegativeInvariant") (target "Performances::falseEvaluations") (provenance implied) (outcome resolved))
  (relationship (kind subsetting) (source "PolaritySpecializations::PositiveAssert") (target "Constraints::assertedConstraintChecks") (provenance implied) (outcome resolved))
  (relationship (kind subsetting) (source "PolaritySpecializations::NegativeAssert") (target "Constraints::negatedConstraintChecks") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_conditional_polarity_specializations.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:7dec7a827c025f625714fd835574591f505ee5c0f4872d3535fd3d1348ee8211") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_conditional_polarity_specializations.md") (qualified-name "PolaritySpecializations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_polarity_specializations.md") (qualified-name "PolaritySpecializations::NegativeAssert"))) (kind assert-constraint) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_polarity_specializations.md") (qualified-name "PolaritySpecializations::NegativeInvariant"))) (kind kerml-invariant) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_polarity_specializations.md") (qualified-name "PolaritySpecializations::PositiveAssert"))) (kind assert-constraint) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_polarity_specializations.md") (qualified-name "PolaritySpecializations::PositiveInvariant"))) (kind kerml-invariant) (membership (kind feature) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_polarity_specializations.md") (qualified-name "PolaritySpecializations::NegativeAssert"))) (target (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::negatedConstraintChecks"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_polarity_specializations.md") (qualified-name "PolaritySpecializations::NegativeInvariant"))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::falseEvaluations"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_polarity_specializations.md") (qualified-name "PolaritySpecializations::PositiveAssert"))) (target (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::assertedConstraintChecks"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_polarity_specializations.md") (qualified-name "PolaritySpecializations::PositiveInvariant"))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::trueEvaluations"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/generated_conditional_polarity_specializations.md") (qualified-name "PolaritySpecializations::NegativeAssert"))) (state literal) (value (kind boolean) (boolean true)))
    (evaluated (declaration (node (document "memory://snapshot/generated_conditional_polarity_specializations.md") (qualified-name "PolaritySpecializations::NegativeInvariant"))) (state literal) (value (kind boolean) (boolean true)))
    (evaluated (declaration (node (document "memory://snapshot/generated_conditional_polarity_specializations.md") (qualified-name "PolaritySpecializations::PositiveAssert"))) (state literal) (value (kind boolean) (boolean true)))
    (evaluated (declaration (node (document "memory://snapshot/generated_conditional_polarity_specializations.md") (qualified-name "PolaritySpecializations::PositiveInvariant"))) (state literal) (value (kind boolean) (boolean true)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_conditional_polarity_specializations.md") (qualified-name "PolaritySpecializations::NegativeAssert")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (source inherited) (from (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::negatedConstraintChecks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::falseEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_polarity_specializations.md") (qualified-name "PolaritySpecializations::NegativeInvariant")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::falseEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_polarity_specializations.md") (qualified-name "PolaritySpecializations::PositiveAssert")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (source inherited) (from (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
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
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_polarity_specializations.md") (qualified-name "PolaritySpecializations::PositiveInvariant")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::trueEvaluations")) (scopes any feature))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/generated_conditional_polarity_specializations.md") (qualified-name "PolaritySpecializations::NegativeAssert"))) (outcome resolved) (literal (value (kind boolean) (boolean true))))
  (declaration (id (node (document "memory://snapshot/generated_conditional_polarity_specializations.md") (qualified-name "PolaritySpecializations::NegativeInvariant"))) (outcome resolved) (literal (value (kind boolean) (boolean true))))
  (declaration (id (node (document "memory://snapshot/generated_conditional_polarity_specializations.md") (qualified-name "PolaritySpecializations::PositiveAssert"))) (outcome resolved) (literal (value (kind boolean) (boolean true))))
  (declaration (id (node (document "memory://snapshot/generated_conditional_polarity_specializations.md") (qualified-name "PolaritySpecializations::PositiveInvariant"))) (outcome resolved) (literal (value (kind boolean) (boolean true))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
