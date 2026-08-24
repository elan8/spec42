# META
~~~ini
description=KerML 8.3.4.8.4 validateFeatureChainExpressionConformance requires the targetFeature of a FeatureChainExpression to be featured within the result of its argument Expression
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.8.4 validateFeatureChainExpressionConformance
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.4.8.4:validateFeatureChainExpressionConformance
blocked_by=lowering-kerml-feature-relationships
type=file
~~~
# SOURCE
~~~kerml
package Expressions {
    classifier Thing {
        feature inner : Thing;
    }
    classifier Other {
        feature elsewhere : Thing;
    }
    classifier Holder {
        feature outer : Thing;

        // Conforming: inner is featured within the type of outer.
        feature good = outer.inner;

        // Invalid: elsewhere is featured within Other, not within outer.
        feature bad = outer.elsewhere;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_chain_expression_conformance.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 14 8) (end 14 38))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_chain_expression_conformance.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 14 22) (end 14 37))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:6274850a969ee53dce9e2ec0a33350dbe4a3c0c7f8e240dd5458bc164c8f34df") (contract-version "parser-owned-resolution-v2"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Holder::bad"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "outer::elsewhere")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Holder::good"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "outer::inner")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Holder::outer"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Other"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Other::elsewhere"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Thing::inner"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Holder::bad"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "outer::elsewhere")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Holder::good"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "outer::inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Thing::inner")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Holder::outer"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Other::elsewhere"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Thing::inner"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Thing")))))
  )
  (relationships
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Holder::good"))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Thing::inner"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Holder::good"))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Holder::outer"))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Holder::outer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Other::elsewhere"))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Other::elsewhere"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Thing::inner"))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Thing::inner"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Holder::bad"))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Holder::good"))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Holder::outer"))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Other::elsewhere"))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Other"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Thing::inner"))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Thing"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Holder::bad"))) (state unsupported))
    (evaluated (declaration (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Holder::good"))) (state unsupported))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Holder::bad")))
      (featured-by (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Holder")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Holder::good")))
      (featured-by (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Holder")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Holder::outer")))
      (featured-by (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Holder")))
      (type (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Other::elsewhere")))
      (featured-by (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Other")))
      (type (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Thing")))
      (subtype (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Holder::outer")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Other::elsewhere")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Thing::inner")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Thing::inner")))
      (featured-by (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Thing")))
      (type (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (range (start 14 22) (end 14 37)) (probe (position 14 22))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Holder::bad"))) (kind memberAccessOperand) (ordinal 0) (authored-target "outer::elsewhere")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (range (start 11 23) (end 11 34)) (probe (position 11 23))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Holder::good"))) (kind memberAccessOperand) (ordinal 0) (authored-target "outer::inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Thing::inner")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (range (start 8 24) (end 8 29)) (probe (position 8 24))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Holder::outer"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (range (start 5 28) (end 5 33)) (probe (position 5 28))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Other::elsewhere"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (range (start 2 24) (end 2 29)) (probe (position 2 24))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Thing::inner"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chain_expression_conformance.md") (qualified-name "Expressions::Thing")))))
    )
  )
)
~~~
