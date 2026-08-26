# META
~~~ini
description=KerML 8.3.4.5.2 validateBindingConnectorIsBinary requires a BindingConnector to be binary
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.5.2 validateBindingConnectorIsBinary
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.4.5.2:validateBindingConnectorIsBinary
blocked_by=parser-gap-69-connector-end-body
type=file
~~~
# SOURCE
~~~kerml
package Bindings {
    classifier Thing;
    classifier Holder {
        feature a : Thing;
        feature b : Thing;
        feature c : Thing;

        // Conforming: a binary binding connector.
        binding of a = b;

        // Invalid: a binding connector with three ends.
        binding tern { end feature e1 :>> a; end feature e2 :>> b; end feature e3 :>> c; }
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_binding_connector_is_binary.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "binding_connector_not_binary")
        (source "semantic")
        (range (start 11 8) (end 11 90))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_binding_connector_is_binary.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 8 8) (end 11 8))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 11 8) (end 12 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:ef4dd47cea28caebaeb42ffaab482d7ccbc39a84f20a6a0c97068df78575a76a") (contract-version "semantic-metadata-projection-v6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::a"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::b"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::c"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::b"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::c"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::a"))) (target (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::b"))) (target (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::c"))) (target (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::c"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::a"))) (target (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::b"))) (target (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::c"))) (target (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::a")))
      (featured-by (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder")))
      (type (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::b")))
      (featured-by (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder")))
      (type (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::c")))
      (featured-by (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder")))
      (type (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Thing")))
      (subtype (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::a")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::b")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::c")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_binding_connector_is_binary.md") (range (start 3 20) (end 3 25)) (probe (position 3 20))
    (reference (id (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::a"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_binding_connector_is_binary.md") (range (start 4 20) (end 4 25)) (probe (position 4 20))
    (reference (id (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::b"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_binding_connector_is_binary.md") (range (start 5 20) (end 5 25)) (probe (position 5 20))
    (reference (id (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::c"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Thing")))))
    )
  )
)
~~~
