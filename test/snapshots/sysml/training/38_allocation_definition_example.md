# META
~~~ini
description=SysML Training 38 (Allocation): Allocation Definition Example
type=file
~~~
# SOURCE
~~~sysml
package 'Allocation Definition Example' {
	package LogicalModel {
		action def ProvidePower;
		action def GenerateTorque;
		
		part def LogicalElement;
		part def TorqueGenerator :> LogicalElement;
		
		action providePower : ProvidePower {
			action generateTorque : GenerateTorque;
		}
		
		part torqueGenerator : TorqueGenerator {
			perform providePower.generateTorque;
		}
		
	}
	
	package PhysicalModel {
		private import LogicalModel::*;
		
		part def PhysicalElement;
		part def PowerTrain :> PhysicalElement;
		
		part powerTrain : PowerTrain {
			part engine {
				perform providePower.generateTorque;
			}
		}
	
		allocation def LogicalToPhysical {
			end logical : LogicalElement;
			end physical : PhysicalElement;
		}
		
		allocation torqueGenAlloc : LogicalToPhysical allocate torqueGenerator to powerTrain;
	}	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "38_allocation_definition_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 12 2) (end 12 86))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 19 17) (end 19 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 25 3) (end 25 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 31 3) (end 31 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 35 57) (end 35 72))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "d98a159ea3b363e53542525f26a1d3d70a7fd9106211083e53731a1ff4c847cb") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Allocation Definition Example"))) (kind "package") (name "Allocation Definition Example") (declared-name "Allocation Definition Example"))
    (element (id (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel"))) (kind "package") (name "LogicalModel") (declared-name "LogicalModel") (parent (node (document "d0") (qualified-name "Allocation Definition Example"))))
    (element (id (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::GenerateTorque"))) (kind "action def") (name "GenerateTorque") (declared-name "GenerateTorque") (parent (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel"))))
    (element (id (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::LogicalElement"))) (kind "part def") (name "LogicalElement") (declared-name "LogicalElement") (parent (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel"))))
    (element (id (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::ProvidePower"))) (kind "action def") (name "ProvidePower") (declared-name "ProvidePower") (parent (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel"))))
    (element (id (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::TorqueGenerator"))) (kind "part def") (name "TorqueGenerator") (declared-name "TorqueGenerator") (parent (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel"))) (authored (membership (kind Owning)) (relationships (specializes (reference "LogicalElement")))))
    (element (id (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::providePower"))) (kind "action") (name "providePower") (declared-name "providePower") (parent (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "ProvidePower")) (perform (reference "Allocation Definition Example::LogicalModel::providePower::generateTorque")))))
    (element (id (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::providePower::generateTorque"))) (kind "action") (name "generateTorque") (declared-name "generateTorque") (parent (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::providePower"))) (authored (membership (kind Feature)) (relationships (typing (reference "GenerateTorque")))))
    (element (id (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::torqueGenerator"))) (kind "part") (name "torqueGenerator") (declared-name "torqueGenerator") (parent (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "TorqueGenerator")) (perform (reference "Allocation Definition Example::LogicalModel::torqueGenerator::providePower::generateTorque")))))
    (element (id (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::torqueGenerator::providePower.generateTorque"))) (kind "action") (name "providePower.generateTorque") (declared-name "providePower.generateTorque") (parent (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::torqueGenerator"))))
    (element (id (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel"))) (kind "package") (name "PhysicalModel") (declared-name "PhysicalModel") (parent (node (document "d0") (qualified-name "Allocation Definition Example"))))
    (element (id (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "LogicalModel::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical"))) (kind "allocation def") (name "LogicalToPhysical") (declared-name "LogicalToPhysical") (parent (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel"))))
    (element (id (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical::logical"))) (kind "interface end") (name "logical") (declared-name "logical") (parent (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical"))) (authored (relationships (typing (reference "LogicalElement")))))
    (element (id (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical::physical"))) (kind "interface end") (name "physical") (declared-name "physical") (parent (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical"))) (authored (relationships (typing (reference "PhysicalElement")))))
    (element (id (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::PhysicalElement"))) (kind "part def") (name "PhysicalElement") (declared-name "PhysicalElement") (parent (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel"))))
    (element (id (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::PowerTrain"))) (kind "part def") (name "PowerTrain") (declared-name "PowerTrain") (parent (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel"))) (authored (membership (kind Owning)) (relationships (specializes (reference "PhysicalElement")))))
    (element (id (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::powerTrain"))) (kind "part") (name "powerTrain") (declared-name "powerTrain") (parent (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "PowerTrain")))))
    (element (id (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::powerTrain::engine"))) (kind "part") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::powerTrain"))) (authored (membership (kind Feature)) (relationships (perform (reference "Allocation Definition Example::PhysicalModel::powerTrain::engine::providePower::generateTorque")))))
    (element (id (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::powerTrain::engine::providePower.generateTorque"))) (kind "action") (name "providePower.generateTorque") (declared-name "providePower.generateTorque") (parent (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::powerTrain::engine"))))
    (element (id (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::torqueGenAlloc"))) (kind "allocation") (name "torqueGenAlloc") (declared-name "torqueGenAlloc") (parent (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "LogicalToPhysical")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::TorqueGenerator"))) (kind specialization) (ordinal 0)) (authored-target "LogicalElement") (outcome (status resolved) (target (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::LogicalElement")))))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::providePower"))) (kind featureTyping) (ordinal 0)) (authored-target "ProvidePower") (outcome (status resolved) (target (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::ProvidePower")))))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::providePower"))) (kind performSource) (ordinal 0)) (authored-target "Allocation Definition Example::LogicalModel::providePower::generateTorque") (outcome (status resolved) (target (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::providePower::generateTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::providePower::generateTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "GenerateTorque") (outcome (status resolved) (target (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::GenerateTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::torqueGenerator"))) (kind featureTyping) (ordinal 0)) (authored-target "TorqueGenerator") (outcome (status resolved) (target (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::TorqueGenerator")))))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::torqueGenerator"))) (kind performSource) (ordinal 0)) (authored-target "Allocation Definition Example::LogicalModel::torqueGenerator::providePower::generateTorque") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel"))) (kind allocateSource) (ordinal 0)) (authored-target "torqueGenerator") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel"))) (kind allocateTarget) (ordinal 0)) (authored-target "powerTrain") (outcome (status resolved) (target (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::powerTrain")))))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "LogicalModel::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical::logical"))) (kind featureTyping) (ordinal 0)) (authored-target "LogicalElement") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical::physical"))) (kind featureTyping) (ordinal 0)) (authored-target "PhysicalElement") (outcome (status resolved) (target (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::PhysicalElement")))))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::PowerTrain"))) (kind specialization) (ordinal 0)) (authored-target "PhysicalElement") (outcome (status resolved) (target (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::PhysicalElement")))))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::powerTrain"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerTrain") (outcome (status resolved) (target (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::PowerTrain")))))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::powerTrain::engine"))) (kind performSource) (ordinal 0)) (authored-target "Allocation Definition Example::PhysicalModel::powerTrain::engine::providePower::generateTorque") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::torqueGenAlloc"))) (kind featureTyping) (ordinal 0)) (authored-target "LogicalToPhysical") (outcome (status resolved) (target (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical")))))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::TorqueGenerator"))) (target (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::LogicalElement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::TorqueGenerator"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::providePower"))) (target (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::ProvidePower"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::providePower"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::providePower"))) (target (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::providePower::generateTorque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::providePower"))) (kind performSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::providePower::generateTorque"))) (target (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::GenerateTorque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::providePower::generateTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::torqueGenerator"))) (target (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::TorqueGenerator"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::torqueGenerator"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical::physical"))) (target (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::PhysicalElement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical::physical"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::PowerTrain"))) (target (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::PhysicalElement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::PowerTrain"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::powerTrain"))) (target (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::PowerTrain"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::powerTrain"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::torqueGenAlloc"))) (target (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::torqueGenAlloc"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 24 20) (end 24 30)) (probe (position 24 20))
      (reference
        (source (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::powerTrain"))
        (kind featureTyping) (ordinal 0) (authored-target "PowerTrain")
        (range (start 24 20) (end 24 30))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::PowerTrain") (range (start 22 2) (end 22 41)))
        )
      )
    )
    (query (range (start 35 76) (end 35 86)) (probe (position 35 76))
      (reference
        (source (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel"))
        (kind allocateTarget) (ordinal 0) (authored-target "powerTrain")
        (range (start 35 76) (end 35 86))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::powerTrain") (range (start 24 2) (end 24 99)))
        )
      )
    )
    (query (range (start 19 17) (end 19 29)) (probe (position 19 17))
      (reference
        (source (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "LogicalModel::*")
        (range (start 19 17) (end 19 29))
        (outcome (status unresolved))
      )
    )
    (query (range (start 6 30) (end 6 44)) (probe (position 6 30))
      (reference
        (source (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::TorqueGenerator"))
        (kind specialization) (ordinal 0) (authored-target "LogicalElement")
        (range (start 6 30) (end 6 44))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::LogicalElement") (range (start 5 2) (end 5 26)))
        )
      )
    )
    (query (range (start 12 25) (end 12 40)) (probe (position 12 25))
      (reference
        (source (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::torqueGenerator"))
        (kind featureTyping) (ordinal 0) (authored-target "TorqueGenerator")
        (range (start 12 25) (end 12 40))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::TorqueGenerator") (range (start 6 2) (end 6 45)))
        )
      )
    )
    (query (range (start 22 25) (end 22 40)) (probe (position 22 25))
      (reference
        (source (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::PowerTrain"))
        (kind specialization) (ordinal 0) (authored-target "PhysicalElement")
        (range (start 22 25) (end 22 40))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::PhysicalElement") (range (start 21 2) (end 21 27)))
        )
      )
    )
    (query (range (start 35 57) (end 35 72)) (probe (position 35 57))
      (reference
        (source (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel"))
        (kind allocateSource) (ordinal 0) (authored-target "torqueGenerator")
        (range (start 35 57) (end 35 72))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
