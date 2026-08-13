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
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:5636759edf98aecf2c19163942f8f9453be470a3a2d2c0fff33e63f6ddef1ea8") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/intersecting_relationship.md") (qualified-name "IntersectCoverage"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/intersecting_relationship.md") (qualified-name "IntersectCoverage::Base"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/intersecting_relationship.md") (qualified-name "IntersectCoverage::a"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/intersecting_relationship.md") (qualified-name "IntersectCoverage::b"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/intersecting_relationship.md") (qualified-name "IntersectCoverage::reading"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "Base"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/intersecting_relationship.md") (qualified-name "IntersectCoverage::reading"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/intersecting_relationship.md") (qualified-name "IntersectCoverage::Base")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/intersecting_relationship.md") (qualified-name "IntersectCoverage::reading"))) (target (node (document "memory://snapshot/intersecting_relationship.md") (qualified-name "IntersectCoverage::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/intersecting_relationship.md") (qualified-name "IntersectCoverage::reading"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
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
~~~
