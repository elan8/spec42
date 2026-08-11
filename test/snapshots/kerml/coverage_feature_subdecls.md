# META
~~~ini
description=Coverage: Feature sub-declarations (multiplicity, feature value, chaining, inverting, type featuring)
type=file
~~~
# SOURCE
~~~kerml
package FeatureSubDeclCoverage {
    feature a [1];
    feature b [0..*];
    feature c = 42;
    feature d := 99;
    feature e default = 0;
    feature f default := 1;
    feature g featured by T;
    feature h inverse of g;

    inverse f of g;
    inverting myInv inverse f of g;
    featuring f by T;
    featuring myFeat of f by T;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "coverage_feature_subdecls.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 10 4) (end 10 110))
      )
    )
  )
)
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwFeature,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwFeature,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwFeature,Ident,Eq,DecimalValue,Semicolon,
KwFeature,Ident,ColonEq,DecimalValue,Semicolon,
KwFeature,Ident,KwDefault,Eq,DecimalValue,Semicolon,
KwFeature,Ident,KwDefault,ColonEq,DecimalValue,Semicolon,
KwFeature,Ident,KwFeatured,KwBy,Ident,Semicolon,
KwFeature,Ident,KwInverse,KwOf,Ident,Semicolon,
KwInverse,Ident,KwOf,Ident,Semicolon,
KwInverting,Ident,KwInverse,Ident,KwOf,Ident,Semicolon,
KwFeaturing,Ident,KwBy,Ident,Semicolon,
KwFeaturing,Ident,KwOf,Ident,KwBy,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'FeatureSubDeclCoverage'
    (feature_def 'a' multiplicity)
    (feature_def 'b' multiplicity)
    (feature_def 'c' value)
    (feature_def 'd' value)
    (feature_def 'e' value)
    (feature_def 'f' value)
    (feature_def 'g' featured by 'T')
    (feature_def 'h' inverse of 'g')
    (feature_inverting_decl)
    (feature_inverting_decl)
    (type_featuring_decl)
    (type_featuring_decl)))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'T'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'T'
~~~
# FORMAT
~~~sysml
package FeatureSubDeclCoverage {
    feature a [1];
    feature b [0..*];
    feature c = 42;
    feature d := 99;
    feature e default = 0;
    feature f default := 1;
    feature g featured by T;
    feature h inverse of g;

    inverse f of g;
    inverting myInv inverse f of g;
    featuring f by T;
    featuring myFeat of f by T;
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "5c74ebec619b1dac2c52bfeb2c12c9abc4bcd14cff480b5e71aca857aa9efa2c") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "FeatureSubDeclCoverage"))) (kind "package") (name "FeatureSubDeclCoverage") (declared-name "FeatureSubDeclCoverage") (range (start (line 0) (character 0)) (end (line 0) (character 339))))
    (element (id (node (document "d0") (qualified-name "FeatureSubDeclCoverage::a"))) (kind "feature decl") (name "a") (declared-name "a") (range (start (line 1) (character 4)) (end (line 1) (character 18))) (parent (node (document "d0") (qualified-name "FeatureSubDeclCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureSubDeclCoverage::b"))) (kind "feature decl") (name "b") (declared-name "b") (range (start (line 2) (character 4)) (end (line 2) (character 21))) (parent (node (document "d0") (qualified-name "FeatureSubDeclCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureSubDeclCoverage::c"))) (kind "feature decl") (name "c") (declared-name "c") (range (start (line 3) (character 4)) (end (line 3) (character 19))) (parent (node (document "d0") (qualified-name "FeatureSubDeclCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureSubDeclCoverage::d"))) (kind "feature decl") (name "d") (declared-name "d") (range (start (line 4) (character 4)) (end (line 4) (character 20))) (parent (node (document "d0") (qualified-name "FeatureSubDeclCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureSubDeclCoverage::e"))) (kind "feature decl") (name "e") (declared-name "e") (range (start (line 5) (character 4)) (end (line 5) (character 26))) (parent (node (document "d0") (qualified-name "FeatureSubDeclCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureSubDeclCoverage::f"))) (kind "feature decl") (name "f") (declared-name "f") (range (start (line 6) (character 4)) (end (line 6) (character 27))) (parent (node (document "d0") (qualified-name "FeatureSubDeclCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureSubDeclCoverage::g"))) (kind "feature decl") (name "g") (declared-name "g") (range (start (line 7) (character 4)) (end (line 7) (character 28))) (parent (node (document "d0") (qualified-name "FeatureSubDeclCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureSubDeclCoverage::h"))) (kind "feature decl") (name "h") (declared-name "h") (range (start (line 8) (character 4)) (end (line 8) (character 27))) (parent (node (document "d0") (qualified-name "FeatureSubDeclCoverage"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
