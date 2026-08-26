# META
~~~ini
description=KerML 8.3.4.8.3 validateConstructorExpressionNoDuplicateFeatureRedefinition forbids two different ownedFeatures of a ConstructorExpression result from redefining the same feature of the instantiatedType
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.8.3 validateConstructorExpressionNoDuplicateFeatureRedefinition
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.4.8.3:validateConstructorExpressionNoDuplicateFeatureRedefinition
blocked_by=semantic-constructor-duplicate-feature-redefinition
type=file
~~~
# SOURCE
~~~kerml
package Expressions {
    classifier Thing;
    struct Point {
        feature x : Thing;
        feature y : Thing;
    }
    classifier Holder {
        feature a : Thing;
        feature b : Thing;

        // Conforming: each initialiser redefines a different feature of Point.
        feature good = Point(x = a, y = b);

        // Invalid: two initialisers redefine the same feature of Point.
        feature bad = Point(x = a, x = b);
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "constructor_duplicate_feature_redefinition")
        (source "semantic")
        (range (start 14 8) (end 14 42))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:ff33f838003c423c212b632be3937a838893b66585deb73efeb8fc9ecc04277a") (contract-version "constructor-expression-specialization-v9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder::a"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder::b"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder::bad"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "a")) (expressionOperand (reference "b")) (invocationCallee (reference "Point")))))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder::good"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "a")) (expressionOperand (reference "b")) (invocationCallee (reference "Point")))))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Point"))) (kind kerml-structure) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Point::x"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Point::y"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder::b"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder::a")))))
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 1))
      (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder::b")))))
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0))
      (authored-target "Point")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Point")))))
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder::a")))))
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 1))
      (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder::b")))))
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0))
      (authored-target "Point")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Point")))))
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Point::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Point::y"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder::a"))) (target (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder::a"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder::b"))) (target (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder::b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder::b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Point"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder::b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Point"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Point::x"))) (target (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Point::x"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Point::y"))) (target (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Point::y"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder::a"))) (target (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder::b"))) (target (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder::bad"))) (target (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder::bad"))) (target (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder::good"))) (target (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder::good"))) (target (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Point::x"))) (target (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Point"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Point::y"))) (target (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Point"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
    (invocation (declaration (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (callee (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Point"))) (supplied 2) (required 0) (start 14 22) (end 14 41))
    (invocation (declaration (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (callee (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Point"))) (supplied 2) (required 0) (start 11 23) (end 11 42))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder::a")))
      (featured-by (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder")))
      (type (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder::b")))
      (featured-by (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder")))
      (type (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder::bad")))
      (featured-by (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder")))
      (supertype (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder::bad")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder::good")))
      (featured-by (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder")))
      (supertype (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder::good")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Point::x")))
      (featured-by (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Point")))
      (type (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Point::y")))
      (featured-by (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Point")))
      (type (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Thing")))
      (subtype (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder::a")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder::b")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Point::x")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Point::y")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (range (start 7 20) (end 7 25)) (probe (position 7 20))
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder::a"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (range (start 8 20) (end 8 25)) (probe (position 8 20))
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder::b"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (range (start 14 32) (end 14 33)) (probe (position 14 32))
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder::a")))))
    )
  )
  (query (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (range (start 14 39) (end 14 40)) (probe (position 14 39))
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 1) (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder::b")))))
    )
  )
  (query (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (range (start 14 22) (end 14 27)) (probe (position 14 22))
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0) (authored-target "Point")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Point")))))
    )
  )
  (query (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (range (start 11 33) (end 11 34)) (probe (position 11 33))
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder::a")))))
    )
  )
  (query (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (range (start 11 40) (end 11 41)) (probe (position 11 40))
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 1) (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Holder::b")))))
    )
  )
  (query (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (range (start 11 23) (end 11 28)) (probe (position 11 23))
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0) (authored-target "Point")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Point")))))
    )
  )
  (query (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (range (start 3 20) (end 3 25)) (probe (position 3 20))
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Point::x"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (range (start 4 20) (end 4 25)) (probe (position 4 20))
    (reference (id (source (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Point::y"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_constructor_expression_no_duplicate_feature_redefinition.md") (qualified-name "Expressions::Thing")))))
    )
  )
)
~~~
