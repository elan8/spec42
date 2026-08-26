# META
~~~ini
description=Generated Connector binary specialization preserves the exact connectorEnd predicate and library anchor
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.5.3:checkConnectorBinarySpecialization
blocked_by=parser-gap-69-connector-end-body
type=file
libraries=standard
~~~
# SOURCE
~~~kerml
package BinaryConnectorSpecialization {
    classifier Thing;
    classifier Holder {
        feature a : Thing;
        feature b : Thing;
        connector pair { end feature source :>> a; end feature target :>> b; }
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind specialization) (source "BinaryConnectorSpecialization::Holder::pair") (target "Links::binaryLinks") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/generated_conditional_binary_connector_specialization.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:e87ca44d45b5100ab464ff5e7e3be917727a2a845d9a514ac9a54650c38fe696") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::a"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::b"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::pair"))) (kind kerml-connector) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::pair::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "a")))))
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::pair::target"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "b")))))
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Thing")))))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::b"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Thing")))))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::pair::source"))) (kind redefinition) (ordinal 0))
      (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::a")))))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::pair::target"))) (kind redefinition) (ordinal 0))
      (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::b")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::a"))) (target (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::b"))) (target (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::pair::source"))) (target (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::pair::source"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::pair::target"))) (target (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::pair::target"))) (kind redefinition) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::a"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::a"))) (target (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::b"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::b"))) (target (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::pair"))) (target (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::pair"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::pair::source"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::pair::source"))) (target (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::pair"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::pair::source"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::pair::target"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::pair::target"))) (target (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::pair"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::pair::target"))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::a")))
      (featured-by (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder")))
      (type (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Thing")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Thing")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (subtype (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::pair::source")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::b")))
      (featured-by (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder")))
      (type (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Thing")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Thing")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (subtype (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::pair::target")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::pair")))
      (featured-by (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::pair::source")))
      (featured-by (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::pair")))
      (effective-type (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Thing")) (source inherited) (from (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::a"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant"))))
      (supertype (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::a")) (scopes any feature))
      (supertype (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Thing")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::pair::target")))
      (featured-by (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::pair")))
      (effective-type (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Thing")) (source inherited) (from (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::b"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant"))))
      (supertype (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::b")) (scopes any feature))
      (supertype (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Thing")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Thing")))
      (subtype (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::a")) (scopes any))
      (subtype (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::b")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (range (start 3 20) (end 3 25)) (probe (position 3 20))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::a"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Thing")))))
    )
  )
  (query (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (range (start 4 20) (end 4 25)) (probe (position 4 20))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::b"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Thing")))))
    )
  )
  (query (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (range (start 5 48) (end 5 49)) (probe (position 5 48))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::pair::source"))) (kind redefinition) (ordinal 0) (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::a")))))
    )
  )
  (query (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (range (start 5 74) (end 5 75)) (probe (position 5 74))
    (reference (id (source (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::pair::target"))) (kind redefinition) (ordinal 0) (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/generated_conditional_binary_connector_specialization.md") (qualified-name "BinaryConnectorSpecialization::Holder::b")))))
    )
  )
)
~~~
