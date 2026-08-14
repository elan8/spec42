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
  (document "memory://snapshot/coverage_feature_subdecls.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 7 4) (end 7 28))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 7 4) (end 7 28))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 10 4) (end 14 0))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:56790a4770c6a0b4a8ca03565fcc54cf91c0375d06d134beee99b07b504c474b") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::a"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))))
    (declaration (id (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::b"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper unbounded))))
    (declaration (id (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::c"))) (kind default-reference) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::d"))) (kind default-reference) (membership (kind feature) (visibility default)) (feature-value (kind assign)))
    (declaration (id (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::e"))) (kind default-reference) (membership (kind feature) (visibility default)) (feature-value (kind bind) (default true)))
    (declaration (id (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::f"))) (kind default-reference) (membership (kind feature) (visibility default)) (feature-value (kind assign) (default true)))
    (declaration (id (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::h"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
  )
  (references
  )
  (relationships
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::c"))) (state literal) (value (kind integer) (integer 42)))
    (evaluated (declaration (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::d"))) (state literal) (value (kind integer) (integer 99)))
    (evaluated (declaration (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::e"))) (state literal) (value (kind integer) (integer 0)))
    (evaluated (declaration (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::f"))) (state literal) (value (kind integer) (integer 1)))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
