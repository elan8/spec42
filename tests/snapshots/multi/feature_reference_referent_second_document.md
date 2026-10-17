# META
~~~ini
description=Per-document lowering relocates every parser reference into its own document, so a feature-reference diagnostic in a later document keeps that document's range
type=multi
~~~
# SOURCE
## alpha.kerml
~~~kerml
package Alpha;
~~~
## beta.kerml
~~~kerml
package Beta {
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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/alpha.kerml"
    (diagnostics
    )
  )
  (document "memory://snapshot/beta.kerml"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "feature_reference_referent_not_feature")
        (source "semantic")
        (range (start 9 22) (end 9 27))
        (related-information
          (related
            (uri "memory://snapshot/beta.kerml")
            (range (start 1 4) (end 1 21))
          )
        )
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:dda81a7b81224a4c3ca7c9d5154d9bfa92e7521f2087276e3b724b8e8cd01f12"))
  (declarations
    (declaration (id (node (document "memory://snapshot/alpha.kerml") (qualified-name "Alpha"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Holder::bad"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/beta.kerml") (path (named (kind package) (name "Beta")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/beta.kerml") (path (named (kind package) (name "Beta")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/beta.kerml") (path (named (kind package) (name "Beta")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/beta.kerml") (path (named (kind package) (name "Beta")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/beta.kerml") (path (named (kind package) (name "Beta")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Holder::good"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/beta.kerml") (path (named (kind package) (name "Beta")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/beta.kerml") (path (named (kind package) (name "Beta")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/beta.kerml") (path (named (kind package) (name "Beta")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/beta.kerml") (path (named (kind package) (name "Beta")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "referent")))))
    (declaration (id (node (document "memory://snapshot/beta.kerml") (path (named (kind package) (name "Beta")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Holder::referent"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/beta.kerml") (path (named (kind package) (name "Beta")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Thing")))))
    (reference (id (source (node (document "memory://snapshot/beta.kerml") (path (named (kind package) (name "Beta")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "referent")
      (outcome (status resolved) (target (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Holder::referent")))))
    (reference (id (source (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Holder::referent"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Thing")))))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/beta.kerml") (path (named (kind package) (name "Beta")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/beta.kerml") (path (named (kind package) (name "Beta")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/beta.kerml") (path (named (kind package) (name "Beta")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Holder::referent"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/beta.kerml") (path (named (kind package) (name "Beta")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Holder::referent"))) (target (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Holder::referent"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Holder::bad"))) (target (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Holder"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Holder::bad"))) (target (node (document "memory://snapshot/beta.kerml") (path (named (kind package) (name "Beta")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/beta.kerml") (path (named (kind package) (name "Beta")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/beta.kerml") (path (named (kind package) (name "Beta")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/beta.kerml") (path (named (kind package) (name "Beta")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Holder::good"))) (target (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Holder"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Holder::good"))) (target (node (document "memory://snapshot/beta.kerml") (path (named (kind package) (name "Beta")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/beta.kerml") (path (named (kind package) (name "Beta")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/beta.kerml") (path (named (kind package) (name "Beta")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/beta.kerml") (path (named (kind package) (name "Beta")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/beta.kerml") (path (named (kind package) (name "Beta")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Holder::referent"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Holder::referent"))) (target (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Holder"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/beta.kerml") (path (named (kind package) (name "Beta")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/beta.kerml") (path (named (kind package) (name "Beta")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Holder::bad")))
      (featured-by (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Holder")))
      (supertype (node (document "memory://snapshot/beta.kerml") (path (named (kind package) (name "Beta")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/beta.kerml") (path (named (kind package) (name "Beta")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Holder")))
    )
    (declaration (id (node (document "memory://snapshot/beta.kerml") (path (named (kind package) (name "Beta")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/beta.kerml") (path (named (kind package) (name "Beta")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Holder::bad")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Holder::good")))
      (featured-by (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Holder")))
      (effective-type (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Thing")) (source inherited) (from (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Holder::referent"))))
      (supertype (node (document "memory://snapshot/beta.kerml") (path (named (kind package) (name "Beta")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Holder::referent")) (scopes any feature))
      (supertype (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/beta.kerml") (path (named (kind package) (name "Beta")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Holder")))
    )
    (declaration (id (node (document "memory://snapshot/beta.kerml") (path (named (kind package) (name "Beta")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/beta.kerml") (path (named (kind package) (name "Beta")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Thing")) (source inherited) (from (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Holder::referent"))))
      (supertype (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Holder::referent")) (scopes any feature))
      (supertype (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Thing")) (scopes any))
      (subtype (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Holder::good")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Holder::referent")))
      (featured-by (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Holder")))
      (type (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Thing")) (scopes any))
      (subtype (node (document "memory://snapshot/beta.kerml") (path (named (kind package) (name "Beta")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Thing")))
      (subtype (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Holder::referent")) (scopes any))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/beta.kerml") (path (named (kind package) (name "Beta")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (feature-reference "Thing" (target (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Thing")))))
  (declaration (id (node (document "memory://snapshot/beta.kerml") (path (named (kind package) (name "Beta")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (feature-reference "referent" (target (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Holder::referent")))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/beta.kerml") (range (start 9 22) (end 9 27)) (probe (position 9 22))
    (reference (id (source (node (document "memory://snapshot/beta.kerml") (path (named (kind package) (name "Beta")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "bad")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Thing")))))
    )
  )
  (query (document "memory://snapshot/beta.kerml") (range (start 6 23) (end 6 31)) (probe (position 6 23))
    (reference (id (source (node (document "memory://snapshot/beta.kerml") (path (named (kind package) (name "Beta")) (named (kind kerml-classifier) (name "Holder")) (named (kind kerml-feature) (name "good")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "referent")
      (outcome (status resolved) (target (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Holder::referent")))))
    )
  )
  (query (document "memory://snapshot/beta.kerml") (range (start 3 27) (end 3 32)) (probe (position 3 27))
    (reference (id (source (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Holder::referent"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/beta.kerml") (qualified-name "Beta::Thing")))))
    )
  )
)
~~~
