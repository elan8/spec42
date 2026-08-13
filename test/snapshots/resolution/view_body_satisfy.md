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
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 3 4) (end 5 5))
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
  )
  (references
  )
  (relationships
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
