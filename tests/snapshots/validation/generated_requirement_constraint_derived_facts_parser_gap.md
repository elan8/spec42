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
blocked_by=parser-gap-74-require-constraint-membership
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
  (requirement-derived-fact (rule_id "sysml-2.0:8.3.21.8:deriveRequirementDefinitionAssumedConstraint") (source "RequirementConstraintDerivedFacts::Definition") (outcome unsupported) (prerequisite canonical_membership_role))
  (requirement-derived-fact (rule_id "sysml-2.0:8.3.21.8:deriveRequirementDefinitionRequiredConstraint") (source "RequirementConstraintDerivedFacts::Definition") (outcome unsupported) (prerequisite canonical_membership_role))
  (requirement-derived-fact (rule_id "sysml-2.0:8.3.21.9:deriveRequirementUsageAssumedConstraint") (source "RequirementConstraintDerivedFacts::Usage") (outcome unsupported) (prerequisite canonical_membership_role))
  (requirement-derived-fact (rule_id "sysml-2.0:8.3.21.9:deriveRequirementUsageRequiredConstraint") (source "RequirementConstraintDerivedFacts::Usage") (outcome unsupported) (prerequisite canonical_membership_role)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 3 8) (end 4 8))
      )
      (diagnostic
        (severity error)
        (code "recovered_requirement_body_element")
        (source "parser")
        (range (start 4 8) (end 5 4))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 7 8) (end 8 8))
      )
      (diagnostic
        (severity error)
        (code "recovered_requirement_body_element")
        (source "parser")
        (range (start 8 8) (end 9 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:63c717f105917a5353d8315db9c8509189bf456a4849f2e2f44d0560565ec198") (contract-version "parser-owned-resolution-v1") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Bound"))) (kind constraint-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Definition"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Usage"))) (kind requirement) (membership (kind feature) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Bound"))) (target (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Definition"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/generated_requirement_constraint_derived_facts_parser_gap.md") (qualified-name "RequirementConstraintDerivedFacts::Usage"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::requirementChecks"))) (provenance implied))
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
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
