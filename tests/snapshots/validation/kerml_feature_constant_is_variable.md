# META
~~~ini
description=KerML 8.3.3.3.4 validateFeatureConstantIsVariable requires a Feature with isConstant = true to have isVariable = true
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.3.3.4 validateFeatureConstantIsVariable
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.3.3.4:validateFeatureConstantIsVariable
type=file
~~~
# SOURCE
~~~kerml
// Conforming: a const feature. KerML textual notation gives the const keyword the derived
// effect isVariable = true (clause 8.2.4.2 FeaturePrefix), so a source document cannot author a
// constant feature that is not also variable and the violating side has no textual counterpart.
// The rule is observable only as the accepted side pinned here.
package Constants {
    classifier Thing;
    class Happening {
        const feature fixed : Thing;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_constant_is_variable.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_constant_is_variable.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:8416abaca568f11aec478011e9de255ca82816d7dfe968d41aa28141d946054d") (contract-version "semantic-metadata-projection-v6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_constant_is_variable.md") (qualified-name "Constants"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_constant_is_variable.md") (qualified-name "Constants::Happening"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_constant_is_variable.md") (qualified-name "Constants::Happening::fixed"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers constant)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_constant_is_variable.md") (qualified-name "Constants::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_constant_is_variable.md") (qualified-name "Constants::Happening::fixed"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_constant_is_variable.md") (qualified-name "Constants::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_constant_is_variable.md") (qualified-name "Constants::Happening::fixed"))) (target (node (document "memory://snapshot/kerml_feature_constant_is_variable.md") (qualified-name "Constants::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_constant_is_variable.md") (qualified-name "Constants::Happening::fixed"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_constant_is_variable.md") (qualified-name "Constants::Happening::fixed"))) (target (node (document "memory://snapshot/kerml_feature_constant_is_variable.md") (qualified-name "Constants::Happening"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_constant_is_variable.md") (qualified-name "Constants::Happening::fixed")))
      (featured-by (node (document "memory://snapshot/kerml_feature_constant_is_variable.md") (qualified-name "Constants::Happening")))
      (type (node (document "memory://snapshot/kerml_feature_constant_is_variable.md") (qualified-name "Constants::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_constant_is_variable.md") (qualified-name "Constants::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_constant_is_variable.md") (qualified-name "Constants::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_constant_is_variable.md") (qualified-name "Constants::Thing")))
      (subtype (node (document "memory://snapshot/kerml_feature_constant_is_variable.md") (qualified-name "Constants::Happening::fixed")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_constant_is_variable.md") (range (start 7 30) (end 7 35)) (probe (position 7 30))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_constant_is_variable.md") (qualified-name "Constants::Happening::fixed"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_constant_is_variable.md") (qualified-name "Constants::Thing")))))
    )
  )
)
~~~
