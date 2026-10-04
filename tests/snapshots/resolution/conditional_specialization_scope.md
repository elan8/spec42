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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:c5483415a33cf338b21273ea295bb9f694f3037c4485dd1d4deae4349b2df2fc") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/conditional_specialization_scope.md") (qualified-name "Demo"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/conditional_specialization_scope.md") (qualified-name "Demo::Bounded"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/conditional_specialization_scope.md") (qualified-name "Demo::Bounded::range"))) (kind assert-constraint) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/conditional_specialization_scope.md") (path (named (kind package) (name "Demo")) (named (kind attribute-def) (name "Bounded")) (named (kind assert-constraint) (name "range")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind feature) (visibility default) (role result-expression)) (facts (expression-result (node (document "memory://snapshot/conditional_specialization_scope.md") (path (named (kind package) (name "Demo")) (named (kind attribute-def) (name "Bounded")) (named (kind assert-constraint) (name "range")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default) (role result-expression)) (relationships (expressionOperand (reference "that")) (expressionOperand (reference "that")))))
    (declaration (id (node (document "memory://snapshot/conditional_specialization_scope.md") (path (named (kind package) (name "Demo")) (named (kind attribute-def) (name "Bounded")) (named (kind assert-constraint) (name "range")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/conditional_specialization_scope.md") (path (named (kind package) (name "Demo")) (named (kind attribute-def) (name "Bounded")) (named (kind assert-constraint) (name "range")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "that")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things::that")))))
    (reference (id (source (node (document "memory://snapshot/conditional_specialization_scope.md") (path (named (kind package) (name "Demo")) (named (kind attribute-def) (name "Bounded")) (named (kind assert-constraint) (name "range")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 1))
      (authored-target "that")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things::that")))))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/conditional_specialization_scope.md") (path (named (kind package) (name "Demo")) (named (kind attribute-def) (name "Bounded")) (named (kind assert-constraint) (name "range")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things::that"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/conditional_specialization_scope.md") (path (named (kind package) (name "Demo")) (named (kind attribute-def) (name "Bounded")) (named (kind assert-constraint) (name "range")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/conditional_specialization_scope.md") (path (named (kind package) (name "Demo")) (named (kind attribute-def) (name "Bounded")) (named (kind assert-constraint) (name "range")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things::that"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/conditional_specialization_scope.md") (path (named (kind package) (name "Demo")) (named (kind attribute-def) (name "Bounded")) (named (kind assert-constraint) (name "range")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/conditional_specialization_scope.md") (qualified-name "Demo::Bounded"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/conditional_specialization_scope.md") (qualified-name "Demo::Bounded::range"))) (target (node (document "memory://snapshot/sysml.library/constraints.md") (qualified-name "Constraints::assertedConstraintChecks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/conditional_specialization_scope.md") (qualified-name "Demo::Bounded::range"))) (target (node (document "memory://snapshot/conditional_specialization_scope.md") (qualified-name "Demo::Bounded"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/conditional_specialization_scope.md") (path (named (kind package) (name "Demo")) (named (kind attribute-def) (name "Bounded")) (named (kind assert-constraint) (name "range")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/conditional_specialization_scope.md") (qualified-name "Demo::Bounded::range"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/conditional_specialization_scope.md") (path (named (kind package) (name "Demo")) (named (kind attribute-def) (name "Bounded")) (named (kind assert-constraint) (name "range")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/conditional_specialization_scope.md") (path (named (kind package) (name "Demo")) (named (kind attribute-def) (name "Bounded")) (named (kind assert-constraint) (name "range")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/conditional_specialization_scope.md") (path (named (kind package) (name "Demo")) (named (kind attribute-def) (name "Bounded")) (named (kind assert-constraint) (name "range")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/conditional_specialization_scope.md") (path (named (kind package) (name "Demo")) (named (kind attribute-def) (name "Bounded")) (named (kind assert-constraint) (name "range")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/conditional_specialization_scope.md") (path (named (kind package) (name "Demo")) (named (kind attribute-def) (name "Bounded")) (named (kind assert-constraint) (name "range")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation::result"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/conditional_specialization_scope.md") (path (named (kind package) (name "Demo")) (named (kind attribute-def) (name "Bounded")) (named (kind assert-constraint) (name "range")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/conditional_specialization_scope.md") (qualified-name "Demo::Bounded")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any subclassification))
    )
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
    (declaration (id (node (document "memory://snapshot/conditional_specialization_scope.md") (path (named (kind package) (name "Demo")) (named (kind attribute-def) (name "Bounded")) (named (kind assert-constraint) (name "range")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/conditional_specialization_scope.md") (qualified-name "Demo::Bounded::range")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/conditional_specialization_scope.md") (path (named (kind package) (name "Demo")) (named (kind attribute-def) (name "Bounded")) (named (kind assert-constraint) (name "range")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/conditional_specialization_scope.md") (path (named (kind package) (name "Demo")) (named (kind attribute-def) (name "Bounded")) (named (kind assert-constraint) (name "range")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation::result"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation::result")) (scopes any feature))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/conditional_specialization_scope.md") (path (named (kind package) (name "Demo")) (named (kind attribute-def) (name "Bounded")) (named (kind assert-constraint) (name "range")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (operator "==" (feature-reference "that" (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things::that")))) (feature-reference "that" (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things::that"))))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/conditional_specialization_scope.md") (range (start 2 34) (end 2 38)) (probe (position 2 34))
    (reference (id (source (node (document "memory://snapshot/conditional_specialization_scope.md") (path (named (kind package) (name "Demo")) (named (kind attribute-def) (name "Bounded")) (named (kind assert-constraint) (name "range")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "that")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things::that")))))
    )
  )
  (query (document "memory://snapshot/conditional_specialization_scope.md") (range (start 2 42) (end 2 46)) (probe (position 2 42))
    (reference (id (source (node (document "memory://snapshot/conditional_specialization_scope.md") (path (named (kind package) (name "Demo")) (named (kind attribute-def) (name "Bounded")) (named (kind assert-constraint) (name "range")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 1) (authored-target "that")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things::that")))))
    )
  )
)
~~~
