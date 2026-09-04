# META
~~~ini
description=KerML checkFeatureChainExpressionResultSpecialization desired semantics
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.8.4:checkFeatureChainExpressionResultSpecialization
~~~
# SOURCE
~~~kerml
package Model {
  classifier Thing { feature inner : Thing; }
  classifier Holder {
    feature outer : Thing;
    feature selected = outer.inner;
  }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics (specialization-check (rule_id "kerml-1.0:8.3.4.8.4:checkFeatureChainExpressionResultSpecialization") (outcome satisfied)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:07c585e6c7abed18b41b6b827ac42bdc2fd8460b27871d955985a9a211223135"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Holder::outer"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Holder::selected"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (memberAccessOperand (reference "outer::inner")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2))))) (kind kerml-feature) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Thing::inner"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Holder::outer"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "outer::inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Thing::inner")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Thing::inner"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Holder::outer"))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Holder::outer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Thing::inner"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Thing::inner"))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Thing::inner"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Holder::outer"))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Holder::selected"))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Holder"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Holder::selected"))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind featureChaining) (source (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2))))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2))))) (provenance implied))
    (relationship (kind featureChaining) (source (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2))))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Thing::inner"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Thing::inner"))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Thing"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0))))) (state unsupported))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Holder::outer")))
      (featured-by (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Holder")))
      (type (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Holder::selected")))
      (featured-by (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Holder")))
      (supertype (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Holder::selected")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2)))))
      (subtype (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)))))
      (effective-type (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Thing::inner"))))
      (supertype (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Thing")) (scopes any))
      (supertype (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Thing::inner")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Thing")))
      (subtype (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Holder::outer")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Thing::inner")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Thing::inner")))
      (featured-by (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Thing")))
      (type (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Thing")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome unsupported))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (range (start 3 20) (end 3 25)) (probe (position 3 20))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Holder::outer"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (range (start 4 23) (end 4 34)) (probe (position 4 23))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "outer::inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Thing::inner")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (range (start 1 37) (end 1 42)) (probe (position 1 37))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Thing::inner"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chain_expression_result_specialization.md") (qualified-name "Model::Thing")))))
    )
  )
)
~~~
