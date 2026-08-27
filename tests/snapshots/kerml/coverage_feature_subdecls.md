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
        (code "unsupported_relationship_body_member")
        (source "semantic")
        (range (start 11 4) (end 11 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 12 19) (end 12 20))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_relationship_body_member")
        (source "semantic")
        (range (start 13 4) (end 13 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 13 29) (end 13 30))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:56790a4770c6a0b4a8ca03565fcc54cf91c0375d06d134beee99b07b504c474b"))
  (declarations
    (declaration (id (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage"))) (kind package) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (explicitRelationshipEndpoint (reference "f")) (explicitRelationshipEndpoint (reference "g")) (explicitRelationshipEndpoint (reference "f")) (explicitRelationshipEndpoint (reference "g")) (explicitRelationshipEndpoint (reference "f")) (explicitRelationshipEndpoint (reference "T")) (explicitRelationshipEndpoint (reference "f")) (explicitRelationshipEndpoint (reference "T")))))
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
    (reference (id (source (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage"))) (kind explicitRelationshipEndpoint) (ordinal 0))
      (authored-target "f")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::f")))))
    (reference (id (source (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage"))) (kind explicitRelationshipEndpoint) (ordinal 1))
      (authored-target "g")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::g")))))
    (reference (id (source (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage"))) (kind explicitRelationshipEndpoint) (ordinal 2))
      (authored-target "f")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::f")))))
    (reference (id (source (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage"))) (kind explicitRelationshipEndpoint) (ordinal 3))
      (authored-target "g")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::g")))))
    (reference (id (source (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage"))) (kind explicitRelationshipEndpoint) (ordinal 4))
      (authored-target "f")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::f")))))
    (reference (id (source (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage"))) (kind explicitRelationshipEndpoint) (ordinal 5))
      (authored-target "T")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage"))) (kind explicitRelationshipEndpoint) (ordinal 6))
      (authored-target "f")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::f")))))
    (reference (id (source (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage"))) (kind explicitRelationshipEndpoint) (ordinal 7))
      (authored-target "T")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::g"))) (kind typeFeaturing) (ordinal 0))
      (authored-target "T")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::h"))) (kind featureInverting) (ordinal 0))
      (authored-target "g")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::g")))))
  )
  (relationships
    (relationship (kind featureInverting) (source (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::h"))) (target (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::g"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::h"))) (kind featureInverting) (ordinal 0)))
    (relationship (kind featureInverting) (source (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::f"))) (target (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::g"))) (provenance authored))
    (relationship (kind featureInverting) (source (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::f"))) (target (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::g"))) (provenance authored))
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
  (query (document "memory://snapshot/coverage_feature_subdecls.md") (range (start 10 12) (end 10 13)) (probe (position 10 12))
    (reference (id (source (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage"))) (kind explicitRelationshipEndpoint) (ordinal 0) (authored-target "f")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::f")))))
    )
  )
  (query (document "memory://snapshot/coverage_feature_subdecls.md") (range (start 10 17) (end 10 18)) (probe (position 10 17))
    (reference (id (source (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage"))) (kind explicitRelationshipEndpoint) (ordinal 1) (authored-target "g")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::g")))))
    )
  )
  (query (document "memory://snapshot/coverage_feature_subdecls.md") (range (start 11 28) (end 11 29)) (probe (position 11 28))
    (reference (id (source (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage"))) (kind explicitRelationshipEndpoint) (ordinal 2) (authored-target "f")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::f")))))
    )
  )
  (query (document "memory://snapshot/coverage_feature_subdecls.md") (range (start 11 33) (end 11 34)) (probe (position 11 33))
    (reference (id (source (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage"))) (kind explicitRelationshipEndpoint) (ordinal 3) (authored-target "g")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::g")))))
    )
  )
  (query (document "memory://snapshot/coverage_feature_subdecls.md") (range (start 12 14) (end 12 15)) (probe (position 12 14))
    (reference (id (source (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage"))) (kind explicitRelationshipEndpoint) (ordinal 4) (authored-target "f")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::f")))))
    )
  )
  (query (document "memory://snapshot/coverage_feature_subdecls.md") (range (start 12 19) (end 12 20)) (probe (position 12 19))
    (reference (id (source (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage"))) (kind explicitRelationshipEndpoint) (ordinal 5) (authored-target "T")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/coverage_feature_subdecls.md") (range (start 13 24) (end 13 25)) (probe (position 13 24))
    (reference (id (source (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage"))) (kind explicitRelationshipEndpoint) (ordinal 6) (authored-target "f")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage::f")))))
    )
  )
  (query (document "memory://snapshot/coverage_feature_subdecls.md") (range (start 13 29) (end 13 30)) (probe (position 13 29))
    (reference (id (source (node (document "memory://snapshot/coverage_feature_subdecls.md") (qualified-name "FeatureSubDeclCoverage"))) (kind explicitRelationshipEndpoint) (ordinal 7) (authored-target "T")
      (outcome (status unresolved)))
    )
  )
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
