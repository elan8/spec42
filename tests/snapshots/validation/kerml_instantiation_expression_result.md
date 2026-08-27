# META
~~~ini
description=KerML 8.3.4.8.7 validateInstantiationExpressionResult requires an InstantiationExpression to own its result parameter
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.8.7 validateInstantiationExpressionResult
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.4.8.7:validateInstantiationExpressionResult
type=file
~~~
# SOURCE
~~~kerml
// Conforming: the invocation below owns the result parameter its value is bound to.
//
// The violating side has no textual counterpart: a KerML source document never authors an
// expression's result parameter separately from the expression, so it cannot produce an
// InstantiationExpression whose result parameter is owned by another type.
package Expressions {
    classifier Thing;
    function Identity {
        in feature input : Thing;
        return feature result : Thing;
    }
    classifier Holder {
        feature source : Thing;
        feature copied = Identity(input = source);
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_instantiation_expression_result.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_instantiation_expression_result.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:5fb952f269aa2a10b407d968aa08c13154e670631ee4615f567dc2206563183f"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Holder::copied"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "source")) (invocationCallee (reference "Identity")))))
    (declaration (id (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Holder::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Identity"))) (kind kerml-function) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Identity::input"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing") (direction in)))))
    (declaration (id (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Identity::result"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Holder::source")))))
    (reference (id (source (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0))
      (authored-target "Identity")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Identity")))))
    (reference (id (source (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Holder::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Identity::input"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Identity::result"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Thing")))))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Holder::source"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Identity"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Holder::source"))) (target (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Holder::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Identity::input"))) (target (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Identity::input"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Identity::result"))) (target (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Identity::result"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Holder::copied"))) (target (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Holder"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Holder::copied"))) (target (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Identity"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Holder::source"))) (target (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Identity::input"))) (target (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Identity"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Identity::result"))) (target (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Identity"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
    (invocation (declaration (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (callee (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Identity"))) (supplied 1) (required 0) (start 13 25) (end 13 49))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Holder::copied")))
      (featured-by (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Holder")))
      (supertype (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (type (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Identity")) (provenance implied))
      (effective-type (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Identity")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Identity")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Holder::copied")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Holder::source")))
      (featured-by (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Holder")))
      (type (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Identity")))
      (subtype (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Identity::input")))
      (featured-by (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Identity")))
      (type (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Identity::result")))
      (featured-by (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Identity")))
      (type (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Thing")))
      (subtype (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Holder::source")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Identity::input")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Identity::result")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_instantiation_expression_result.md") (range (start 13 42) (end 13 48)) (probe (position 13 42))
    (reference (id (source (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Holder::source")))))
    )
  )
  (query (document "memory://snapshot/kerml_instantiation_expression_result.md") (range (start 13 25) (end 13 33)) (probe (position 13 25))
    (reference (id (source (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0) (authored-target "Identity")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Identity")))))
    )
  )
  (query (document "memory://snapshot/kerml_instantiation_expression_result.md") (range (start 12 25) (end 12 30)) (probe (position 12 25))
    (reference (id (source (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Holder::source"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_instantiation_expression_result.md") (range (start 8 27) (end 8 32)) (probe (position 8 27))
    (reference (id (source (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Identity::input"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_instantiation_expression_result.md") (range (start 9 32) (end 9 37)) (probe (position 9 32))
    (reference (id (source (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Identity::result"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_instantiation_expression_result.md") (qualified-name "Expressions::Thing")))))
    )
  )
)
~~~
