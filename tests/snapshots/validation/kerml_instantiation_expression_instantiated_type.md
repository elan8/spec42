# META
~~~ini
description=KerML 8.3.4.8.7 validateInstantiationExpressionInstantiatedType requires an InstantiationExpression to have an instantiatedType
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.8.7 validateInstantiationExpressionInstantiatedType
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.4.8.7:validateInstantiationExpressionInstantiatedType
blocked_by=semantic-instantiation-expression-validation
type=file
~~~
# SOURCE
~~~kerml
package Expressions {
    classifier Thing;
    function Identity {
        in feature input : Thing;
        return feature result : Thing;
    }
    classifier Holder {
        feature source : Thing;

        // Conforming: the invocation names a resolvable instantiated type.
        feature copied = Identity(input = source);

        // Invalid: the instantiated type does not resolve, so the expression has none.
        feature wrong = Missing(input = source);
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 13 8) (end 13 48))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 13 24) (end 13 31))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:157b680144984afa0c68fff9d452381fe2a7e5a132a5f0352f854d979eda76fb"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::copied"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "source")) (invocationCallee (reference "Identity")))))
    (declaration (id (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::wrong"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "wrong")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "wrong")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "wrong")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "wrong")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "source")) (invocationCallee (reference "Missing")))))
    (declaration (id (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "wrong")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Identity"))) (kind kerml-function) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Identity::input"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing") (direction in)))))
    (declaration (id (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Identity::result"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::source")))))
    (reference (id (source (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0))
      (authored-target "Identity")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Identity")))))
    (reference (id (source (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "wrong")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::source")))))
    (reference (id (source (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "wrong")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0))
      (authored-target "Missing")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Identity::input"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Identity::result"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Thing")))))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::source"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Identity"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::source"))) (target (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "wrong")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::source"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "wrong")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Identity::input"))) (target (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Identity::input"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Identity::result"))) (target (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Identity::result"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::copied"))) (target (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Holder"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::copied"))) (target (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Identity"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::source"))) (target (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::wrong"))) (target (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Holder"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::wrong"))) (target (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "wrong")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "wrong")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "wrong")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Identity::input"))) (target (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Identity"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Identity::result"))) (target (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Identity"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "wrong")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
    (invocation (declaration (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (callee (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Identity"))) (supplied 1) (required 0) (start 10 25) (end 10 49))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::copied")))
      (featured-by (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Holder")))
      (supertype (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (type (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Identity")) (provenance implied))
      (effective-type (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Identity")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Identity")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::copied")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::source")))
      (featured-by (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Holder")))
      (type (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::wrong")))
      (featured-by (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Holder")))
      (supertype (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "wrong")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "wrong")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "wrong")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::wrong")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Identity")))
      (subtype (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Identity::input")))
      (featured-by (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Identity")))
      (type (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Identity::result")))
      (featured-by (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Identity")))
      (type (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Thing")))
      (subtype (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::source")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Identity::input")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Identity::result")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (range (start 10 42) (end 10 48)) (probe (position 10 42))
    (reference (id (source (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::source")))))
    )
  )
  (query (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (range (start 10 25) (end 10 33)) (probe (position 10 25))
    (reference (id (source (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0) (authored-target "Identity")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Identity")))))
    )
  )
  (query (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (range (start 7 25) (end 7 30)) (probe (position 7 25))
    (reference (id (source (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::source"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (range (start 13 40) (end 13 46)) (probe (position 13 40))
    (reference (id (source (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "wrong")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::source")))))
    )
  )
  (query (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (range (start 13 24) (end 13 31)) (probe (position 13 24))
    (reference (id (source (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "wrong")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0) (authored-target "Missing")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (range (start 3 27) (end 3 32)) (probe (position 3 27))
    (reference (id (source (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Identity::input"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (range (start 4 32) (end 4 37)) (probe (position 4 32))
    (reference (id (source (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Identity::result"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_instantiation_expression_instantiated_type.md") (qualified-name "Expressions::Thing")))))
    )
  )
)
~~~
