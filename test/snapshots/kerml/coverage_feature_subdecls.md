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
