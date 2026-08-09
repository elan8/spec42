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
(model
  (namespace
    (package 'FeatureSubDeclCoverage'
      (feature_def 'a'
        (multiplicity_range [1]))
      (feature_def 'b'
        (multiplicity_range [0..*]))
      (feature_def 'c'
        (feature_value (=)))
      (feature_def 'd'
        (feature_value (:=)))
      (feature_def 'e'
        (feature_value (default =)))
      (feature_def 'f'
        (feature_value (default :=)))
      (feature_def 'g')
      (feature_def 'h'
        (feature_inverting_decl :> 'FeatureSubDeclCoverage::g'[feature_def]))
      (feature_inverting_decl)
      (feature_inverting_decl 'myInv')
      (type_featuring_decl)
      (type_featuring_decl 'myFeat'))))
~~~
