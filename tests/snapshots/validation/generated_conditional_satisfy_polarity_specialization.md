# META
~~~ini
description=Generated satisfy specialization chooses the exact negated requirement-check anchor
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.21.10:checkSatisfyRequirementUsageSpecialization
blocked_by=semantic-query-gap-anonymous-library-specialization-forms
type=file
libraries=standard
~~~
# SOURCE
~~~sysml
package SatisfyPolaritySpecialization {
    requirement def Safety;
    part def Vehicle;
    not satisfy Safety by Vehicle;
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind specialization) (source "SatisfyPolaritySpecialization::<anonymous>") (target "Requirements::notSatisfiedRequirementChecks") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:035c9ace5a4fd3467d0db77331ca0f06f722f743cc3fd9787579fa2ea5a9b188") (contract-version "parser-owned-resolution-v1") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (path (named (kind package) (name "SatisfyPolaritySpecialization")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfy) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (satisfySource (reference "Safety")) (satisfyTarget (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::Safety"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (path (named (kind package) (name "SatisfyPolaritySpecialization")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0))
      (authored-target "Safety")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::Safety")))))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (path (named (kind package) (name "SatisfyPolaritySpecialization")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfyTarget) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::Vehicle")))))
  )
  (relationships
    (relationship (kind satisfySource) (source (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (path (named (kind package) (name "SatisfyPolaritySpecialization")) (anonymous (kind satisfy) (ordinal 0))))) (target (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::Safety"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (path (named (kind package) (name "SatisfyPolaritySpecialization")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0)))
    (relationship (kind satisfyTarget) (source (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (path (named (kind package) (name "SatisfyPolaritySpecialization")) (anonymous (kind satisfy) (ordinal 0))))) (target (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (path (named (kind package) (name "SatisfyPolaritySpecialization")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfyTarget) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (path (named (kind package) (name "SatisfyPolaritySpecialization")) (anonymous (kind satisfy) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::notSatisfiedRequirementChecks"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::Safety"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::Vehicle"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (path (named (kind package) (name "SatisfyPolaritySpecialization")) (anonymous (kind satisfy) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::negatedConstraintChecks")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::falseEvaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::notSatisfiedRequirementChecks")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::Safety")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::Vehicle")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (range (start 3 16) (end 3 22)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (path (named (kind package) (name "SatisfyPolaritySpecialization")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0) (authored-target "Safety")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::Safety")))))
    )
  )
  (query (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (range (start 3 26) (end 3 33)) (probe (position 3 26))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (path (named (kind package) (name "SatisfyPolaritySpecialization")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfyTarget) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_satisfy_polarity_specialization.md") (qualified-name "SatisfyPolaritySpecialization::Vehicle")))))
    )
  )
)
~~~
