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
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 1 4) (end 1 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 2 4) (end 2 20))
      )
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
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 7 16) (end 7 20))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 20) (end 10 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 11 4) (end 11 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 11 4) (end 11 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 12 16) (end 12 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 13 20) (end 13 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 15 4) (end 15 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 15 4) (end 15 34))
      )
      (diagnostic
        (severity error)
        (code "recovered_package_body_element")
        (source "parser")
        (range (start 15 34) (end 16 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 16 4) (end 16 42))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 16 4) (end 16 42))
      )
      (diagnostic
        (severity error)
        (code "recovered_package_body_element")
        (source "parser")
        (range (start 16 42) (end 18 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 20 19) (end 20 37))
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
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:7a9bb88d3c4b1c3ad264e9edd76f68423257a29ce0610bd33500ba6b555a25a3") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::all_v"))) (kind default-reference) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::cond"))) (kind default-reference) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::h"))) (kind default-reference) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::items"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Item"))))
    (declaration (id (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::meta_access"))) (kind default-reference) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::n"))) (kind default-reference) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::r"))) (kind default-reference) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::s"))) (kind default-reference) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::seq"))) (kind default-reference) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::items"))) (kind featureTyping) (ordinal 0))
      (authored-target "Item")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::r"))) (value (kind real) (real 3.14)))
    (evaluated (declaration (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::s"))) (value (kind string) (value "hello")))
    (evaluated (declaration (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::seq"))) (value (kind non-constant)))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/coverage_expressions.md") (range (start 10 20) (end 10 24)) (probe (position 10 20))
    (reference (id (source (node (document "memory://snapshot/coverage_expressions.md") (qualified-name "ExpressionCoverage::items"))) (kind featureTyping) (ordinal 0) (authored-target "Item")
      (outcome (status unresolved)))
  )
)
~~~
