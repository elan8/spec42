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
  (document "coverage_expressions.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_package_body_element")
        (source "sysml")
        (range (start 15 34) (end 15 195))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "8853cee9ec7516d18367d1c46605fb53f3ee9b3b9e391a336c2d00b5920f4944") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ExpressionCoverage"))) (kind "package") (name "ExpressionCoverage") (declared-name "ExpressionCoverage"))
    (element (id (node (document "d0") (qualified-name "ExpressionCoverage::Item"))) (kind "classifier decl") (name "Item") (declared-name "Item") (parent (node (document "d0") (qualified-name "ExpressionCoverage"))))
    (element (id (node (document "d0") (qualified-name "ExpressionCoverage::Vehicle"))) (kind "classifier decl") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "ExpressionCoverage"))))
    (element (id (node (document "d0") (qualified-name "ExpressionCoverage::all_v"))) (kind "feature decl") (name "all_v") (declared-name "all_v") (parent (node (document "d0") (qualified-name "ExpressionCoverage"))))
    (element (id (node (document "d0") (qualified-name "ExpressionCoverage::arr"))) (kind "feature decl") (name "arr") (declared-name "arr") (parent (node (document "d0") (qualified-name "ExpressionCoverage"))))
    (element (id (node (document "d0") (qualified-name "ExpressionCoverage::coll"))) (kind "feature decl") (name "coll") (declared-name "coll") (parent (node (document "d0") (qualified-name "ExpressionCoverage"))))
    (element (id (node (document "d0") (qualified-name "ExpressionCoverage::h"))) (kind "feature decl") (name "h") (declared-name "h") (parent (node (document "d0") (qualified-name "ExpressionCoverage"))))
    (element (id (node (document "d0") (qualified-name "ExpressionCoverage::inf"))) (kind "feature decl") (name "inf") (declared-name "inf") (parent (node (document "d0") (qualified-name "ExpressionCoverage"))))
    (element (id (node (document "d0") (qualified-name "ExpressionCoverage::items"))) (kind "feature decl") (name "items") (declared-name "items") (parent (node (document "d0") (qualified-name "ExpressionCoverage"))))
    (element (id (node (document "d0") (qualified-name "ExpressionCoverage::n"))) (kind "feature decl") (name "n") (declared-name "n") (parent (node (document "d0") (qualified-name "ExpressionCoverage"))))
    (element (id (node (document "d0") (qualified-name "ExpressionCoverage::r"))) (kind "feature decl") (name "r") (declared-name "r") (parent (node (document "d0") (qualified-name "ExpressionCoverage"))))
    (element (id (node (document "d0") (qualified-name "ExpressionCoverage::r2"))) (kind "feature decl") (name "r2") (declared-name "r2") (parent (node (document "d0") (qualified-name "ExpressionCoverage"))))
    (element (id (node (document "d0") (qualified-name "ExpressionCoverage::s"))) (kind "feature decl") (name "s") (declared-name "s") (parent (node (document "d0") (qualified-name "ExpressionCoverage"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
