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
  (document "memory://snapshot/12b_allocation_1.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 44) (end 16 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 50 1) (end 54 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:ae82e7099dc27604c1368eaff5be48ff11690cd1b44014c410d0006167e757b7") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (path (named (kind package) (name "12b-Allocation-1")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "SI") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (path (named (kind package) (name "12b-Allocation-1")) (anonymous (kind import) (ordinal 1)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "RequirementModel") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (path (named (kind package) (name "12b-Allocation-1")) (anonymous (kind import) (ordinal 2)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "LogicalModel") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (path (named (kind package) (name "12b-Allocation-1")) (anonymous (kind import) (ordinal 3)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "PhysicalModel") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (path (named (kind package) (name "12b-Allocation-1")) (named (kind package) (name "LogicalModel")) (anonymous (kind satisfy) (ordinal 0)))))) (kind satisfy) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (satisfySource (reference "torqueGeneration")) (satisfyTarget (reference "torqueGenerator"))))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::GenerateTorque"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::GenerateTorque::torque"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ISQ::torque") (direction out))))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::LogicalElement"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "LogicalElement"))))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator::generateTorque"))) (kind perform-action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "GenerateTorque"))))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::providePower"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::providePower::generateTorque"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "GenerateTorque"))))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TorqueGenerator"))))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (path (named (kind package) (name "12b-Allocation-1")) (named (kind package) (name "LogicalModel")) (named (kind part) (name "torqueGenerator")) (anonymous (kind perform-action) (ordinal 0)))))) (kind perform-action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "generateTorque"))))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalToPhysical"))) (kind allocation-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalToPhysical::logical"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LogicalElement"))))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalToPhysical::physical"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PhysicalElement"))))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel::PhysicalElement"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel::PowerTrain"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "PhysicalElement"))))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel::powerTrain"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PowerTrain"))))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel::powerTrain::engine"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (path (named (kind package) (name "12b-Allocation-1")) (named (kind package) (name "PhysicalModel")) (named (kind part) (name "powerTrain")) (named (kind part) (name "engine")) (anonymous (kind perform-action) (ordinal 0)))))) (kind perform-action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::RequirementModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (path (named (kind package) (name "12b-Allocation-1")) (named (kind package) (name "RequirementModel")) (named (kind requirement) (name "torqueGeneration")) (anonymous (kind constraint) (ordinal 0)))))) (kind constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "generator::generateTorque::torque"))))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration::generator"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TorqueGenerator"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (path (named (kind package) (name "12b-Allocation-1")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "SI")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (path (named (kind package) (name "12b-Allocation-1")) (anonymous (kind import) (ordinal 1)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "RequirementModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::RequirementModel")))))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (path (named (kind package) (name "12b-Allocation-1")) (anonymous (kind import) (ordinal 2)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "LogicalModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel")))))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (path (named (kind package) (name "12b-Allocation-1")) (anonymous (kind import) (ordinal 3)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "PhysicalModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel")))))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (path (named (kind package) (name "12b-Allocation-1")) (named (kind package) (name "LogicalModel")) (anonymous (kind satisfy) (ordinal 0)))))) (kind satisfySource) (ordinal 0))
      (authored-target "torqueGeneration")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration")))))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (path (named (kind package) (name "12b-Allocation-1")) (named (kind package) (name "LogicalModel")) (anonymous (kind satisfy) (ordinal 0)))))) (kind satisfyTarget) (ordinal 0))
      (authored-target "torqueGenerator")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator")))))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::GenerateTorque::torque"))) (kind featureTyping) (ordinal 0))
      (authored-target "ISQ::torque")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator"))) (kind specialization) (ordinal 0))
      (authored-target "LogicalElement")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::LogicalElement")))))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator::generateTorque"))) (kind featureTyping) (ordinal 0))
      (authored-target "GenerateTorque")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::GenerateTorque")))))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::providePower::generateTorque"))) (kind featureTyping) (ordinal 0))
      (authored-target "GenerateTorque")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::GenerateTorque")))))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator"))) (kind featureTyping) (ordinal 0))
      (authored-target "TorqueGenerator")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator")))))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (path (named (kind package) (name "12b-Allocation-1")) (named (kind package) (name "LogicalModel")) (named (kind part) (name "torqueGenerator")) (anonymous (kind perform-action) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "generateTorque")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator::generateTorque")))))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalToPhysical::logical"))) (kind featureTyping) (ordinal 0))
      (authored-target "LogicalElement")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::LogicalElement")))))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalToPhysical::physical"))) (kind featureTyping) (ordinal 0))
      (authored-target "PhysicalElement")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel::PhysicalElement")))))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel::PowerTrain"))) (kind specialization) (ordinal 0))
      (authored-target "PhysicalElement")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel::PhysicalElement")))))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel::powerTrain"))) (kind featureTyping) (ordinal 0))
      (authored-target "PowerTrain")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel::PowerTrain")))))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (path (named (kind package) (name "12b-Allocation-1")) (named (kind package) (name "RequirementModel")) (named (kind requirement) (name "torqueGeneration")) (anonymous (kind constraint) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "generator::generateTorque::torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::GenerateTorque::torque")))))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration::generator"))) (kind featureTyping) (ordinal 0))
      (authored-target "TorqueGenerator")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator")))))
  )
  (relationships
    (relationship (kind satisfySource) (source (node (document "memory://snapshot/12b_allocation_1.md") (path (named (kind package) (name "12b-Allocation-1")) (named (kind package) (name "LogicalModel")) (anonymous (kind satisfy) (ordinal 0)))))) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/12b_allocation_1.md") (path (named (kind package) (name "12b-Allocation-1")) (named (kind package) (name "LogicalModel")) (anonymous (kind satisfy) (ordinal 0)))))) (kind satisfySource) (ordinal 0)))
    (relationship (kind satisfyTarget) (source (node (document "memory://snapshot/12b_allocation_1.md") (path (named (kind package) (name "12b-Allocation-1")) (named (kind package) (name "LogicalModel")) (anonymous (kind satisfy) (ordinal 0)))))) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/12b_allocation_1.md") (path (named (kind package) (name "12b-Allocation-1")) (named (kind package) (name "LogicalModel")) (anonymous (kind satisfy) (ordinal 0)))))) (kind satisfyTarget) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator"))) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::LogicalElement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator::generateTorque"))) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::GenerateTorque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator::generateTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::providePower::generateTorque"))) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::GenerateTorque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::providePower::generateTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator"))) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/12b_allocation_1.md") (path (named (kind package) (name "12b-Allocation-1")) (named (kind package) (name "LogicalModel")) (named (kind part) (name "torqueGenerator")) (anonymous (kind perform-action) (ordinal 0)))))) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator::generateTorque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/12b_allocation_1.md") (path (named (kind package) (name "12b-Allocation-1")) (named (kind package) (name "LogicalModel")) (named (kind part) (name "torqueGenerator")) (anonymous (kind perform-action) (ordinal 0)))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalToPhysical::logical"))) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::LogicalElement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalToPhysical::logical"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalToPhysical::physical"))) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel::PhysicalElement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalToPhysical::physical"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel::PowerTrain"))) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel::PhysicalElement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel::PowerTrain"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel::powerTrain"))) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel::PowerTrain"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel::powerTrain"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/12b_allocation_1.md") (path (named (kind package) (name "12b-Allocation-1")) (named (kind package) (name "RequirementModel")) (named (kind requirement) (name "torqueGeneration")) (anonymous (kind constraint) (ordinal 0)))))) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::GenerateTorque::torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/12b_allocation_1.md") (path (named (kind package) (name "12b-Allocation-1")) (named (kind package) (name "RequirementModel")) (named (kind requirement) (name "torqueGeneration")) (anonymous (kind constraint) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration::generator"))) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration::generator"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/12b_allocation_1.md") (range (start 1 16) (end 1 21)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (path (named (kind package) (name "12b-Allocation-1")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0) (authored-target "SI")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/12b_allocation_1.md") (range (start 2 16) (end 2 35)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (path (named (kind package) (name "12b-Allocation-1")) (anonymous (kind import) (ordinal 1)))))) (kind namespaceImport) (ordinal 0) (authored-target "RequirementModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::RequirementModel")))))
  )
  (query (document "memory://snapshot/12b_allocation_1.md") (range (start 3 16) (end 3 31)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (path (named (kind package) (name "12b-Allocation-1")) (anonymous (kind import) (ordinal 2)))))) (kind namespaceImport) (ordinal 0) (authored-target "LogicalModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel")))))
  )
  (query (document "memory://snapshot/12b_allocation_1.md") (range (start 4 16) (end 4 32)) (probe (position 4 16))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (path (named (kind package) (name "12b-Allocation-1")) (anonymous (kind import) (ordinal 3)))))) (kind namespaceImport) (ordinal 0) (authored-target "PhysicalModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel")))))
  )
  (query (document "memory://snapshot/12b_allocation_1.md") (range (start 31 10) (end 31 26)) (probe (position 31 10))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (path (named (kind package) (name "12b-Allocation-1")) (named (kind package) (name "LogicalModel")) (anonymous (kind satisfy) (ordinal 0)))))) (kind satisfySource) (ordinal 0) (authored-target "torqueGeneration")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration")))))
  )
  (query (document "memory://snapshot/12b_allocation_1.md") (range (start 31 30) (end 31 45)) (probe (position 31 30))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (path (named (kind package) (name "12b-Allocation-1")) (named (kind package) (name "LogicalModel")) (anonymous (kind satisfy) (ordinal 0)))))) (kind satisfyTarget) (ordinal 0) (authored-target "torqueGenerator")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator")))))
  )
  (query (document "memory://snapshot/12b_allocation_1.md") (range (start 16 44) (end 16 55)) (probe (position 16 44))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::GenerateTorque::torque"))) (kind featureTyping) (ordinal 0) (authored-target "ISQ::torque")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/12b_allocation_1.md") (range (start 19 30) (end 19 44)) (probe (position 19 30))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator"))) (kind specialization) (ordinal 0) (authored-target "LogicalElement")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::LogicalElement")))))
  )
  (query (document "memory://snapshot/12b_allocation_1.md") (range (start 20 35) (end 20 49)) (probe (position 20 35))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator::generateTorque"))) (kind featureTyping) (ordinal 0) (authored-target "GenerateTorque")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::GenerateTorque")))))
  )
  (query (document "memory://snapshot/12b_allocation_1.md") (range (start 24 27) (end 24 41)) (probe (position 24 27))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::providePower::generateTorque"))) (kind featureTyping) (ordinal 0) (authored-target "GenerateTorque")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::GenerateTorque")))))
  )
  (query (document "memory://snapshot/12b_allocation_1.md") (range (start 27 25) (end 27 40)) (probe (position 27 25))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator"))) (kind featureTyping) (ordinal 0) (authored-target "TorqueGenerator")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator")))))
  )
  (query (document "memory://snapshot/12b_allocation_1.md") (range (start 28 43) (end 28 57)) (probe (position 28 43))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (path (named (kind package) (name "12b-Allocation-1")) (named (kind package) (name "LogicalModel")) (named (kind part) (name "torqueGenerator")) (anonymous (kind perform-action) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "generateTorque")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator::generateTorque")))))
  )
  (query (document "memory://snapshot/12b_allocation_1.md") (range (start 46 16) (end 46 30)) (probe (position 46 16))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalToPhysical::logical"))) (kind featureTyping) (ordinal 0) (authored-target "LogicalElement")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::LogicalElement")))))
  )
  (query (document "memory://snapshot/12b_allocation_1.md") (range (start 47 17) (end 47 32)) (probe (position 47 17))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalToPhysical::physical"))) (kind featureTyping) (ordinal 0) (authored-target "PhysicalElement")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel::PhysicalElement")))))
  )
  (query (document "memory://snapshot/12b_allocation_1.md") (range (start 36 25) (end 36 40)) (probe (position 36 25))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel::PowerTrain"))) (kind specialization) (ordinal 0) (authored-target "PhysicalElement")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel::PhysicalElement")))))
  )
  (query (document "memory://snapshot/12b_allocation_1.md") (range (start 38 20) (end 38 30)) (probe (position 38 20))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel::powerTrain"))) (kind featureTyping) (ordinal 0) (authored-target "PowerTrain")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel::PowerTrain")))))
  )
  (query (document "memory://snapshot/12b_allocation_1.md") (range (start 10 5) (end 10 36)) (probe (position 10 5))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (path (named (kind package) (name "12b-Allocation-1")) (named (kind package) (name "RequirementModel")) (named (kind requirement) (name "torqueGeneration")) (anonymous (kind constraint) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 0) (authored-target "generator::generateTorque::torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::GenerateTorque::torque")))))
  )
  (query (document "memory://snapshot/12b_allocation_1.md") (range (start 8 22) (end 8 37)) (probe (position 8 22))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration::generator"))) (kind featureTyping) (ordinal 0) (authored-target "TorqueGenerator")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator")))))
  )
)
~~~
