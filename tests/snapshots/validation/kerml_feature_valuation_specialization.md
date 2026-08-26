# META
~~~ini
description=KerML checkFeatureValuationSpecialization desired semantics
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.3.3.4:checkFeatureValuationSpecialization
type=file
~~~
# SOURCE
~~~kerml
package Model {
    classifier Thing;
    feature source : Thing;

    // An ordinary, undirected value with no authored specialization subsets the result of its
    // owned value Expression. Each following feature covers one condition that suppresses it.
    feature inferred = source;
    feature typed : Thing = source;
    out feature directed = source;
    feature defaulted default = source;
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics (specialization-check (rule_id "kerml-1.0:8.3.3.3.4:checkFeatureValuationSpecialization") (outcome satisfied)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_feature_valuation_specialization.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:23d48e1c4ee9c6d6be705d705bb9016303e97cbae603a03bb4f363560ba656f1") (contract-version "constructor-expression-result-v8"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::defaulted"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "defaulted")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "defaulted")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (default true)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "defaulted")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "defaulted")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "source")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "defaulted")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::directed"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)) (feature-value (kind bind) (value (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "directed")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "directed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "directed")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "directed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "source")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "directed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::inferred"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "inferred")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "inferred")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "inferred")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "inferred")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "source")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "inferred")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::source"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::typed"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "typed")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "typed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "typed")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "typed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "source")))))
    (declaration (id (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "typed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "defaulted")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::source")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "directed")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::source")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "inferred")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::source")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::typed"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "typed")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::source")))))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "defaulted")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::source"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "defaulted")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "directed")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::source"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "directed")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "inferred")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::source"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "inferred")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::source"))) (target (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::typed"))) (target (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::typed"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "typed")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::source"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "typed")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "defaulted")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "defaulted")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "directed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "directed")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::inferred"))) (target (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "inferred")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "inferred")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "inferred")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "typed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "typed")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "defaulted")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "directed")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "inferred")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "typed")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::Thing")))
      (subtype (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::source")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::typed")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "defaulted")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "defaulted")) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "directed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "directed")) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::inferred")))
      (supertype (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "inferred")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "inferred")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "inferred")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::inferred")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::source")))
      (type (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::typed")))
      (type (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "typed")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "typed")) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_feature_valuation_specialization.md") (range (start 9 32) (end 9 38)) (probe (position 9 32))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "defaulted")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::source")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_valuation_specialization.md") (range (start 8 27) (end 8 33)) (probe (position 8 27))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "directed")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::source")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_valuation_specialization.md") (range (start 6 23) (end 6 29)) (probe (position 6 23))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "inferred")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::source")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_valuation_specialization.md") (range (start 2 21) (end 2 26)) (probe (position 2 21))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::source"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_valuation_specialization.md") (range (start 7 20) (end 7 25)) (probe (position 7 20))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::typed"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_feature_valuation_specialization.md") (range (start 7 28) (end 7 34)) (probe (position 7 28))
    (reference (id (source (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (path (named (kind package) (name "Model")) (named (kind kerml-feature) (name "typed")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "source")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_feature_valuation_specialization.md") (qualified-name "Model::source")))))
    )
  )
)
~~~
