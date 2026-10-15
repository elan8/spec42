# META
~~~ini
description=A feature projects its cross feature and feature target, and a function or expression projects its first owned return parameter
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.3.3.3.4:deriveFeatureCrossFeature
rule_id=kerml-1.0:8.3.3.3.4:deriveFeatureFeatureTarget
rule_id=kerml-1.0:8.3.4.7.4:deriveFunctionResult
rule_id=kerml-1.0:8.3.4.7.3:deriveExpressionResult
libraries=none
~~~
# SOURCE
~~~kerml
package Model {
    classifier Thing {
        feature inner;
    }
    assoc Link {
        end feature source : Thing;
        end feature crossed : Thing crosses source.inner;
    }
    classifier Vehicle {
        feature base;
        feature path chains base;
        feature plain;
    }
    function Computing {
        return feature result : Thing;
    }
    expr Value {
        return feature result : Thing;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (action-derived-fact (rule_id "kerml-1.0:8.3.3.3.4:deriveFeatureCrossFeature") (source "Model::Link::crossed") (target "Model::Thing::inner") (outcome resolved))
  (action-derived-fact (rule_id "kerml-1.0:8.3.3.3.4:deriveFeatureFeatureTarget") (source "Model::Vehicle::path") (target "Model::Vehicle::base") (outcome resolved))
  (action-derived-fact (rule_id "kerml-1.0:8.3.3.3.4:deriveFeatureFeatureTarget") (source "Model::Vehicle::plain") (target "Model::Vehicle::plain") (outcome resolved))
  (action-derived-fact (rule_id "kerml-1.0:8.3.4.7.4:deriveFunctionResult") (source "Model::Computing") (target "Model::Computing::result") (outcome resolved))
  (action-derived-fact (rule_id "kerml-1.0:8.3.4.7.3:deriveExpressionResult") (source "Model::Value") (target "Model::Value::result") (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_cross_feature_and_result.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "cross_feature_type_mismatch")
        (source "semantic")
        (range (start 6 44) (end 6 56))
      )
      (diagnostic
        (severity warning)
        (code "feature_chaining_single_operand")
        (source "semantic")
        (range (start 10 28) (end 10 32))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:9b4fac1a41be1c7a4fd683039ed61b946ec92abaa42781440c79d016d6479feb"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Computing"))) (kind kerml-function) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Computing::result"))) (kind parameter) (membership (kind feature) (visibility default) (role return-parameter)) (authored (membership (kind feature) (visibility default) (role return-parameter)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Link"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Link::crossed"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")) (crossSubsetting (reference "source::inner")))))
    (declaration (id (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Link::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Thing::inner"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Value"))) (kind kerml-expression) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Value::result"))) (kind parameter) (membership (kind feature) (visibility default) (role return-parameter)) (authored (membership (kind feature) (visibility default) (role return-parameter)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Vehicle"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Vehicle::base"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Vehicle::path"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureChaining (reference "base")))))
    (declaration (id (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Vehicle::plain"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Computing::result"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Link::crossed"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Link::crossed"))) (kind crossSubsetting) (ordinal 0))
      (authored-target "source::inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Thing::inner")))))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Link::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Value::result"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Vehicle::path"))) (kind featureChaining) (ordinal 0))
      (authored-target "base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Vehicle::base")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Computing::result"))) (target (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Computing::result"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Link::crossed"))) (target (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Link::crossed"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind crossSubsetting) (source (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Link::crossed"))) (target (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Thing::inner"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Link::crossed"))) (kind crossSubsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Link::source"))) (target (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Link::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Value::result"))) (target (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Value::result"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind featureChaining) (source (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Vehicle::path"))) (target (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Vehicle::base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Vehicle::path"))) (kind featureChaining) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Computing::result"))) (target (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Computing"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Link::crossed"))) (target (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Link"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Link::source"))) (target (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Link"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Thing::inner"))) (target (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Thing"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Value::result"))) (target (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Value"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Vehicle::base"))) (target (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Vehicle::path"))) (target (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Vehicle::plain"))) (target (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Vehicle"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Computing::result")))
      (featured-by (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Computing")))
      (type (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Link::crossed")))
      (featured-by (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Link")))
      (type (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Thing")) (scopes any))
      (supertype (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Thing::inner")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Link::source")))
      (featured-by (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Link")))
      (type (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Thing")))
      (subtype (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Computing::result")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Link::crossed")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Link::source")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Value::result")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Thing::inner")))
      (featured-by (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Thing")))
      (subtype (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Link::crossed")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Value::result")))
      (featured-by (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Value")))
      (type (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Vehicle::base")))
      (featured-by (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Vehicle::path")))
      (featured-by (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Vehicle::plain")))
      (featured-by (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Vehicle")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_cross_feature_and_result.md") (range (start 14 32) (end 14 37)) (probe (position 14 32))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Computing::result"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_cross_feature_and_result.md") (range (start 6 30) (end 6 35)) (probe (position 6 30))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Link::crossed"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_cross_feature_and_result.md") (range (start 6 44) (end 6 56)) (probe (position 6 44))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Link::crossed"))) (kind crossSubsetting) (ordinal 0) (authored-target "source::inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Thing::inner")))))
    )
  )
  (query (document "memory://snapshot/kerml_cross_feature_and_result.md") (range (start 5 29) (end 5 34)) (probe (position 5 29))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Link::source"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_cross_feature_and_result.md") (range (start 17 32) (end 17 37)) (probe (position 17 32))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Value::result"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_cross_feature_and_result.md") (range (start 10 28) (end 10 32)) (probe (position 10 28))
    (reference (id (source (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Vehicle::path"))) (kind featureChaining) (ordinal 0) (authored-target "base")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_cross_feature_and_result.md") (qualified-name "Model::Vehicle::base")))))
    )
  )
)
~~~
