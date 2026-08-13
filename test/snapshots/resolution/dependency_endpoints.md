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
  (document "memory://snapshot/dependency_endpoints.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 3 4) (end 3 37))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:76777b26487a9109cac088b98f883a3a8b1a01cefb04c5e9f4f8976363db0988") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/dependency_endpoints.md") (qualified-name "DependencyCoverage"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/dependency_endpoints.md") (qualified-name "DependencyCoverage::Source"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/dependency_endpoints.md") (qualified-name "DependencyCoverage::Target"))) (kind part-def) (membership (kind owning) (visibility default)))
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
