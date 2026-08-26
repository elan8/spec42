# META
~~~ini
description=KerML 8.3.4.10.2 checkFeatureValueBindingConnector requires the canonical binding connector relating a non-default FeatureValue to its value result
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.10.2:checkFeatureValueBindingConnector
blocked_by=lowering-gap-binding-connector-feature-value-endpoints
type=file
~~~
# SOURCE
~~~kerml
package Values {
    classifier Thing;
    classifier Holder {
        feature value : Thing;
        feature result : Thing;
        binding value = result;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (binding-connector-check
    (rule_id "kerml-1.0:8.3.4.10.2:checkFeatureValueBindingConnector")
    (outcome satisfied)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_value_binding_connector.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:29b5722f6fdf7c8d2ef0af7eb93c63514240176555300bcad473e6cc5d3e6150") (contract-version "constructor-expression-result-v8"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Holder")) (anonymous (kind kerml-binding) (ordinal 0))))) (kind kerml-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindSource (reference "value")) (bindTarget (reference "result")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Holder::result"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Holder::value"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Holder")) (anonymous (kind kerml-binding) (ordinal 0))))) (kind bindSource) (ordinal 0))
      (authored-target "value")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Holder::value")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Holder")) (anonymous (kind kerml-binding) (ordinal 0))))) (kind bindTarget) (ordinal 0))
      (authored-target "result")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Holder::result")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Holder::result"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Holder::value"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Thing")))))
  )
  (relationships
    (relationship (kind bindSource) (source (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Holder")) (anonymous (kind kerml-binding) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Holder::value"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Holder")) (anonymous (kind kerml-binding) (ordinal 0))))) (kind bindSource) (ordinal 0)))
    (relationship (kind bindTarget) (source (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Holder")) (anonymous (kind kerml-binding) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Holder::result"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Holder")) (anonymous (kind kerml-binding) (ordinal 0))))) (kind bindTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Holder::result"))) (target (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Holder::result"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Holder::value"))) (target (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Holder::value"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Holder")) (anonymous (kind kerml-binding) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Holder::result"))) (target (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Holder::value"))) (target (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Holder"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Holder")) (anonymous (kind kerml-binding) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Holder")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Holder::result")))
      (featured-by (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Holder")))
      (type (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Holder::value")))
      (featured-by (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Holder")))
      (type (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Thing")))
      (subtype (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Holder::result")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Holder::value")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_value_binding_connector.md") (range (start 5 16) (end 5 21)) (probe (position 5 16))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Holder")) (anonymous (kind kerml-binding) (ordinal 0))))) (kind bindSource) (ordinal 0) (authored-target "value")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Holder::value")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_value_binding_connector.md") (range (start 5 24) (end 5 30)) (probe (position 5 24))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (path (named (kind package) (name "Values")) (named (kind kerml-classifier) (name "Holder")) (anonymous (kind kerml-binding) (ordinal 0))))) (kind bindTarget) (ordinal 0) (authored-target "result")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Holder::result")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_value_binding_connector.md") (range (start 4 25) (end 4 30)) (probe (position 4 25))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Holder::result"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_value_binding_connector.md") (range (start 3 24) (end 3 29)) (probe (position 3 24))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Holder::value"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_value_binding_connector.md") (qualified-name "Values::Thing")))))
    )
  )
)
~~~
