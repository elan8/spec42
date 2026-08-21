# META
~~~ini
description=KerML 8.3.4.8.8 checkInvocationExpressionBehaviorBindingConnector requires an invocation expression's behavior/result canonical binding connector when its instantiated type is not a Function
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.8.8:checkInvocationExpressionBehaviorBindingConnector
blocked_by=lowering-gap-binding-connector-invocation-behavior-endpoints
type=file
~~~
# SOURCE
~~~kerml
package Invocations {
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
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (binding-connector-check
    (rule_id "kerml-1.0:8.3.4.8.8:checkInvocationExpressionBehaviorBindingConnector")
    (outcome satisfied)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:3728447cfa2d95e490a9688e04555e3c9ec4e181484d142b5af41bf6b5f29dcb") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::copied"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "source")) (invocationCallee (reference "Identity")))))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity"))) (kind kerml-function) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity::input"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing") (direction in)))))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity::result"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::copied"))) (kind expressionOperand) (ordinal 0))
      (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::source")))))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::copied"))) (kind invocationCallee) (ordinal 0))
      (authored-target "Identity")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity")))))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity::input"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity::result"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing")))))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::copied"))) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::source"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::copied"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::copied"))) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::copied"))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::source"))) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity::input"))) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity::input"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity::result"))) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity::result"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::copied"))) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::source"))) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity::input"))) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity::result"))) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::copied"))) (state non-constant))
    (invocation (declaration (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::copied"))) (callee (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity"))) (supplied 1) (required 0) (start 8 25) (end 8 49))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::copied")))
      (featured-by (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::source")))
      (featured-by (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder")))
      (type (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity::input")))
      (featured-by (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity")))
      (type (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity::result")))
      (featured-by (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity")))
      (type (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing")))
      (subtype (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::source")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity::input")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity::result")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (range (start 8 42) (end 8 48)) (probe (position 8 42))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::copied"))) (kind expressionOperand) (ordinal 0) (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::source")))))
    )
  )
  (query (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (range (start 8 25) (end 8 33)) (probe (position 8 25))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::copied"))) (kind invocationCallee) (ordinal 0) (authored-target "Identity")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity")))))
    )
  )
  (query (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (range (start 7 25) (end 7 30)) (probe (position 7 25))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::source"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (range (start 3 27) (end 3 32)) (probe (position 3 27))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity::input"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (range (start 4 32) (end 4 37)) (probe (position 4 32))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity::result"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing")))))
    )
  )
)
~~~
