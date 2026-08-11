# META
~~~ini
description=Dependency endpoint resolution coverage
type=file
observed_gap=The dependency declaration is admitted as a dependency element, but its source and target endpoints are not published as authored references or relationships.
~~~
# SOURCE
~~~sysml
package DependencyCoverage {
    part def Source;
    part def Target;
    dependency from Source to Target;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "dependency_endpoints.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "27187138feadae208a5dcf65a51f6d67b652e974bf50a4ec19ddbdd0f7f6c248") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "DependencyCoverage"))) (kind "package") (name "DependencyCoverage") (declared-name "DependencyCoverage"))
    (element (id (node (document "d0") (qualified-name "DependencyCoverage::Source"))) (kind "part def") (name "Source") (declared-name "Source") (parent (node (document "d0") (qualified-name "DependencyCoverage"))))
    (element (id (node (document "d0") (qualified-name "DependencyCoverage::Target"))) (kind "part def") (name "Target") (declared-name "Target") (parent (node (document "d0") (qualified-name "DependencyCoverage"))))
    (element (id (node (document "d0") (qualified-name "DependencyCoverage::dependency"))) (kind "dependency") (name "dependency") (declared-name "dependency") (parent (node (document "d0") (qualified-name "DependencyCoverage"))))
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
