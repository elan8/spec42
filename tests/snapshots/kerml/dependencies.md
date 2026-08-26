# META
~~~ini
description=KerML Simple Tests: Dependencies
type=file
~~~
# SOURCE
~~~kerml
package Dependencies {
	
	package System {
		package 'Application Layer';
		package 'Service Layer';
		package 'Data Layer';
	}
	
	public import System::*;
	
	dependency Use from 'Application Layer' to 'Service Layer';
	dependency from 'Service Layer' to 'Data Layer';
	
	feature x;
	feature y;
	feature z;
	
	dependency z to x, y {
		feature e;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/dependencies.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:526982ee6349689d97406de6cf14dd886e218a1139374ed9119523131c81bbb0") (contract-version "owned-cross-feature-typing-v4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/dependencies.md") (qualified-name "Dependencies"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/dependencies.md") (path (named (kind package) (name "Dependencies")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "System") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/dependencies.md") (path (named (kind package) (name "Dependencies")) (anonymous (kind dependency) (ordinal 0))))) (kind dependency) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (dependencyClient (reference "Service Layer")) (dependencySupplier (reference "Data Layer")))))
    (declaration (id (node (document "memory://snapshot/dependencies.md") (path (named (kind package) (name "Dependencies")) (anonymous (kind dependency) (ordinal 1))))) (kind dependency) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (dependencyClient (reference "z")) (dependencySupplier (reference "x")) (dependencySupplier (reference "y")))))
    (declaration (id (node (document "memory://snapshot/dependencies.md") (path (named (kind package) (name "Dependencies")) (anonymous (kind dependency) (ordinal 1)) (named (kind kerml-feature) (name "e"))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/dependencies.md") (qualified-name "Dependencies::System"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/dependencies.md") (qualified-name "Dependencies::System::Application Layer"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/dependencies.md") (qualified-name "Dependencies::System::Data Layer"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/dependencies.md") (qualified-name "Dependencies::System::Service Layer"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/dependencies.md") (qualified-name "Dependencies::Use"))) (kind dependency) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (dependencyClient (reference "Application Layer")) (dependencySupplier (reference "Service Layer")))))
    (declaration (id (node (document "memory://snapshot/dependencies.md") (qualified-name "Dependencies::x"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/dependencies.md") (qualified-name "Dependencies::y"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/dependencies.md") (qualified-name "Dependencies::z"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/dependencies.md") (path (named (kind package) (name "Dependencies")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "System")
      (outcome (status resolved) (target (node (document "memory://snapshot/dependencies.md") (qualified-name "Dependencies::System")))))
    (reference (id (source (node (document "memory://snapshot/dependencies.md") (path (named (kind package) (name "Dependencies")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0))
      (authored-target "Service Layer")
      (outcome (status resolved) (target (node (document "memory://snapshot/dependencies.md") (qualified-name "Dependencies::System::Service Layer")))))
    (reference (id (source (node (document "memory://snapshot/dependencies.md") (path (named (kind package) (name "Dependencies")) (anonymous (kind dependency) (ordinal 1))))) (kind dependencyClient) (ordinal 0))
      (authored-target "z")
      (outcome (status resolved) (target (node (document "memory://snapshot/dependencies.md") (qualified-name "Dependencies::z")))))
    (reference (id (source (node (document "memory://snapshot/dependencies.md") (path (named (kind package) (name "Dependencies")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 0))
      (authored-target "Data Layer")
      (outcome (status resolved) (target (node (document "memory://snapshot/dependencies.md") (qualified-name "Dependencies::System::Data Layer")))))
    (reference (id (source (node (document "memory://snapshot/dependencies.md") (path (named (kind package) (name "Dependencies")) (anonymous (kind dependency) (ordinal 1))))) (kind dependencySupplier) (ordinal 0))
      (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/dependencies.md") (qualified-name "Dependencies::x")))))
    (reference (id (source (node (document "memory://snapshot/dependencies.md") (path (named (kind package) (name "Dependencies")) (anonymous (kind dependency) (ordinal 1))))) (kind dependencySupplier) (ordinal 1))
      (authored-target "y")
      (outcome (status resolved) (target (node (document "memory://snapshot/dependencies.md") (qualified-name "Dependencies::y")))))
    (reference (id (source (node (document "memory://snapshot/dependencies.md") (qualified-name "Dependencies::Use"))) (kind dependencyClient) (ordinal 0))
      (authored-target "Application Layer")
      (outcome (status resolved) (target (node (document "memory://snapshot/dependencies.md") (qualified-name "Dependencies::System::Application Layer")))))
    (reference (id (source (node (document "memory://snapshot/dependencies.md") (qualified-name "Dependencies::Use"))) (kind dependencySupplier) (ordinal 0))
      (authored-target "Service Layer")
      (outcome (status resolved) (target (node (document "memory://snapshot/dependencies.md") (qualified-name "Dependencies::System::Service Layer")))))
  )
  (relationships
    (relationship (kind dependencyClient) (source (node (document "memory://snapshot/dependencies.md") (path (named (kind package) (name "Dependencies")) (anonymous (kind dependency) (ordinal 0))))) (target (node (document "memory://snapshot/dependencies.md") (qualified-name "Dependencies::System::Service Layer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/dependencies.md") (path (named (kind package) (name "Dependencies")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0)))
    (relationship (kind dependencyClient) (source (node (document "memory://snapshot/dependencies.md") (path (named (kind package) (name "Dependencies")) (anonymous (kind dependency) (ordinal 1))))) (target (node (document "memory://snapshot/dependencies.md") (qualified-name "Dependencies::z"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/dependencies.md") (path (named (kind package) (name "Dependencies")) (anonymous (kind dependency) (ordinal 1))))) (kind dependencyClient) (ordinal 0)))
    (relationship (kind dependencySupplier) (source (node (document "memory://snapshot/dependencies.md") (path (named (kind package) (name "Dependencies")) (anonymous (kind dependency) (ordinal 0))))) (target (node (document "memory://snapshot/dependencies.md") (qualified-name "Dependencies::System::Data Layer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/dependencies.md") (path (named (kind package) (name "Dependencies")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 0)))
    (relationship (kind dependencySupplier) (source (node (document "memory://snapshot/dependencies.md") (path (named (kind package) (name "Dependencies")) (anonymous (kind dependency) (ordinal 1))))) (target (node (document "memory://snapshot/dependencies.md") (qualified-name "Dependencies::x"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/dependencies.md") (path (named (kind package) (name "Dependencies")) (anonymous (kind dependency) (ordinal 1))))) (kind dependencySupplier) (ordinal 0)))
    (relationship (kind dependencySupplier) (source (node (document "memory://snapshot/dependencies.md") (path (named (kind package) (name "Dependencies")) (anonymous (kind dependency) (ordinal 1))))) (target (node (document "memory://snapshot/dependencies.md") (qualified-name "Dependencies::y"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/dependencies.md") (path (named (kind package) (name "Dependencies")) (anonymous (kind dependency) (ordinal 1))))) (kind dependencySupplier) (ordinal 1)))
    (relationship (kind dependencyClient) (source (node (document "memory://snapshot/dependencies.md") (qualified-name "Dependencies::Use"))) (target (node (document "memory://snapshot/dependencies.md") (qualified-name "Dependencies::System::Application Layer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/dependencies.md") (qualified-name "Dependencies::Use"))) (kind dependencyClient) (ordinal 0)))
    (relationship (kind dependencySupplier) (source (node (document "memory://snapshot/dependencies.md") (qualified-name "Dependencies::Use"))) (target (node (document "memory://snapshot/dependencies.md") (qualified-name "Dependencies::System::Service Layer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/dependencies.md") (qualified-name "Dependencies::Use"))) (kind dependencySupplier) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/dependencies.md") (path (named (kind package) (name "Dependencies")) (anonymous (kind dependency) (ordinal 1)) (named (kind kerml-feature) (name "e"))))) (target (node (document "memory://snapshot/dependencies.md") (path (named (kind package) (name "Dependencies")) (anonymous (kind dependency) (ordinal 1))))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/dependencies.md") (path (named (kind package) (name "Dependencies")) (anonymous (kind dependency) (ordinal 1)) (named (kind kerml-feature) (name "e")))))
      (featured-by (node (document "memory://snapshot/dependencies.md") (path (named (kind package) (name "Dependencies")) (anonymous (kind dependency) (ordinal 1)))))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/dependencies.md") (range (start 8 15) (end 8 24)) (probe (position 8 15))
    (reference (id (source (node (document "memory://snapshot/dependencies.md") (path (named (kind package) (name "Dependencies")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "System")
      (outcome (status resolved) (target (node (document "memory://snapshot/dependencies.md") (qualified-name "Dependencies::System")))))
    )
  )
  (query (document "memory://snapshot/dependencies.md") (range (start 11 17) (end 11 32)) (probe (position 11 17))
    (reference (id (source (node (document "memory://snapshot/dependencies.md") (path (named (kind package) (name "Dependencies")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0) (authored-target "Service Layer")
      (outcome (status resolved) (target (node (document "memory://snapshot/dependencies.md") (qualified-name "Dependencies::System::Service Layer")))))
    )
  )
  (query (document "memory://snapshot/dependencies.md") (range (start 17 12) (end 17 13)) (probe (position 17 12))
    (reference (id (source (node (document "memory://snapshot/dependencies.md") (path (named (kind package) (name "Dependencies")) (anonymous (kind dependency) (ordinal 1))))) (kind dependencyClient) (ordinal 0) (authored-target "z")
      (outcome (status resolved) (target (node (document "memory://snapshot/dependencies.md") (qualified-name "Dependencies::z")))))
    )
  )
  (query (document "memory://snapshot/dependencies.md") (range (start 11 36) (end 11 48)) (probe (position 11 36))
    (reference (id (source (node (document "memory://snapshot/dependencies.md") (path (named (kind package) (name "Dependencies")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 0) (authored-target "Data Layer")
      (outcome (status resolved) (target (node (document "memory://snapshot/dependencies.md") (qualified-name "Dependencies::System::Data Layer")))))
    )
  )
  (query (document "memory://snapshot/dependencies.md") (range (start 17 17) (end 17 18)) (probe (position 17 17))
    (reference (id (source (node (document "memory://snapshot/dependencies.md") (path (named (kind package) (name "Dependencies")) (anonymous (kind dependency) (ordinal 1))))) (kind dependencySupplier) (ordinal 0) (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/dependencies.md") (qualified-name "Dependencies::x")))))
    )
  )
  (query (document "memory://snapshot/dependencies.md") (range (start 17 20) (end 17 21)) (probe (position 17 20))
    (reference (id (source (node (document "memory://snapshot/dependencies.md") (path (named (kind package) (name "Dependencies")) (anonymous (kind dependency) (ordinal 1))))) (kind dependencySupplier) (ordinal 1) (authored-target "y")
      (outcome (status resolved) (target (node (document "memory://snapshot/dependencies.md") (qualified-name "Dependencies::y")))))
    )
  )
  (query (document "memory://snapshot/dependencies.md") (range (start 10 21) (end 10 40)) (probe (position 10 21))
    (reference (id (source (node (document "memory://snapshot/dependencies.md") (qualified-name "Dependencies::Use"))) (kind dependencyClient) (ordinal 0) (authored-target "Application Layer")
      (outcome (status resolved) (target (node (document "memory://snapshot/dependencies.md") (qualified-name "Dependencies::System::Application Layer")))))
    )
  )
  (query (document "memory://snapshot/dependencies.md") (range (start 10 44) (end 10 59)) (probe (position 10 44))
    (reference (id (source (node (document "memory://snapshot/dependencies.md") (qualified-name "Dependencies::Use"))) (kind dependencySupplier) (ordinal 0) (authored-target "Service Layer")
      (outcome (status resolved) (target (node (document "memory://snapshot/dependencies.md") (qualified-name "Dependencies::System::Service Layer")))))
    )
  )
)
~~~
