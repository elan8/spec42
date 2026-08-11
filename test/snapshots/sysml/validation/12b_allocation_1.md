# META
~~~ini
description=SysML Validation (12-Dependency Relationships): 12b-Allocation-1
type=file
~~~
# SOURCE
~~~sysml
package '12b-Allocation-1' {
	private import SI::*;
	private import RequirementModel::*;
	private import LogicalModel::*;
	private import PhysicalModel::*;
	
	package RequirementModel {
		requirement torqueGeneration {
			subject generator: TorqueGenerator;
			require constraint { 
				 generator.generateTorque.torque > 0.0 [N*m]
			}
		}
	}
	
	package LogicalModel {
		action def GenerateTorque { out torque :> ISQ::torque; }
		
		part def LogicalElement;
		part def TorqueGenerator :> LogicalElement {
			perform action generateTorque : GenerateTorque;
		}	
		
		action providePower {
			action generateTorque : GenerateTorque;
		}
		
		part torqueGenerator : TorqueGenerator {
			perform providePower.generateTorque :>> generateTorque;
		}
		
		satisfy torqueGeneration by torqueGenerator;			
	}
	
	package PhysicalModel {
		part def PhysicalElement;
		part def PowerTrain :> PhysicalElement;
		
		part powerTrain : PowerTrain {
			part engine {
				perform providePower.generateTorque;
			}
		}
	}
	
	allocation def LogicalToPhysical {
		end logical : LogicalElement;
		end physical : PhysicalElement;
	}
	
	allocation torqueGenAlloc : LogicalToPhysical 
		allocate logical ::> torqueGenerator to physical ::> powerTrain {
			
		allocate torqueGenerator.generateTorque to powerTrain.engine.generateTorque;		
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "12b_allocation_1.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 30) (end 16 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 27 2) (end 27 105))
      )
      (diagnostic
        (severity warning)
        (code "satisfy_target_invalid_kind")
        (source "semantic")
        (range (start 31 10) (end 31 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 39 3) (end 39 62))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package '12b-Allocation-1' {
    private import SI::*;
    private import RequirementModel::*;
    private import LogicalModel::*;
    private import PhysicalModel::*;

    package RequirementModel {
        requirement torqueGeneration {
            subject generator: TorqueGenerator;
            require constraint {
                generator.generateTorque.torque > 0.0 [N*m]
            }
        }
    }

    package LogicalModel {
        action def GenerateTorque { out torque :> ISQ::torque; }

        part def LogicalElement;
        part def TorqueGenerator :> LogicalElement {
            perform action generateTorque : GenerateTorque;
        }

        action providePower {
            action generateTorque : GenerateTorque;
        }

        part torqueGenerator : TorqueGenerator {
            perform providePower.generateTorque :>> generateTorque;
        }

        satisfy torqueGeneration by torqueGenerator;
    }

    package PhysicalModel {
        part def PhysicalElement;
        part def PowerTrain :> PhysicalElement;

        part powerTrain : PowerTrain {
            part engine {
                perform providePower.generateTorque;
            }
        }
    }

    allocation def LogicalToPhysical {
        end logical : LogicalElement;
        end physical : PhysicalElement;
    }

    allocation torqueGenAlloc : LogicalToPhysical
    allocate logical ::> torqueGenerator to physical ::> powerTrain {

        allocate torqueGenerator.generateTorque to powerTrain.engine.generateTorque;
    }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "d6f6c48a9d1623c96251490305d9045664f536f1d93997e6a3152c3ee0b2feb2") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1"))) (kind "package") (name "12b-Allocation-1") (declared-name "12b-Allocation-1") (range (start (line 0) (character 0)) (end (line 0) (character 1318))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 22))) (parent (node (document "d0") (qualified-name "12b-Allocation-1"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 18))))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 36))) (parent (node (document "d0") (qualified-name "12b-Allocation-1"))) (authored (membership (kind Import) (visibility "private") (import (reference "RequirementModel::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 32))))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 1)) (end (line 3) (character 32))) (parent (node (document "d0") (qualified-name "12b-Allocation-1"))) (authored (membership (kind Import) (visibility "private") (import (reference "LogicalModel::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 16)) (end (line 3) (character 28))))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::*#import3"))) (kind "import") (name "*") (declared-name "*") (range (start (line 4) (character 1)) (end (line 4) (character 33))) (parent (node (document "d0") (qualified-name "12b-Allocation-1"))) (authored (membership (kind Import) (visibility "private") (import (reference "PhysicalModel::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 4) (character 16)) (end (line 4) (character 29))))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel"))) (kind "package") (name "LogicalModel") (declared-name "LogicalModel") (range (start (line 15) (character 1)) (end (line 15) (character 454))) (parent (node (document "d0") (qualified-name "12b-Allocation-1"))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::GenerateTorque"))) (kind "action def") (name "GenerateTorque") (declared-name "GenerateTorque") (range (start (line 16) (character 2)) (end (line 16) (character 58))) (parent (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel"))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::GenerateTorque::torque"))) (kind "in out parameter") (name "torque") (declared-name "torque") (range (start (line 16) (character 30)) (end (line 16) (character 56))) (parent (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::GenerateTorque"))) (authored (relationships (typing (reference "ISQ::torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::LogicalElement"))) (kind "part def") (name "LogicalElement") (declared-name "LogicalElement") (range (start (line 18) (character 2)) (end (line 18) (character 26))) (parent (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel"))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator"))) (kind "part def") (name "TorqueGenerator") (declared-name "TorqueGenerator") (range (start (line 19) (character 2)) (end (line 19) (character 101))) (parent (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel"))) (authored (membership (kind Owning)) (relationships (specializes (reference "LogicalElement") (range (start (line 19) (character 30)) (end (line 19) (character 44)))) (perform (reference "12b-Allocation-1::LogicalModel::TorqueGenerator::generateTorque") (range none)))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator::generateTorque"))) (kind "action") (name "generateTorque") (declared-name "generateTorque") (range (start (line 20) (character 3)) (end (line 20) (character 50))) (parent (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator"))) (authored (relationships (typing (reference "GenerateTorque") (range none)))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::providePower"))) (kind "action") (name "providePower") (declared-name "providePower") (range (start (line 23) (character 2)) (end (line 23) (character 70))) (parent (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel"))) (authored (membership (kind Feature)) (relationships (perform (reference "12b-Allocation-1::LogicalModel::providePower::generateTorque") (range none)))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::providePower::generateTorque"))) (kind "action") (name "generateTorque") (declared-name "generateTorque") (range (start (line 24) (character 3)) (end (line 24) (character 42))) (parent (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::providePower"))) (authored (membership (kind Feature)) (relationships (typing (reference "GenerateTorque") (range none)))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator"))) (kind "part") (name "torqueGenerator") (declared-name "torqueGenerator") (range (start (line 27) (character 2)) (end (line 27) (character 105))) (parent (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "TorqueGenerator") (range (start (line 27) (character 25)) (end (line 27) (character 40)))) (perform (reference "12b-Allocation-1::LogicalModel::torqueGenerator::providePower::generateTorque") (range none)))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator::providePower.generateTorque"))) (kind "action") (name "providePower.generateTorque") (declared-name "providePower.generateTorque") (range (start (line 28) (character 3)) (end (line 28) (character 58))) (parent (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator"))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::LogicalToPhysical"))) (kind "allocation def") (name "LogicalToPhysical") (declared-name "LogicalToPhysical") (range (start (line 45) (character 1)) (end (line 45) (character 104))) (parent (node (document "d0") (qualified-name "12b-Allocation-1"))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::LogicalToPhysical::logical"))) (kind "interface end") (name "logical") (declared-name "logical") (range (start (line 46) (character 2)) (end (line 46) (character 31))) (parent (node (document "d0") (qualified-name "12b-Allocation-1::LogicalToPhysical"))) (authored (relationships (typing (reference "LogicalElement") (range none)))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::LogicalToPhysical::physical"))) (kind "interface end") (name "physical") (declared-name "physical") (range (start (line 47) (character 2)) (end (line 47) (character 33))) (parent (node (document "d0") (qualified-name "12b-Allocation-1::LogicalToPhysical"))) (authored (relationships (typing (reference "PhysicalElement") (range none)))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel"))) (kind "package") (name "PhysicalModel") (declared-name "PhysicalModel") (range (start (line 34) (character 1)) (end (line 34) (character 200))) (parent (node (document "d0") (qualified-name "12b-Allocation-1"))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::PhysicalElement"))) (kind "part def") (name "PhysicalElement") (declared-name "PhysicalElement") (range (start (line 35) (character 2)) (end (line 35) (character 27))) (parent (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel"))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::PowerTrain"))) (kind "part def") (name "PowerTrain") (declared-name "PowerTrain") (range (start (line 36) (character 2)) (end (line 36) (character 41))) (parent (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel"))) (authored (membership (kind Owning)) (relationships (specializes (reference "PhysicalElement") (range (start (line 36) (character 25)) (end (line 36) (character 40)))))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::powerTrain"))) (kind "part") (name "powerTrain") (declared-name "powerTrain") (range (start (line 38) (character 2)) (end (line 38) (character 99))) (parent (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "PowerTrain") (range (start (line 38) (character 20)) (end (line 38) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::powerTrain::engine"))) (kind "part") (name "engine") (declared-name "engine") (range (start (line 39) (character 3)) (end (line 39) (character 62))) (parent (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::powerTrain"))) (authored (membership (kind Feature)) (relationships (perform (reference "12b-Allocation-1::PhysicalModel::powerTrain::engine::providePower::generateTorque") (range none)))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::powerTrain::engine::providePower.generateTorque"))) (kind "action") (name "providePower.generateTorque") (declared-name "providePower.generateTorque") (range (start (line 40) (character 4)) (end (line 40) (character 40))) (parent (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::powerTrain::engine"))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel"))) (kind "package") (name "RequirementModel") (declared-name "RequirementModel") (range (start (line 6) (character 1)) (end (line 6) (character 185))) (parent (node (document "d0") (qualified-name "12b-Allocation-1"))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration"))) (kind "requirement") (name "torqueGeneration") (declared-name "torqueGeneration") (range (start (line 7) (character 2)) (end (line 7) (character 154))) (parent (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel"))) (authored (membership (kind Feature)) (relationships (subject (reference "12b-Allocation-1::RequirementModel::torqueGeneration::generator") (range none)))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration::_requireConstraint_0"))) (kind "require constraint") (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (range (start (line 9) (character 3)) (end (line 9) (character 78))) (parent (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration"))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration::generator"))) (kind "subject") (name "generator") (declared-name "generator") (range (start (line 8) (character 3)) (end (line 8) (character 38))) (parent (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration"))) (authored (relationships (typing (reference "TorqueGenerator") (range none)))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::torqueGenAlloc"))) (kind "allocation") (name "torqueGenAlloc") (declared-name "torqueGenAlloc") (range (start (line 50) (character 1)) (end (line 50) (character 203))) (parent (node (document "d0") (qualified-name "12b-Allocation-1"))) (authored (membership (kind Feature)) (relationships (typing (reference "LogicalToPhysical") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1"))) (kind allocateSource) (ordinal 0)) (authored-target "torqueGenerator") (range (start (line 51) (character 23)) (end (line 51) (character 38))) (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1"))) (kind allocateTarget) (ordinal 0)) (authored-target "powerTrain") (range (start (line 51) (character 55)) (end (line 51) (character 65))) (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::powerTrain")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (range (start (line 1) (character 16)) (end (line 1) (character 18))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "RequirementModel::*") (range (start (line 2) (character 16)) (end (line 2) (character 32))) (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "LogicalModel::*") (range (start (line 3) (character 16)) (end (line 3) (character 28))) (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::*#import3"))) (kind namespaceImport) (ordinal 0)) (authored-target "PhysicalModel::*") (range (start (line 4) (character 16)) (end (line 4) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel"))) (kind satisfySource) (ordinal 0)) (authored-target "torqueGeneration") (range (start (line 31) (character 10)) (end (line 31) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel"))) (kind satisfyTarget) (ordinal 0)) (authored-target "torqueGenerator") (range (start (line 31) (character 30)) (end (line 31) (character 45))) (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::GenerateTorque::torque"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::torque") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator"))) (kind specialization) (ordinal 0)) (authored-target "LogicalElement") (range (start (line 19) (character 30)) (end (line 19) (character 44))) (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::LogicalElement")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator"))) (kind performSource) (ordinal 0)) (authored-target "12b-Allocation-1::LogicalModel::TorqueGenerator::generateTorque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator::generateTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator::generateTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "GenerateTorque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::GenerateTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::providePower"))) (kind performSource) (ordinal 0)) (authored-target "12b-Allocation-1::LogicalModel::providePower::generateTorque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::providePower::generateTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::providePower::generateTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "GenerateTorque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::GenerateTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator"))) (kind featureTyping) (ordinal 0)) (authored-target "TorqueGenerator") (range (start (line 27) (character 25)) (end (line 27) (character 40))) (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator"))) (kind performSource) (ordinal 0)) (authored-target "12b-Allocation-1::LogicalModel::torqueGenerator::providePower::generateTorque") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalToPhysical::logical"))) (kind featureTyping) (ordinal 0)) (authored-target "LogicalElement") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::LogicalElement")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalToPhysical::physical"))) (kind featureTyping) (ordinal 0)) (authored-target "PhysicalElement") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::PhysicalElement")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::PowerTrain"))) (kind specialization) (ordinal 0)) (authored-target "PhysicalElement") (range (start (line 36) (character 25)) (end (line 36) (character 40))) (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::PhysicalElement")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::powerTrain"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerTrain") (range (start (line 38) (character 20)) (end (line 38) (character 30))) (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::PowerTrain")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::powerTrain::engine"))) (kind performSource) (ordinal 0)) (authored-target "12b-Allocation-1::PhysicalModel::powerTrain::engine::providePower::generateTorque") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "12b-Allocation-1::RequirementModel::torqueGeneration::generator") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration::generator")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration::generator"))) (kind featureTyping) (ordinal 0)) (authored-target "TorqueGenerator") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::torqueGenAlloc"))) (kind featureTyping) (ordinal 0)) (authored-target "LogicalToPhysical") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalToPhysical")))))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator"))) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::LogicalElement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator"))) (kind specialization) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator"))) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator::generateTorque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator"))) (kind performSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator::generateTorque"))) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::GenerateTorque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator::generateTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::providePower"))) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::providePower::generateTorque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::providePower"))) (kind performSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::providePower::generateTorque"))) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::GenerateTorque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::providePower::generateTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator"))) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind allocate) (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator"))) (target (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::powerTrain"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "12b-Allocation-1"))) (kind allocateSource) (ordinal 0)) (expression (kind allocate) (source "torqueGenerator") (target "powerTrain") (source-range (start (line 51) (character 23)) (end (line 51) (character 38))) (target-range (start (line 51) (character 55)) (end (line 51) (character 65)))))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalToPhysical::logical"))) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::LogicalElement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalToPhysical::logical"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalToPhysical::physical"))) (target (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::PhysicalElement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalToPhysical::physical"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::PowerTrain"))) (target (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::PhysicalElement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::PowerTrain"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::powerTrain"))) (target (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::PowerTrain"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::powerTrain"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration"))) (target (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration::generator"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind satisfy) (source (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration"))) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel"))) (kind satisfySource) (ordinal 0)) (expression (kind satisfy) (source "torqueGeneration") (target "torqueGenerator") (source-range (start (line 31) (character 10)) (end (line 31) (character 26))) (target-range (start (line 31) (character 30)) (end (line 31) (character 45)))))
    (relationship (kind subject) (source (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration"))) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration::generator"))) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration::generator"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "12b-Allocation-1::torqueGenAlloc"))) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalToPhysical"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "12b-Allocation-1::torqueGenAlloc"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration")) (expression (status "incomplete") (error "expression is incomplete")) (analysis (status "incomplete")))
    (node (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration::_requireConstraint_0")) (expression (status "incomplete") (error "expression is incomplete")))
  )
)
~~~
