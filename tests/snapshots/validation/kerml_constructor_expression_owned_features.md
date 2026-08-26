# META
~~~ini
description=KerML 8.3.4.8.3 validateConstructorExpressionOwnedFeatures forbids a ConstructorExpression from owning any Feature other than its result
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.8.3 validateConstructorExpressionOwnedFeatures
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.4.8.3:validateConstructorExpressionOwnedFeatures
type=file
~~~
# SOURCE
~~~kerml
// Conforming: the constructor expression below owns only its result parameter; the named
// initialisers are owned features of that result, not of the constructor.
//
// The violating side has no textual counterpart: KerML constructor syntax authors every named
// initialiser inside the constructed result, so a source document cannot give the constructor
// expression itself an owned feature besides its result.
package Expressions {
    classifier Thing;
    struct Point {
        feature x : Thing;
        feature y : Thing;
    }
    classifier Holder {
        feature a : Thing;
        feature b : Thing;
        feature origin = Point(x = a, y = b);
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_constructor_expression_owned_features.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_constructor_expression_owned_features.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:a61f73221a4252c32c882693419fd01f5e831f0b1a0830967d7ff19570b81f2e") (contract-version "owned-cross-feature-typing-v4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder::a"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder::b"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder::origin"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "a")) (expressionOperand (reference "b")) (invocationCallee (reference "Point")))))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Point"))) (kind kerml-structure) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Point::x"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Point::y"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder::b"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder::origin"))) (kind expressionOperand) (ordinal 0))
      (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder::a")))))
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder::origin"))) (kind expressionOperand) (ordinal 1))
      (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder::b")))))
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder::origin"))) (kind invocationCallee) (ordinal 0))
      (authored-target "Point")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Point")))))
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Point::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Point::y"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder::a"))) (target (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder::b"))) (target (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder::origin"))) (target (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder::origin"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder::origin"))) (target (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder::b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder::origin"))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder::origin"))) (target (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Point"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder::origin"))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Point::x"))) (target (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Point::x"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Point::y"))) (target (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Point::y"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder::a"))) (target (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder::b"))) (target (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder::origin"))) (target (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Point::x"))) (target (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Point"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Point::y"))) (target (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Point"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder::origin"))) (state non-constant))
    (invocation (declaration (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder::origin"))) (callee (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Point"))) (supplied 2) (required 0) (start 15 25) (end 15 44))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder::a")))
      (featured-by (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder")))
      (type (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder::b")))
      (featured-by (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder")))
      (type (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder::origin")))
      (featured-by (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Point::x")))
      (featured-by (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Point")))
      (type (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Point::y")))
      (featured-by (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Point")))
      (type (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Thing")))
      (subtype (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder::a")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder::b")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Point::x")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Point::y")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (range (start 13 20) (end 13 25)) (probe (position 13 20))
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder::a"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (range (start 14 20) (end 14 25)) (probe (position 14 20))
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder::b"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (range (start 15 35) (end 15 36)) (probe (position 15 35))
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder::origin"))) (kind expressionOperand) (ordinal 0) (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder::a")))))
    )
  )
  (query (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (range (start 15 42) (end 15 43)) (probe (position 15 42))
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder::origin"))) (kind expressionOperand) (ordinal 1) (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder::b")))))
    )
  )
  (query (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (range (start 15 25) (end 15 30)) (probe (position 15 25))
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Holder::origin"))) (kind invocationCallee) (ordinal 0) (authored-target "Point")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Point")))))
    )
  )
  (query (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (range (start 9 20) (end 9 25)) (probe (position 9 20))
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Point::x"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (range (start 10 20) (end 10 25)) (probe (position 10 20))
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Point::y"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_owned_features.md") (qualified-name "Expressions::Thing")))))
    )
  )
)
~~~
