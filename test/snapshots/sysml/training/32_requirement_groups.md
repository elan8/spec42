# META
~~~ini
description=SysML Training 32 (Requirements): Requirement Groups
type=file
~~~
# SOURCE
~~~sysml
package 'Requirement Groups' {
	private import 'Requirement Definitions'::*;
	private import 'Requirement Usages'::*;
	
	part def Engine {
		port clutchPort: ClutchPort;
		perform action generateTorque: GenerateTorque;
	}
	
	requirement vehicleSpecification {
		doc /* Overall vehicle requirements group */
		
		subject vehicle : Vehicle;
		
		require fullVehicleMassLimit;
		require emptyVehicleMassLimit;
	}
	
	requirement engineSpecification {
		doc /* Engine power requirements group */
		
		subject engine : Engine;
		
		requirement drivePowerInterface : DrivePowerInterface {
			subject = engine.clutchPort;
		}
		
		requirement torqueGeneration : TorqueGeneration {
			subject = engine.generateTorque;	
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "32_requirement_groups.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 2) (end 5 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 2) (end 6 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 2) (end 12 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 23 2) (end 23 93))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 27 2) (end 27 92))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Requirement Groups' {
    private import 'Requirement Definitions'::*;
    private import 'Requirement Usages'::*;

    part def Engine {
        port clutchPort: ClutchPort;
        perform action generateTorque: GenerateTorque;
    }

    requirement vehicleSpecification {
        doc /* Overall vehicle requirements group */

        subject vehicle : Vehicle;

        require fullVehicleMassLimit;
        require emptyVehicleMassLimit;
    }

    requirement engineSpecification {
        doc /* Engine power requirements group */

        subject engine : Engine;

        requirement drivePowerInterface : DrivePowerInterface {
            subject = engine.clutchPort;
        }

        requirement torqueGeneration : TorqueGeneration {
            subject = engine.generateTorque;
        }
    }

}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "09c595a4f00015d58be76be18847de257932085200f2170de9b6d451b444c8da") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Requirement Groups"))) (kind "package") (name "Requirement Groups") (declared-name "Requirement Groups") (range (start (line 0) (character 0)) (end (line 0) (character 720))))
    (element (id (node (document "d0") (qualified-name "Requirement Groups::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 45))) (parent (node (document "d0") (qualified-name "Requirement Groups"))) (authored (membership (kind Import) (visibility "private") (import (reference "Requirement Definitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 41))))))
    (element (id (node (document "d0") (qualified-name "Requirement Groups::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 40))) (parent (node (document "d0") (qualified-name "Requirement Groups"))) (authored (membership (kind Import) (visibility "private") (import (reference "Requirement Usages::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 36))))))
    (element (id (node (document "d0") (qualified-name "Requirement Groups::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (range (start (line 4) (character 1)) (end (line 4) (character 101))) (parent (node (document "d0") (qualified-name "Requirement Groups"))) (authored (membership (kind Owning)) (relationships (perform (reference "Requirement Groups::Engine::generateTorque") (range none)))))
    (element (id (node (document "d0") (qualified-name "Requirement Groups::Engine::clutchPort"))) (kind "port") (name "clutchPort") (declared-name "clutchPort") (range (start (line 5) (character 2)) (end (line 5) (character 30))) (parent (node (document "d0") (qualified-name "Requirement Groups::Engine"))) (authored (membership (kind Feature)) (relationships (typing (reference "ClutchPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "Requirement Groups::Engine::generateTorque"))) (kind "action") (name "generateTorque") (declared-name "generateTorque") (range (start (line 6) (character 2)) (end (line 6) (character 48))) (parent (node (document "d0") (qualified-name "Requirement Groups::Engine"))) (authored (relationships (typing (reference "GenerateTorque") (range none)))))
    (element (id (node (document "d0") (qualified-name "Requirement Groups::engineSpecification"))) (kind "requirement") (name "engineSpecification") (declared-name "engineSpecification") (range (start (line 18) (character 1)) (end (line 18) (character 304))) (parent (node (document "d0") (qualified-name "Requirement Groups"))) (authored (membership (kind Feature)) (relationships (subject (reference "Requirement Groups::engineSpecification::engine") (range none)))))
    (element (id (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::_documentation"))) (kind "documentation") (name "") (range (start (line 18) (character 1)) (end (line 18) (character 304))) (parent (node (document "d0") (qualified-name "Requirement Groups::engineSpecification"))))
    (element (id (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::drivePowerInterface"))) (kind "requirement") (name "drivePowerInterface") (declared-name "drivePowerInterface") (range (start (line 23) (character 2)) (end (line 23) (character 93))) (parent (node (document "d0") (qualified-name "Requirement Groups::engineSpecification"))) (authored (membership (kind Feature)) (relationships (typing (reference "DrivePowerInterface") (range none)) (subject (reference "Requirement Groups::engineSpecification::drivePowerInterface::") (range none)))))
    (element (id (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::drivePowerInterface::"))) (kind "subject") (name "") (range (start (line 24) (character 3)) (end (line 24) (character 31))) (parent (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::drivePowerInterface"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::engine"))) (kind "subject") (name "engine") (declared-name "engine") (range (start (line 21) (character 2)) (end (line 21) (character 26))) (parent (node (document "d0") (qualified-name "Requirement Groups::engineSpecification"))) (authored (relationships (typing (reference "Engine") (range none)))))
    (element (id (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::torqueGeneration"))) (kind "requirement") (name "torqueGeneration") (declared-name "torqueGeneration") (range (start (line 27) (character 2)) (end (line 27) (character 92))) (parent (node (document "d0") (qualified-name "Requirement Groups::engineSpecification"))) (authored (membership (kind Feature)) (relationships (typing (reference "TorqueGeneration") (range none)) (subject (reference "Requirement Groups::engineSpecification::torqueGeneration::") (range none)))))
    (element (id (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::torqueGeneration::"))) (kind "subject") (name "") (range (start (line 28) (character 3)) (end (line 28) (character 35))) (parent (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::torqueGeneration"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Requirement Groups::vehicleSpecification"))) (kind "requirement") (name "vehicleSpecification") (declared-name "vehicleSpecification") (range (start (line 9) (character 1)) (end (line 9) (character 185))) (parent (node (document "d0") (qualified-name "Requirement Groups"))) (authored (membership (kind Feature)) (relationships (subject (reference "Requirement Groups::vehicleSpecification::vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "Requirement Groups::vehicleSpecification::_documentation"))) (kind "documentation") (name "") (range (start (line 9) (character 1)) (end (line 9) (character 185))) (parent (node (document "d0") (qualified-name "Requirement Groups::vehicleSpecification"))))
    (element (id (node (document "d0") (qualified-name "Requirement Groups::vehicleSpecification::_requireConstraint_0"))) (kind "require constraint") (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (range (start (line 14) (character 2)) (end (line 14) (character 31))) (parent (node (document "d0") (qualified-name "Requirement Groups::vehicleSpecification"))))
    (element (id (node (document "d0") (qualified-name "Requirement Groups::vehicleSpecification::_requireConstraint_1"))) (kind "require constraint") (name "_requireConstraint_1") (declared-name "_requireConstraint_1") (range (start (line 15) (character 2)) (end (line 15) (character 32))) (parent (node (document "d0") (qualified-name "Requirement Groups::vehicleSpecification"))))
    (element (id (node (document "d0") (qualified-name "Requirement Groups::vehicleSpecification::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (range (start (line 12) (character 2)) (end (line 12) (character 28))) (parent (node (document "d0") (qualified-name "Requirement Groups::vehicleSpecification"))) (authored (relationships (typing (reference "Vehicle") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Requirement Groups::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Requirement Definitions::*") (range (start (line 1) (character 16)) (end (line 1) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Groups::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Requirement Usages::*") (range (start (line 2) (character 16)) (end (line 2) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Groups::Engine"))) (kind performSource) (ordinal 0)) (authored-target "Requirement Groups::Engine::generateTorque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirement Groups::Engine::generateTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Groups::Engine::clutchPort"))) (kind featureTyping) (ordinal 0)) (authored-target "ClutchPort") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Groups::Engine::generateTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "GenerateTorque") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Groups::engineSpecification"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "Requirement Groups::engineSpecification::engine") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::drivePowerInterface"))) (kind featureTyping) (ordinal 0)) (authored-target "DrivePowerInterface") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::drivePowerInterface"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "Requirement Groups::engineSpecification::drivePowerInterface::") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::drivePowerInterface::")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::drivePowerInterface::"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::drivePowerInterface::")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirement Groups::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::torqueGeneration"))) (kind featureTyping) (ordinal 0)) (authored-target "TorqueGeneration") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::torqueGeneration"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "Requirement Groups::engineSpecification::torqueGeneration::") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::torqueGeneration::")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::torqueGeneration::"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::torqueGeneration::")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Groups::vehicleSpecification"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "Requirement Groups::vehicleSpecification::vehicle") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirement Groups::vehicleSpecification::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Groups::vehicleSpecification::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range none) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Requirement Groups::Engine"))) (target (node (document "d0") (qualified-name "Requirement Groups::Engine::generateTorque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirement Groups::Engine"))) (kind performSource) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "Requirement Groups::engineSpecification"))) (target (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirement Groups::engineSpecification"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind subject) (source (node (document "d0") (qualified-name "Requirement Groups::engineSpecification"))) (target (node (document "d0") (qualified-name "Requirement Groups::Engine"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::drivePowerInterface"))) (target (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::drivePowerInterface::"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::drivePowerInterface"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind subject) (source (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::drivePowerInterface"))) (target (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::drivePowerInterface::"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::drivePowerInterface::"))) (target (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::drivePowerInterface::"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::drivePowerInterface::"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::engine"))) (target (node (document "d0") (qualified-name "Requirement Groups::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::torqueGeneration"))) (target (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::torqueGeneration::"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::torqueGeneration"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind subject) (source (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::torqueGeneration"))) (target (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::torqueGeneration::"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::torqueGeneration::"))) (target (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::torqueGeneration::"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::torqueGeneration::"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "Requirement Groups::vehicleSpecification"))) (target (node (document "d0") (qualified-name "Requirement Groups::vehicleSpecification::vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirement Groups::vehicleSpecification"))) (kind referenceSubsetting) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 2 16) (end 2 36)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "Requirement Groups::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "Requirement Usages::*")
        (range (start 2 16) (end 2 36))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 41)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Requirement Groups::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Requirement Definitions::*")
        (range (start 1 16) (end 1 41))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
