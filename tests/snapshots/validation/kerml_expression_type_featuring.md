# META
~~~ini
description=KerML 8.3.4.7.3 checkExpressionTypeFeaturing requires an Expression owned by a FeatureValue to share the featureWithValue featuringTypes
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.4.7.3:checkExpressionTypeFeaturing
type=file
~~~
# SOURCE
~~~kerml
package Expressions {
    classifier Thing;
    classifier Holder {
        feature referent : Thing;
        // The value Expression is featured by `Holder`, like the featureWithValue `value`.
        feature value = referent;
    }
    // A featureWithValue with no featuring type gives its value Expression none either.
    feature unfeatured = 1;
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship
    (kind type_featuring)
    (source (anonymous (owner "Expressions::Holder::value") (kind Expression) (ordinal 0)))
    (target "Expressions::Holder")
    (provenance implied)
    (outcome resolved))
  (relationship
    (kind type_featuring)
    (source (anonymous (owner "Expressions::unfeatured") (kind LiteralInteger) (ordinal 0)))
    (provenance implied)
    (outcome absent)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_expression_type_featuring.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:249525af0a1d106896948d7aa01954643ae6f02ebe21e0adb01c23ea4b219335"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder::referent"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder::value"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/kerml_expression_type_featuring.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "value")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_expression_type_featuring.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "value")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_expression_type_featuring.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "value")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_expression_type_featuring.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "value")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "referent")))))
    (declaration (id (node (document "memory://snapshot/kerml_expression_type_featuring.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "value")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::unfeatured"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/kerml_expression_type_featuring.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-feature) (name "unfeatured")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_expression_type_featuring.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-feature) (name "unfeatured")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_expression_type_featuring.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-feature) (name "unfeatured")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_expression_type_featuring.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-feature) (name "unfeatured")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_expression_type_featuring.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-feature) (name "unfeatured")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder::referent"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_expression_type_featuring.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "value")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "referent")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder::referent")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder::referent"))) (target (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder::referent"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_expression_type_featuring.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "value")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder::referent"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_expression_type_featuring.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "value")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder::referent"))) (target (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder::value"))) (target (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder::value"))) (target (node (document "memory://snapshot/kerml_expression_type_featuring.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "value")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_expression_type_featuring.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "value")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_expression_type_featuring.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "value")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder::referent"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_expression_type_featuring.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "value")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_expression_type_featuring.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "value")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::unfeatured"))) (target (node (document "memory://snapshot/kerml_expression_type_featuring.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-feature) (name "unfeatured")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_expression_type_featuring.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-feature) (name "unfeatured")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_expression_type_featuring.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-feature) (name "unfeatured")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_expression_type_featuring.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "value")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/kerml_expression_type_featuring.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-feature) (name "unfeatured")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (state literal) (value (kind integer) (integer 1)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder::referent")))
      (featured-by (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder")))
      (type (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Thing")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_expression_type_featuring.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "value")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder::value")))
      (featured-by (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder")))
      (effective-type (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder::referent"))))
      (supertype (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder::referent")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_expression_type_featuring.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "value")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_expression_type_featuring.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "value")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder")))
    )
    (declaration (id (node (document "memory://snapshot/kerml_expression_type_featuring.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "value")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_expression_type_featuring.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "value")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Thing")) (source inherited) (from (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder::referent"))))
      (supertype (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder::referent")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Thing")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder::value")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Thing")))
      (subtype (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder::referent")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::unfeatured")))
      (supertype (node (document "memory://snapshot/kerml_expression_type_featuring.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-feature) (name "unfeatured")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_expression_type_featuring.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-feature) (name "unfeatured")) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_expression_type_featuring.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-feature) (name "unfeatured")) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::unfeatured")) (scopes any feature))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/kerml_expression_type_featuring.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "value")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (feature-reference "referent" (target (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder::referent")))))
  (declaration (id (node (document "memory://snapshot/kerml_expression_type_featuring.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-feature) (name "unfeatured")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (outcome resolved) (literal (value (kind integer) (integer 1))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_expression_type_featuring.md") (range (start 3 27) (end 3 32)) (probe (position 3 27))
    (reference (id (source (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder::referent"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_expression_type_featuring.md") (range (start 5 24) (end 5 32)) (probe (position 5 24))
    (reference (id (source (node (document "memory://snapshot/kerml_expression_type_featuring.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "value")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "referent")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_expression_type_featuring.md") (qualified-name "Expressions::Holder::referent")))))
    )
  )
)
~~~
