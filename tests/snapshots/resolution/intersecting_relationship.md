# META
~~~ini
description=Intersecting relationship resolution coverage
type=file
observed_gap=The authored intersects targets are accepted by the parser but are absent from the published semantic facts; only the primary typing is currently visible.
~~~
# SOURCE
~~~sysml
package IntersectCoverage {
    part def Base;
    attribute a;
    attribute b;
    attribute reading : Base intersects a, b;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/intersecting_relationship.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "incompatible_type_kind")
        (source "semantic")
        (range (start 4 24) (end 4 28))
        (related-information
          (related
            (uri "memory://snapshot/intersecting_relationship.md")
            (range (start 1 4) (end 1 18))
          )
        )
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:5636759edf98aecf2c19163942f8f9453be470a3a2d2c0fff33e63f6ddef1ea8") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/intersecting_relationship.md") (qualified-name "IntersectCoverage"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/intersecting_relationship.md") (qualified-name "IntersectCoverage::Base"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/intersecting_relationship.md") (qualified-name "IntersectCoverage::a"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/intersecting_relationship.md") (qualified-name "IntersectCoverage::b"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/intersecting_relationship.md") (qualified-name "IntersectCoverage::reading"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Base")) (intersects (reference "a")) (intersects (reference "b")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/intersecting_relationship.md") (qualified-name "IntersectCoverage::reading"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/intersecting_relationship.md") (qualified-name "IntersectCoverage::Base")))))
    (reference (id (source (node (document "memory://snapshot/intersecting_relationship.md") (qualified-name "IntersectCoverage::reading"))) (kind intersects) (ordinal 0))
      (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/intersecting_relationship.md") (qualified-name "IntersectCoverage::a")))))
    (reference (id (source (node (document "memory://snapshot/intersecting_relationship.md") (qualified-name "IntersectCoverage::reading"))) (kind intersects) (ordinal 1))
      (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/intersecting_relationship.md") (qualified-name "IntersectCoverage::b")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/intersecting_relationship.md") (qualified-name "IntersectCoverage::reading"))) (target (node (document "memory://snapshot/intersecting_relationship.md") (qualified-name "IntersectCoverage::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/intersecting_relationship.md") (qualified-name "IntersectCoverage::reading"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind intersects) (source (node (document "memory://snapshot/intersecting_relationship.md") (qualified-name "IntersectCoverage::reading"))) (target (node (document "memory://snapshot/intersecting_relationship.md") (qualified-name "IntersectCoverage::a"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/intersecting_relationship.md") (qualified-name "IntersectCoverage::reading"))) (kind intersects) (ordinal 0)))
    (relationship (kind intersects) (source (node (document "memory://snapshot/intersecting_relationship.md") (qualified-name "IntersectCoverage::reading"))) (target (node (document "memory://snapshot/intersecting_relationship.md") (qualified-name "IntersectCoverage::b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/intersecting_relationship.md") (qualified-name "IntersectCoverage::reading"))) (kind intersects) (ordinal 1)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/intersecting_relationship.md") (qualified-name "IntersectCoverage::Base")))
      (subtype (node (document "memory://snapshot/intersecting_relationship.md") (qualified-name "IntersectCoverage::reading")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/intersecting_relationship.md") (qualified-name "IntersectCoverage::reading")))
      (type (node (document "memory://snapshot/intersecting_relationship.md") (qualified-name "IntersectCoverage::Base")) (provenance authored))
      (effective-type (node (document "memory://snapshot/intersecting_relationship.md") (qualified-name "IntersectCoverage::Base")) (source direct))
      (supertype (node (document "memory://snapshot/intersecting_relationship.md") (qualified-name "IntersectCoverage::Base")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/intersecting_relationship.md") (range (start 4 24) (end 4 28)) (probe (position 4 24))
    (reference (id (source (node (document "memory://snapshot/intersecting_relationship.md") (qualified-name "IntersectCoverage::reading"))) (kind featureTyping) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/intersecting_relationship.md") (qualified-name "IntersectCoverage::Base")))))
    )
  )
  (query (document "memory://snapshot/intersecting_relationship.md") (range (start 4 40) (end 4 41)) (probe (position 4 40))
    (reference (id (source (node (document "memory://snapshot/intersecting_relationship.md") (qualified-name "IntersectCoverage::reading"))) (kind intersects) (ordinal 0) (authored-target "a")
      (outcome (status resolved) (target (node (document "memory://snapshot/intersecting_relationship.md") (qualified-name "IntersectCoverage::a")))))
    )
  )
  (query (document "memory://snapshot/intersecting_relationship.md") (range (start 4 43) (end 4 44)) (probe (position 4 43))
    (reference (id (source (node (document "memory://snapshot/intersecting_relationship.md") (qualified-name "IntersectCoverage::reading"))) (kind intersects) (ordinal 1) (authored-target "b")
      (outcome (status resolved) (target (node (document "memory://snapshot/intersecting_relationship.md") (qualified-name "IntersectCoverage::b")))))
    )
  )
)
~~~
