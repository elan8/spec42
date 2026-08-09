# META
~~~ini
description=Coverage: FeatureDeclExtra helper for expression, step, boolean_expression, invariant with specializations
type=file
~~~
# SOURCE
~~~kerml
package FeatureDeclCoverage {
	classifier Base;
	feature f;
	step s :> f;
	expr e :> f;
	bool b :> f;
	inv i :> f;
	step s2 [0..1] = 42;
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
KwFeature,Ident,Semicolon,
KwStep,Ident,ColonGt,Ident,Semicolon,
KwExpr,Ident,ColonGt,Ident,Semicolon,
KwBool,Ident,ColonGt,Ident,Semicolon,
KwInv,Ident,ColonGt,Ident,Semicolon,
KwStep,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Eq,DecimalValue,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'FeatureDeclCoverage'
    (classifier_def 'Base')
    (feature_def 'f')
    (step_def)
    (expression_def)
    (boolean_expr_def)
    (invariant_def)
    (step_def)))
~~~
# FORMAT
~~~sysml
package FeatureDeclCoverage {
    classifier Base;
    feature f;
    step s :> f;
    expr e :> f;
    bool b :> f;
    inv i :> f;
    step s2 [0..1] = 42;
}
~~~
# SMG
~~~
(model
  (namespace
    (package 'FeatureDeclCoverage'
      (classifier_def 'Base')
      (feature_def 'f')
      (step_def 's' :> 'FeatureDeclCoverage::f'[feature_def])
      (expression_def 'e' :> 'FeatureDeclCoverage::f'[feature_def])
      (boolean_expr_def 'b' :> 'FeatureDeclCoverage::f'[feature_def])
      (invariant_def 'i' :> 'FeatureDeclCoverage::f'[feature_def])
      (step_def 's2'
        (multiplicity_range [0..1])
        (feature_value (=))))))
~~~
