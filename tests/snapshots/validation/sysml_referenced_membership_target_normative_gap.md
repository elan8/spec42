# META
~~~ini
description=SysML 8.3.21.7 and 8.3.26.10 referenced constraint and rendering derivations return an undeclared variable in their pinned bodies
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=sysml-2.0:8.3.21.7:deriveRequirementConstraintMembershipReferencedConstraint
rule_id=sysml-2.0:8.3.26.10:deriveVewRenderingMembershipReferencedRendering
blocked_by=normative-specification-gap-referenced-membership-target-bodies
libraries=standard
type=file
~~~
# SOURCE
~~~sysml
package ReferencedTargets {
    requirement def Limit {
        require constraint withinLimit { true }
    }
    rendering asTree;
    view def Overview {
        render asTree;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (action-derived-fact (rule_id "sysml-2.0:8.3.21.7:deriveRequirementConstraintMembershipReferencedConstraint") (source "ReferencedTargets::Limit::withinLimit") (target "ReferencedTargets::Limit::withinLimit") (outcome resolved))
  (action-derived-fact (rule_id "sysml-2.0:8.3.26.10:deriveVewRenderingMembershipReferencedRendering") (source "ReferencedTargets::Overview") (target "ReferencedTargets::asTree") (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:a0a1948d5d61b7076d37447e9aae91e951ab237a99d2f1125426d0a8020e6dee") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (qualified-name "ReferencedTargets"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (qualified-name "ReferencedTargets::Limit"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (qualified-name "ReferencedTargets::Limit::withinLimit"))) (kind require-constraint) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (path (named (kind package) (name "ReferencedTargets")) (named (kind requirement-def) (name "Limit")) (named (kind require-constraint) (name "withinLimit")) (anonymous (kind kerml-literal-boolean) (ordinal 0))))) (kind kerml-literal-boolean) (membership (kind feature) (visibility default) (role result-expression)) (facts (expression-result (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (path (named (kind package) (name "ReferencedTargets")) (named (kind requirement-def) (name "Limit")) (named (kind require-constraint) (name "withinLimit")) (anonymous (kind kerml-literal-boolean) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (path (named (kind package) (name "ReferencedTargets")) (named (kind requirement-def) (name "Limit")) (named (kind require-constraint) (name "withinLimit")) (anonymous (kind kerml-literal-boolean) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (qualified-name "ReferencedTargets::Overview"))) (kind view-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (path (named (kind package) (name "ReferencedTargets")) (named (kind view-def) (name "Overview")) (anonymous (kind rendering) (ordinal 0))))) (kind rendering) (membership (kind feature) (visibility default) (role view-rendering)) (authored (membership (kind feature) (visibility default) (role view-rendering)) (relationships (subsetting (reference "asTree")))))
    (declaration (id (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (qualified-name "ReferencedTargets::asTree"))) (kind rendering) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (path (named (kind package) (name "ReferencedTargets")) (named (kind view-def) (name "Overview")) (anonymous (kind rendering) (ordinal 0))))) (kind subsetting) (ordinal 0))
      (authored-target "asTree")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (qualified-name "ReferencedTargets::asTree")))))
  )
  (relationships
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (path (named (kind package) (name "ReferencedTargets")) (named (kind view-def) (name "Overview")) (anonymous (kind rendering) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (qualified-name "ReferencedTargets::asTree"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (path (named (kind package) (name "ReferencedTargets")) (named (kind view-def) (name "Overview")) (anonymous (kind rendering) (ordinal 0))))) (kind subsetting) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (qualified-name "ReferencedTargets::Limit"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (qualified-name "ReferencedTargets::Limit::withinLimit"))) (target (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::constraintChecks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (qualified-name "ReferencedTargets::Limit::withinLimit"))) (target (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (qualified-name "ReferencedTargets::Limit"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (qualified-name "ReferencedTargets::Limit::withinLimit"))) (target (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::constraints"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (path (named (kind package) (name "ReferencedTargets")) (named (kind requirement-def) (name "Limit")) (named (kind require-constraint) (name "withinLimit")) (anonymous (kind kerml-literal-boolean) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalBooleanEvaluations"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (path (named (kind package) (name "ReferencedTargets")) (named (kind requirement-def) (name "Limit")) (named (kind require-constraint) (name "withinLimit")) (anonymous (kind kerml-literal-boolean) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (qualified-name "ReferencedTargets::Limit::withinLimit"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (path (named (kind package) (name "ReferencedTargets")) (named (kind requirement-def) (name "Limit")) (named (kind require-constraint) (name "withinLimit")) (anonymous (kind kerml-literal-boolean) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (path (named (kind package) (name "ReferencedTargets")) (named (kind requirement-def) (name "Limit")) (named (kind require-constraint) (name "withinLimit")) (anonymous (kind kerml-literal-boolean) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-predicate) (name "LiteralBooleanEvaluation")) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (path (named (kind package) (name "ReferencedTargets")) (named (kind requirement-def) (name "Limit")) (named (kind require-constraint) (name "withinLimit")) (anonymous (kind kerml-literal-boolean) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (path (named (kind package) (name "ReferencedTargets")) (named (kind requirement-def) (name "Limit")) (named (kind require-constraint) (name "withinLimit")) (anonymous (kind kerml-literal-boolean) (ordinal 0))))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (qualified-name "ReferencedTargets::Overview"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (path (named (kind package) (name "ReferencedTargets")) (named (kind view-def) (name "Overview")) (anonymous (kind rendering) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (path (named (kind package) (name "ReferencedTargets")) (named (kind view-def) (name "Overview")) (anonymous (kind rendering) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (qualified-name "ReferencedTargets::Overview"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (path (named (kind package) (name "ReferencedTargets")) (named (kind view-def) (name "Overview")) (anonymous (kind rendering) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View::viewRendering"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (path (named (kind package) (name "ReferencedTargets")) (named (kind view-def) (name "Overview")) (anonymous (kind rendering) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::renderings"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (qualified-name "ReferencedTargets::asTree"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::renderings"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (path (named (kind package) (name "ReferencedTargets")) (named (kind requirement-def) (name "Limit")) (named (kind require-constraint) (name "withinLimit")) (anonymous (kind kerml-literal-boolean) (ordinal 0))))) (state literal) (value (kind boolean) (boolean true)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (qualified-name "ReferencedTargets::Limit")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::ConstraintCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (qualified-name "ReferencedTargets::Limit::withinLimit")))
      (featured-by (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (qualified-name "ReferencedTargets::Limit")))
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
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementCheck::constraints")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/requirements.md") (qualified-name "Requirements::RequirementConstraintCheck::constraints")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (path (named (kind package) (name "ReferencedTargets")) (named (kind requirement-def) (name "Limit")) (named (kind require-constraint) (name "withinLimit")) (anonymous (kind kerml-literal-boolean) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (qualified-name "ReferencedTargets::Limit::withinLimit")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralBooleanEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalBooleanEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::BooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralBooleanEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::booleanEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalBooleanEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (path (named (kind package) (name "ReferencedTargets")) (named (kind requirement-def) (name "Limit")) (named (kind require-constraint) (name "withinLimit")) (anonymous (kind kerml-literal-boolean) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (path (named (kind package) (name "ReferencedTargets")) (named (kind requirement-def) (name "Limit")) (named (kind require-constraint) (name "withinLimit")) (anonymous (kind kerml-literal-boolean) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation::result"))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-predicate) (name "BooleanEvaluation")) (anonymous (kind parameter) (ordinal 0))))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-predicate) (name "LiteralBooleanEvaluation")) (anonymous (kind parameter) (ordinal 0))))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralEvaluation")) (anonymous (kind parameter) (ordinal 0))))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-predicate) (name "BooleanEvaluation")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation::result")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-predicate) (name "LiteralBooleanEvaluation")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralEvaluation")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (qualified-name "ReferencedTargets::Overview")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (path (named (kind package) (name "ReferencedTargets")) (named (kind view-def) (name "Overview")) (anonymous (kind rendering) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (qualified-name "ReferencedTargets::Overview")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::Rendering")) (source inherited) (from (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View::viewRendering"))))
      (effective-type (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::Rendering")) (source inherited) (from (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::renderings"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::Rendering")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View::viewRendering")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::renderings")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (qualified-name "ReferencedTargets::asTree")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (qualified-name "ReferencedTargets::asTree")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::Rendering")) (source inherited) (from (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::renderings"))))
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
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::Rendering")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::renderings")) (scopes any feature))
      (subtype (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (path (named (kind package) (name "ReferencedTargets")) (named (kind view-def) (name "Overview")) (anonymous (kind rendering) (ordinal 0)))) (scopes any feature))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (path (named (kind package) (name "ReferencedTargets")) (named (kind requirement-def) (name "Limit")) (named (kind require-constraint) (name "withinLimit")) (anonymous (kind kerml-literal-boolean) (ordinal 0))))) (outcome resolved) (literal (value (kind boolean) (boolean true))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (range (start 6 15) (end 6 21)) (probe (position 6 15))
    (reference (id (source (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (path (named (kind package) (name "ReferencedTargets")) (named (kind view-def) (name "Overview")) (anonymous (kind rendering) (ordinal 0))))) (kind subsetting) (ordinal 0) (authored-target "asTree")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_referenced_membership_target_normative_gap.md") (qualified-name "ReferencedTargets::asTree")))))
    )
  )
)
~~~
