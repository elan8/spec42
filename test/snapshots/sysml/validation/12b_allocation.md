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
  (document "12b_allocation.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 2) (end 9 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 16 3) (end 16 62))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "130ae23fece4382a7835e2250e6e680c6db6acae6a949ffdeb72b3d2d9acf861") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "12b-Allocation"))) (kind "package") (name "12b-Allocation") (declared-name "12b-Allocation"))
    (element (id (node (document "d0") (qualified-name "12b-Allocation::"))) (kind "allocation") (name "") (parent (node (document "d0") (qualified-name "12b-Allocation"))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "12b-Allocation"))) (authored (membership (kind Import) (visibility "private") (import (reference "LogicalModel::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "12b-Allocation"))) (authored (membership (kind Import) (visibility "private") (import (reference "PhysicalModel::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation::LogicalModel"))) (kind "package") (name "LogicalModel") (declared-name "LogicalModel") (parent (node (document "d0") (qualified-name "12b-Allocation"))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation::LogicalModel::providePower"))) (kind "action") (name "providePower") (declared-name "providePower") (parent (node (document "d0") (qualified-name "12b-Allocation::LogicalModel"))) (authored (membership (kind Feature)) (relationships (perform (reference "12b-Allocation::LogicalModel::providePower::generateTorque")))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation::LogicalModel::providePower::generateTorque"))) (kind "action") (name "generateTorque") (declared-name "generateTorque") (parent (node (document "d0") (qualified-name "12b-Allocation::LogicalModel::providePower"))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation::LogicalModel::torqueGenerator"))) (kind "part") (name "torqueGenerator") (declared-name "torqueGenerator") (parent (node (document "d0") (qualified-name "12b-Allocation::LogicalModel"))) (authored (membership (kind Feature)) (relationships (perform (reference "12b-Allocation::LogicalModel::torqueGenerator::providePower::generateTorque")))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation::LogicalModel::torqueGenerator::providePower.generateTorque"))) (kind "action") (name "providePower.generateTorque") (declared-name "providePower.generateTorque") (parent (node (document "d0") (qualified-name "12b-Allocation::LogicalModel::torqueGenerator"))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation::PhysicalModel"))) (kind "package") (name "PhysicalModel") (declared-name "PhysicalModel") (parent (node (document "d0") (qualified-name "12b-Allocation"))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation::PhysicalModel::powerTrain"))) (kind "part") (name "powerTrain") (declared-name "powerTrain") (parent (node (document "d0") (qualified-name "12b-Allocation::PhysicalModel"))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation::PhysicalModel::powerTrain::engine"))) (kind "part") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "12b-Allocation::PhysicalModel::powerTrain"))) (authored (membership (kind Feature)) (relationships (perform (reference "12b-Allocation::PhysicalModel::powerTrain::engine::providePower::generateTorque")))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation::PhysicalModel::powerTrain::engine::providePower.generateTorque"))) (kind "action") (name "providePower.generateTorque") (declared-name "providePower.generateTorque") (parent (node (document "d0") (qualified-name "12b-Allocation::PhysicalModel::powerTrain::engine"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation"))) (kind allocateSource) (ordinal 0)) (authored-target "torqueGenerator") (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation::LogicalModel::torqueGenerator")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation"))) (kind allocateTarget) (ordinal 0)) (authored-target "powerTrain") (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation::PhysicalModel::powerTrain")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "LogicalModel::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation::LogicalModel")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "PhysicalModel::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation::PhysicalModel")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation::LogicalModel::providePower"))) (kind performSource) (ordinal 0)) (authored-target "12b-Allocation::LogicalModel::providePower::generateTorque") (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation::LogicalModel::providePower::generateTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation::LogicalModel::torqueGenerator"))) (kind performSource) (ordinal 0)) (authored-target "12b-Allocation::LogicalModel::torqueGenerator::providePower::generateTorque") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation::PhysicalModel::powerTrain::engine"))) (kind performSource) (ordinal 0)) (authored-target "12b-Allocation::PhysicalModel::powerTrain::engine::providePower::generateTorque") (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind perform) (source (node (document "d0") (qualified-name "12b-Allocation::LogicalModel::providePower"))) (target (node (document "d0") (qualified-name "12b-Allocation::LogicalModel::providePower::generateTorque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "12b-Allocation::LogicalModel::providePower"))) (kind performSource) (ordinal 0)))
    (relationship (kind allocate) (source (node (document "d0") (qualified-name "12b-Allocation::LogicalModel::torqueGenerator"))) (target (node (document "d0") (qualified-name "12b-Allocation::PhysicalModel::powerTrain"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "12b-Allocation"))) (kind allocateSource) (ordinal 0)) (expression (kind allocate) (source "torqueGenerator") (target "powerTrain")))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 22 29) (end 22 39)) (probe (position 22 29))
      (reference
        (source (document "d0") (qualified-name "12b-Allocation"))
        (kind allocateTarget) (ordinal 0) (authored-target "powerTrain")
        (range (start 22 29) (end 22 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "12b-Allocation::PhysicalModel::powerTrain") (range (start 15 2) (end 15 86)))
        )
      )
    )
    (query (range (start 1 16) (end 1 28)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "12b-Allocation::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "LogicalModel::*")
        (range (start 1 16) (end 1 28))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "12b-Allocation::LogicalModel") (range (start 4 1) (end 4 152)))
        )
      )
    )
    (query (range (start 2 16) (end 2 29)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "12b-Allocation::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "PhysicalModel::*")
        (range (start 2 16) (end 2 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "12b-Allocation::PhysicalModel") (range (start 14 1) (end 14 114)))
        )
      )
    )
    (query (range (start 22 10) (end 22 25)) (probe (position 22 10))
      (reference
        (source (document "d0") (qualified-name "12b-Allocation"))
        (kind allocateSource) (ordinal 0) (authored-target "torqueGenerator")
        (range (start 22 10) (end 22 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "12b-Allocation::LogicalModel::torqueGenerator") (range (start 9 2) (end 9 68)))
        )
      )
    )
  )
)
~~~
