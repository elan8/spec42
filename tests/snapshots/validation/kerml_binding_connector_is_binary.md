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
blocked_by=lowering-gap-binding-body-end-count
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
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:ef4dd47cea28caebaeb42ffaab482d7ccbc39a84f20a6a0c97068df78575a76a"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (path (named (kind package) (name "Bindings")) (named (kind kerml-classifier) (name "Holder")) (anonymous (kind kerml-binding) (ordinal 0))))) (kind kerml-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindSource (reference "a")) (bindTarget (reference "b")))))
    (declaration (id (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::a"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::b"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::c"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::tern"))) (kind kerml-binding) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::tern::e1"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "a")))))
    (declaration (id (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::tern::e2"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "b")))))
    (declaration (id (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::tern::e3"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "c")))))
    (declaration (id (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (path (named (kind package) (name "Bindings")) (named (kind kerml-classifier) (name "Holder")) (anonymous (kind kerml-binding) (ordinal 0))))) (kind bindSource) (ordinal 0))
      (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::a")))))
    (reference (id (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (path (named (kind package) (name "Bindings")) (named (kind kerml-classifier) (name "Holder")) (anonymous (kind kerml-binding) (ordinal 0))))) (kind bindTarget) (ordinal 0))
      (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::b")))))
    (reference (id (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::b"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::c"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::tern::e1"))) (kind redefinition) (ordinal 0))
      (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::a")))))
    (reference (id (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::tern::e2"))) (kind redefinition) (ordinal 0))
      (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::b")))))
    (reference (id (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::tern::e3"))) (kind redefinition) (ordinal 0))
      (authored-target "c")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::c")))))
  )
  (relationships
    (relationship (kind bindSource) (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (path (named (kind package) (name "Bindings")) (named (kind kerml-classifier) (name "Holder")) (anonymous (kind kerml-binding) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (path (named (kind package) (name "Bindings")) (named (kind kerml-classifier) (name "Holder")) (anonymous (kind kerml-binding) (ordinal 0))))) (kind bindSource) (ordinal 0)))
    (relationship (kind bindTarget) (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (path (named (kind package) (name "Bindings")) (named (kind kerml-classifier) (name "Holder")) (anonymous (kind kerml-binding) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (path (named (kind package) (name "Bindings")) (named (kind kerml-classifier) (name "Holder")) (anonymous (kind kerml-binding) (ordinal 0))))) (kind bindTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::a"))) (target (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::b"))) (target (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::c"))) (target (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::c"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::tern::e1"))) (target (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::tern::e1"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::tern::e2"))) (target (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::tern::e2"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::tern::e3"))) (target (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::c"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::tern::e3"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (path (named (kind package) (name "Bindings")) (named (kind kerml-classifier) (name "Holder")) (anonymous (kind kerml-binding) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::a"))) (target (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::b"))) (target (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::c"))) (target (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::tern"))) (target (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::tern::e1"))) (target (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::tern"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::tern::e2"))) (target (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::tern"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::tern::e3"))) (target (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::tern"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (path (named (kind package) (name "Bindings")) (named (kind kerml-classifier) (name "Holder")) (anonymous (kind kerml-binding) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::a")))
      (featured-by (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder")))
      (type (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Thing")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::tern::e1")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::b")))
      (featured-by (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder")))
      (type (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Thing")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::tern::e2")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::c")))
      (featured-by (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder")))
      (type (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Thing")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::tern::e3")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::tern")))
      (featured-by (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::tern::e1")))
      (featured-by (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::tern")))
      (effective-type (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::a"))))
      (supertype (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::a")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::tern::e2")))
      (featured-by (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::tern")))
      (effective-type (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::b"))))
      (supertype (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::b")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::tern::e3")))
      (featured-by (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::tern")))
      (effective-type (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::c"))))
      (supertype (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::c")) (scopes any feature))
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
  (query (document "memory://snapshot/kerml_binding_connector_is_binary.md") (range (start 8 19) (end 8 20)) (probe (position 8 19))
    (reference (id (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (path (named (kind package) (name "Bindings")) (named (kind kerml-classifier) (name "Holder")) (anonymous (kind kerml-binding) (ordinal 0))))) (kind bindSource) (ordinal 0) (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::a")))))
    )
  )
  (query (document "memory://snapshot/kerml_binding_connector_is_binary.md") (range (start 8 23) (end 8 24)) (probe (position 8 23))
    (reference (id (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (path (named (kind package) (name "Bindings")) (named (kind kerml-classifier) (name "Holder")) (anonymous (kind kerml-binding) (ordinal 0))))) (kind bindTarget) (ordinal 0) (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::b")))))
    )
  )
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
  (query (document "memory://snapshot/kerml_binding_connector_is_binary.md") (range (start 11 42) (end 11 43)) (probe (position 11 42))
    (reference (id (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::tern::e1"))) (kind redefinition) (ordinal 0) (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::a")))))
    )
  )
  (query (document "memory://snapshot/kerml_binding_connector_is_binary.md") (range (start 11 64) (end 11 65)) (probe (position 11 64))
    (reference (id (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::tern::e2"))) (kind redefinition) (ordinal 0) (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::b")))))
    )
  )
  (query (document "memory://snapshot/kerml_binding_connector_is_binary.md") (range (start 11 86) (end 11 87)) (probe (position 11 86))
    (reference (id (source (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::tern::e3"))) (kind redefinition) (ordinal 0) (authored-target "c")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_binding_connector_is_binary.md") (qualified-name "Bindings::Holder::c")))))
    )
  )
)
~~~
