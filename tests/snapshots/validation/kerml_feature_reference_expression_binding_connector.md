# META
~~~ini
description=KerML 8.3.4.8.5 checkFeatureReferenceExpressionBindingConnector requires a canonical binding connector between targetFeature and result
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.8.5:checkFeatureReferenceExpressionBindingConnector
blocked_by=lowering-gap-binding-connector-feature-reference-endpoints
type=file
~~~
# SOURCE
~~~kerml
package References {
    classifier Thing;
    classifier Holder {
        feature referent : Thing;
        feature reference = referent;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (binding-connector-check
    (rule_id "kerml-1.0:8.3.4.8.5:checkFeatureReferenceExpressionBindingConnector")
    (outcome satisfied)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:e5e7f1b5383712b62518016a40e253c0c1948a688a47e122fed5815232e331c9") (contract-version "semantic-metadata-projection-v6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (qualified-name "References"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (qualified-name "References::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (qualified-name "References::Holder::reference"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (path (named (kind package) (name "References")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "reference")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (path (named (kind package) (name "References")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "reference")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (path (named (kind package) (name "References")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "reference")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (path (named (kind package) (name "References")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "reference")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "referent")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (path (named (kind package) (name "References")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "reference")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (qualified-name "References::Holder::referent"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (qualified-name "References::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (path (named (kind package) (name "References")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "reference")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "referent")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (qualified-name "References::Holder::referent")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (qualified-name "References::Holder::referent"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (qualified-name "References::Thing")))))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (path (named (kind package) (name "References")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "reference")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (qualified-name "References::Holder::referent"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (path (named (kind package) (name "References")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "reference")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (qualified-name "References::Holder::referent"))) (target (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (qualified-name "References::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (qualified-name "References::Holder::referent"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (qualified-name "References::Holder::reference"))) (target (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (qualified-name "References::Holder"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (qualified-name "References::Holder::reference"))) (target (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (path (named (kind package) (name "References")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "reference")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (path (named (kind package) (name "References")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "reference")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (path (named (kind package) (name "References")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "reference")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (qualified-name "References::Holder::referent"))) (target (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (qualified-name "References::Holder"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (path (named (kind package) (name "References")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "reference")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (qualified-name "References::Holder::reference")))
      (featured-by (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (qualified-name "References::Holder")))
      (supertype (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (path (named (kind package) (name "References")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "reference")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (path (named (kind package) (name "References")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "reference")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (path (named (kind package) (name "References")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "reference")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (qualified-name "References::Holder::reference")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (qualified-name "References::Holder::referent")))
      (featured-by (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (qualified-name "References::Holder")))
      (type (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (qualified-name "References::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (qualified-name "References::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (qualified-name "References::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (qualified-name "References::Thing")))
      (subtype (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (qualified-name "References::Holder::referent")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (range (start 4 28) (end 4 36)) (probe (position 4 28))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (path (named (kind package) (name "References")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "reference")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "referent")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (qualified-name "References::Holder::referent")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (range (start 3 27) (end 3 32)) (probe (position 3 27))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (qualified-name "References::Holder::referent"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_reference_expression_binding_connector.md") (qualified-name "References::Thing")))))
    )
  )
)
~~~
