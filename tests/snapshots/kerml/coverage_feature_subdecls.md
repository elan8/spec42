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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 7 26) (end 7 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 10 4) (end 10 19))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 11 4) (end 11 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 12 4) (end 12 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 13 4) (end 13 31))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:56790a4770c6a0b4a8ca03565fcc54cf91c0375d06d134beee99b07b504c474b") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::a"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))))
    (declaration (id (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::b"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper unbounded))))
    (declaration (id (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::c"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "c")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "c")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "c")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "c")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "c")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::d"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind assign) (value (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "d")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "d")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "d")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "d")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "d")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::e"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "e")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "e")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (default true)))
    (declaration (id (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "e")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "e")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "e")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::f"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (feature-value (kind assign) (value (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "f")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "f")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (default true)))
    (declaration (id (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "f")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "f")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "f")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::g"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (typeFeaturing (reference "T")))))
    (declaration (id (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::h"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureInverting (reference "g")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::g"))) (kind typeFeaturing) (ordinal 0))
      (authored-target "T")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::h"))) (kind featureInverting) (ordinal 0))
      (authored-target "g")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::g")))))
  )
  (relationships
    (relationship (kind featureInverting) (source (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::h"))) (target (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::g"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::h"))) (kind featureInverting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::c"))) (target (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "c")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "c")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "c")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::d"))) (target (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "d")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "d")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "d")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "e")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "e")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "f")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "f")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "c")) (anonymous (kind kerml-expression) (ordinal 0))))) (state literal) (value (kind integer) (integer 42)))
    (evaluated (declaration (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "d")) (anonymous (kind kerml-expression) (ordinal 0))))) (state literal) (value (kind integer) (integer 99)))
    (evaluated (declaration (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "e")) (anonymous (kind kerml-expression) (ordinal 0))))) (state literal) (value (kind integer) (integer 0)))
    (evaluated (declaration (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "f")) (anonymous (kind kerml-expression) (ordinal 0))))) (state literal) (value (kind integer) (integer 1)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::c")))
      (supertype (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "c")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "c")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "c")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::c")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::d")))
      (supertype (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "d")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "d")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "d")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::d")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "e")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "e")) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "f")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/coverage_feature_subdecls.md") (path (named (kind package) (name "FeatureSubDeclCoverage")) (named (kind kerml-feature) (name "f")) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/coverage_feature_subdecls.md") (range (start 7 26) (end 7 27)) (probe (position 7 26))
    (reference (id (source (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::g"))) (kind typeFeaturing) (ordinal 0) (authored-target "T")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/coverage_feature_subdecls.md") (range (start 8 25) (end 8 26)) (probe (position 8 25))
    (reference (id (source (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::h"))) (kind featureInverting) (ordinal 0) (authored-target "g")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::g")))))
    )
  )
)
~~~
