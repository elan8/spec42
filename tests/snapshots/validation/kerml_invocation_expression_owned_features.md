# META
~~~ini
description=KerML 8.3.4.8.8 validateInvocationExpressionOwnedFeatures requires every ownedFeature of an InvocationExpression other than its result to have direction in
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.8.8 validateInvocationExpressionOwnedFeatures
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.4.8.8:validateInvocationExpressionOwnedFeatures
type=file
~~~
# SOURCE
~~~kerml
// Conforming: every argument feature an invocation owns is an input parameter, which the
// concrete syntax gives direction in.
//
// The violating side has no textual counterpart: KerML argument syntax authors only input
// parameter redefinitions inside an invocation, so a source document cannot give an invocation
// an owned feature with an out or inout direction.
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
        feature invoked = Pair(left = a, right = b);
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_invocation_expression_owned_features.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_invocation_expression_owned_features.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:cb20a2442ecb6853d2bbc57d08c0609f9fa43bed083f2bac3cfe403af7480297") (contract-version "semantic-metadata-projection-v6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Holder::a"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Holder::b"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Holder::invoked"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "invoked")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "invoked")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "invoked")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "invoked")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "a")) (expressionOperand (reference "b")) (invocationCallee (reference "Pair")))))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "invoked")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Pair"))) (kind kerml-function) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Pair::left"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing") (direction in)))))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Pair::result"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Pair::right"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing") (direction in)))))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Holder::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Holder::b"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "invoked")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Holder::a")))))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "invoked")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 1))
      (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Holder::b")))))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "invoked")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0))
      (authored-target "Pair")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Pair")))))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Pair::left"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Pair::result"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Pair::right"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Holder::a"))) (target (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Holder::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Holder::b"))) (target (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Holder::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "invoked")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Holder::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "invoked")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "invoked")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Holder::b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "invoked")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "invoked")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Pair"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "invoked")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Pair::left"))) (target (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Pair::left"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Pair::result"))) (target (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Pair::result"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Pair::right"))) (target (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Pair::right"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Holder::a"))) (target (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Holder::b"))) (target (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Holder::invoked"))) (target (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Holder"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Holder::invoked"))) (target (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "invoked")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "invoked")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "invoked")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Pair::left"))) (target (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Pair"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Pair::result"))) (target (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Pair"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Pair::right"))) (target (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Pair"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "invoked")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
    (invocation (declaration (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "invoked")) (anonymous (kind kerml-expression) (ordinal 0))))) (callee (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Pair"))) (supplied 2) (required 0) (start 16 26) (end 16 51))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Holder::a")))
      (featured-by (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Holder")))
      (type (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Holder::b")))
      (featured-by (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Holder")))
      (type (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Holder::invoked")))
      (featured-by (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Holder")))
      (supertype (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "invoked")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "invoked")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "invoked")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Holder::invoked")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Pair::left")))
      (featured-by (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Pair")))
      (type (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Pair::result")))
      (featured-by (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Pair")))
      (type (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Pair::right")))
      (featured-by (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Pair")))
      (type (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Thing")))
      (subtype (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Holder::a")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Holder::b")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Pair::left")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Pair::result")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Pair::right")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (range (start 14 20) (end 14 25)) (probe (position 14 20))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Holder::a"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (range (start 15 20) (end 15 25)) (probe (position 15 20))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Holder::b"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (range (start 16 38) (end 16 39)) (probe (position 16 38))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "invoked")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Holder::a")))))
    )
  )
  (query (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (range (start 16 49) (end 16 50)) (probe (position 16 49))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "invoked")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 1) (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Holder::b")))))
    )
  )
  (query (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (range (start 16 26) (end 16 30)) (probe (position 16 26))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "invoked")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0) (authored-target "Pair")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Pair")))))
    )
  )
  (query (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (range (start 9 26) (end 9 31)) (probe (position 9 26))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Pair::left"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (range (start 11 32) (end 11 37)) (probe (position 11 32))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Pair::result"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (range (start 10 27) (end 10 32)) (probe (position 10 27))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Pair::right"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_owned_features.md") (qualified-name "Expressions::Thing")))))
    )
  )
)
~~~
