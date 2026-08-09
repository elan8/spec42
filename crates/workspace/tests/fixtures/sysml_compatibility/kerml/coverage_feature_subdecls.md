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
# EXPECTED
~~~
semantic.unresolved_name 'T'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'T'
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
# FORMAT
~~~sysml
package FeatureSubDeclCoverage {
    feature a[1];
    feature b[0..*];
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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "FeatureSubDeclCoverage"))) (name "FeatureSubDeclCoverage") (declared-name "FeatureSubDeclCoverage")
      (contains
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "FeatureSubDeclCoverage::a"))) (name "a") (declared-name "a"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "FeatureSubDeclCoverage::b"))) (name "b") (declared-name "b"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "FeatureSubDeclCoverage::c"))) (name "c") (declared-name "c"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "FeatureSubDeclCoverage::d"))) (name "d") (declared-name "d"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "FeatureSubDeclCoverage::e"))) (name "e") (declared-name "e"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "FeatureSubDeclCoverage::f"))) (name "f") (declared-name "f"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "FeatureSubDeclCoverage::g"))) (name "g") (declared-name "g"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "FeatureSubDeclCoverage::h"))) (name "h") (declared-name "h"))
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
