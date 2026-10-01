# META
~~~ini
description=KerML 8.3.4.5.3 checkConnectorTypeFeaturing requires each relatedFeature to be featured within every connector featuringType
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.5.3:checkConnectorTypeFeaturing
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

    // Of a featuring type and its specialization, the innermost (most specialized) is the context.
    classifier Sub specializes Holder { feature c : Thing; }
    connector mixed { end feature e1 ::> Holder::a; end feature e2 ::> Sub::c; }

    // Related features with no common featuring type give the connector none.
    feature x : Thing;
    feature y : Thing;
    connector loose { end feature e1 ::> x; end feature e2 ::> y; }
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
    (outcome resolved))
  (relationship
    (kind type_featuring)
    (source "Connectors::mixed")
    (target "Connectors::Sub")
    (provenance implied)
    (outcome resolved))
  (relationship
    (kind type_featuring)
    (source "Connectors::loose")
    (provenance implied)
    (outcome absent)))
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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:488912c0d03d0db55f3f56d4050947c31ff2f0d7e83c85230ba6c388d4d56aea"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::a"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::b"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Sub"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Holder")))))
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Sub::c"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::loose"))) (kind kerml-connector) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::loose::e1"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (referenceSubsetting (reference "x")))))
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::loose::e2"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (referenceSubsetting (reference "y")))))
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::mixed"))) (kind kerml-connector) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::mixed::e1"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (referenceSubsetting (reference "Holder::a")))))
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::mixed::e2"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (referenceSubsetting (reference "Sub::c")))))
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::pair"))) (kind kerml-connector) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::pair::e1"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (referenceSubsetting (reference "Holder::a")))))
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::pair::e2"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (referenceSubsetting (reference "Holder::b")))))
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::x"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::y"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::b"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Sub"))) (kind specialization) (ordinal 0))
      (authored-target "Holder")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder")))))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Sub::c"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::loose::e1"))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::x")))))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::loose::e2"))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "y")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::y")))))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::mixed::e1"))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "Holder::a")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::a")))))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::mixed::e2"))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "Sub::c")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Sub::c")))))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::pair::e1"))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "Holder::a")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::a")))))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::pair::e2"))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "Holder::b")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::b")))))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::y"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::a"))) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::b"))) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Sub"))) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Sub"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Sub::c"))) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Sub::c"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::loose::e1"))) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::x"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::loose::e1"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::loose::e2"))) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::y"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::loose::e2"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::mixed::e1"))) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::mixed::e1"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::mixed::e2"))) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Sub::c"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::mixed::e2"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::pair::e1"))) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::pair::e1"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::pair::e2"))) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::pair::e2"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::x"))) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::x"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::y"))) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::y"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::a"))) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::b"))) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Sub::c"))) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Sub"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::loose::e1"))) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::loose"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::loose::e2"))) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::loose"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::mixed"))) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Sub"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::mixed::e1"))) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::mixed"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::mixed::e2"))) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::mixed"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::pair"))) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder"))) (provenance implied))
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
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder")))
      (subtype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Sub")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::a")))
      (featured-by (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder")))
      (type (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::mixed::e1")) (scopes any feature))
      (subtype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::pair::e1")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::b")))
      (featured-by (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder")))
      (type (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::pair::e2")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Sub")))
      (supertype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Sub::c")))
      (featured-by (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Sub")))
      (type (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::mixed::e2")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")))
      (subtype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::a")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::b")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Sub::c")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::x")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::y")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::loose::e1")))
      (featured-by (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::loose")))
      (effective-type (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::x"))))
      (supertype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")) (scopes any))
      (supertype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::x")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::loose::e2")))
      (featured-by (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::loose")))
      (effective-type (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::y"))))
      (supertype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")) (scopes any))
      (supertype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::y")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::mixed")))
      (featured-by (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Sub")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::mixed::e1")))
      (featured-by (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::mixed")))
      (effective-type (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::a"))))
      (supertype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::a")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::mixed::e2")))
      (featured-by (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::mixed")))
      (effective-type (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Sub::c"))))
      (supertype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Sub::c")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::pair")))
      (featured-by (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder")))
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
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::x")))
      (type (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::loose::e1")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::y")))
      (type (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::loose::e2")) (scopes any feature))
    )
)
~~~
# CONNECTIONS
~~~sexpr
(connections
  (connector (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::loose"))) (kind connection))
  (connector (id (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::mixed"))) (kind connection))
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
  (query (document "memory://snapshot/kerml_connector_type_featuring.md") (range (start 12 31) (end 12 37)) (probe (position 12 31))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Sub"))) (kind specialization) (ordinal 0) (authored-target "Holder")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder")))))
    )
  )
  (query (document "memory://snapshot/kerml_connector_type_featuring.md") (range (start 12 52) (end 12 57)) (probe (position 12 52))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Sub::c"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_connector_type_featuring.md") (range (start 18 41) (end 18 42)) (probe (position 18 41))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::loose::e1"))) (kind referenceSubsetting) (ordinal 0) (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::x")))))
    )
  )
  (query (document "memory://snapshot/kerml_connector_type_featuring.md") (range (start 18 63) (end 18 64)) (probe (position 18 63))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::loose::e2"))) (kind referenceSubsetting) (ordinal 0) (authored-target "y")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::y")))))
    )
  )
  (query (document "memory://snapshot/kerml_connector_type_featuring.md") (range (start 13 41) (end 13 50)) (probe (position 13 41))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::mixed::e1"))) (kind referenceSubsetting) (ordinal 0) (authored-target "Holder::a")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Holder::a")))))
    )
  )
  (query (document "memory://snapshot/kerml_connector_type_featuring.md") (range (start 13 71) (end 13 77)) (probe (position 13 71))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::mixed::e2"))) (kind referenceSubsetting) (ordinal 0) (authored-target "Sub::c")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Sub::c")))))
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
  (query (document "memory://snapshot/kerml_connector_type_featuring.md") (range (start 16 16) (end 16 21)) (probe (position 16 16))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::x"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_connector_type_featuring.md") (range (start 17 16) (end 17 21)) (probe (position 17 16))
    (reference (id (source (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::y"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_connector_type_featuring.md") (qualified-name "Connectors::Thing")))))
    )
  )
)
~~~
