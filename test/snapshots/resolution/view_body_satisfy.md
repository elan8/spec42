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
      (diagnostic
        (severity warning)
        (code "unsupported_view_definition_member")
        (source "semantic")
        (range (start 4 8) (end 4 38))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:eff25300bd9aa03e87a896691f7f3b6f12173de710746404dd04345caa4b6108") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::ArchitectureView"))) (kind view-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::ArchitectureViewpoint"))) (kind viewpoint-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::architecture"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ArchitectureView"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::architecture"))) (kind featureTyping) (ordinal 0))
      (authored-target "ArchitectureView")
      (outcome (status resolved) (target (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::ArchitectureView")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::architecture"))) (target (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::ArchitectureView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/view_body_satisfy.md") (qualified-name "ViewCoverage::architecture"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
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
~~~
