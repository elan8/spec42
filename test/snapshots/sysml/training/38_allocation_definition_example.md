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
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "d98a159ea3b363e53542525f26a1d3d70a7fd9106211083e53731a1ff4c847cb") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Allocation Definition Example"))) (kind "package") (name "Allocation Definition Example") (declared-name "Allocation Definition Example") (range (start (line 0) (character 0)) (end (line 0) (character 827))))
    (element (id (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel"))) (kind "package") (name "LogicalModel") (declared-name "LogicalModel") (range (start (line 1) (character 1)) (end (line 1) (character 340))) (parent (node (document "d0") (qualified-name "Allocation Definition Example"))))
    (element (id (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::GenerateTorque"))) (kind "action def") (name "GenerateTorque") (declared-name "GenerateTorque") (range (start (line 3) (character 2)) (end (line 3) (character 28))) (parent (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel"))))
    (element (id (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::LogicalElement"))) (kind "part def") (name "LogicalElement") (declared-name "LogicalElement") (range (start (line 5) (character 2)) (end (line 5) (character 26))) (parent (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel"))))
    (element (id (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::ProvidePower"))) (kind "action def") (name "ProvidePower") (declared-name "ProvidePower") (range (start (line 2) (character 2)) (end (line 2) (character 26))) (parent (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel"))))
    (element (id (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::TorqueGenerator"))) (kind "part def") (name "TorqueGenerator") (declared-name "TorqueGenerator") (range (start (line 6) (character 2)) (end (line 6) (character 45))) (parent (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel"))) (authored (membership (kind Owning)) (relationships (specializes (reference "LogicalElement") (range (start (line 6) (character 30)) (end (line 6) (character 44)))))))
    (element (id (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::providePower"))) (kind "action") (name "providePower") (declared-name "providePower") (range (start (line 8) (character 2)) (end (line 8) (character 85))) (parent (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "ProvidePower") (range none)) (perform (reference "Allocation Definition Example::LogicalModel::providePower::generateTorque") (range none)))))
    (element (id (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::providePower::generateTorque"))) (kind "action") (name "generateTorque") (declared-name "generateTorque") (range (start (line 9) (character 3)) (end (line 9) (character 42))) (parent (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::providePower"))) (authored (membership (kind Feature)) (relationships (typing (reference "GenerateTorque") (range none)))))
    (element (id (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::torqueGenerator"))) (kind "part") (name "torqueGenerator") (declared-name "torqueGenerator") (range (start (line 12) (character 2)) (end (line 12) (character 86))) (parent (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "TorqueGenerator") (range (start (line 12) (character 25)) (end (line 12) (character 40)))) (perform (reference "Allocation Definition Example::LogicalModel::torqueGenerator::providePower::generateTorque") (range none)))))
    (element (id (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::torqueGenerator::providePower.generateTorque"))) (kind "action") (name "providePower.generateTorque") (declared-name "providePower.generateTorque") (range (start (line 13) (character 3)) (end (line 13) (character 39))) (parent (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::torqueGenerator"))))
    (element (id (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel"))) (kind "package") (name "PhysicalModel") (declared-name "PhysicalModel") (range (start (line 18) (character 1)) (end (line 18) (character 439))) (parent (node (document "d0") (qualified-name "Allocation Definition Example"))))
    (element (id (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 19) (character 2)) (end (line 19) (character 33))) (parent (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "LogicalModel::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 19) (character 17)) (end (line 19) (character 29))))))
    (element (id (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical"))) (kind "allocation def") (name "LogicalToPhysical") (declared-name "LogicalToPhysical") (range (start (line 30) (character 2)) (end (line 30) (character 108))) (parent (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel"))))
    (element (id (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical::logical"))) (kind "interface end") (name "logical") (declared-name "logical") (range (start (line 31) (character 3)) (end (line 31) (character 32))) (parent (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical"))) (authored (relationships (typing (reference "LogicalElement") (range none)))))
    (element (id (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical::physical"))) (kind "interface end") (name "physical") (declared-name "physical") (range (start (line 32) (character 3)) (end (line 32) (character 34))) (parent (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical"))) (authored (relationships (typing (reference "PhysicalElement") (range none)))))
    (element (id (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::PhysicalElement"))) (kind "part def") (name "PhysicalElement") (declared-name "PhysicalElement") (range (start (line 21) (character 2)) (end (line 21) (character 27))) (parent (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel"))))
    (element (id (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::PowerTrain"))) (kind "part def") (name "PowerTrain") (declared-name "PowerTrain") (range (start (line 22) (character 2)) (end (line 22) (character 41))) (parent (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel"))) (authored (membership (kind Owning)) (relationships (specializes (reference "PhysicalElement") (range (start (line 22) (character 25)) (end (line 22) (character 40)))))))
    (element (id (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::powerTrain"))) (kind "part") (name "powerTrain") (declared-name "powerTrain") (range (start (line 24) (character 2)) (end (line 24) (character 99))) (parent (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "PowerTrain") (range (start (line 24) (character 20)) (end (line 24) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::powerTrain::engine"))) (kind "part") (name "engine") (declared-name "engine") (range (start (line 25) (character 3)) (end (line 25) (character 62))) (parent (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::powerTrain"))) (authored (membership (kind Feature)) (relationships (perform (reference "Allocation Definition Example::PhysicalModel::powerTrain::engine::providePower::generateTorque") (range none)))))
    (element (id (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::powerTrain::engine::providePower.generateTorque"))) (kind "action") (name "providePower.generateTorque") (declared-name "providePower.generateTorque") (range (start (line 26) (character 4)) (end (line 26) (character 40))) (parent (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::powerTrain::engine"))))
    (element (id (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::torqueGenAlloc"))) (kind "allocation") (name "torqueGenAlloc") (declared-name "torqueGenAlloc") (range (start (line 35) (character 2)) (end (line 35) (character 87))) (parent (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "LogicalToPhysical") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::TorqueGenerator"))) (kind specialization) (ordinal 0)) (authored-target "LogicalElement") (range (start (line 6) (character 30)) (end (line 6) (character 44))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::LogicalElement")))))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::providePower"))) (kind featureTyping) (ordinal 0)) (authored-target "ProvidePower") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::ProvidePower")))))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::providePower"))) (kind performSource) (ordinal 0)) (authored-target "Allocation Definition Example::LogicalModel::providePower::generateTorque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::providePower::generateTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::providePower::generateTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "GenerateTorque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::GenerateTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::torqueGenerator"))) (kind featureTyping) (ordinal 0)) (authored-target "TorqueGenerator") (range (start (line 12) (character 25)) (end (line 12) (character 40))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::TorqueGenerator")))))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Definition Example::LogicalModel::torqueGenerator"))) (kind performSource) (ordinal 0)) (authored-target "Allocation Definition Example::LogicalModel::torqueGenerator::providePower::generateTorque") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel"))) (kind allocateSource) (ordinal 0)) (authored-target "torqueGenerator") (range (start (line 35) (character 57)) (end (line 35) (character 72))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel"))) (kind allocateTarget) (ordinal 0)) (authored-target "powerTrain") (range (start (line 35) (character 76)) (end (line 35) (character 86))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::powerTrain")))))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "LogicalModel::*") (range (start (line 19) (character 17)) (end (line 19) (character 29))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical::logical"))) (kind featureTyping) (ordinal 0)) (authored-target "LogicalElement") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical::physical"))) (kind featureTyping) (ordinal 0)) (authored-target "PhysicalElement") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::PhysicalElement")))))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::PowerTrain"))) (kind specialization) (ordinal 0)) (authored-target "PhysicalElement") (range (start (line 22) (character 25)) (end (line 22) (character 40))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::PhysicalElement")))))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::powerTrain"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerTrain") (range (start (line 24) (character 20)) (end (line 24) (character 30))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::PowerTrain")))))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::powerTrain::engine"))) (kind performSource) (ordinal 0)) (authored-target "Allocation Definition Example::PhysicalModel::powerTrain::engine::providePower::generateTorque") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::torqueGenAlloc"))) (kind featureTyping) (ordinal 0)) (authored-target "LogicalToPhysical") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Allocation Definition Example::PhysicalModel::LogicalToPhysical")))))
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
