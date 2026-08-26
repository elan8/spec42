# META
~~~ini
description=KerML 8.3.4.10.2 validateFeatureValueIsInitial requires the featureWithValue of a FeatureValue with isInitial = true to have isVariable = true
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.4.10.2 validateFeatureValueIsInitial
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.4.10.2:validateFeatureValueIsInitial
blocked_by=semantic-initial-value-feature-not-variable
type=file
~~~
# SOURCE
~~~kerml
package Values {
    classifier Thing;
    class Happening {
        feature source : Thing;

        // Conforming: an initial value on a variable feature.
        var feature tracked : Thing := source;

        // Invalid: an initial value on a feature that is not variable.
        feature fixed : Thing := source;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_value_is_initial.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "initial_value_feature_not_variable")
        (source "semantic")
        (range (start 9 8) (end 9 40))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_value_is_initial.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:1b334534dbc7b63cb757ddf50e194529d2cf2061612c490d8e15d34e811e2ef5") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Happening"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Happening::fixed"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind assign) (value (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (path (named (kind package) (name "Values")) (named (kind class-def) (name "Happening")) (named (kind kerml-feature) (name "fixed")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (path (named (kind package) (name "Values")) (named (kind class-def) (name "Happening")) (named (kind kerml-feature) (name "fixed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (path (named (kind package) (name "Values")) (named (kind class-def) (name "Happening")) (named (kind kerml-feature) (name "fixed")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (path (named (kind package) (name "Values")) (named (kind class-def) (name "Happening")) (named (kind kerml-feature) (name "fixed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "source")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (path (named (kind package) (name "Values")) (named (kind class-def) (name "Happening")) (named (kind kerml-feature) (name "fixed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Happening::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Happening::tracked"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers var)) (feature-value (kind assign) (value (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (path (named (kind package) (name "Values")) (named (kind class-def) (name "Happening")) (named (kind kerml-feature) (name "tracked")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (path (named (kind package) (name "Values")) (named (kind class-def) (name "Happening")) (named (kind kerml-feature) (name "tracked")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (path (named (kind package) (name "Values")) (named (kind class-def) (name "Happening")) (named (kind kerml-feature) (name "tracked")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (path (named (kind package) (name "Values")) (named (kind class-def) (name "Happening")) (named (kind kerml-feature) (name "tracked")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "source")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (path (named (kind package) (name "Values")) (named (kind class-def) (name "Happening")) (named (kind kerml-feature) (name "tracked")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Happening::fixed"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (path (named (kind package) (name "Values")) (named (kind class-def) (name "Happening")) (named (kind kerml-feature) (name "fixed")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Happening::source")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Happening::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Happening::tracked"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (path (named (kind package) (name "Values")) (named (kind class-def) (name "Happening")) (named (kind kerml-feature) (name "tracked")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Happening::source")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Happening::fixed"))) (target (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Happening::fixed"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (path (named (kind package) (name "Values")) (named (kind class-def) (name "Happening")) (named (kind kerml-feature) (name "fixed")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Happening::source"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (path (named (kind package) (name "Values")) (named (kind class-def) (name "Happening")) (named (kind kerml-feature) (name "fixed")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Happening::source"))) (target (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Happening::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Happening::tracked"))) (target (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Happening::tracked"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (path (named (kind package) (name "Values")) (named (kind class-def) (name "Happening")) (named (kind kerml-feature) (name "tracked")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Happening::source"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (path (named (kind package) (name "Values")) (named (kind class-def) (name "Happening")) (named (kind kerml-feature) (name "tracked")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Happening::fixed"))) (target (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Happening"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (path (named (kind package) (name "Values")) (named (kind class-def) (name "Happening")) (named (kind kerml-feature) (name "fixed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (path (named (kind package) (name "Values")) (named (kind class-def) (name "Happening")) (named (kind kerml-feature) (name "fixed")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Happening::source"))) (target (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Happening"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (path (named (kind package) (name "Values")) (named (kind class-def) (name "Happening")) (named (kind kerml-feature) (name "tracked")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (path (named (kind package) (name "Values")) (named (kind class-def) (name "Happening")) (named (kind kerml-feature) (name "tracked")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (path (named (kind package) (name "Values")) (named (kind class-def) (name "Happening")) (named (kind kerml-feature) (name "fixed")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (path (named (kind package) (name "Values")) (named (kind class-def) (name "Happening")) (named (kind kerml-feature) (name "tracked")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Happening::fixed")))
      (featured-by (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Happening")))
      (type (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (path (named (kind package) (name "Values")) (named (kind class-def) (name "Happening")) (named (kind kerml-feature) (name "fixed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (path (named (kind package) (name "Values")) (named (kind class-def) (name "Happening")) (named (kind kerml-feature) (name "fixed")) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Happening::source")))
      (featured-by (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Happening")))
      (type (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Happening::tracked")))
      (type (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (path (named (kind package) (name "Values")) (named (kind class-def) (name "Happening")) (named (kind kerml-feature) (name "tracked")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (path (named (kind package) (name "Values")) (named (kind class-def) (name "Happening")) (named (kind kerml-feature) (name "tracked")) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Thing")))
      (subtype (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Happening::fixed")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Happening::source")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Happening::tracked")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_value_is_initial.md") (range (start 9 24) (end 9 29)) (probe (position 9 24))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Happening::fixed"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_value_is_initial.md") (range (start 9 33) (end 9 39)) (probe (position 9 33))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (path (named (kind package) (name "Values")) (named (kind class-def) (name "Happening")) (named (kind kerml-feature) (name "fixed")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Happening::source")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_value_is_initial.md") (range (start 3 25) (end 3 30)) (probe (position 3 25))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Happening::source"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_value_is_initial.md") (range (start 6 30) (end 6 35)) (probe (position 6 30))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Happening::tracked"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_value_is_initial.md") (range (start 6 39) (end 6 45)) (probe (position 6 39))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (path (named (kind package) (name "Values")) (named (kind class-def) (name "Happening")) (named (kind kerml-feature) (name "tracked")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_value_is_initial.md") (qualified-name "Values::Happening::source")))))
    )
  )
)
~~~
