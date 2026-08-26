# META
~~~ini
description=KerML checkFeatureChainExpressionResultSpecialization remains unresolved when the target feature cannot be resolved
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.8.4:checkFeatureChainExpressionResultSpecialization
~~~
# SOURCE
~~~kerml
package Model {
  classifier Thing;
  classifier Holder {
    feature outer : Thing;
    feature selected = outer.missing;
  }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics (specialization-check (rule_id "kerml-1.0:8.3.4.8.4:checkFeatureChainExpressionResultSpecialization") (outcome unresolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 4 23) (end 4 36))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:8ac95afd4bb7f13c7ec13260e2709336d7a98c8e8e5b92d792d696786a9b8fe5") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (qualified-name "Model::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (qualified-name "Model::Holder::outer"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (qualified-name "Model::Holder::selected"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (memberAccessOperand (reference "outer::missing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2))))) (kind kerml-feature) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (qualified-name "Model::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (qualified-name "Model::Holder::outer"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (qualified-name "Model::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "outer::missing")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (qualified-name "Model::Holder::outer"))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (qualified-name "Model::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (qualified-name "Model::Holder::outer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (qualified-name "Model::Holder::outer"))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (qualified-name "Model::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (qualified-name "Model::Holder::selected"))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (qualified-name "Model::Holder"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (qualified-name "Model::Holder::selected"))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind featureChaining) (source (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2))))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2))))) (provenance implied))
    (relationship (kind featureChaining) (source (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2))))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0))))) (state unsupported))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (qualified-name "Model::Holder::outer")))
      (featured-by (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (qualified-name "Model::Holder")))
      (type (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (qualified-name "Model::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (qualified-name "Model::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (qualified-name "Model::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (qualified-name "Model::Holder::selected")))
      (featured-by (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (qualified-name "Model::Holder")))
      (supertype (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (qualified-name "Model::Holder::selected")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2)))))
      (subtype (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)))))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (qualified-name "Model::Thing")))
      (subtype (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (qualified-name "Model::Holder::outer")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (range (start 3 20) (end 3 25)) (probe (position 3 20))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (qualified-name "Model::Holder::outer"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (qualified-name "Model::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (range (start 4 23) (end 4 36)) (probe (position 4 23))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization_unresolved.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "outer::missing")
      (outcome (status unresolved)))
    )
  )
)
~~~
