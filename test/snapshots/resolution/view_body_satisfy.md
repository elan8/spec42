# META
~~~ini
description=View-body satisfy endpoint resolution coverage
type=file
observed_gap=The view-body satisfy statement is accepted but is not currently published as an authored satisfy reference or relationship.
~~~
# SOURCE
~~~sysml
package ViewCoverage {
    viewpoint def ArchitectureViewpoint;
    view def ArchitectureView;
    view architecture : ArchitectureView {
        satisfy ArchitectureViewpoint;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/view_body_satisfy.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:eff25300bd9aa03e87a896691f7f3b6f12173de710746404dd04345caa4b6108") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::ArchitectureView"))) (kind view-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::ArchitectureViewpoint"))) (kind viewpoint-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::architecture"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ArchitectureView")) (satisfyViewpoint (reference "ArchitectureViewpoint")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::architecture"))) (kind featureTyping) (ordinal 0))
      (authored-target "ArchitectureView")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::ArchitectureView")))))
    (reference (id (source (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::architecture"))) (kind satisfyViewpoint) (ordinal 0))
      (authored-target "ArchitectureViewpoint")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::ArchitectureViewpoint")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::architecture"))) (target (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::ArchitectureView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::architecture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind satisfyViewpoint) (source (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::architecture"))) (target (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::ArchitectureViewpoint"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::architecture"))) (kind satisfyViewpoint) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::architecture")))
      (supertype (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::ArchitectureView")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/view_body_satisfy.md") (range (start 3 24) (end 3 40)) (probe (position 3 24))
    (reference (id (source (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::architecture"))) (kind featureTyping) (ordinal 0) (authored-target "ArchitectureView")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::ArchitectureView")))))
    )
  )
  (query (document "memory://snapshot/view_body_satisfy.md") (range (start 4 16) (end 4 37)) (probe (position 4 16))
    (reference (id (source (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::architecture"))) (kind satisfyViewpoint) (ordinal 0) (authored-target "ArchitectureViewpoint")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::ArchitectureViewpoint")))))
    )
  )
)
~~~
