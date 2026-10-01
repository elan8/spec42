# META
~~~ini
description=KerML 8.3.3.3.4 checkFeatureEndRedefinition is unresolved for a bare connector end, which mints no end Feature whose redefinitions could be read
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.3.3.4:checkFeatureEndRedefinition
coverage_role=secondary
type=file
~~~
# SOURCE
~~~kerml
package Redefinition {
    classifier A;
    classifier B;
    assoc Pair { end feature first : A; end feature second : B; }
    classifier Holder {
        feature a : A;
        feature b : B;
        connector link : Pair from a to b;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics (redefinition-check (rule_id "kerml-1.0:8.3.3.3.4:checkFeatureEndRedefinition") (outcome unresolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:67338feb2c027fed57d4127c661f078ea134dc749844d1f8e152e52d353f45ce"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::A"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::B"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder::a"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder::b"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "B")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder::link"))) (kind kerml-connector) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Pair")) (connectorEnd (reference "a")) (connectorEnd (reference "b")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Pair"))) (kind kerml-association) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Pair::first"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "A")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Pair::second"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "B")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::A")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder::b"))) (kind featureTyping) (ordinal 0))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::B")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder::link"))) (kind featureTyping) (ordinal 0))
      (authored-target "Pair")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Pair")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder::link"))) (kind connectorEnd) (ordinal 0))
      (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder::a")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder::link"))) (kind connectorEnd) (ordinal 1))
      (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder::b")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Pair::first"))) (kind featureTyping) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::A")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Pair::second"))) (kind featureTyping) (ordinal 0))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::B")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder::a"))) (target (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder::b"))) (target (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder::link"))) (target (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Pair"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder::link"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder::link"))) (target (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder::link"))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder::link"))) (target (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder::b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder::link"))) (kind connectorEnd) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Pair::first"))) (target (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Pair::first"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Pair::second"))) (target (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Pair::second"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder::a"))) (target (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder::b"))) (target (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder::link"))) (target (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Pair::first"))) (target (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Pair"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Pair::second"))) (target (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Pair"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::A")))
      (subtype (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder::a")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Pair::first")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::B")))
      (subtype (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder::b")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Pair::second")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder::a")))
      (featured-by (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder")))
      (type (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::A")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::A")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder::b")))
      (featured-by (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder")))
      (type (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::B")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::B")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::B")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder::link")))
      (featured-by (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder")))
      (type (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Pair")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Pair")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Pair")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Pair")))
      (subtype (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder::link")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Pair::first")))
      (featured-by (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Pair")))
      (type (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::A")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::A")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::A")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Pair::second")))
      (featured-by (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Pair")))
      (type (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::B")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::B")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::B")) (scopes any))
    )
)
~~~
# CONNECTIONS
~~~sexpr
(connections
  (connector (id (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder::link"))) (kind connection) (type (resolved (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Pair")))) (end bare (feature (resolved (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder::a"))))) (end bare (feature (resolved (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder::b"))))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (range (start 5 20) (end 5 21)) (probe (position 5 20))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder::a"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::A")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (range (start 6 20) (end 6 21)) (probe (position 6 20))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder::b"))) (kind featureTyping) (ordinal 0) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::B")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (range (start 7 25) (end 7 29)) (probe (position 7 25))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder::link"))) (kind featureTyping) (ordinal 0) (authored-target "Pair")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Pair")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (range (start 7 35) (end 7 36)) (probe (position 7 35))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder::link"))) (kind connectorEnd) (ordinal 0) (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder::a")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (range (start 7 40) (end 7 41)) (probe (position 7 40))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder::link"))) (kind connectorEnd) (ordinal 1) (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Holder::b")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (range (start 3 37) (end 3 38)) (probe (position 3 37))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Pair::first"))) (kind featureTyping) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::A")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (range (start 3 61) (end 3 62)) (probe (position 3 61))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::Pair::second"))) (kind featureTyping) (ordinal 0) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_end_redefinition_bare_end.md") (qualified-name "Redefinition::B")))))
    )
  )
)
~~~
