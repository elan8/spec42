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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "coverage_feature_decl_helpers.md"
    (diagnostics
    )
  )
)
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
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "cc47ce602a300330b09a9953b4af421aa1b8596cb3bbf2a46e47fdbce8c5187c") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "FeatureDeclCoverage"))) (kind "package") (name "FeatureDeclCoverage") (declared-name "FeatureDeclCoverage") (range (start (line 0) (character 0)) (end (line 0) (character 138))))
    (element (id (node (document "d0") (qualified-name "FeatureDeclCoverage::Base"))) (kind "classifier decl") (name "Base") (declared-name "Base") (range (start (line 1) (character 1)) (end (line 1) (character 17))) (parent (node (document "d0") (qualified-name "FeatureDeclCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureDeclCoverage::b"))) (kind "kermlDecl") (name "b") (declared-name "b") (range (start (line 5) (character 1)) (end (line 5) (character 13))) (parent (node (document "d0") (qualified-name "FeatureDeclCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureDeclCoverage::e"))) (kind "kermlDecl") (name "e") (declared-name "e") (range (start (line 4) (character 1)) (end (line 4) (character 13))) (parent (node (document "d0") (qualified-name "FeatureDeclCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureDeclCoverage::f"))) (kind "feature decl") (name "f") (declared-name "f") (range (start (line 2) (character 1)) (end (line 2) (character 11))) (parent (node (document "d0") (qualified-name "FeatureDeclCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureDeclCoverage::i"))) (kind "kermlDecl") (name "i") (declared-name "i") (range (start (line 6) (character 1)) (end (line 6) (character 12))) (parent (node (document "d0") (qualified-name "FeatureDeclCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureDeclCoverage::s"))) (kind "kermlDecl") (name "s") (declared-name "s") (range (start (line 3) (character 1)) (end (line 3) (character 13))) (parent (node (document "d0") (qualified-name "FeatureDeclCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureDeclCoverage::s2"))) (kind "kermlDecl") (name "s2") (declared-name "s2") (range (start (line 7) (character 1)) (end (line 7) (character 21))) (parent (node (document "d0") (qualified-name "FeatureDeclCoverage"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
