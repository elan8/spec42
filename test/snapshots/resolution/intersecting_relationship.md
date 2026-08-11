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
  (document "intersecting_relationship.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "147f0e52537207465e0ab29a327bcaa7c37762327fef6a9d61def40455dc1305") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "IntersectCoverage"))) (kind "package") (name "IntersectCoverage") (declared-name "IntersectCoverage"))
    (element (id (node (document "d0") (qualified-name "IntersectCoverage::Base"))) (kind "part def") (name "Base") (declared-name "Base") (parent (node (document "d0") (qualified-name "IntersectCoverage"))))
    (element (id (node (document "d0") (qualified-name "IntersectCoverage::a"))) (kind "attribute def") (name "a") (declared-name "a") (parent (node (document "d0") (qualified-name "IntersectCoverage"))))
    (element (id (node (document "d0") (qualified-name "IntersectCoverage::b"))) (kind "attribute def") (name "b") (declared-name "b") (parent (node (document "d0") (qualified-name "IntersectCoverage"))))
    (element (id (node (document "d0") (qualified-name "IntersectCoverage::reading"))) (kind "attribute def") (name "reading") (declared-name "reading") (parent (node (document "d0") (qualified-name "IntersectCoverage"))) (authored (membership (kind Owning)) (relationships (typing (reference "Base")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "IntersectCoverage::reading"))) (kind featureTyping) (ordinal 0)) (authored-target "Base") (outcome (status resolved) (target (node (document "d0") (qualified-name "IntersectCoverage::Base")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "IntersectCoverage::reading"))) (target (node (document "d0") (qualified-name "IntersectCoverage::Base"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "IntersectCoverage::reading"))) (kind featureTyping) (ordinal 0)))
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
