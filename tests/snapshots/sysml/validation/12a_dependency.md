# META
~~~ini
description=SysML Validation (12-Dependency Relationships): 12a-Dependency
type=file
~~~
# SOURCE
~~~sysml
package '12a-Dependency' {
	
	package 'Application Layer';
	package 'Service Layer';
	package 'Data Layer';
	
	dependency Use from 'Application Layer' to 'Service Layer';
	dependency from 'Service Layer' to 'Data Layer';
	
	attribute x;
	attribute y;
	attribute z;
	
	dependency z to x, y;
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/12a_dependency.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:b98fc60512e7c7032f2ce12083bf62b01a2242d80f6c0ad80795e05165aeb776") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/12a_dependency.md") (qualified-name "12a-Dependency"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12a_dependency.md") (path (named (kind package) (name "12a-Dependency")) (anonymous (kind dependency) (ordinal 0))))) (kind dependency) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (dependencyClient (reference "Service Layer")) (dependencySupplier (reference "Data Layer")))))
    (declaration (id (node (document "memory://snapshot/12a_dependency.md") (path (named (kind package) (name "12a-Dependency")) (anonymous (kind dependency) (ordinal 1))))) (kind dependency) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (dependencyClient (reference "z")) (dependencySupplier (reference "x")) (dependencySupplier (reference "y")))))
    (declaration (id (node (document "memory://snapshot/12a_dependency.md") (qualified-name "12a-Dependency::Application Layer"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12a_dependency.md") (qualified-name "12a-Dependency::Data Layer"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12a_dependency.md") (qualified-name "12a-Dependency::Service Layer"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12a_dependency.md") (qualified-name "12a-Dependency::Use"))) (kind dependency) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (dependencyClient (reference "Application Layer")) (dependencySupplier (reference "Service Layer")))))
    (declaration (id (node (document "memory://snapshot/12a_dependency.md") (qualified-name "12a-Dependency::x"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12a_dependency.md") (qualified-name "12a-Dependency::y"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12a_dependency.md") (qualified-name "12a-Dependency::z"))) (kind attribute) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/12a_dependency.md") (path (named (kind package) (name "12a-Dependency")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0))
      (authored-target "Service Layer")
      (outcome (status resolved) (target (node (document "memory://snapshot/12a_dependency.md") (qualified-name "12a-Dependency::Service Layer")))))
    (reference (id (source (node (document "memory://snapshot/12a_dependency.md") (path (named (kind package) (name "12a-Dependency")) (anonymous (kind dependency) (ordinal 1))))) (kind dependencyClient) (ordinal 0))
      (authored-target "z")
      (outcome (status resolved) (target (node (document "memory://snapshot/12a_dependency.md") (qualified-name "12a-Dependency::z")))))
    (reference (id (source (node (document "memory://snapshot/12a_dependency.md") (path (named (kind package) (name "12a-Dependency")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 0))
      (authored-target "Data Layer")
      (outcome (status resolved) (target (node (document "memory://snapshot/12a_dependency.md") (qualified-name "12a-Dependency::Data Layer")))))
    (reference (id (source (node (document "memory://snapshot/12a_dependency.md") (path (named (kind package) (name "12a-Dependency")) (anonymous (kind dependency) (ordinal 1))))) (kind dependencySupplier) (ordinal 0))
      (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/12a_dependency.md") (qualified-name "12a-Dependency::x")))))
    (reference (id (source (node (document "memory://snapshot/12a_dependency.md") (path (named (kind package) (name "12a-Dependency")) (anonymous (kind dependency) (ordinal 1))))) (kind dependencySupplier) (ordinal 1))
      (authored-target "y")
      (outcome (status resolved) (target (node (document "memory://snapshot/12a_dependency.md") (qualified-name "12a-Dependency::y")))))
    (reference (id (source (node (document "memory://snapshot/12a_dependency.md") (qualified-name "12a-Dependency::Use"))) (kind dependencyClient) (ordinal 0))
      (authored-target "Application Layer")
      (outcome (status resolved) (target (node (document "memory://snapshot/12a_dependency.md") (qualified-name "12a-Dependency::Application Layer")))))
    (reference (id (source (node (document "memory://snapshot/12a_dependency.md") (qualified-name "12a-Dependency::Use"))) (kind dependencySupplier) (ordinal 0))
      (authored-target "Service Layer")
      (outcome (status resolved) (target (node (document "memory://snapshot/12a_dependency.md") (qualified-name "12a-Dependency::Service Layer")))))
  )
  (relationships
    (relationship (kind dependencyClient) (source (node (document "memory://snapshot/12a_dependency.md") (path (named (kind package) (name "12a-Dependency")) (anonymous (kind dependency) (ordinal 0))))) (target (node (document "memory://snapshot/12a_dependency.md") (qualified-name "12a-Dependency::Service Layer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/12a_dependency.md") (path (named (kind package) (name "12a-Dependency")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0)))
    (relationship (kind dependencyClient) (source (node (document "memory://snapshot/12a_dependency.md") (path (named (kind package) (name "12a-Dependency")) (anonymous (kind dependency) (ordinal 1))))) (target (node (document "memory://snapshot/12a_dependency.md") (qualified-name "12a-Dependency::z"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/12a_dependency.md") (path (named (kind package) (name "12a-Dependency")) (anonymous (kind dependency) (ordinal 1))))) (kind dependencyClient) (ordinal 0)))
    (relationship (kind dependencySupplier) (source (node (document "memory://snapshot/12a_dependency.md") (path (named (kind package) (name "12a-Dependency")) (anonymous (kind dependency) (ordinal 0))))) (target (node (document "memory://snapshot/12a_dependency.md") (qualified-name "12a-Dependency::Data Layer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/12a_dependency.md") (path (named (kind package) (name "12a-Dependency")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 0)))
    (relationship (kind dependencySupplier) (source (node (document "memory://snapshot/12a_dependency.md") (path (named (kind package) (name "12a-Dependency")) (anonymous (kind dependency) (ordinal 1))))) (target (node (document "memory://snapshot/12a_dependency.md") (qualified-name "12a-Dependency::x"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/12a_dependency.md") (path (named (kind package) (name "12a-Dependency")) (anonymous (kind dependency) (ordinal 1))))) (kind dependencySupplier) (ordinal 0)))
    (relationship (kind dependencySupplier) (source (node (document "memory://snapshot/12a_dependency.md") (path (named (kind package) (name "12a-Dependency")) (anonymous (kind dependency) (ordinal 1))))) (target (node (document "memory://snapshot/12a_dependency.md") (qualified-name "12a-Dependency::y"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/12a_dependency.md") (path (named (kind package) (name "12a-Dependency")) (anonymous (kind dependency) (ordinal 1))))) (kind dependencySupplier) (ordinal 1)))
    (relationship (kind dependencyClient) (source (node (document "memory://snapshot/12a_dependency.md") (qualified-name "12a-Dependency::Use"))) (target (node (document "memory://snapshot/12a_dependency.md") (qualified-name "12a-Dependency::Application Layer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/12a_dependency.md") (qualified-name "12a-Dependency::Use"))) (kind dependencyClient) (ordinal 0)))
    (relationship (kind dependencySupplier) (source (node (document "memory://snapshot/12a_dependency.md") (qualified-name "12a-Dependency::Use"))) (target (node (document "memory://snapshot/12a_dependency.md") (qualified-name "12a-Dependency::Service Layer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/12a_dependency.md") (qualified-name "12a-Dependency::Use"))) (kind dependencySupplier) (ordinal 0)))
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
  (query (document "memory://snapshot/12a_dependency.md") (range (start 7 17) (end 7 32)) (probe (position 7 17))
    (reference (id (source (node (document "memory://snapshot/12a_dependency.md") (path (named (kind package) (name "12a-Dependency")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0) (authored-target "Service Layer")
      (outcome (status resolved) (target (node (document "memory://snapshot/12a_dependency.md") (qualified-name "12a-Dependency::Service Layer")))))
    )
  )
  (query (document "memory://snapshot/12a_dependency.md") (range (start 13 12) (end 13 13)) (probe (position 13 12))
    (reference (id (source (node (document "memory://snapshot/12a_dependency.md") (path (named (kind package) (name "12a-Dependency")) (anonymous (kind dependency) (ordinal 1))))) (kind dependencyClient) (ordinal 0) (authored-target "z")
      (outcome (status resolved) (target (node (document "memory://snapshot/12a_dependency.md") (qualified-name "12a-Dependency::z")))))
    )
  )
  (query (document "memory://snapshot/12a_dependency.md") (range (start 7 36) (end 7 48)) (probe (position 7 36))
    (reference (id (source (node (document "memory://snapshot/12a_dependency.md") (path (named (kind package) (name "12a-Dependency")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 0) (authored-target "Data Layer")
      (outcome (status resolved) (target (node (document "memory://snapshot/12a_dependency.md") (qualified-name "12a-Dependency::Data Layer")))))
    )
  )
  (query (document "memory://snapshot/12a_dependency.md") (range (start 13 17) (end 13 18)) (probe (position 13 17))
    (reference (id (source (node (document "memory://snapshot/12a_dependency.md") (path (named (kind package) (name "12a-Dependency")) (anonymous (kind dependency) (ordinal 1))))) (kind dependencySupplier) (ordinal 0) (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/12a_dependency.md") (qualified-name "12a-Dependency::x")))))
    )
  )
  (query (document "memory://snapshot/12a_dependency.md") (range (start 13 20) (end 13 21)) (probe (position 13 20))
    (reference (id (source (node (document "memory://snapshot/12a_dependency.md") (path (named (kind package) (name "12a-Dependency")) (anonymous (kind dependency) (ordinal 1))))) (kind dependencySupplier) (ordinal 1) (authored-target "y")
      (outcome (status resolved) (target (node (document "memory://snapshot/12a_dependency.md") (qualified-name "12a-Dependency::y")))))
    )
  )
  (query (document "memory://snapshot/12a_dependency.md") (range (start 6 21) (end 6 40)) (probe (position 6 21))
    (reference (id (source (node (document "memory://snapshot/12a_dependency.md") (qualified-name "12a-Dependency::Use"))) (kind dependencyClient) (ordinal 0) (authored-target "Application Layer")
      (outcome (status resolved) (target (node (document "memory://snapshot/12a_dependency.md") (qualified-name "12a-Dependency::Application Layer")))))
    )
  )
  (query (document "memory://snapshot/12a_dependency.md") (range (start 6 44) (end 6 59)) (probe (position 6 44))
    (reference (id (source (node (document "memory://snapshot/12a_dependency.md") (qualified-name "12a-Dependency::Use"))) (kind dependencySupplier) (ordinal 0) (authored-target "Service Layer")
      (outcome (status resolved) (target (node (document "memory://snapshot/12a_dependency.md") (qualified-name "12a-Dependency::Service Layer")))))
    )
  )
)
~~~
