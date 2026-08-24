# META
~~~ini
description=Coverage: Expression literals, postfix operators, conditionals, sequences
type=file
~~~
# SOURCE
~~~kerml
package ExpressionCoverage {
    classifier Vehicle;
    classifier Item;

    feature s = "hello";
    feature r = 3.14;
    feature r2 = .5;
    feature n = null;
    feature inf = *;

    feature items : Item[*];
    feature arr = items[0];
    feature h = items#(0);
    feature all_v = all Vehicle;

    feature coll = items.{in i; i};
    feature sel = items.?{in i; i != null};

    feature seq = (1, 2, 3);

    feature cond = if true ? 1 else 0;

    feature meta_access = Vehicle.metadata;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/coverage_expressions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 6 4) (end 6 20))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 6 4) (end 6 20))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 8 4) (end 8 20))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 8 4) (end 8 20))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 13 20) (end 13 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 22 26) (end 22 42))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery,unsupported-syntax) (has-evaluation true) (source-digest "blake3:7a9bb88d3c4b1c3ad264e9edd76f68423257a29ce0610bd33500ba6b555a25a3") (contract-version "lossless-publication-completeness-v3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::Item"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::Vehicle"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::all_v"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::arr"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "items")))))
    (declaration (id (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::coll"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "items")))))
    (declaration (id (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::cond"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::h"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "items")))))
    (declaration (id (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::items"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower unbounded) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Item")))))
    (declaration (id (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::meta_access"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::n"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::r"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::s"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::sel"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "items")))))
    (declaration (id (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::seq"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::arr"))) (kind expressionOperand) (ordinal 0))
      (authored-target "items")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::items")))))
    (reference (id (source (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::coll"))) (kind expressionOperand) (ordinal 0))
      (authored-target "items")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::items")))))
    (reference (id (source (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::h"))) (kind expressionOperand) (ordinal 0))
      (authored-target "items")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::items")))))
    (reference (id (source (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::items"))) (kind featureTyping) (ordinal 0))
      (authored-target "Item")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::Item")))))
    (reference (id (source (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::sel"))) (kind expressionOperand) (ordinal 0))
      (authored-target "items")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::items")))))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::arr"))) (target (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::items"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::arr"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::coll"))) (target (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::items"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::coll"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::h"))) (target (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::items"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::h"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::items"))) (target (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::Item"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::items"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::sel"))) (target (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::items"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::sel"))) (kind expressionOperand) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::all_v"))) (state unsupported))
    (evaluated (declaration (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::arr"))) (state unsupported))
    (evaluated (declaration (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::coll"))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::cond"))) (state unsupported))
    (evaluated (declaration (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::h"))) (state unsupported))
    (evaluated (declaration (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::meta_access"))) (state unsupported))
    (evaluated (declaration (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::n"))) (state unsupported))
    (evaluated (declaration (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::r"))) (state literal) (value (kind real) (real 3.14)))
    (evaluated (declaration (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::s"))) (state literal) (value (kind string) (value "hello")))
    (evaluated (declaration (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::sel"))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::seq"))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::Item")))
      (subtype (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::items")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::items")))
      (type (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::Item")) (provenance authored))
      (effective-type (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::Item")) (source direct))
      (supertype (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::Item")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/coverage_expressions.md") (range (start 11 18) (end 11 23)) (probe (position 11 18))
    (reference (id (source (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::arr"))) (kind expressionOperand) (ordinal 0) (authored-target "items")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::items")))))
    )
  )
  (query (document "memory://snapshot/coverage_expressions.md") (range (start 15 19) (end 15 24)) (probe (position 15 19))
    (reference (id (source (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::coll"))) (kind expressionOperand) (ordinal 0) (authored-target "items")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::items")))))
    )
  )
  (query (document "memory://snapshot/coverage_expressions.md") (range (start 12 16) (end 12 21)) (probe (position 12 16))
    (reference (id (source (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::h"))) (kind expressionOperand) (ordinal 0) (authored-target "items")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::items")))))
    )
  )
  (query (document "memory://snapshot/coverage_expressions.md") (range (start 10 20) (end 10 24)) (probe (position 10 20))
    (reference (id (source (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::items"))) (kind featureTyping) (ordinal 0) (authored-target "Item")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::Item")))))
    )
  )
  (query (document "memory://snapshot/coverage_expressions.md") (range (start 16 18) (end 16 23)) (probe (position 16 18))
    (reference (id (source (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::sel"))) (kind expressionOperand) (ordinal 0) (authored-target "items")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::items")))))
    )
  )
)
~~~
