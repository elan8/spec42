# META
~~~ini
description=KerML 8.3.4.7.4 validateFunctionResultParameterMembership requires a Function to have exactly one owned or inherited featureMembership that is a ResultParameterMembership
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.7.4 validateFunctionResultParameterMembership
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.4.7.4:validateFunctionResultParameterMembership
type=file
~~~
# SOURCE
~~~kerml
package Functions {
    classifier Thing;

    // Conforming: exactly one return parameter.
    function One {
        return feature result : Thing;
    }

    // Conforming: no owned return parameter; the result parameter is inherited from
    // Performances::Evaluation, so the function still has exactly one.
    function None {
        in feature input : Thing;
    }

    // Invalid: two return parameters.
    function Two {
        return feature first : Thing;
        return feature second : Thing;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_function_result_parameter_membership.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "function_result_parameter_count")
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
  (document "memory://snapshot/kerml_function_result_parameter_membership.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "function_result_parameter_count")
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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:29f7c1d7d7a7a179da776c74bf509c2737743cf237da332fc59c967b5661a82f"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::None"))) (kind kerml-function) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::None::input"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing") (direction in)))))
    (declaration (id (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::One"))) (kind kerml-function) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::One::result"))) (kind parameter) (membership (kind feature) (visibility default) (role return-parameter)) (authored (membership (kind feature) (visibility default) (role return-parameter)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Two"))) (kind kerml-function) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Two::first"))) (kind parameter) (membership (kind feature) (visibility default) (role return-parameter)) (authored (membership (kind feature) (visibility default) (role return-parameter)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Two::second"))) (kind parameter) (membership (kind feature) (visibility default) (role return-parameter)) (authored (membership (kind feature) (visibility default) (role return-parameter)) (relationships (featureTyping (reference "Thing")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::None::input"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::One::result"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Two::first"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Two::second"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Thing")))))
  )
  (relationships
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::None::input"))) (target (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::None::input"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::One::result"))) (target (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::One::result"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Two::first"))) (target (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Two::first"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Two::second"))) (target (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Two::second"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::None::input"))) (target (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::None"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::One::result"))) (target (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::One"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Two::first"))) (target (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Two"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Two::second"))) (target (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Two"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::None::input")))
      (featured-by (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::None")))
      (type (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::One::result")))
      (featured-by (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::One")))
      (type (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Thing")))
      (subtype (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::None::input")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::One::result")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Two::first")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Two::second")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Two::first")))
      (featured-by (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Two")))
      (type (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Two::second")))
      (featured-by (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Two")))
      (type (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Thing")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_function_result_parameter_membership.md") (range (start 11 27) (end 11 32)) (probe (position 11 27))
    (reference (id (source (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::None::input"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_function_result_parameter_membership.md") (range (start 5 32) (end 5 37)) (probe (position 5 32))
    (reference (id (source (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::One::result"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_function_result_parameter_membership.md") (range (start 16 31) (end 16 36)) (probe (position 16 31))
    (reference (id (source (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Two::first"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_function_result_parameter_membership.md") (range (start 17 32) (end 17 37)) (probe (position 17 32))
    (reference (id (source (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Two::second"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_function_result_parameter_membership.md") (qualified-name "Functions::Thing")))))
    )
  )
)
~~~
