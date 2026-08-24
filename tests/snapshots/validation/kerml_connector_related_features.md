# META
~~~ini
description=KerML 8.3.4.5.3 validateConnectorRelatedFeatures requires a concrete Connector to have at least two relatedFeatures
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.5.3 validateConnectorRelatedFeatures
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.4.5.3:validateConnectorRelatedFeatures
blocked_by=semantic-connector-related-features-insufficient
type=file
~~~
# SOURCE
~~~kerml
package Connectors {
    classifier Thing;
    classifier Holder {
        feature a : Thing;
        feature b : Thing;

        // Conforming: a concrete connector with two related features.
        connector pair { end feature e1 :>> a; end feature e2 :>> b; }

        // Invalid: a concrete connector with only one related feature.
        connector lone { end feature only :>> a; }
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_connector_related_features.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "connector_related_features_insufficient")
        (source "semantic")
        (range (start 10 8) (end 10 50))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_connector_related_features.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:30192216a900f52171f5c504fd0c3db3b8a54dc36275817641c37aa5567a08d7") (contract-version "parser-owned-resolution-v2"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::a"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::b"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::lone"))) (kind kerml-connector) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::lone::only"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "a")))))
    (declaration (id (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::pair"))) (kind kerml-connector) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::pair::e1"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "a")))))
    (declaration (id (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::pair::e2"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "b")))))
    (declaration (id (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::b"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::lone::only"))) (kind redefinition) (ordinal 0))
      (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::a")))))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::pair::e1"))) (kind redefinition) (ordinal 0))
      (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::a")))))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::pair::e2"))) (kind redefinition) (ordinal 0))
      (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::b")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::a"))) (target (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::b"))) (target (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::lone::only"))) (target (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::lone::only"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::pair::e1"))) (target (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::pair::e1"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::pair::e2"))) (target (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::pair::e2"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::a"))) (target (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::b"))) (target (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::lone"))) (target (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::lone::only"))) (target (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::lone"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::pair"))) (target (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::pair::e1"))) (target (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::pair"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::pair::e2"))) (target (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::pair"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::a")))
      (featured-by (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder")))
      (type (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Thing")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::lone::only")) (scopes any feature))
      (subtype (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::pair::e1")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::b")))
      (featured-by (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder")))
      (type (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Thing")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::pair::e2")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::lone")))
      (featured-by (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::lone::only")))
      (featured-by (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::lone")))
      (effective-type (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::a"))))
      (supertype (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::a")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::pair")))
      (featured-by (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::pair::e1")))
      (featured-by (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::pair")))
      (effective-type (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::a"))))
      (supertype (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::a")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::pair::e2")))
      (featured-by (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::pair")))
      (effective-type (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::b"))))
      (supertype (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::b")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Thing")))
      (subtype (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::a")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::b")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_connector_related_features.md") (range (start 3 20) (end 3 25)) (probe (position 3 20))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::a"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_connector_related_features.md") (range (start 4 20) (end 4 25)) (probe (position 4 20))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::b"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_connector_related_features.md") (range (start 10 46) (end 10 47)) (probe (position 10 46))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::lone::only"))) (kind redefinition) (ordinal 0) (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::a")))))
    )
  )
  (query (document "memory://snapshot/kerml_connector_related_features.md") (range (start 7 44) (end 7 45)) (probe (position 7 44))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::pair::e1"))) (kind redefinition) (ordinal 0) (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::a")))))
    )
  )
  (query (document "memory://snapshot/kerml_connector_related_features.md") (range (start 7 66) (end 7 67)) (probe (position 7 66))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::pair::e2"))) (kind redefinition) (ordinal 0) (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_related_features.md") (qualified-name "Connectors::Holder::b")))))
    )
  )
)
~~~
