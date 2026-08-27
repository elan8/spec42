# META
~~~ini
description=KerML 8.3.4.8.8 validateInvocationExpressionNoDuplicateParameterRedefinition forbids two different ownedFeatures of an InvocationExpression from redefining the same feature of the instantiatedType
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.8.8 validateInvocationExpressionNoDuplicateParameterRedefinition
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.4.8.8:validateInvocationExpressionNoDuplicateParameterRedefinition
blocked_by=semantic-invocation-duplicate-parameter-redefinition
type=file
~~~
# SOURCE
~~~kerml
package Expressions {
    classifier Thing;
    function Pair {
        in feature left : Thing;
        in feature right : Thing;
        return feature result : Thing;
    }
    classifier Holder {
        feature a : Thing;
        feature b : Thing;

        // Conforming: each argument redefines a different input parameter.
        feature good = Pair(left = a, right = b);

        // Invalid: two arguments redefine the same input parameter.
        feature bad = Pair(left = a, left = b);
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "invocation_duplicate_parameter_redefinition")
        (source "semantic")
        (range (start 15 8) (end 15 47))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:f05e485ed9f500bb73ac08f6ed9678aa36fd503aa29f1799e1305ed543b53fbd"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder::a"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder::b"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder::bad"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "a")) (expressionOperand (reference "b")) (invocationCallee (reference "Pair")))))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder::good"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "a")) (expressionOperand (reference "b")) (invocationCallee (reference "Pair")))))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair"))) (kind kerml-function) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair::left"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing") (direction in)))))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair::result"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair::right"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing") (direction in)))))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder::b"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder::a")))))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 1))
      (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder::b")))))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0))
      (authored-target "Pair")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair")))))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder::a")))))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 1))
      (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder::b")))))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0))
      (authored-target "Pair")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair")))))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair::left"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair::result"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair::right"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder::a"))) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder::b"))) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder::b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder::b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair::left"))) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair::left"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair::result"))) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair::result"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair::right"))) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair::right"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder::a"))) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder::b"))) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder::bad"))) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder::bad"))) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder::good"))) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder::good"))) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair::left"))) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair::result"))) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair::right"))) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
    (invocation (declaration (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (callee (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair"))) (supplied 2) (required 0) (start 15 22) (end 15 46))
    (invocation (declaration (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (callee (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair"))) (supplied 2) (required 0) (start 12 23) (end 12 48))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder::a")))
      (featured-by (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder")))
      (type (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder::b")))
      (featured-by (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder")))
      (type (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder::bad")))
      (featured-by (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder")))
      (supertype (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (type (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair")) (provenance implied))
      (effective-type (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder::bad")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder::good")))
      (featured-by (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder")))
      (supertype (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (type (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair")) (provenance implied))
      (effective-type (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder::good")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair")))
      (subtype (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0)))) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0)))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair::left")))
      (featured-by (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair")))
      (type (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair::result")))
      (featured-by (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair")))
      (type (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair::right")))
      (featured-by (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair")))
      (type (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Thing")))
      (subtype (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder::a")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder::b")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair::left")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair::result")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair::right")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (range (start 8 20) (end 8 25)) (probe (position 8 20))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder::a"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (range (start 9 20) (end 9 25)) (probe (position 9 20))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder::b"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (range (start 15 34) (end 15 35)) (probe (position 15 34))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder::a")))))
    )
  )
  (query (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (range (start 15 44) (end 15 45)) (probe (position 15 44))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 1) (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder::b")))))
    )
  )
  (query (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (range (start 15 22) (end 15 26)) (probe (position 15 22))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0) (authored-target "Pair")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair")))))
    )
  )
  (query (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (range (start 12 35) (end 12 36)) (probe (position 12 35))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder::a")))))
    )
  )
  (query (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (range (start 12 46) (end 12 47)) (probe (position 12 46))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 1) (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Holder::b")))))
    )
  )
  (query (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (range (start 12 23) (end 12 27)) (probe (position 12 23))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0) (authored-target "Pair")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair")))))
    )
  )
  (query (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (range (start 3 26) (end 3 31)) (probe (position 3 26))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair::left"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (range (start 5 32) (end 5 37)) (probe (position 5 32))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair::result"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (range (start 4 27) (end 4 32)) (probe (position 4 27))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Pair::right"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_no_duplicate_parameter_redefinition.md") (qualified-name "Expressions::Thing")))))
    )
  )
)
~~~
