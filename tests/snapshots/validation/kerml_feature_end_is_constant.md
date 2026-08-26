# META
~~~ini
description=KerML 8.3.3.3.4 validateFeatureEndIsConstant requires a Feature with isEnd = true and isVariable = true to have isConstant = true
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.3.4 validateFeatureEndIsConstant
source_expectation=accepted
rule_family=validate
expectation=by_construction
rule_id=kerml-1.0:8.3.3.3.4:validateFeatureEndIsConstant
blocked_by=abstract-syntax-nonrepresentable-variable-end
type=file
~~~
# SOURCE
~~~kerml
package Ends {
    classifier Thing;
    assoc Constants {
        // Conforming: a constant end feature. KerML's `EndFeaturePrefix` spells only
        // `const? end` and `var` lives in the exclusive `BasicFeaturePrefix` alternative, while
        // SysML's `RefPrefix` has no `var`/`variable` slot, so the violating variable end
        // feature has no concrete-syntax spelling.
        const end feature fixed : Thing;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_end_is_constant.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:56dd6d8fc154c8f4521689e42d6c9e1f4dedda57e7c00d6ffc1dd4cc64be2ffa") (contract-version "owned-cross-feature-typing-v4"))
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
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_end_is_constant.md") (qualified-name "Ends::Constants::fixed"))) (target (node (document "memory://snapshot/kerml_feature_end_is_constant.md") (qualified-name "Ends::Constants"))) (provenance implied))
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
  (query (document "memory://snapshot/kerml_feature_end_is_constant.md") (range (start 7 34) (end 7 39)) (probe (position 7 34))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_is_constant.md") (qualified-name "Ends::Constants::fixed"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_is_constant.md") (qualified-name "Ends::Thing")))))
    )
  )
)
~~~
