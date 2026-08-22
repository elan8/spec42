# META
~~~ini
description=SysML Example (Simple Tests): DependencyTest
type=file
~~~
# SOURCE
~~~sysml
package DependencyTest {
	
	package System {
		package 'Application Layer';
		package 'Service Layer';
		package 'Data Layer';
	}
	
	private import System::*;
	
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
  (document "memory://snapshot/dependency_test.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:2dea2a5d07ff7629067141fd56c59465f8628bf8e3c84838c935b7a6707fb0be") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/dependency_test.md") (qualified-name "DependencyTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/dependency_test.md") (path (named (kind package) (name "DependencyTest")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "System") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/dependency_test.md") (path (named (kind package) (name "DependencyTest")) (anonymous (kind dependency) (ordinal 0))))) (kind dependency) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (dependencyClient (reference "Service Layer")) (dependencySupplier (reference "Data Layer")))))
    (declaration (id (node (document "memory://snapshot/dependency_test.md") (path (named (kind package) (name "DependencyTest")) (anonymous (kind dependency) (ordinal 1))))) (kind dependency) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (dependencyClient (reference "z")) (dependencySupplier (reference "x")) (dependencySupplier (reference "y")))))
    (declaration (id (node (document "memory://snapshot/dependency_test.md") (qualified-name "DependencyTest::System"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/dependency_test.md") (qualified-name "DependencyTest::System::Application Layer"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/dependency_test.md") (qualified-name "DependencyTest::System::Data Layer"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/dependency_test.md") (qualified-name "DependencyTest::System::Service Layer"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/dependency_test.md") (qualified-name "DependencyTest::Use"))) (kind dependency) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (dependencyClient (reference "Application Layer")) (dependencySupplier (reference "Service Layer")))))
    (declaration (id (node (document "memory://snapshot/dependency_test.md") (qualified-name "DependencyTest::x"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/dependency_test.md") (qualified-name "DependencyTest::y"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/dependency_test.md") (qualified-name "DependencyTest::z"))) (kind attribute) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/dependency_test.md") (path (named (kind package) (name "DependencyTest")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "System")
      (outcome (status resolved) (target (node (document "memory://snapshot/dependency_test.md") (qualified-name "DependencyTest::System")))))
    (reference (id (source (node (document "memory://snapshot/dependency_test.md") (path (named (kind package) (name "DependencyTest")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0))
      (authored-target "Service Layer")
      (outcome (status resolved) (target (node (document "memory://snapshot/dependency_test.md") (qualified-name "DependencyTest::System::Service Layer")))))
    (reference (id (source (node (document "memory://snapshot/dependency_test.md") (path (named (kind package) (name "DependencyTest")) (anonymous (kind dependency) (ordinal 1))))) (kind dependencyClient) (ordinal 0))
      (authored-target "z")
      (outcome (status resolved) (target (node (document "memory://snapshot/dependency_test.md") (qualified-name "DependencyTest::z")))))
    (reference (id (source (node (document "memory://snapshot/dependency_test.md") (path (named (kind package) (name "DependencyTest")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 0))
      (authored-target "Data Layer")
      (outcome (status resolved) (target (node (document "memory://snapshot/dependency_test.md") (qualified-name "DependencyTest::System::Data Layer")))))
    (reference (id (source (node (document "memory://snapshot/dependency_test.md") (path (named (kind package) (name "DependencyTest")) (anonymous (kind dependency) (ordinal 1))))) (kind dependencySupplier) (ordinal 0))
      (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/dependency_test.md") (qualified-name "DependencyTest::x")))))
    (reference (id (source (node (document "memory://snapshot/dependency_test.md") (path (named (kind package) (name "DependencyTest")) (anonymous (kind dependency) (ordinal 1))))) (kind dependencySupplier) (ordinal 1))
      (authored-target "y")
      (outcome (status resolved) (target (node (document "memory://snapshot/dependency_test.md") (qualified-name "DependencyTest::y")))))
    (reference (id (source (node (document "memory://snapshot/dependency_test.md") (qualified-name "DependencyTest::Use"))) (kind dependencyClient) (ordinal 0))
      (authored-target "Application Layer")
      (outcome (status resolved) (target (node (document "memory://snapshot/dependency_test.md") (qualified-name "DependencyTest::System::Application Layer")))))
    (reference (id (source (node (document "memory://snapshot/dependency_test.md") (qualified-name "DependencyTest::Use"))) (kind dependencySupplier) (ordinal 0))
      (authored-target "Service Layer")
      (outcome (status resolved) (target (node (document "memory://snapshot/dependency_test.md") (qualified-name "DependencyTest::System::Service Layer")))))
  )
  (relationships
    (relationship (kind dependencyClient) (source (node (document "memory://snapshot/dependency_test.md") (path (named (kind package) (name "DependencyTest")) (anonymous (kind dependency) (ordinal 0))))) (target (node (document "memory://snapshot/dependency_test.md") (qualified-name "DependencyTest::System::Service Layer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/dependency_test.md") (path (named (kind package) (name "DependencyTest")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0)))
    (relationship (kind dependencyClient) (source (node (document "memory://snapshot/dependency_test.md") (path (named (kind package) (name "DependencyTest")) (anonymous (kind dependency) (ordinal 1))))) (target (node (document "memory://snapshot/dependency_test.md") (qualified-name "DependencyTest::z"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/dependency_test.md") (path (named (kind package) (name "DependencyTest")) (anonymous (kind dependency) (ordinal 1))))) (kind dependencyClient) (ordinal 0)))
    (relationship (kind dependencySupplier) (source (node (document "memory://snapshot/dependency_test.md") (path (named (kind package) (name "DependencyTest")) (anonymous (kind dependency) (ordinal 0))))) (target (node (document "memory://snapshot/dependency_test.md") (qualified-name "DependencyTest::System::Data Layer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/dependency_test.md") (path (named (kind package) (name "DependencyTest")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 0)))
    (relationship (kind dependencySupplier) (source (node (document "memory://snapshot/dependency_test.md") (path (named (kind package) (name "DependencyTest")) (anonymous (kind dependency) (ordinal 1))))) (target (node (document "memory://snapshot/dependency_test.md") (qualified-name "DependencyTest::x"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/dependency_test.md") (path (named (kind package) (name "DependencyTest")) (anonymous (kind dependency) (ordinal 1))))) (kind dependencySupplier) (ordinal 0)))
    (relationship (kind dependencySupplier) (source (node (document "memory://snapshot/dependency_test.md") (path (named (kind package) (name "DependencyTest")) (anonymous (kind dependency) (ordinal 1))))) (target (node (document "memory://snapshot/dependency_test.md") (qualified-name "DependencyTest::y"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/dependency_test.md") (path (named (kind package) (name "DependencyTest")) (anonymous (kind dependency) (ordinal 1))))) (kind dependencySupplier) (ordinal 1)))
    (relationship (kind dependencyClient) (source (node (document "memory://snapshot/dependency_test.md") (qualified-name "DependencyTest::Use"))) (target (node (document "memory://snapshot/dependency_test.md") (qualified-name "DependencyTest::System::Application Layer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/dependency_test.md") (qualified-name "DependencyTest::Use"))) (kind dependencyClient) (ordinal 0)))
    (relationship (kind dependencySupplier) (source (node (document "memory://snapshot/dependency_test.md") (qualified-name "DependencyTest::Use"))) (target (node (document "memory://snapshot/dependency_test.md") (qualified-name "DependencyTest::System::Service Layer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/dependency_test.md") (qualified-name "DependencyTest::Use"))) (kind dependencySupplier) (ordinal 0)))
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
  (query (document "memory://snapshot/dependency_test.md") (range (start 8 16) (end 8 25)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/dependency_test.md") (path (named (kind package) (name "DependencyTest")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "System")
      (outcome (status resolved) (target (node (document "memory://snapshot/dependency_test.md") (qualified-name "DependencyTest::System")))))
    )
  )
  (query (document "memory://snapshot/dependency_test.md") (range (start 11 17) (end 11 32)) (probe (position 11 17))
    (reference (id (source (node (document "memory://snapshot/dependency_test.md") (path (named (kind package) (name "DependencyTest")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0) (authored-target "Service Layer")
      (outcome (status resolved) (target (node (document "memory://snapshot/dependency_test.md") (qualified-name "DependencyTest::System::Service Layer")))))
    )
  )
  (query (document "memory://snapshot/dependency_test.md") (range (start 17 12) (end 17 13)) (probe (position 17 12))
    (reference (id (source (node (document "memory://snapshot/dependency_test.md") (path (named (kind package) (name "DependencyTest")) (anonymous (kind dependency) (ordinal 1))))) (kind dependencyClient) (ordinal 0) (authored-target "z")
      (outcome (status resolved) (target (node (document "memory://snapshot/dependency_test.md") (qualified-name "DependencyTest::z")))))
    )
  )
  (query (document "memory://snapshot/dependency_test.md") (range (start 11 36) (end 11 48)) (probe (position 11 36))
    (reference (id (source (node (document "memory://snapshot/dependency_test.md") (path (named (kind package) (name "DependencyTest")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 0) (authored-target "Data Layer")
      (outcome (status resolved) (target (node (document "memory://snapshot/dependency_test.md") (qualified-name "DependencyTest::System::Data Layer")))))
    )
  )
  (query (document "memory://snapshot/dependency_test.md") (range (start 17 17) (end 17 18)) (probe (position 17 17))
    (reference (id (source (node (document "memory://snapshot/dependency_test.md") (path (named (kind package) (name "DependencyTest")) (anonymous (kind dependency) (ordinal 1))))) (kind dependencySupplier) (ordinal 0) (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/dependency_test.md") (qualified-name "DependencyTest::x")))))
    )
  )
  (query (document "memory://snapshot/dependency_test.md") (range (start 17 20) (end 17 21)) (probe (position 17 20))
    (reference (id (source (node (document "memory://snapshot/dependency_test.md") (path (named (kind package) (name "DependencyTest")) (anonymous (kind dependency) (ordinal 1))))) (kind dependencySupplier) (ordinal 1) (authored-target "y")
      (outcome (status resolved) (target (node (document "memory://snapshot/dependency_test.md") (qualified-name "DependencyTest::y")))))
    )
  )
  (query (document "memory://snapshot/dependency_test.md") (range (start 10 21) (end 10 40)) (probe (position 10 21))
    (reference (id (source (node (document "memory://snapshot/dependency_test.md") (qualified-name "DependencyTest::Use"))) (kind dependencyClient) (ordinal 0) (authored-target "Application Layer")
      (outcome (status resolved) (target (node (document "memory://snapshot/dependency_test.md") (qualified-name "DependencyTest::System::Application Layer")))))
    )
  )
  (query (document "memory://snapshot/dependency_test.md") (range (start 10 44) (end 10 59)) (probe (position 10 44))
    (reference (id (source (node (document "memory://snapshot/dependency_test.md") (qualified-name "DependencyTest::Use"))) (kind dependencySupplier) (ordinal 0) (authored-target "Service Layer")
      (outcome (status resolved) (target (node (document "memory://snapshot/dependency_test.md") (qualified-name "DependencyTest::System::Service Layer")))))
    )
  )
)
~~~
