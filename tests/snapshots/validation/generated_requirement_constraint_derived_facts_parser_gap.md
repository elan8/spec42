# META
~~~ini
description=Requirement constraint derived facts remain explicit until require and assume memberships reach canonical lowering
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=sysml-2.0:8.3.21.8:deriveRequirementDefinitionAssumedConstraint
rule_id=sysml-2.0:8.3.21.8:deriveRequirementDefinitionRequiredConstraint
rule_id=sysml-2.0:8.3.21.9:deriveRequirementUsageAssumedConstraint
rule_id=sysml-2.0:8.3.21.9:deriveRequirementUsageRequiredConstraint
type=file
libraries=standard
~~~
# SOURCE
~~~sysml
package RequirementConstraintDerivedFacts {
    constraint def Bound;
    requirement def Definition {
        assume constraint assumed : Bound;
        require constraint required : Bound;
    }
    requirement Usage {
        assume constraint assumed : Bound;
        require constraint required : Bound;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (requirement-derived-fact (rule_id "sysml-2.0:8.3.21.8:deriveRequirementDefinitionAssumedConstraint") (source "RequirementConstraintDerivedFacts::Definition") (target "RequirementConstraintDerivedFacts::Definition::assumed") (outcome resolved))
  (requirement-derived-fact (rule_id "sysml-2.0:8.3.21.8:deriveRequirementDefinitionRequiredConstraint") (source "RequirementConstraintDerivedFacts::Definition") (target "RequirementConstraintDerivedFacts::Definition::required") (outcome resolved))
  (requirement-derived-fact (rule_id "sysml-2.0:8.3.21.9:deriveRequirementUsageAssumedConstraint") (source "RequirementConstraintDerivedFacts::Usage") (target "RequirementConstraintDerivedFacts::Usage::assumed") (outcome resolved))
  (requirement-derived-fact (rule_id "sysml-2.0:8.3.21.9:deriveRequirementUsageRequiredConstraint") (source "RequirementConstraintDerivedFacts::Usage") (target "RequirementConstraintDerivedFacts::Usage::required") (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:63c717f105917a5353d8315db9c8509189bf456a4849f2e2f44d0560565ec198") (contract-version "parser-owned-resolution-v1") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Bound"))) (kind constraint-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Definition"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Definition::assumed"))) (kind assume-constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Bound")))))
    (declaration (id (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Definition::required"))) (kind require-constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Bound")))))
    (declaration (id (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Usage"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Usage::assumed"))) (kind assume-constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Bound")))))
    (declaration (id (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Usage::required"))) (kind require-constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Bound")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Definition::assumed"))) (kind featureTyping) (ordinal 0))
      (authored-target "Bound")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Bound")))))
    (reference (id (source (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Definition::required"))) (kind featureTyping) (ordinal 0))
      (authored-target "Bound")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Bound")))))
    (reference (id (source (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Usage::assumed"))) (kind featureTyping) (ordinal 0))
      (authored-target "Bound")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Bound")))))
    (reference (id (source (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Usage::required"))) (kind featureTyping) (ordinal 0))
      (authored-target "Bound")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Bound")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Definition::assumed"))) (target (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Bound"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Definition::assumed"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Definition::required"))) (target (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Bound"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Definition::required"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Usage::assumed"))) (target (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Bound"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Usage::assumed"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Usage::required"))) (target (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Bound"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Usage::required"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Bound"))) (target (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Definition"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Definition::assumed"))) (target (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Definition::assumed"))) (target (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Definition"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Definition::assumed"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::assumptions"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Definition::required"))) (target (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Definition::required"))) (target (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Definition"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Definition::required"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::constraints"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Usage"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Usage::assumed"))) (target (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Usage::assumed"))) (target (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Usage"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Usage::assumed"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::assumptions"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Usage::required"))) (target (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Usage::required"))) (target (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Usage"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Usage::required"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::constraints"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Bound")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Definition::assumed")) (scopes any))
      (subtype (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Definition::required")) (scopes any))
      (subtype (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Usage::assumed")) (scopes any))
      (subtype (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Usage::required")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Definition")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Definition::assumed")))
      (featured-by (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Definition")))
      (type (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Bound")) (provenance authored))
      (effective-type (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Bound")) (source direct))
      (supertype (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Bound")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::assumptions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck::assumptions")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Definition::required")))
      (featured-by (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Definition")))
      (type (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Bound")) (provenance authored))
      (effective-type (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Bound")) (source direct))
      (supertype (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Bound")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::constraints")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck::constraints")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Usage")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Usage::assumed")))
      (featured-by (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Usage")))
      (type (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Bound")) (provenance authored))
      (effective-type (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Bound")) (source direct))
      (supertype (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Bound")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::assumptions")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck::assumptions")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Usage::required")))
      (featured-by (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Usage")))
      (type (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Bound")) (provenance authored))
      (effective-type (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Bound")) (source direct))
      (supertype (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Bound")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::constraints")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck::constraints")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (range (start 3 36) (end 3 41)) (probe (position 3 36))
    (reference (id (source (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Definition::assumed"))) (kind featureTyping) (ordinal 0) (authored-target "Bound")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Bound")))))
    )
  )
  (query (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (range (start 4 38) (end 4 43)) (probe (position 4 38))
    (reference (id (source (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Definition::required"))) (kind featureTyping) (ordinal 0) (authored-target "Bound")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Bound")))))
    )
  )
  (query (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (range (start 7 36) (end 7 41)) (probe (position 7 36))
    (reference (id (source (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Usage::assumed"))) (kind featureTyping) (ordinal 0) (authored-target "Bound")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Bound")))))
    )
  )
  (query (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (range (start 8 38) (end 8 43)) (probe (position 8 38))
    (reference (id (source (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Usage::required"))) (kind featureTyping) (ordinal 0) (authored-target "Bound")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Bound")))))
    )
  )
)
~~~
