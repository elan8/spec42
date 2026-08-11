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
  (document "view_body_satisfy.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "b4a980d23dd6278730574b3491a081b6fb4c9a808d7f527f8bc4705871bf5424") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ViewCoverage"))) (kind "package") (name "ViewCoverage") (declared-name "ViewCoverage"))
    (element (id (node (document "d0") (qualified-name "ViewCoverage::ArchitectureView"))) (kind "view def") (name "ArchitectureView") (declared-name "ArchitectureView") (parent (node (document "d0") (qualified-name "ViewCoverage"))))
    (element (id (node (document "d0") (qualified-name "ViewCoverage::ArchitectureViewpoint"))) (kind "viewpoint def") (name "ArchitectureViewpoint") (declared-name "ArchitectureViewpoint") (parent (node (document "d0") (qualified-name "ViewCoverage"))))
    (element (id (node (document "d0") (qualified-name "ViewCoverage::architecture"))) (kind "view") (name "architecture") (declared-name "architecture") (parent (node (document "d0") (qualified-name "ViewCoverage"))) (authored (membership (kind Feature)) (relationships (typing (reference "ArchitectureView")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ViewCoverage::architecture"))) (kind featureTyping) (ordinal 0)) (authored-target "ArchitectureView") (outcome (status resolved) (target (node (document "d0") (qualified-name "ViewCoverage::ArchitectureView")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ViewCoverage::architecture"))) (target (node (document "d0") (qualified-name "ViewCoverage::ArchitectureView"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ViewCoverage::architecture"))) (kind featureTyping) (ordinal 0)))
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
