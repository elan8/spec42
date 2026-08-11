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
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "620120a4903666b6ac2fd4f1e68cc8945e6e22fc6f05cccdf1020c33ae9f6522") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "FeatureSubDeclCoverage"))) (kind "package") (name "FeatureSubDeclCoverage") (declared-name "FeatureSubDeclCoverage"))
    (element (id (node (document "d0") (qualified-name "FeatureSubDeclCoverage::a"))) (kind "feature decl") (name "a") (declared-name "a") (parent (node (document "d0") (qualified-name "FeatureSubDeclCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureSubDeclCoverage::b"))) (kind "feature decl") (name "b") (declared-name "b") (parent (node (document "d0") (qualified-name "FeatureSubDeclCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureSubDeclCoverage::c"))) (kind "feature decl") (name "c") (declared-name "c") (parent (node (document "d0") (qualified-name "FeatureSubDeclCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureSubDeclCoverage::d"))) (kind "feature decl") (name "d") (declared-name "d") (parent (node (document "d0") (qualified-name "FeatureSubDeclCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureSubDeclCoverage::e"))) (kind "feature decl") (name "e") (declared-name "e") (parent (node (document "d0") (qualified-name "FeatureSubDeclCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureSubDeclCoverage::f"))) (kind "feature decl") (name "f") (declared-name "f") (parent (node (document "d0") (qualified-name "FeatureSubDeclCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureSubDeclCoverage::g"))) (kind "feature decl") (name "g") (declared-name "g") (parent (node (document "d0") (qualified-name "FeatureSubDeclCoverage"))))
    (element (id (node (document "d0") (qualified-name "FeatureSubDeclCoverage::h"))) (kind "feature decl") (name "h") (declared-name "h") (parent (node (document "d0") (qualified-name "FeatureSubDeclCoverage"))))
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
