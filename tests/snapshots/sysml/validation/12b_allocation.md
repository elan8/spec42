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
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 31))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 9 2) (end 11 3))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 15 2) (end 19 3))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 16 3) (end 18 4))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 23 11) (end 23 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 23 45) (end 23 77))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:261b1fe02503a1a9e2b33fbf4d5de237fbcf031cd9d444a8b7ba78e92d92efdc"))
  (declarations
    (declaration (id (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "LogicalModel") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "PhysicalModel") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (anonymous (kind allocate) (ordinal 0))))) (kind allocate) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (allocateSource (reference "torqueGenerator")) (allocateTarget (reference "powerTrain")))))
    (declaration (id (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (anonymous (kind allocate) (ordinal 0)) (anonymous (kind allocate) (ordinal 0))))) (kind allocate) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "torqueGenerator::generateTorque")) (memberAccessOperand (reference "powerTrain::engine::generateTorque")))))
    (declaration (id (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::LogicalModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::LogicalModel::providePower"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::LogicalModel::providePower::generateTorque"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::LogicalModel::torqueGenerator"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (named (kind package) (name "LogicalModel")) (named (kind part) (name "torqueGenerator")) (anonymous (kind perform-action) (ordinal 0))))) (kind perform-action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (referenceSubsetting (reference "providePower::generateTorque")))))
    (declaration (id (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::PhysicalModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::PhysicalModel::powerTrain"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::PhysicalModel::powerTrain::engine"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (named (kind package) (name "PhysicalModel")) (named (kind part) (name "powerTrain")) (named (kind part) (name "engine")) (anonymous (kind perform-action) (ordinal 0))))) (kind perform-action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (referenceSubsetting (reference "providePower::generateTorque")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "LogicalModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::LogicalModel")))))
    (reference (id (source (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "PhysicalModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::PhysicalModel")))))
    (reference (id (source (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (anonymous (kind allocate) (ordinal 0))))) (kind allocateSource) (ordinal 0))
      (authored-target "torqueGenerator")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::LogicalModel::torqueGenerator")))))
    (reference (id (source (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (anonymous (kind allocate) (ordinal 0))))) (kind allocateTarget) (ordinal 0))
      (authored-target "powerTrain")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::PhysicalModel::powerTrain")))))
    (reference (id (source (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (anonymous (kind allocate) (ordinal 0)) (anonymous (kind allocate) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "torqueGenerator::generateTorque")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (anonymous (kind allocate) (ordinal 0)) (anonymous (kind allocate) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "powerTrain::engine::generateTorque")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (named (kind package) (name "LogicalModel")) (named (kind part) (name "torqueGenerator")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "providePower::generateTorque")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::LogicalModel::providePower::generateTorque")))))
    (reference (id (source (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (named (kind package) (name "PhysicalModel")) (named (kind part) (name "powerTrain")) (named (kind part) (name "engine")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "providePower::generateTorque")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::LogicalModel::providePower::generateTorque")))))
  )
  (relationships
    (relationship (kind allocateSource) (source (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (anonymous (kind allocate) (ordinal 0))))) (target (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::LogicalModel::torqueGenerator"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (anonymous (kind allocate) (ordinal 0))))) (kind allocateSource) (ordinal 0)))
    (relationship (kind allocateTarget) (source (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (anonymous (kind allocate) (ordinal 0))))) (target (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::PhysicalModel::powerTrain"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (anonymous (kind allocate) (ordinal 0))))) (kind allocateTarget) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (named (kind package) (name "LogicalModel")) (named (kind part) (name "torqueGenerator")) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::LogicalModel::providePower::generateTorque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (named (kind package) (name "LogicalModel")) (named (kind part) (name "torqueGenerator")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (named (kind package) (name "PhysicalModel")) (named (kind part) (name "powerTrain")) (named (kind part) (name "engine")) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::LogicalModel::providePower::generateTorque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (named (kind package) (name "PhysicalModel")) (named (kind part) (name "powerTrain")) (named (kind part) (name "engine")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (anonymous (kind allocate) (ordinal 0)) (anonymous (kind allocate) (ordinal 0))))) (target (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (anonymous (kind allocate) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::LogicalModel::providePower::generateTorque"))) (target (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::LogicalModel::providePower"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (named (kind package) (name "LogicalModel")) (named (kind part) (name "torqueGenerator")) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::LogicalModel::torqueGenerator"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::PhysicalModel::powerTrain::engine"))) (target (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::PhysicalModel::powerTrain"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (named (kind package) (name "PhysicalModel")) (named (kind part) (name "powerTrain")) (named (kind part) (name "engine")) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::PhysicalModel::powerTrain::engine"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (anonymous (kind allocate) (ordinal 0)) (anonymous (kind allocate) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (anonymous (kind allocate) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::LogicalModel::providePower::generateTorque")))
      (featured-by (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::LogicalModel::providePower")))
    )
    (declaration (id (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (named (kind package) (name "LogicalModel")) (named (kind part) (name "torqueGenerator")) (anonymous (kind perform-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::LogicalModel::torqueGenerator")))
    )
    (declaration (id (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::PhysicalModel::powerTrain::engine")))
      (featured-by (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::PhysicalModel::powerTrain")))
    )
    (declaration (id (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (named (kind package) (name "PhysicalModel")) (named (kind part) (name "powerTrain")) (named (kind part) (name "engine")) (anonymous (kind perform-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::PhysicalModel::powerTrain::engine")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/12b_allocation.md") (range (start 1 16) (end 1 31)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "LogicalModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::LogicalModel")))))
    )
  )
  (query (document "memory://snapshot/12b_allocation.md") (range (start 2 16) (end 2 32)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "PhysicalModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::PhysicalModel")))))
    )
  )
  (query (document "memory://snapshot/12b_allocation.md") (range (start 22 10) (end 22 25)) (probe (position 22 10))
    (reference (id (source (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (anonymous (kind allocate) (ordinal 0))))) (kind allocateSource) (ordinal 0) (authored-target "torqueGenerator")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::LogicalModel::torqueGenerator")))))
    )
  )
  (query (document "memory://snapshot/12b_allocation.md") (range (start 22 29) (end 22 39)) (probe (position 22 29))
    (reference (id (source (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (anonymous (kind allocate) (ordinal 0))))) (kind allocateTarget) (ordinal 0) (authored-target "powerTrain")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::PhysicalModel::powerTrain")))))
    )
  )
  (query (document "memory://snapshot/12b_allocation.md") (range (start 23 11) (end 23 41)) (probe (position 23 11))
    (reference (id (source (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (anonymous (kind allocate) (ordinal 0)) (anonymous (kind allocate) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "torqueGenerator::generateTorque")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/12b_allocation.md") (range (start 23 45) (end 23 77)) (probe (position 23 45))
    (reference (id (source (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (anonymous (kind allocate) (ordinal 0)) (anonymous (kind allocate) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1) (authored-target "powerTrain::engine::generateTorque")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/12b_allocation.md") (range (start 10 11) (end 10 38)) (probe (position 10 11))
    (reference (id (source (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (named (kind package) (name "LogicalModel")) (named (kind part) (name "torqueGenerator")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0) (authored-target "providePower::generateTorque")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::LogicalModel::providePower::generateTorque")))))
    )
  )
  (query (document "memory://snapshot/12b_allocation.md") (range (start 17 12) (end 17 39)) (probe (position 17 12))
    (reference (id (source (node (document "memory://snapshot/12b_allocation.md") (path (named (kind package) (name "12b-Allocation")) (named (kind package) (name "PhysicalModel")) (named (kind part) (name "powerTrain")) (named (kind part) (name "engine")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0) (authored-target "providePower::generateTorque")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation.md") (qualified-name "12b-Allocation::LogicalModel::providePower::generateTorque")))))
    )
  )
)
~~~
