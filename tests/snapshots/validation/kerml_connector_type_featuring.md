# META
~~~ini
description=KerML 8.3.4.5.3 checkConnectorTypeFeaturing requires each relatedFeature to be featured within every connector featuringType
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.5.3:checkConnectorTypeFeaturing
blocked_by=lowering-gap-type-featuring-connector-related-features
type=file
~~~
# SOURCE
~~~kerml
package Connectors {
    classifier Thing;
    classifier Holder {
        feature a : Thing;
        feature b : Thing;
        connector pair { end feature e1 :>> a; end feature e2 :>> b; }
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship
    (kind type_featuring)
    (source "Connectors::Holder::pair::e1")
    (target "Connectors::Holder")
    (provenance implied)
    (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_connector_type_featuring.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:0821579b2a9b540def7a8731b7e179c8430cfba1450107fe93057f1571646d03") (contract-version "feature-value-expression-results-v5"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::a"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::b"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::pair"))) (kind kerml-connector) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::pair::e1"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "a")))))
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::pair::e2"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "b")))))
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::b"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::pair::e1"))) (kind redefinition) (ordinal 0))
      (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::a")))))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::pair::e2"))) (kind redefinition) (ordinal 0))
      (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::b")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::a"))) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::b"))) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::pair::e1"))) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::pair::e1"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::pair::e2"))) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::pair::e2"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::a"))) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::b"))) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::pair"))) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::pair::e1"))) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::pair"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::pair::e2"))) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::pair"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::a")))
      (featured-by (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder")))
      (type (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::pair::e1")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::b")))
      (featured-by (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder")))
      (type (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::pair::e2")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::pair")))
      (featured-by (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::pair::e1")))
      (featured-by (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::pair")))
      (effective-type (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::a"))))
      (supertype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::a")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::pair::e2")))
      (featured-by (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::pair")))
      (effective-type (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::b"))))
      (supertype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::b")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")))
      (subtype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::a")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::b")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_connector_type_featuring.md") (range (start 3 20) (end 3 25)) (probe (position 3 20))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::a"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_connector_type_featuring.md") (range (start 4 20) (end 4 25)) (probe (position 4 20))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::b"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_connector_type_featuring.md") (range (start 5 44) (end 5 45)) (probe (position 5 44))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::pair::e1"))) (kind redefinition) (ordinal 0) (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::a")))))
    )
  )
  (query (document "memory://snapshot/kerml_connector_type_featuring.md") (range (start 5 66) (end 5 67)) (probe (position 5 66))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::pair::e2"))) (kind redefinition) (ordinal 0) (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::b")))))
    )
  )
)
~~~
