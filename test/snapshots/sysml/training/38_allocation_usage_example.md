# META
~~~ini
description=SysML Training 38 (Allocation): Allocation Usage Example
type=file
~~~
# SOURCE
~~~sysml
package 'Allocation Usage Example' {
	package LogicalModel {
		action def ProvidePower;
		action def GenerateTorque;
		
		part def TorqueGenerator;
		
		action providePower : ProvidePower {
			action generateTorque : GenerateTorque;
		}
		
		part torqueGenerator : TorqueGenerator {
			perform providePower.generateTorque;
		}
	}
	
	package PhysicalModel {
		private import LogicalModel::*;
	
		part def PowerTrain;
		part def Engine;
		
		part powerTrain : PowerTrain {
			part engine : Engine {
				perform providePower.generateTorque;
			}
		}
		
		allocate torqueGenerator to powerTrain {
			allocate torqueGenerator.generateTorque to powerTrain.engine.generateTorque;
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "38_allocation_usage_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 2) (end 11 86))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 17 17) (end 17 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 23 3) (end 23 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 28 11) (end 28 26))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "fc3b79d81bfedcc64b7c8e8b1d153839f59bb25fde3762b86ecad0527c60c161") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Allocation Usage Example"))) (kind "package") (name "Allocation Usage Example") (declared-name "Allocation Usage Example"))
    (element (id (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel"))) (kind "package") (name "LogicalModel") (declared-name "LogicalModel") (parent (node (document "d0") (qualified-name "Allocation Usage Example"))))
    (element (id (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::GenerateTorque"))) (kind "action def") (name "GenerateTorque") (declared-name "GenerateTorque") (parent (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel"))))
    (element (id (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::ProvidePower"))) (kind "action def") (name "ProvidePower") (declared-name "ProvidePower") (parent (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel"))))
    (element (id (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::TorqueGenerator"))) (kind "part def") (name "TorqueGenerator") (declared-name "TorqueGenerator") (parent (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel"))))
    (element (id (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::providePower"))) (kind "action") (name "providePower") (declared-name "providePower") (parent (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "ProvidePower")) (perform (reference "Allocation Usage Example::LogicalModel::providePower::generateTorque")))))
    (element (id (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::providePower::generateTorque"))) (kind "action") (name "generateTorque") (declared-name "generateTorque") (parent (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::providePower"))) (authored (membership (kind Feature)) (relationships (typing (reference "GenerateTorque")))))
    (element (id (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::torqueGenerator"))) (kind "part") (name "torqueGenerator") (declared-name "torqueGenerator") (parent (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "TorqueGenerator")) (perform (reference "Allocation Usage Example::LogicalModel::torqueGenerator::providePower::generateTorque")))))
    (element (id (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::torqueGenerator::providePower.generateTorque"))) (kind "action") (name "providePower.generateTorque") (declared-name "providePower.generateTorque") (parent (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::torqueGenerator"))))
    (element (id (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel"))) (kind "package") (name "PhysicalModel") (declared-name "PhysicalModel") (parent (node (document "d0") (qualified-name "Allocation Usage Example"))))
    (element (id (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::"))) (kind "allocation") (name "") (parent (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel"))))
    (element (id (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "LogicalModel::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (parent (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel"))))
    (element (id (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::PowerTrain"))) (kind "part def") (name "PowerTrain") (declared-name "PowerTrain") (parent (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel"))))
    (element (id (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain"))) (kind "part") (name "powerTrain") (declared-name "powerTrain") (parent (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "PowerTrain")))))
    (element (id (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain::engine"))) (kind "part") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine")) (perform (reference "Allocation Usage Example::PhysicalModel::powerTrain::engine::providePower::generateTorque")))))
    (element (id (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain::engine::providePower.generateTorque"))) (kind "action") (name "providePower.generateTorque") (declared-name "providePower.generateTorque") (parent (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain::engine"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::providePower"))) (kind featureTyping) (ordinal 0)) (authored-target "ProvidePower") (outcome (status resolved) (target (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::ProvidePower")))))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::providePower"))) (kind performSource) (ordinal 0)) (authored-target "Allocation Usage Example::LogicalModel::providePower::generateTorque") (outcome (status resolved) (target (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::providePower::generateTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::providePower::generateTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "GenerateTorque") (outcome (status resolved) (target (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::GenerateTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::torqueGenerator"))) (kind featureTyping) (ordinal 0)) (authored-target "TorqueGenerator") (outcome (status resolved) (target (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::TorqueGenerator")))))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::torqueGenerator"))) (kind performSource) (ordinal 0)) (authored-target "Allocation Usage Example::LogicalModel::torqueGenerator::providePower::generateTorque") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel"))) (kind allocateSource) (ordinal 0)) (authored-target "torqueGenerator") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel"))) (kind allocateTarget) (ordinal 0)) (authored-target "powerTrain") (outcome (status resolved) (target (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain")))))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "LogicalModel::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerTrain") (outcome (status resolved) (target (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::PowerTrain")))))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain::engine"))) (kind performSource) (ordinal 0)) (authored-target "Allocation Usage Example::PhysicalModel::powerTrain::engine::providePower::generateTorque") (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::providePower"))) (target (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::ProvidePower"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::providePower"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::providePower"))) (target (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::providePower::generateTorque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::providePower"))) (kind performSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::providePower::generateTorque"))) (target (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::GenerateTorque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::providePower::generateTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::torqueGenerator"))) (target (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::TorqueGenerator"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::torqueGenerator"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain"))) (target (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::PowerTrain"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain::engine"))) (target (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain::engine"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 23 17) (end 23 23)) (probe (position 23 17))
      (reference
        (source (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain::engine"))
        (kind featureTyping) (ordinal 0) (authored-target "Engine")
        (range (start 23 17) (end 23 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::Engine") (range (start 20 2) (end 20 18)))
        )
      )
    )
    (query (range (start 22 20) (end 22 30)) (probe (position 22 20))
      (reference
        (source (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain"))
        (kind featureTyping) (ordinal 0) (authored-target "PowerTrain")
        (range (start 22 20) (end 22 30))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::PowerTrain") (range (start 19 2) (end 19 22)))
        )
      )
    )
    (query (range (start 28 30) (end 28 40)) (probe (position 28 30))
      (reference
        (source (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel"))
        (kind allocateTarget) (ordinal 0) (authored-target "powerTrain")
        (range (start 28 30) (end 28 40))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::powerTrain") (range (start 22 2) (end 22 108)))
        )
      )
    )
    (query (range (start 17 17) (end 17 29)) (probe (position 17 17))
      (reference
        (source (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "LogicalModel::*")
        (range (start 17 17) (end 17 29))
        (outcome (status unresolved))
      )
    )
    (query (range (start 11 25) (end 11 40)) (probe (position 11 25))
      (reference
        (source (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::torqueGenerator"))
        (kind featureTyping) (ordinal 0) (authored-target "TorqueGenerator")
        (range (start 11 25) (end 11 40))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Allocation Usage Example::LogicalModel::TorqueGenerator") (range (start 5 2) (end 5 27)))
        )
      )
    )
    (query (range (start 28 11) (end 28 26)) (probe (position 28 11))
      (reference
        (source (document "d0") (qualified-name "Allocation Usage Example::PhysicalModel"))
        (kind allocateSource) (ordinal 0) (authored-target "torqueGenerator")
        (range (start 28 11) (end 28 26))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
