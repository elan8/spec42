# META
~~~ini
description=SysML Training 37 (Dependencies): Dependency Example
type=file
~~~
# SOURCE
~~~sysml
package 'Dependency Example' {
	
	part 'System Assembly' {
		part 'Computer Subsystem' {
			// ...
		}
		
		part 'Storage Subsystem' {
			// ...
		}
	}
	
	package 'Software Design' {
		item def MessageSchema {
			// ...
		}
		item def DataSchema {
			// ...
		}
	}
	
	dependency from 'System Assembly'::'Computer Subsystem' to 'Software Design';
	
	dependency Schemata 
		from 'System Assembly'::'Storage Subsystem' 
		to 'Software Design'::MessageSchema, 'Software Design'::DataSchema;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/37_dependency_example.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 2 1) (end 10 2))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 3 2) (end 5 3))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 7 2) (end 9 3))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:4839b208edada9d5eee9db58736807e8da58de16f2a620513c8491506e8872e3") (contract-version "semantic-metadata-projection-v6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/37_dependency_example.md") (path (named (kind package) (name "Dependency Example")) (anonymous (kind dependency) (ordinal 0))))) (kind dependency) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (dependencyClient (reference "System Assembly::Computer Subsystem")) (dependencySupplier (reference "Software Design")))))
    (declaration (id (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example::Schemata"))) (kind dependency) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (dependencyClient (reference "System Assembly::Storage Subsystem")) (dependencySupplier (reference "Software Design::MessageSchema")) (dependencySupplier (reference "Software Design::DataSchema")))))
    (declaration (id (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example::Software Design"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example::Software Design::DataSchema"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example::Software Design::MessageSchema"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example::System Assembly"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example::System Assembly::Computer Subsystem"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example::System Assembly::Storage Subsystem"))) (kind part) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/37_dependency_example.md") (path (named (kind package) (name "Dependency Example")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0))
      (authored-target "System Assembly::Computer Subsystem")
      (outcome (status resolved) (target (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example::System Assembly::Computer Subsystem")))))
    (reference (id (source (node (document "memory://snapshot/37_dependency_example.md") (path (named (kind package) (name "Dependency Example")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 0))
      (authored-target "Software Design")
      (outcome (status resolved) (target (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example::Software Design")))))
    (reference (id (source (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example::Schemata"))) (kind dependencyClient) (ordinal 0))
      (authored-target "System Assembly::Storage Subsystem")
      (outcome (status resolved) (target (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example::System Assembly::Storage Subsystem")))))
    (reference (id (source (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example::Schemata"))) (kind dependencySupplier) (ordinal 0))
      (authored-target "Software Design::MessageSchema")
      (outcome (status resolved) (target (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example::Software Design::MessageSchema")))))
    (reference (id (source (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example::Schemata"))) (kind dependencySupplier) (ordinal 1))
      (authored-target "Software Design::DataSchema")
      (outcome (status resolved) (target (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example::Software Design::DataSchema")))))
  )
  (relationships
    (relationship (kind dependencyClient) (source (node (document "memory://snapshot/37_dependency_example.md") (path (named (kind package) (name "Dependency Example")) (anonymous (kind dependency) (ordinal 0))))) (target (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example::System Assembly::Computer Subsystem"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/37_dependency_example.md") (path (named (kind package) (name "Dependency Example")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0)))
    (relationship (kind dependencySupplier) (source (node (document "memory://snapshot/37_dependency_example.md") (path (named (kind package) (name "Dependency Example")) (anonymous (kind dependency) (ordinal 0))))) (target (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example::Software Design"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/37_dependency_example.md") (path (named (kind package) (name "Dependency Example")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 0)))
    (relationship (kind dependencyClient) (source (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example::Schemata"))) (target (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example::System Assembly::Storage Subsystem"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example::Schemata"))) (kind dependencyClient) (ordinal 0)))
    (relationship (kind dependencySupplier) (source (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example::Schemata"))) (target (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example::Software Design::MessageSchema"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example::Schemata"))) (kind dependencySupplier) (ordinal 0)))
    (relationship (kind dependencySupplier) (source (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example::Schemata"))) (target (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example::Software Design::DataSchema"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example::Schemata"))) (kind dependencySupplier) (ordinal 1)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example::System Assembly::Computer Subsystem"))) (target (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example::System Assembly"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example::System Assembly::Storage Subsystem"))) (target (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example::System Assembly"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example::System Assembly::Computer Subsystem")))
      (featured-by (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example::System Assembly")))
    )
    (declaration (id (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example::System Assembly::Storage Subsystem")))
      (featured-by (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example::System Assembly")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/37_dependency_example.md") (range (start 21 17) (end 21 56)) (probe (position 21 17))
    (reference (id (source (node (document "memory://snapshot/37_dependency_example.md") (path (named (kind package) (name "Dependency Example")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0) (authored-target "System Assembly::Computer Subsystem")
      (outcome (status resolved) (target (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example::System Assembly::Computer Subsystem")))))
    )
  )
  (query (document "memory://snapshot/37_dependency_example.md") (range (start 21 60) (end 21 77)) (probe (position 21 60))
    (reference (id (source (node (document "memory://snapshot/37_dependency_example.md") (path (named (kind package) (name "Dependency Example")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 0) (authored-target "Software Design")
      (outcome (status resolved) (target (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example::Software Design")))))
    )
  )
  (query (document "memory://snapshot/37_dependency_example.md") (range (start 24 7) (end 24 45)) (probe (position 24 7))
    (reference (id (source (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example::Schemata"))) (kind dependencyClient) (ordinal 0) (authored-target "System Assembly::Storage Subsystem")
      (outcome (status resolved) (target (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example::System Assembly::Storage Subsystem")))))
    )
  )
  (query (document "memory://snapshot/37_dependency_example.md") (range (start 25 5) (end 25 37)) (probe (position 25 5))
    (reference (id (source (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example::Schemata"))) (kind dependencySupplier) (ordinal 0) (authored-target "Software Design::MessageSchema")
      (outcome (status resolved) (target (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example::Software Design::MessageSchema")))))
    )
  )
  (query (document "memory://snapshot/37_dependency_example.md") (range (start 25 39) (end 25 68)) (probe (position 25 39))
    (reference (id (source (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example::Schemata"))) (kind dependencySupplier) (ordinal 1) (authored-target "Software Design::DataSchema")
      (outcome (status resolved) (target (node (document "memory://snapshot/37_dependency_example.md") (qualified-name "Dependency Example::Software Design::DataSchema")))))
    )
  )
)
~~~
