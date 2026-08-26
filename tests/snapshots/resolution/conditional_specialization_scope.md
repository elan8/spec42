# META
~~~ini
description=Fact-settled conditional library specialization participates in expression name resolution
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.20.2:checkAssertConstraintUsageSpecialization
libraries=standard
~~~
# SOURCE
~~~sysml
package Demo {
    attribute def Bounded {
        assert constraint range { that == that }
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/conditional_specialization_scope.md"
    (diagnostics
    )
  )
)
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind subsetting) (source "Demo::Bounded::range") (target "Constraints::assertedConstraintChecks") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/conditional_specialization_scope.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:1eeac2d22ccb02fe4ad6a27788cc043cb1187e46ae33207e9add1d12f2203dbd") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/conditional_specialization_scope.md") (qualified-name "Demo"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/conditional_specialization_scope.md") (qualified-name "Demo::Bounded"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/conditional_specialization_scope.md") (qualified-name "Demo::Bounded::range"))) (kind assert-constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "that")) (expressionOperand (reference "that")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/conditional_specialization_scope.md") (qualified-name "Demo::Bounded::range"))) (kind expressionOperand) (ordinal 0))
      (authored-target "that")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things::that")))))
    (reference (id (source (node (document "memory://snapshot/conditional_specialization_scope.md") (qualified-name "Demo::Bounded::range"))) (kind expressionOperand) (ordinal 1))
      (authored-target "that")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things::that")))))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/conditional_specialization_scope.md") (qualified-name "Demo::Bounded::range"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things::that"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/conditional_specialization_scope.md") (qualified-name "Demo::Bounded::range"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/conditional_specialization_scope.md") (qualified-name "Demo::Bounded::range"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things::that"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/conditional_specialization_scope.md") (qualified-name "Demo::Bounded::range"))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/conditional_specialization_scope.md") (qualified-name "Demo::Bounded::range"))) (target (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::assertedConstraintChecks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/conditional_specialization_scope.md") (qualified-name "Demo::Bounded::range"))) (target (node (document "memory://snapshot/conditional_specialization_scope.md") (qualified-name "Demo::Bounded"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/conditional_specialization_scope.md") (qualified-name "Demo::Bounded::range"))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/conditional_specialization_scope.md") (qualified-name "Demo::Bounded::range")))
      (featured-by (node (document "memory://snapshot/conditional_specialization_scope.md") (qualified-name "Demo::Bounded")))
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
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/conditional_specialization_scope.md") (range (start 2 34) (end 2 38)) (probe (position 2 34))
    (reference (id (source (node (document "memory://snapshot/conditional_specialization_scope.md") (qualified-name "Demo::Bounded::range"))) (kind expressionOperand) (ordinal 0) (authored-target "that")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things::that")))))
    )
  )
  (query (document "memory://snapshot/conditional_specialization_scope.md") (range (start 2 42) (end 2 46)) (probe (position 2 42))
    (reference (id (source (node (document "memory://snapshot/conditional_specialization_scope.md") (qualified-name "Demo::Bounded::range"))) (kind expressionOperand) (ordinal 1) (authored-target "that")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things::that")))))
    )
  )
)
~~~
