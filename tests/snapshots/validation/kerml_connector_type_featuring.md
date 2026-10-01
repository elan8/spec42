# META
~~~ini
description=KerML 8.3.4.5.3 checkConnectorTypeFeaturing requires each relatedFeature to be featured within every connector featuringType
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.5.3:checkConnectorTypeFeaturing
blocked_by=semantic-connector-context-featuring-type
type=file
~~~
# SOURCE
~~~kerml
package Connectors {
    classifier Thing;
    classifier Holder {
        feature a : Thing;
        feature b : Thing;
    }

    // A connector with no owning type is featured by the innermost featuring type its related
    // features share, so each related feature is featured within it.
    connector pair { end feature e1 ::> Holder::a; end feature e2 ::> Holder::b; }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship
    (kind type_featuring)
    (source "Connectors::pair")
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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:85ae814fb2b1c28efee97f9d6767e862d6a9af7aeff124209703e5e6211c896b"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::a"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::b"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::pair"))) (kind kerml-connector) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::pair::e1"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (referenceSubsetting (reference "Holder::a")))))
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::pair::e2"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (referenceSubsetting (reference "Holder::b")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::b"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::pair::e1"))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "Holder::a")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::a")))))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::pair::e2"))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "Holder::b")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::b")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::a"))) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::b"))) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::pair::e1"))) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::pair::e1"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::pair::e2"))) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::pair::e2"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::a"))) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::b"))) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::pair::e1"))) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::pair"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::pair::e2"))) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::pair"))) (provenance implied))
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
      (subtype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::pair::e1")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::b")))
      (featured-by (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder")))
      (type (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::pair::e2")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")))
      (subtype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::a")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::b")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::pair::e1")))
      (featured-by (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::pair")))
      (effective-type (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::a"))))
      (supertype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::a")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::pair::e2")))
      (featured-by (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::pair")))
      (effective-type (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::b"))))
      (supertype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::b")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")) (scopes any))
    )
)
~~~
# CONNECTIONS
~~~sexpr
(connections
  (connector (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::pair"))) (kind connection))
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
  (query (document "memory://snapshot/kerml_connector_type_featuring.md") (range (start 9 40) (end 9 49)) (probe (position 9 40))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::pair::e1"))) (kind referenceSubsetting) (ordinal 0) (authored-target "Holder::a")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::a")))))
    )
  )
  (query (document "memory://snapshot/kerml_connector_type_featuring.md") (range (start 9 70) (end 9 79)) (probe (position 9 70))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::pair::e2"))) (kind referenceSubsetting) (ordinal 0) (authored-target "Holder::b")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::b")))))
    )
  )
)
~~~
