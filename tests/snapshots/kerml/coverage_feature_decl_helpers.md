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
  (document "memory://snapshot/coverage_feature_decl_helpers.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 6 1) (end 6 12))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 6 1) (end 6 12))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery,unsupported-syntax) (has-evaluation true) (source-digest "blake3:4383bf7d587d38cb89fbaf501cd8a6723613fd23d4aa52c1e3dc99ebd16c8517") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (qualified-name "FeatureDeclCoverage"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (qualified-name "FeatureDeclCoverage::Base"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (qualified-name "FeatureDeclCoverage::b"))) (kind kerml-boolean-expression) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "f")))))
    (declaration (id (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (qualified-name "FeatureDeclCoverage::e"))) (kind kerml-expression) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "f")))))
    (declaration (id (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (qualified-name "FeatureDeclCoverage::f"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (qualified-name "FeatureDeclCoverage::s"))) (kind kerml-step) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "f")))))
    (declaration (id (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (qualified-name "FeatureDeclCoverage::s2"))) (kind kerml-step) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))) (feature-value (kind bind) (value (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (path (named (kind package) (name "FeatureDeclCoverage")) (named (kind kerml-step) (name "s2")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (path (named (kind package) (name "FeatureDeclCoverage")) (named (kind kerml-step) (name "s2")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (path (named (kind package) (name "FeatureDeclCoverage")) (named (kind kerml-step) (name "s2")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (path (named (kind package) (name "FeatureDeclCoverage")) (named (kind kerml-step) (name "s2")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (path (named (kind package) (name "FeatureDeclCoverage")) (named (kind kerml-step) (name "s2")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (qualified-name "FeatureDeclCoverage::b"))) (kind subsetting) (ordinal 0))
      (authored-target "f")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (qualified-name "FeatureDeclCoverage::f")))))
    (reference (id (source (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (qualified-name "FeatureDeclCoverage::e"))) (kind subsetting) (ordinal 0))
      (authored-target "f")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (qualified-name "FeatureDeclCoverage::f")))))
    (reference (id (source (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (qualified-name "FeatureDeclCoverage::s"))) (kind subsetting) (ordinal 0))
      (authored-target "f")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (qualified-name "FeatureDeclCoverage::f")))))
  )
  (relationships
    (relationship (kind subsetting) (source (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (qualified-name "FeatureDeclCoverage::b"))) (target (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (qualified-name "FeatureDeclCoverage::f"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (qualified-name "FeatureDeclCoverage::b"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (qualified-name "FeatureDeclCoverage::e"))) (target (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (qualified-name "FeatureDeclCoverage::f"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (qualified-name "FeatureDeclCoverage::e"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (qualified-name "FeatureDeclCoverage::s"))) (target (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (qualified-name "FeatureDeclCoverage::f"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (qualified-name "FeatureDeclCoverage::s"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (qualified-name "FeatureDeclCoverage::s2"))) (target (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (path (named (kind package) (name "FeatureDeclCoverage")) (named (kind kerml-step) (name "s2")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (path (named (kind package) (name "FeatureDeclCoverage")) (named (kind kerml-step) (name "s2")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (path (named (kind package) (name "FeatureDeclCoverage")) (named (kind kerml-step) (name "s2")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (path (named (kind package) (name "FeatureDeclCoverage")) (named (kind kerml-step) (name "s2")) (anonymous (kind kerml-expression) (ordinal 0))))) (state literal) (value (kind integer) (integer 42)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (qualified-name "FeatureDeclCoverage::b")))
      (supertype (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (qualified-name "FeatureDeclCoverage::f")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (qualified-name "FeatureDeclCoverage::e")))
      (supertype (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (qualified-name "FeatureDeclCoverage::f")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (qualified-name "FeatureDeclCoverage::f")))
      (subtype (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (qualified-name "FeatureDeclCoverage::b")) (scopes any feature))
      (subtype (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (qualified-name "FeatureDeclCoverage::e")) (scopes any feature))
      (subtype (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (qualified-name "FeatureDeclCoverage::s")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (qualified-name "FeatureDeclCoverage::s")))
      (supertype (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (qualified-name "FeatureDeclCoverage::f")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (qualified-name "FeatureDeclCoverage::s2")))
      (supertype (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (path (named (kind package) (name "FeatureDeclCoverage")) (named (kind kerml-step) (name "s2")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (path (named (kind package) (name "FeatureDeclCoverage")) (named (kind kerml-step) (name "s2")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (path (named (kind package) (name "FeatureDeclCoverage")) (named (kind kerml-step) (name "s2")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (qualified-name "FeatureDeclCoverage::s2")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/coverage_feature_decl_helpers.md") (range (start 5 11) (end 5 12)) (probe (position 5 11))
    (reference (id (source (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (qualified-name "FeatureDeclCoverage::b"))) (kind subsetting) (ordinal 0) (authored-target "f")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (qualified-name "FeatureDeclCoverage::f")))))
    )
  )
  (query (document "memory://snapshot/coverage_feature_decl_helpers.md") (range (start 4 11) (end 4 12)) (probe (position 4 11))
    (reference (id (source (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (qualified-name "FeatureDeclCoverage::e"))) (kind subsetting) (ordinal 0) (authored-target "f")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (qualified-name "FeatureDeclCoverage::f")))))
    )
  )
  (query (document "memory://snapshot/coverage_feature_decl_helpers.md") (range (start 3 11) (end 3 12)) (probe (position 3 11))
    (reference (id (source (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (qualified-name "FeatureDeclCoverage::s"))) (kind subsetting) (ordinal 0) (authored-target "f")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_feature_decl_helpers.md") (qualified-name "FeatureDeclCoverage::f")))))
    )
  )
)
~~~
