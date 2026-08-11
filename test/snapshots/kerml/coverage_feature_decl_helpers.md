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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "FeatureDeclCoverage"))) (name "FeatureDeclCoverage") (declared-name "FeatureDeclCoverage")
      (contains
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "FeatureDeclCoverage::Base"))) (name "Base") (declared-name "Base"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "FeatureDeclCoverage::b"))) (name "b") (declared-name "b"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "FeatureDeclCoverage::e"))) (name "e") (declared-name "e"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "FeatureDeclCoverage::f"))) (name "f") (declared-name "f"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "FeatureDeclCoverage::i"))) (name "i") (declared-name "i"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "FeatureDeclCoverage::s"))) (name "s") (declared-name "s"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "FeatureDeclCoverage::s2"))) (name "s2") (declared-name "s2"))
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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "kerml/coverage_feature_decl_helpers.md"
    (diagnostics
    )
  )
)
~~~
