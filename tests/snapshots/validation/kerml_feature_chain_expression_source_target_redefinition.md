# META
~~~ini
description=KerML 8.3.4.8.4 checkFeatureChainExpressionSourceTargetRedefinition requires the source-target feature to redefine the chain target
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.8.4:checkFeatureChainExpressionSourceTargetRedefinition
type=file
~~~
# SOURCE
~~~kerml
package Redefinition {
    classifier Engine { feature power; }
    classifier Vehicle { feature engine : Engine; }
    classifier Holder {
        feature vehicle : Vehicle;
        feature selected = vehicle.engine;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics (redefinition-check (rule_id "kerml-1.0:8.3.4.8.4:checkFeatureChainExpressionSourceTargetRedefinition") (outcome satisfied)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:5caee43cf1c33f66e745ca84516faaa1dc0c0c13e55a74b90f3813ad4758cf4c"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Engine"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Engine::power"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Holder::selected"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (memberAccessOperand (reference "vehicle::engine")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2))))) (kind kerml-feature) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Holder::vehicle"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Vehicle"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Vehicle::engine"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "vehicle::engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Vehicle::engine")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Holder::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Vehicle::engine"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Engine")))))
  )
  (relationships
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Vehicle::engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Holder::vehicle"))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Holder::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Vehicle::engine"))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Vehicle::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Engine::power"))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Engine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Holder::selected"))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Holder"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Holder::selected"))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind featureChaining) (source (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2))))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2))))) (provenance implied))
    (relationship (kind featureChaining) (source (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2))))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Vehicle::engine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Holder::vehicle"))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Vehicle::engine"))) (target (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Vehicle"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0))))) (state unsupported))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Engine")))
      (subtype (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Vehicle::engine")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Engine::power")))
      (featured-by (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Engine")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Holder::selected")))
      (featured-by (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Holder")))
      (supertype (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Holder::selected")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 2)))))
      (subtype (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)))))
      (effective-type (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Engine")) (source inherited) (from (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Vehicle::engine"))))
      (supertype (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Engine")) (scopes any))
      (supertype (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Vehicle::engine")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Holder::vehicle")))
      (featured-by (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Holder")))
      (type (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Vehicle")))
      (subtype (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Holder::vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Vehicle::engine")))
      (featured-by (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Vehicle")))
      (type (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Engine")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Engine")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Engine")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome unsupported))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (range (start 5 27) (end 5 41)) (probe (position 5 27))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "selected")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "vehicle::engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Vehicle::engine")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (range (start 4 26) (end 4 33)) (probe (position 4 26))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Holder::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (range (start 2 42) (end 2 48)) (probe (position 2 42))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Vehicle::engine"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_chain_expression_source_target_redefinition.md") (qualified-name "Redefinition::Engine")))))
    )
  )
)
~~~
