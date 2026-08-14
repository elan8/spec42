# META
~~~ini
description=SysML Validation (12-Dependency Relationships): 12b-Allocation
type=file
~~~
# SOURCE
~~~sysml
package '12b-Allocation' {
	private import LogicalModel::*;
	private import PhysicalModel::*;
	
	package LogicalModel {
		action providePower {
			action generateTorque;
		}
		
		part torqueGenerator {
			perform providePower.generateTorque;
		}
	}
	
	package PhysicalModel {
		part powerTrain {
			part engine {
				perform providePower.generateTorque;
			}
		}
	}
	
	allocate torqueGenerator to powerTrain {
		allocate torqueGenerator.generateTorque to powerTrain.engine.generateTorque;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/12b_allocation.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 22 1) (end 24 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:261b1fe02503a1a9e2b33fbf4d5de237fbcf031cd9d444a8b7ba78e92d92efdc") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "LogicalModel") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (anonymous (kind import) (ordinal 1)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "PhysicalModel") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::LogicalModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::LogicalModel::providePower"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::LogicalModel::providePower::generateTorque"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::LogicalModel::torqueGenerator"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (named (kind package) (name "LogicalModel")) (named (kind part) (name "torqueGenerator")) (anonymous (kind perform-action) (ordinal 0)))))) (kind perform-action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::PhysicalModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::PhysicalModel::powerTrain"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::PhysicalModel::powerTrain::engine"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (named (kind package) (name "PhysicalModel")) (named (kind part) (name "powerTrain")) (named (kind part) (name "engine")) (anonymous (kind perform-action) (ordinal 0)))))) (kind perform-action) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "LogicalModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::LogicalModel")))))
    (reference (id (source (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (anonymous (kind import) (ordinal 1)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "PhysicalModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::PhysicalModel")))))
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
  (query (document "memory://snapshot/12b_allocation.md") (range (start 1 16) (end 1 31)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0) (authored-target "LogicalModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::LogicalModel")))))
  )
  (query (document "memory://snapshot/12b_allocation.md") (range (start 2 16) (end 2 32)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (anonymous (kind import) (ordinal 1)))))) (kind namespaceImport) (ordinal 0) (authored-target "PhysicalModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::PhysicalModel")))))
  )
)
~~~
