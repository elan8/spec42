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
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "8853cee9ec7516d18367d1c46605fb53f3ee9b3b9e391a336c2d00b5920f4944") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ExpressionCoverage"))) (kind "package") (name "ExpressionCoverage") (declared-name "ExpressionCoverage") (range (start (line 0) (character 0)) (end (line 0) (character 501))))
    (element (id (node (document "d0") (qualified-name "ExpressionCoverage::Item"))) (kind "classifier decl") (name "Item") (declared-name "Item") (range (start (line 2) (character 4)) (end (line 2) (character 20))) (parent (node (document "d0") (qualified-name "ExpressionCoverage"))))
    (element (id (node (document "d0") (qualified-name "ExpressionCoverage::Vehicle"))) (kind "classifier decl") (name "Vehicle") (declared-name "Vehicle") (range (start (line 1) (character 4)) (end (line 1) (character 23))) (parent (node (document "d0") (qualified-name "ExpressionCoverage"))))
    (element (id (node (document "d0") (qualified-name "ExpressionCoverage::all_v"))) (kind "feature decl") (name "all_v") (declared-name "all_v") (range (start (line 13) (character 4)) (end (line 13) (character 32))) (parent (node (document "d0") (qualified-name "ExpressionCoverage"))))
    (element (id (node (document "d0") (qualified-name "ExpressionCoverage::arr"))) (kind "feature decl") (name "arr") (declared-name "arr") (range (start (line 11) (character 4)) (end (line 11) (character 27))) (parent (node (document "d0") (qualified-name "ExpressionCoverage"))))
    (element (id (node (document "d0") (qualified-name "ExpressionCoverage::coll"))) (kind "feature decl") (name "coll") (declared-name "coll") (range (start (line 15) (character 4)) (end (line 15) (character 34))) (parent (node (document "d0") (qualified-name "ExpressionCoverage"))))
    (element (id (node (document "d0") (qualified-name "ExpressionCoverage::h"))) (kind "feature decl") (name "h") (declared-name "h") (range (start (line 12) (character 4)) (end (line 12) (character 26))) (parent (node (document "d0") (qualified-name "ExpressionCoverage"))))
    (element (id (node (document "d0") (qualified-name "ExpressionCoverage::inf"))) (kind "feature decl") (name "inf") (declared-name "inf") (range (start (line 8) (character 4)) (end (line 8) (character 20))) (parent (node (document "d0") (qualified-name "ExpressionCoverage"))))
    (element (id (node (document "d0") (qualified-name "ExpressionCoverage::items"))) (kind "feature decl") (name "items") (declared-name "items") (range (start (line 10) (character 4)) (end (line 10) (character 28))) (parent (node (document "d0") (qualified-name "ExpressionCoverage"))))
    (element (id (node (document "d0") (qualified-name "ExpressionCoverage::n"))) (kind "feature decl") (name "n") (declared-name "n") (range (start (line 7) (character 4)) (end (line 7) (character 21))) (parent (node (document "d0") (qualified-name "ExpressionCoverage"))))
    (element (id (node (document "d0") (qualified-name "ExpressionCoverage::r"))) (kind "feature decl") (name "r") (declared-name "r") (range (start (line 5) (character 4)) (end (line 5) (character 21))) (parent (node (document "d0") (qualified-name "ExpressionCoverage"))))
    (element (id (node (document "d0") (qualified-name "ExpressionCoverage::r2"))) (kind "feature decl") (name "r2") (declared-name "r2") (range (start (line 6) (character 4)) (end (line 6) (character 20))) (parent (node (document "d0") (qualified-name "ExpressionCoverage"))))
    (element (id (node (document "d0") (qualified-name "ExpressionCoverage::s"))) (kind "feature decl") (name "s") (declared-name "s") (range (start (line 4) (character 4)) (end (line 4) (character 24))) (parent (node (document "d0") (qualified-name "ExpressionCoverage"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
