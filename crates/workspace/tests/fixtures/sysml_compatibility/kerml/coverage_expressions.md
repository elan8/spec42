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
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwClassifier,Ident,Semicolon,
KwClassifier,Ident,Semicolon,
KwFeature,Ident,Eq,StringValue,Semicolon,
KwFeature,Ident,Eq,DecimalValue,Dot,DecimalValue,Semicolon,
KwFeature,Ident,Eq,Dot,DecimalValue,Semicolon,
KwFeature,Ident,Eq,KwNull,Semicolon,
KwFeature,Ident,Eq,Star,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Semicolon,
KwFeature,Ident,Eq,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwFeature,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,Semicolon,
KwFeature,Ident,Eq,KwAll,Ident,Semicolon,
KwFeature,Ident,Eq,Ident,Dot,OpenCurly,KwIn,Ident,Semicolon,Ident,CloseCurly,Semicolon,
KwFeature,Ident,Eq,Ident,DotQuestion,OpenCurly,KwIn,Ident,Semicolon,Ident,BangEq,KwNull,CloseCurly,Semicolon,
KwFeature,Ident,Eq,OpenParen,DecimalValue,Comma,DecimalValue,Comma,DecimalValue,CloseParen,Semicolon,
KwFeature,Ident,Eq,KwIf,KwTrue,Question,DecimalValue,KwElse,DecimalValue,Semicolon,
KwFeature,Ident,Eq,Ident,Dot,KwMetadata,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'ExpressionCoverage'
    (classifier_def 'Vehicle')
    (classifier_def 'Item')
    (feature_def 's' value)
    (feature_def 'r' value)
    (feature_def 'r2' value)
    (feature_def 'n' value)
    (feature_def 'inf' value)
    (feature_def 'items' : 'Item' multiplicity)
    (feature_def 'arr' value)
    (feature_def 'h' value)
    (feature_def 'all_v' value)
    (feature_def 'coll' value)
    (feature_def 'sel' value)
    (feature_def 'seq' value)
    (feature_def 'cond' value)
    (feature_def 'meta_access' value)))
~~~
# FORMAT
~~~sysml
package ExpressionCoverage {
    classifier Vehicle;
    classifier Item;

    feature s = "hello";
    feature r = 3.14;
    feature r2 = .5;
    feature n = null;
    feature inf = *;

    feature items : Item [*];
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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "ExpressionCoverage"))) (name "ExpressionCoverage") (declared-name "ExpressionCoverage")
      (contains
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "ExpressionCoverage::Item"))) (name "Item") (declared-name "Item"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "ExpressionCoverage::Vehicle"))) (name "Vehicle") (declared-name "Vehicle"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "ExpressionCoverage::all_v"))) (name "all_v") (declared-name "all_v"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "ExpressionCoverage::arr"))) (name "arr") (declared-name "arr"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "ExpressionCoverage::coll"))) (name "coll") (declared-name "coll"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "ExpressionCoverage::h"))) (name "h") (declared-name "h"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "ExpressionCoverage::inf"))) (name "inf") (declared-name "inf"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "ExpressionCoverage::items"))) (name "items") (declared-name "items"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "ExpressionCoverage::n"))) (name "n") (declared-name "n"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "ExpressionCoverage::r"))) (name "r") (declared-name "r"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "ExpressionCoverage::r2"))) (name "r2") (declared-name "r2"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "ExpressionCoverage::s"))) (name "s") (declared-name "s"))
      )
    )
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
