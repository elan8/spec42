# META
~~~ini
description=KerML 8.3.4.8.8 checkInvocationExpressionBehaviorBindingConnector requires an invocation expression's behavior/result canonical binding connector when its instantiated type is not a Function
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.8.8:checkInvocationExpressionBehaviorBindingConnector
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
    behavior Step {
        in feature input : Thing;
    }
    classifier Holder {
        feature source : Thing;
        // A Function invocation implies no behavior binding connector.
        feature copied = Identity(input = source);
        // A non-Function invocation binds the expression to its result.
        feature performed = Step(input = source);
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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:129f173d0dbcd5013a159e35d7008d3a3fbacd777e83c27004db9bf48c14698c"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::copied"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "source")) (invocationCallee (reference "Identity")))))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)) (effective-identification (name unresolved) (short-name unresolved) (provenance first-redefinition)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "input")))))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::performed"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "performed")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "performed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "performed")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "performed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "source")) (invocationCallee (reference "Step")))))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "performed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "performed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)) (effective-identification (name unresolved) (short-name unresolved) (provenance first-redefinition)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "input")))))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity"))) (kind kerml-function) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity::input"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing") (direction in)))))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity::result"))) (kind parameter) (membership (kind feature) (visibility default) (role return-parameter)) (authored (membership (kind feature) (visibility default) (role return-parameter)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Step"))) (kind kerml-behavior) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Step::input"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing") (direction in)))))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::source")))))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0))
      (authored-target "Identity")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity")))))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "input")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity::input")))))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "performed")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::source")))))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "performed")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0))
      (authored-target "Step")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Step")))))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "performed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "input")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Step::input")))))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity::input"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity::result"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Step::input"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing")))))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::source"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity::input"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "performed")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::source"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "performed")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "performed")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Step"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "performed")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "performed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Step::input"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "performed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::source"))) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity::input"))) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity::input"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity::result"))) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity::result"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Step::input"))) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Step::input"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::copied"))) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::copied"))) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity::result"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::performed"))) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::performed"))) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "performed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "performed")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "performed")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Step"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "performed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "performed")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "performed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "performed")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "performed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Step"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::source"))) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity::input"))) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity::result"))) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Step::input"))) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Step"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "performed")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
    (invocation (declaration (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (callee (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity"))) (supplied 1) (required 0) (start 12 25) (end 12 49))
    (invocation (declaration (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "performed")) (anonymous (kind kerml-expression) (ordinal 0))))) (callee (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Step"))) (supplied 1) (required 0) (start 14 28) (end 14 48))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::copied")))
      (featured-by (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder")))
      (effective-type (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity::result"))))
      (supertype (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity::result")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder")))
      (type (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity")) (provenance implied))
      (effective-type (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity::result"))))
      (supertype (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity::result")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::copied")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity::input"))))
      (supertype (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity::input")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::performed")))
      (featured-by (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder")))
      (effective-type (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Step")) (source inherited) (from (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "performed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))))
      (supertype (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "performed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Step")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "performed")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder")))
      (type (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Step")) (provenance implied))
      (effective-type (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Step")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Step")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "performed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "performed")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (type (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Step")) (provenance implied))
      (effective-type (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Step")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Step")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::performed")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "performed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "performed")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Step::input"))))
      (supertype (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Step::input")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::source")))
      (featured-by (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder")))
      (type (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity")))
      (subtype (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity::input")))
      (featured-by (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity")))
      (type (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity::result")))
      (featured-by (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity")))
      (type (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Step")))
      (subtype (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "performed")) (anonymous (kind kerml-expression) (ordinal 0)))) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "performed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Step::input")))
      (featured-by (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Step")))
      (type (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "performed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing")))
      (subtype (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::source")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity::input")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity::result")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Step::input")) (scopes any))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (unsupported (feature-reference "source" (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::source"))))))
  (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "performed")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (unsupported (feature-reference "source" (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::source"))))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (range (start 12 42) (end 12 48)) (probe (position 12 42))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::source")))))
    )
  )
  (query (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (range (start 12 25) (end 12 33)) (probe (position 12 25))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0) (authored-target "Identity")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity")))))
    )
  )
  (query (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (range (start 12 34) (end 12 39)) (probe (position 12 34))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "copied")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "input")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Identity::input")))))
    )
  )
  (query (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (range (start 14 41) (end 14 47)) (probe (position 14 41))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "performed")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Holder::source")))))
    )
  )
  (query (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (range (start 14 28) (end 14 32)) (probe (position 14 28))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "performed")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0) (authored-target "Step")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Step")))))
    )
  )
  (query (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (range (start 14 33) (end 14 38)) (probe (position 14 33))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (path (named (kind package) (name "Invocations")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "performed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "input")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Step::input")))))
    )
  )
  (query (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (range (start 10 25) (end 10 30)) (probe (position 10 25))
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
  (query (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (range (start 7 27) (end 7 32)) (probe (position 7 27))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Step::input"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_behavior_binding_connector.md") (qualified-name "Invocations::Thing")))))
    )
  )
)
~~~
