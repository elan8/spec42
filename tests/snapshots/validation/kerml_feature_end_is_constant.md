# META
~~~ini
description=KerML 8.3.3.3.4 validateFeatureEndIsConstant requires a Feature with isEnd = true and isVariable = true to have isConstant = true
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.3.4 validateFeatureEndIsConstant
type=file
skip_validation=the pinned parser accepts `const end feature` -- which publishes (modifiers end constant) -- but rejects `var end feature`, reporting unrecognized_declaration_in_scope, so the violating variable end feature never reaches semantics
~~~
# SOURCE
~~~kerml
package Ends {
    classifier Thing;
    assoc Constants {
        // Conforming: a variable end feature declared const.
        const end feature fixed : Thing;

        // Invalid: a variable end feature that is not constant.
        var end feature loose : Thing;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_end_is_constant.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "end_feature_variable_not_constant")
        (source "semantic")
        (range (start 7 8) (end 7 38))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_end_is_constant.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 7 8) (end 8 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:d0e4e80a92fe328120372edc007e81734184b0250a8a2a40b7fa5cc284a35cc9") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_is_constant.md") (qualified-name "Ends"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_is_constant.md") (qualified-name "Ends::Constants"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_is_constant.md") (qualified-name "Ends::Constants::fixed"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end constant)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_is_constant.md") (qualified-name "Ends::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_is_constant.md") (qualified-name "Ends::Constants::fixed"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_is_constant.md") (qualified-name "Ends::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_end_is_constant.md") (qualified-name "Ends::Constants::fixed"))) (target (node (document "memory://snapshot/kerml_feature_end_is_constant.md") (qualified-name "Ends::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_end_is_constant.md") (qualified-name "Ends::Constants::fixed"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_is_constant.md") (qualified-name "Ends::Constants::fixed")))
      (featured-by (node (document "memory://snapshot/kerml_feature_end_is_constant.md") (qualified-name "Ends::Constants")))
      (type (node (document "memory://snapshot/kerml_feature_end_is_constant.md") (qualified-name "Ends::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_end_is_constant.md") (qualified-name "Ends::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_end_is_constant.md") (qualified-name "Ends::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_is_constant.md") (qualified-name "Ends::Thing")))
      (subtype (node (document "memory://snapshot/kerml_feature_end_is_constant.md") (qualified-name "Ends::Constants::fixed")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_end_is_constant.md") (range (start 4 34) (end 4 39)) (probe (position 4 34))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_is_constant.md") (qualified-name "Ends::Constants::fixed"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_is_constant.md") (qualified-name "Ends::Thing")))))
    )
  )
)
~~~
