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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "d6f6c48a9d1623c96251490305d9045664f536f1d93997e6a3152c3ee0b2feb2") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1"))) (kind "package") (name "12b-Allocation-1") (declared-name "12b-Allocation-1"))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "12b-Allocation-1"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "12b-Allocation-1"))) (authored (membership (kind Import) (visibility "private") (import (reference "RequirementModel::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::*#import2"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "12b-Allocation-1"))) (authored (membership (kind Import) (visibility "private") (import (reference "LogicalModel::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::*#import3"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "12b-Allocation-1"))) (authored (membership (kind Import) (visibility "private") (import (reference "PhysicalModel::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel"))) (kind "package") (name "LogicalModel") (declared-name "LogicalModel") (parent (node (document "d0") (qualified-name "12b-Allocation-1"))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::GenerateTorque"))) (kind "action def") (name "GenerateTorque") (declared-name "GenerateTorque") (parent (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel"))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::GenerateTorque::torque"))) (kind "in out parameter") (name "torque") (declared-name "torque") (parent (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::GenerateTorque"))) (authored (relationships (typing (reference "ISQ::torque")))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::LogicalElement"))) (kind "part def") (name "LogicalElement") (declared-name "LogicalElement") (parent (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel"))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator"))) (kind "part def") (name "TorqueGenerator") (declared-name "TorqueGenerator") (parent (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel"))) (authored (membership (kind Owning)) (relationships (specializes (reference "LogicalElement")) (perform (reference "12b-Allocation-1::LogicalModel::TorqueGenerator::generateTorque")))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator::generateTorque"))) (kind "action") (name "generateTorque") (declared-name "generateTorque") (parent (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator"))) (authored (relationships (typing (reference "GenerateTorque")))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::providePower"))) (kind "action") (name "providePower") (declared-name "providePower") (parent (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel"))) (authored (membership (kind Feature)) (relationships (perform (reference "12b-Allocation-1::LogicalModel::providePower::generateTorque")))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::providePower::generateTorque"))) (kind "action") (name "generateTorque") (declared-name "generateTorque") (parent (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::providePower"))) (authored (membership (kind Feature)) (relationships (typing (reference "GenerateTorque")))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator"))) (kind "part") (name "torqueGenerator") (declared-name "torqueGenerator") (parent (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "TorqueGenerator")) (perform (reference "12b-Allocation-1::LogicalModel::torqueGenerator::providePower::generateTorque")))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator::providePower.generateTorque"))) (kind "action") (name "providePower.generateTorque") (declared-name "providePower.generateTorque") (parent (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator"))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::LogicalToPhysical"))) (kind "allocation def") (name "LogicalToPhysical") (declared-name "LogicalToPhysical") (parent (node (document "d0") (qualified-name "12b-Allocation-1"))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::LogicalToPhysical::logical"))) (kind "interface end") (name "logical") (declared-name "logical") (parent (node (document "d0") (qualified-name "12b-Allocation-1::LogicalToPhysical"))) (authored (relationships (typing (reference "LogicalElement")))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::LogicalToPhysical::physical"))) (kind "interface end") (name "physical") (declared-name "physical") (parent (node (document "d0") (qualified-name "12b-Allocation-1::LogicalToPhysical"))) (authored (relationships (typing (reference "PhysicalElement")))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel"))) (kind "package") (name "PhysicalModel") (declared-name "PhysicalModel") (parent (node (document "d0") (qualified-name "12b-Allocation-1"))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::PhysicalElement"))) (kind "part def") (name "PhysicalElement") (declared-name "PhysicalElement") (parent (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel"))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::PowerTrain"))) (kind "part def") (name "PowerTrain") (declared-name "PowerTrain") (parent (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel"))) (authored (membership (kind Owning)) (relationships (specializes (reference "PhysicalElement")))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::powerTrain"))) (kind "part") (name "powerTrain") (declared-name "powerTrain") (parent (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "PowerTrain")))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::powerTrain::engine"))) (kind "part") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::powerTrain"))) (authored (membership (kind Feature)) (relationships (perform (reference "12b-Allocation-1::PhysicalModel::powerTrain::engine::providePower::generateTorque")))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::powerTrain::engine::providePower.generateTorque"))) (kind "action") (name "providePower.generateTorque") (declared-name "providePower.generateTorque") (parent (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::powerTrain::engine"))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel"))) (kind "package") (name "RequirementModel") (declared-name "RequirementModel") (parent (node (document "d0") (qualified-name "12b-Allocation-1"))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration"))) (kind "requirement") (name "torqueGeneration") (declared-name "torqueGeneration") (parent (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel"))) (authored (membership (kind Feature)) (relationships (subject (reference "12b-Allocation-1::RequirementModel::torqueGeneration::generator")))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration::_requireConstraint_0"))) (kind "require constraint") (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (parent (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration"))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration::generator"))) (kind "subject") (name "generator") (declared-name "generator") (parent (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration"))) (authored (relationships (typing (reference "TorqueGenerator")))))
    (element (id (node (document "d0") (qualified-name "12b-Allocation-1::torqueGenAlloc"))) (kind "allocation") (name "torqueGenAlloc") (declared-name "torqueGenAlloc") (parent (node (document "d0") (qualified-name "12b-Allocation-1"))) (authored (membership (kind Feature)) (relationships (typing (reference "LogicalToPhysical")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1"))) (kind allocateSource) (ordinal 0)) (authored-target "torqueGenerator") (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1"))) (kind allocateTarget) (ordinal 0)) (authored-target "powerTrain") (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::powerTrain")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "RequirementModel::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "LogicalModel::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::*#import3"))) (kind namespaceImport) (ordinal 0)) (authored-target "PhysicalModel::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel"))) (kind satisfySource) (ordinal 0)) (authored-target "torqueGeneration") (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel"))) (kind satisfyTarget) (ordinal 0)) (authored-target "torqueGenerator") (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::GenerateTorque::torque"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::torque") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator"))) (kind specialization) (ordinal 0)) (authored-target "LogicalElement") (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::LogicalElement")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator"))) (kind performSource) (ordinal 0)) (authored-target "12b-Allocation-1::LogicalModel::TorqueGenerator::generateTorque") (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator::generateTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator::generateTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "GenerateTorque") (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::GenerateTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::providePower"))) (kind performSource) (ordinal 0)) (authored-target "12b-Allocation-1::LogicalModel::providePower::generateTorque") (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::providePower::generateTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::providePower::generateTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "GenerateTorque") (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::GenerateTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator"))) (kind featureTyping) (ordinal 0)) (authored-target "TorqueGenerator") (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator"))) (kind performSource) (ordinal 0)) (authored-target "12b-Allocation-1::LogicalModel::torqueGenerator::providePower::generateTorque") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalToPhysical::logical"))) (kind featureTyping) (ordinal 0)) (authored-target "LogicalElement") (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::LogicalElement")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalToPhysical::physical"))) (kind featureTyping) (ordinal 0)) (authored-target "PhysicalElement") (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::PhysicalElement")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::PowerTrain"))) (kind specialization) (ordinal 0)) (authored-target "PhysicalElement") (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::PhysicalElement")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::powerTrain"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerTrain") (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::PowerTrain")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::powerTrain::engine"))) (kind performSource) (ordinal 0)) (authored-target "12b-Allocation-1::PhysicalModel::powerTrain::engine::providePower::generateTorque") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "12b-Allocation-1::RequirementModel::torqueGeneration::generator") (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration::generator")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration::generator"))) (kind featureTyping) (ordinal 0)) (authored-target "TorqueGenerator") (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator")))))
    (reference (id (source (node (document "d0") (qualified-name "12b-Allocation-1::torqueGenAlloc"))) (kind featureTyping) (ordinal 0)) (authored-target "LogicalToPhysical") (outcome (status resolved) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalToPhysical")))))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator"))) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::LogicalElement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator"))) (kind specialization) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator"))) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator::generateTorque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator"))) (kind performSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator::generateTorque"))) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::GenerateTorque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator::generateTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::providePower"))) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::providePower::generateTorque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::providePower"))) (kind performSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::providePower::generateTorque"))) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::GenerateTorque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::providePower::generateTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator"))) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind allocate) (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator"))) (target (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::powerTrain"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "12b-Allocation-1"))) (kind allocateSource) (ordinal 0)) (expression (kind allocate) (source "torqueGenerator") (target "powerTrain")))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalToPhysical::logical"))) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::LogicalElement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalToPhysical::logical"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalToPhysical::physical"))) (target (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::PhysicalElement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalToPhysical::physical"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::PowerTrain"))) (target (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::PhysicalElement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::PowerTrain"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::powerTrain"))) (target (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::PowerTrain"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::powerTrain"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration"))) (target (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration::generator"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind satisfy) (source (node (document "d0") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration"))) (target (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "12b-Allocation-1::LogicalModel"))) (kind satisfySource) (ordinal 0)) (expression (kind satisfy) (source "torqueGeneration") (target "torqueGenerator")))
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
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 1 16) (end 1 18)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "12b-Allocation-1::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "SI::*")
        (range (start 1 16) (end 1 18))
        (outcome (status unresolved))
      )
    )
    (query (range (start 38 20) (end 38 30)) (probe (position 38 20))
      (reference
        (source (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::powerTrain"))
        (kind featureTyping) (ordinal 0) (authored-target "PowerTrain")
        (range (start 38 20) (end 38 30))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::PowerTrain") (range (start 36 2) (end 36 41)))
        )
      )
    )
    (query (range (start 51 55) (end 51 65)) (probe (position 51 55))
      (reference
        (source (document "d0") (qualified-name "12b-Allocation-1"))
        (kind allocateTarget) (ordinal 0) (authored-target "powerTrain")
        (range (start 51 55) (end 51 65))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::powerTrain") (range (start 38 2) (end 38 99)))
        )
      )
    )
    (query (range (start 3 16) (end 3 28)) (probe (position 3 16))
      (reference
        (source (document "d0") (qualified-name "12b-Allocation-1::*#import2"))
        (kind namespaceImport) (ordinal 0) (authored-target "LogicalModel::*")
        (range (start 3 16) (end 3 28))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "12b-Allocation-1::LogicalModel") (range (start 15 1) (end 15 454)))
        )
      )
    )
    (query (range (start 4 16) (end 4 29)) (probe (position 4 16))
      (reference
        (source (document "d0") (qualified-name "12b-Allocation-1::*#import3"))
        (kind namespaceImport) (ordinal 0) (authored-target "PhysicalModel::*")
        (range (start 4 16) (end 4 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel") (range (start 34 1) (end 34 200)))
        )
      )
    )
    (query (range (start 19 30) (end 19 44)) (probe (position 19 30))
      (reference
        (source (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator"))
        (kind specialization) (ordinal 0) (authored-target "LogicalElement")
        (range (start 19 30) (end 19 44))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::LogicalElement") (range (start 18 2) (end 18 26)))
        )
      )
    )
    (query (range (start 27 25) (end 27 40)) (probe (position 27 25))
      (reference
        (source (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator"))
        (kind featureTyping) (ordinal 0) (authored-target "TorqueGenerator")
        (range (start 27 25) (end 27 40))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator") (range (start 19 2) (end 19 101)))
        )
      )
    )
    (query (range (start 31 30) (end 31 45)) (probe (position 31 30))
      (reference
        (source (document "d0") (qualified-name "12b-Allocation-1::LogicalModel"))
        (kind satisfyTarget) (ordinal 0) (authored-target "torqueGenerator")
        (range (start 31 30) (end 31 45))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator") (range (start 27 2) (end 27 105)))
        )
      )
    )
    (query (range (start 36 25) (end 36 40)) (probe (position 36 25))
      (reference
        (source (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::PowerTrain"))
        (kind specialization) (ordinal 0) (authored-target "PhysicalElement")
        (range (start 36 25) (end 36 40))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "12b-Allocation-1::PhysicalModel::PhysicalElement") (range (start 35 2) (end 35 27)))
        )
      )
    )
    (query (range (start 51 23) (end 51 38)) (probe (position 51 23))
      (reference
        (source (document "d0") (qualified-name "12b-Allocation-1"))
        (kind allocateSource) (ordinal 0) (authored-target "torqueGenerator")
        (range (start 51 23) (end 51 38))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator") (range (start 27 2) (end 27 105)))
        )
      )
    )
    (query (range (start 2 16) (end 2 32)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "12b-Allocation-1::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "RequirementModel::*")
        (range (start 2 16) (end 2 32))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "12b-Allocation-1::RequirementModel") (range (start 6 1) (end 6 185)))
        )
      )
    )
    (query (range (start 31 10) (end 31 26)) (probe (position 31 10))
      (reference
        (source (document "d0") (qualified-name "12b-Allocation-1::LogicalModel"))
        (kind satisfySource) (ordinal 0) (authored-target "torqueGeneration")
        (range (start 31 10) (end 31 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration") (range (start 7 2) (end 7 154)))
        )
      )
    )
  )
)
~~~
