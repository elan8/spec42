# META
~~~ini
description=KerML 8.3.4.7.3 validateExpressionResultParameterMembership requires an Expression to have exactly one owned or inherited featureMembership that is a ResultParameterMembership
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.7.3 validateExpressionResultParameterMembership
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.4.7.3:validateExpressionResultParameterMembership
type=file
~~~
# SOURCE
~~~kerml
package Expressions {
    classifier Thing;

    // Conforming: exactly one return parameter.
    expr One {
        return feature result : Thing;
    }

    // Conforming: no owned return parameter; the result parameter is inherited from
    // Performances::evaluations, so the expression still has exactly one.
    expr None {
        in feature input : Thing;
    }

    // Invalid: two return parameters.
    expr Two {
        return feature first : Thing;
        return feature second : Thing;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_expression_result_parameter_membership.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "expression_result_parameter_count")
        (source "semantic")
        (range (start 17 8) (end 17 38))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_expression_result_parameter_membership.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "expression_result_parameter_count")
        (source "semantic")
        (range (start 17 8) (end 17 38))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:5b2c3ea004c325da3a1e3d07ab64448a9f7f07f4fd1164f06f3923f90fe4c412"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::None"))) (kind kerml-expression) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::None::input"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing") (direction in)))))
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::One"))) (kind kerml-expression) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::One::result"))) (kind parameter) (membership (kind feature) (visibility default) (role return-parameter)) (authored (membership (kind feature) (visibility default) (role return-parameter)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Two"))) (kind kerml-expression) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Two::first"))) (kind parameter) (membership (kind feature) (visibility default) (role return-parameter)) (authored (membership (kind feature) (visibility default) (role return-parameter)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Two::second"))) (kind parameter) (membership (kind feature) (visibility default) (role return-parameter)) (authored (membership (kind feature) (visibility default) (role return-parameter)) (relationships (featureTyping (reference "Thing")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::None::input"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::One::result"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Two::first"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Two::second"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Thing")))))
  )
  (relationships
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::None::input"))) (target (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::None::input"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::One::result"))) (target (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::One::result"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Two::first"))) (target (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Two::first"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Two::second"))) (target (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Two::second"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::None::input"))) (target (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::None"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::One::result"))) (target (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::One"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Two::first"))) (target (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Two"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Two::second"))) (target (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Two"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::None::input")))
      (featured-by (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::None")))
      (type (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::One::result")))
      (featured-by (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::One")))
      (type (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Thing")))
      (subtype (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::None::input")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::One::result")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Two::first")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Two::second")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Two::first")))
      (featured-by (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Two")))
      (type (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Two::second")))
      (featured-by (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Two")))
      (type (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (range (start 11 27) (end 11 32)) (probe (position 11 27))
    (reference (id (source (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::None::input"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (range (start 5 32) (end 5 37)) (probe (position 5 32))
    (reference (id (source (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::One::result"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (range (start 16 31) (end 16 36)) (probe (position 16 31))
    (reference (id (source (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Two::first"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (range (start 17 32) (end 17 37)) (probe (position 17 32))
    (reference (id (source (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Two::second"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_expression_result_parameter_membership.md") (qualified-name "Expressions::Thing")))))
    )
  )
)
~~~
