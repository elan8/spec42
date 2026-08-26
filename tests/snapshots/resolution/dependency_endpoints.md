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
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:76777b26487a9109cac088b98f883a3a8b1a01cefb04c5e9f4f8976363db0988") (contract-version "constructor-expression-specialization-v9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/dependency_endpoints.md") (qualified-name "DependencyCoverage"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/dependency_endpoints.md") (path (named (kind package) (name "DependencyCoverage")) (anonymous (kind dependency) (ordinal 0))))) (kind dependency) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (dependencyClient (reference "Source")) (dependencySupplier (reference "Target")))))
    (declaration (id (node (document "memory://snapshot/dependency_endpoints.md") (qualified-name "DependencyCoverage::Source"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/dependency_endpoints.md") (qualified-name "DependencyCoverage::Target"))) (kind part-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/dependency_endpoints.md") (path (named (kind package) (name "DependencyCoverage")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0))
      (authored-target "Source")
      (outcome (status resolved) (target (node (document "memory://snapshot/dependency_endpoints.md") (qualified-name "DependencyCoverage::Source")))))
    (reference (id (source (node (document "memory://snapshot/dependency_endpoints.md") (path (named (kind package) (name "DependencyCoverage")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 0))
      (authored-target "Target")
      (outcome (status resolved) (target (node (document "memory://snapshot/dependency_endpoints.md") (qualified-name "DependencyCoverage::Target")))))
  )
  (relationships
    (relationship (kind dependencyClient) (source (node (document "memory://snapshot/dependency_endpoints.md") (path (named (kind package) (name "DependencyCoverage")) (anonymous (kind dependency) (ordinal 0))))) (target (node (document "memory://snapshot/dependency_endpoints.md") (qualified-name "DependencyCoverage::Source"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/dependency_endpoints.md") (path (named (kind package) (name "DependencyCoverage")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0)))
    (relationship (kind dependencySupplier) (source (node (document "memory://snapshot/dependency_endpoints.md") (path (named (kind package) (name "DependencyCoverage")) (anonymous (kind dependency) (ordinal 0))))) (target (node (document "memory://snapshot/dependency_endpoints.md") (qualified-name "DependencyCoverage::Target"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/dependency_endpoints.md") (path (named (kind package) (name "DependencyCoverage")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/dependency_endpoints.md") (range (start 3 20) (end 3 26)) (probe (position 3 20))
    (reference (id (source (node (document "memory://snapshot/dependency_endpoints.md") (path (named (kind package) (name "DependencyCoverage")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0) (authored-target "Source")
      (outcome (status resolved) (target (node (document "memory://snapshot/dependency_endpoints.md") (qualified-name "DependencyCoverage::Source")))))
    )
  )
  (query (document "memory://snapshot/dependency_endpoints.md") (range (start 3 30) (end 3 36)) (probe (position 3 30))
    (reference (id (source (node (document "memory://snapshot/dependency_endpoints.md") (path (named (kind package) (name "DependencyCoverage")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 0) (authored-target "Target")
      (outcome (status resolved) (target (node (document "memory://snapshot/dependency_endpoints.md") (qualified-name "DependencyCoverage::Target")))))
    )
  )
)
~~~
