# META
~~~ini
description=KerML 8.3.4.8.5 validateFeatureReferenceExpressionReferentIsFeature requires the first non-ParameterMembership ownedMembership of a FeatureReferenceExpression to have a Feature as its memberElement
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.8.5 validateFeatureReferenceExpressionReferentIsFeature
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.4.8.5:validateFeatureReferenceExpressionReferentIsFeature
blocked_by=semantic-feature-reference-referent-not-feature
type=file
~~~
# SOURCE
~~~kerml
package Expressions {
    classifier Thing;
    classifier Holder {
        feature referent : Thing;

        // Conforming: the referenced element is a feature.
        feature good = referent;

        // Invalid: the referenced element is a classifier, not a feature.
        feature bad = Thing;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "feature_reference_referent_not_feature")
        (source "semantic")
        (range (start 9 8) (end 9 28))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:1b19a6b1586cf8ae6dd7995497c2d289ba4456d25e363dba6fd35eeacc83a753") (contract-version "operator-expression-arguments-v7"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (qualified-name "Expressions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (qualified-name "Expressions::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (qualified-name "Expressions::Holder::bad"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (qualified-name "Expressions::Holder::good"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "referent")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (qualified-name "Expressions::Holder::referent"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (qualified-name "Expressions::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (qualified-name "Expressions::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "referent")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (qualified-name "Expressions::Holder::referent")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (qualified-name "Expressions::Holder::referent"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (qualified-name "Expressions::Thing")))))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (qualified-name "Expressions::Holder::referent"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (qualified-name "Expressions::Holder::referent"))) (target (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (qualified-name "Expressions::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (qualified-name "Expressions::Holder::referent"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (qualified-name "Expressions::Holder::bad"))) (target (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (qualified-name "Expressions::Holder"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (qualified-name "Expressions::Holder::bad"))) (target (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (qualified-name "Expressions::Holder::good"))) (target (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (qualified-name "Expressions::Holder"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (qualified-name "Expressions::Holder::good"))) (target (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (qualified-name "Expressions::Holder::referent"))) (target (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (qualified-name "Expressions::Holder"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (qualified-name "Expressions::Holder::bad")))
      (featured-by (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (qualified-name "Expressions::Holder")))
      (supertype (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (qualified-name "Expressions::Holder::bad")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (qualified-name "Expressions::Holder::good")))
      (featured-by (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (qualified-name "Expressions::Holder")))
      (supertype (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (qualified-name "Expressions::Holder::good")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (qualified-name "Expressions::Holder::referent")))
      (featured-by (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (qualified-name "Expressions::Holder")))
      (type (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (qualified-name "Expressions::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (qualified-name "Expressions::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (qualified-name "Expressions::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (qualified-name "Expressions::Thing")))
      (subtype (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (qualified-name "Expressions::Holder::referent")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (range (start 9 22) (end 9 27)) (probe (position 9 22))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (qualified-name "Expressions::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (range (start 6 23) (end 6 31)) (probe (position 6 23))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (path (named (kind package) (name "Expressions")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "referent")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (qualified-name "Expressions::Holder::referent")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (range (start 3 27) (end 3 32)) (probe (position 3 27))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (qualified-name "Expressions::Holder::referent"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_reference_expression_referent_is_feature.md") (qualified-name "Expressions::Thing")))))
    )
  )
)
~~~
