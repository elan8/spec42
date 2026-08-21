# META
~~~ini
description=KerML 8.3.4.8.8 validateInvocationExpressionInstantiatedType requires the instantiatedType of an InvocationExpression to be a Behavior, or a Feature with a single Behavior type
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.8.8 validateInvocationExpressionInstantiatedType
type=file
skip_validation=no semantic rule checks the metaclass family of an invocation expression instantiated type; the canonical code invocation_instantiated_type_not_behavior does not exist yet
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

        // Conforming: the instantiated type is a function, which is a behavior.
        feature copied = Identity(input = source);

        // Invalid: a classifier is not a behavior.
        feature wrong = Thing(input = source);
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "invocation_instantiated_type_not_behavior")
        (source "semantic")
        (range (start 13 8) (end 13 46))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:d988487e46949b88cbfc33f95a4e8381c2e11698c691da1191206dfead322eb7") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::copied"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "source")) (invocationCallee (reference "Identity")))))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::wrong"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "source")) (invocationCallee (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Identity"))) (kind kerml-function) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Identity::input"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing") (direction in)))))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Identity::result"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::copied"))) (kind expressionOperand) (ordinal 0))
      (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::source")))))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::copied"))) (kind invocationCallee) (ordinal 0))
      (authored-target "Identity")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Identity")))))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::wrong"))) (kind expressionOperand) (ordinal 0))
      (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::source")))))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::wrong"))) (kind invocationCallee) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Identity::input"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Identity::result"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Thing")))))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::copied"))) (target (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::source"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::copied"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::copied"))) (target (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Identity"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::copied"))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::source"))) (target (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::wrong"))) (target (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::source"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::wrong"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::wrong"))) (target (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::wrong"))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Identity::input"))) (target (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Identity::input"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Identity::result"))) (target (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Identity::result"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::copied"))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::wrong"))) (state non-constant))
    (invocation (declaration (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::copied"))) (callee (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Identity"))) (supplied 1) (required 0) (start 10 25) (end 10 49))
    (invocation (declaration (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::wrong"))) (callee (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Thing"))) (supplied 1) (required 0) (start 13 24) (end 13 45))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::copied")))
      (featured-by (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Holder")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::source")))
      (featured-by (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Holder")))
      (type (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::wrong")))
      (featured-by (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Holder")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Identity::input")))
      (featured-by (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Identity")))
      (type (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Identity::result")))
      (featured-by (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Identity")))
      (type (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Thing")))
      (subtype (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::source")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Identity::input")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Identity::result")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (range (start 10 42) (end 10 48)) (probe (position 10 42))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::copied"))) (kind expressionOperand) (ordinal 0) (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::source")))))
    )
  )
  (query (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (range (start 10 25) (end 10 33)) (probe (position 10 25))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::copied"))) (kind invocationCallee) (ordinal 0) (authored-target "Identity")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Identity")))))
    )
  )
  (query (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (range (start 7 25) (end 7 30)) (probe (position 7 25))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::source"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (range (start 13 38) (end 13 44)) (probe (position 13 38))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::wrong"))) (kind expressionOperand) (ordinal 0) (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::source")))))
    )
  )
  (query (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (range (start 13 24) (end 13 29)) (probe (position 13 24))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Holder::wrong"))) (kind invocationCallee) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (range (start 3 27) (end 3 32)) (probe (position 3 27))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Identity::input"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (range (start 4 32) (end 4 37)) (probe (position 4 32))
    (reference (id (source (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Identity::result"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_invocation_expression_instantiated_type.md") (qualified-name "Expressions::Thing")))))
    )
  )
)
~~~
