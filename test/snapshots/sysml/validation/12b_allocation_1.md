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
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 8 3) (end 8 38))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 9 3) (end 11 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 16 30) (end 16 56))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 20 3) (end 20 50))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 28 3) (end 28 58))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 31 2) (end 31 46))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 40 4) (end 40 40))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 45 1) (end 48 2))
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
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "SI") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "RequirementModel") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "LogicalModel") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "PhysicalModel") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::GenerateTorque"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::LogicalElement"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "LogicalElement"))))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::providePower"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::providePower::generateTorque"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "GenerateTorque"))))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TorqueGenerator"))))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel::PhysicalElement"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel::PowerTrain"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "PhysicalElement"))))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel::powerTrain"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PowerTrain"))))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel::powerTrain::engine"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::RequirementModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::RequirementModel::torqueGeneration"))) (kind requirement) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "SI")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "RequirementModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::RequirementModel")))))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "LogicalModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel")))))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0))
      (authored-target "PhysicalModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel")))))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator"))) (kind specialization) (ordinal 0))
      (authored-target "LogicalElement")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::LogicalElement")))))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::providePower::generateTorque"))) (kind featureTyping) (ordinal 0))
      (authored-target "GenerateTorque")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::GenerateTorque")))))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator"))) (kind featureTyping) (ordinal 0))
      (authored-target "TorqueGenerator")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator")))))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel::PowerTrain"))) (kind specialization) (ordinal 0))
      (authored-target "PhysicalElement")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel::PhysicalElement")))))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel::powerTrain"))) (kind featureTyping) (ordinal 0))
      (authored-target "PowerTrain")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel::PowerTrain")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator"))) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::LogicalElement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::providePower::generateTorque"))) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::GenerateTorque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::providePower::generateTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator"))) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel::PowerTrain"))) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel::PhysicalElement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel::PowerTrain"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel::powerTrain"))) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel::PowerTrain"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel::powerTrain"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/12b_allocation_1.md") (range (start 1 16) (end 1 21)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "SI")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/12b_allocation_1.md") (range (start 2 16) (end 2 35)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "RequirementModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::RequirementModel")))))
  )
  (query (document "memory://snapshot/12b_allocation_1.md") (range (start 3 16) (end 3 31)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "LogicalModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel")))))
  )
  (query (document "memory://snapshot/12b_allocation_1.md") (range (start 4 16) (end 4 32)) (probe (position 4 16))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0) (authored-target "PhysicalModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel")))))
  )
  (query (document "memory://snapshot/12b_allocation_1.md") (range (start 19 30) (end 19 44)) (probe (position 19 30))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator"))) (kind specialization) (ordinal 0) (authored-target "LogicalElement")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::LogicalElement")))))
  )
  (query (document "memory://snapshot/12b_allocation_1.md") (range (start 24 27) (end 24 41)) (probe (position 24 27))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::providePower::generateTorque"))) (kind featureTyping) (ordinal 0) (authored-target "GenerateTorque")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::GenerateTorque")))))
  )
  (query (document "memory://snapshot/12b_allocation_1.md") (range (start 27 25) (end 27 40)) (probe (position 27 25))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::torqueGenerator"))) (kind featureTyping) (ordinal 0) (authored-target "TorqueGenerator")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::LogicalModel::TorqueGenerator")))))
  )
  (query (document "memory://snapshot/12b_allocation_1.md") (range (start 36 25) (end 36 40)) (probe (position 36 25))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel::PowerTrain"))) (kind specialization) (ordinal 0) (authored-target "PhysicalElement")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel::PhysicalElement")))))
  )
  (query (document "memory://snapshot/12b_allocation_1.md") (range (start 38 20) (end 38 30)) (probe (position 38 20))
    (reference (id (source (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel::powerTrain"))) (kind featureTyping) (ordinal 0) (authored-target "PowerTrain")
      (outcome (status resolved) (target (node (document "memory://snapshot/12b_allocation_1.md") (qualified-name "12b-Allocation-1::PhysicalModel::PowerTrain")))))
  )
)
~~~
