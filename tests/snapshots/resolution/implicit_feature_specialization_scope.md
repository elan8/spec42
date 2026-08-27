# META
~~~ini
description=Implied Feature specialization inherits Base::things members during name resolution
type=file
libraries=standard
~~~
# SOURCE
~~~kerml
package Demo {
    feature outer {
        feature inner subsets that;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship
    (kind subsetting)
    (source "Demo::outer::inner")
    (target "Base::things::that")
    (provenance authored)
    (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/implicit_feature_specialization_scope.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:fdc338a6a7d6161badb51a6638a8bbb6035bf5f48516e8eed3fe8f84797c9eae") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/implicit_feature_specialization_scope.md") (qualified-name "Demo"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/implicit_feature_specialization_scope.md") (qualified-name "Demo::outer"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/implicit_feature_specialization_scope.md") (qualified-name "Demo::outer::inner"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "that")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/implicit_feature_specialization_scope.md") (qualified-name "Demo::outer::inner"))) (kind subsetting) (ordinal 0))
      (authored-target "that")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things::that")))))
  )
  (relationships
    (relationship (kind subsetting) (source (node (document "memory://snapshot/implicit_feature_specialization_scope.md") (qualified-name "Demo::outer::inner"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things::that"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/implicit_feature_specialization_scope.md") (qualified-name "Demo::outer::inner"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/implicit_feature_specialization_scope.md") (qualified-name "Demo::outer"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/implicit_feature_specialization_scope.md") (qualified-name "Demo::outer::inner"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/implicit_feature_specialization_scope.md") (qualified-name "Demo::outer::inner"))) (target (node (document "memory://snapshot/implicit_feature_specialization_scope.md") (qualified-name "Demo::outer"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/implicit_feature_specialization_scope.md") (qualified-name "Demo::outer")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/implicit_feature_specialization_scope.md") (qualified-name "Demo::outer::inner")))
      (featured-by (node (document "memory://snapshot/implicit_feature_specialization_scope.md") (qualified-name "Demo::outer")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things::that"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things::that")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/implicit_feature_specialization_scope.md") (range (start 2 30) (end 2 34)) (probe (position 2 30))
    (reference (id (source (node (document "memory://snapshot/implicit_feature_specialization_scope.md") (qualified-name "Demo::outer::inner"))) (kind subsetting) (ordinal 0) (authored-target "that")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things::that")))))
    )
  )
)
~~~
