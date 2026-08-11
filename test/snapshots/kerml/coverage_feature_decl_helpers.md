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
    (element (id (node (document "d0") (qualified-name "FeatureDeclCoverage"))) (kind "package") (name "FeatureDeclCoverage") (declared-name "FeatureDeclCoverage"))
    (element (id (node (document "d0") (qualified-name "FeatureDeclCoverage::Base"))) (kind "classifier decl") (name "Base") (declared-name "Base") (parent (node (document "d0") (qualified-name "FeatureDeclCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureDeclCoverage::b"))) (kind "kermlDecl") (name "b") (declared-name "b") (parent (node (document "d0") (qualified-name "FeatureDeclCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureDeclCoverage::e"))) (kind "kermlDecl") (name "e") (declared-name "e") (parent (node (document "d0") (qualified-name "FeatureDeclCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureDeclCoverage::f"))) (kind "feature decl") (name "f") (declared-name "f") (parent (node (document "d0") (qualified-name "FeatureDeclCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureDeclCoverage::i"))) (kind "kermlDecl") (name "i") (declared-name "i") (parent (node (document "d0") (qualified-name "FeatureDeclCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureDeclCoverage::s"))) (kind "kermlDecl") (name "s") (declared-name "s") (parent (node (document "d0") (qualified-name "FeatureDeclCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureDeclCoverage::s2"))) (kind "kermlDecl") (name "s2") (declared-name "s2") (parent (node (document "d0") (qualified-name "FeatureDeclCoverage"))))
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
