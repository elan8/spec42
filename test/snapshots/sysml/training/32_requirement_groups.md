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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "09c595a4f00015d58be76be18847de257932085200f2170de9b6d451b444c8da") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Requirement Groups"))) (kind "package") (name "Requirement Groups") (declared-name "Requirement Groups"))
    (element (id (node (document "d0") (qualified-name "Requirement Groups::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Requirement Groups"))) (authored (membership (kind Import) (visibility "private") (import (reference "Requirement Definitions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Requirement Groups::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Requirement Groups"))) (authored (membership (kind Import) (visibility "private") (import (reference "Requirement Usages::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Requirement Groups::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (parent (node (document "d0") (qualified-name "Requirement Groups"))) (authored (membership (kind Owning)) (relationships (perform (reference "Requirement Groups::Engine::generateTorque")))))
    (element (id (node (document "d0") (qualified-name "Requirement Groups::Engine::clutchPort"))) (kind "port") (name "clutchPort") (declared-name "clutchPort") (parent (node (document "d0") (qualified-name "Requirement Groups::Engine"))) (authored (membership (kind Feature)) (relationships (typing (reference "ClutchPort")))))
    (element (id (node (document "d0") (qualified-name "Requirement Groups::Engine::generateTorque"))) (kind "action") (name "generateTorque") (declared-name "generateTorque") (parent (node (document "d0") (qualified-name "Requirement Groups::Engine"))) (authored (relationships (typing (reference "GenerateTorque")))))
    (element (id (node (document "d0") (qualified-name "Requirement Groups::engineSpecification"))) (kind "requirement") (name "engineSpecification") (declared-name "engineSpecification") (parent (node (document "d0") (qualified-name "Requirement Groups"))) (authored (membership (kind Feature)) (relationships (subject (reference "Requirement Groups::engineSpecification::engine")))))
    (element (id (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Requirement Groups::engineSpecification"))))
    (element (id (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::drivePowerInterface"))) (kind "requirement") (name "drivePowerInterface") (declared-name "drivePowerInterface") (parent (node (document "d0") (qualified-name "Requirement Groups::engineSpecification"))) (authored (membership (kind Feature)) (relationships (typing (reference "DrivePowerInterface")) (subject (reference "Requirement Groups::engineSpecification::drivePowerInterface::")))))
    (element (id (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::drivePowerInterface::"))) (kind "subject") (name "") (parent (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::drivePowerInterface"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::engine"))) (kind "subject") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "Requirement Groups::engineSpecification"))) (authored (relationships (typing (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::torqueGeneration"))) (kind "requirement") (name "torqueGeneration") (declared-name "torqueGeneration") (parent (node (document "d0") (qualified-name "Requirement Groups::engineSpecification"))) (authored (membership (kind Feature)) (relationships (typing (reference "TorqueGeneration")) (subject (reference "Requirement Groups::engineSpecification::torqueGeneration::")))))
    (element (id (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::torqueGeneration::"))) (kind "subject") (name "") (parent (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::torqueGeneration"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Requirement Groups::vehicleSpecification"))) (kind "requirement") (name "vehicleSpecification") (declared-name "vehicleSpecification") (parent (node (document "d0") (qualified-name "Requirement Groups"))) (authored (membership (kind Feature)) (relationships (subject (reference "Requirement Groups::vehicleSpecification::vehicle")))))
    (element (id (node (document "d0") (qualified-name "Requirement Groups::vehicleSpecification::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Requirement Groups::vehicleSpecification"))))
    (element (id (node (document "d0") (qualified-name "Requirement Groups::vehicleSpecification::_requireConstraint_0"))) (kind "require constraint") (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (parent (node (document "d0") (qualified-name "Requirement Groups::vehicleSpecification"))))
    (element (id (node (document "d0") (qualified-name "Requirement Groups::vehicleSpecification::_requireConstraint_1"))) (kind "require constraint") (name "_requireConstraint_1") (declared-name "_requireConstraint_1") (parent (node (document "d0") (qualified-name "Requirement Groups::vehicleSpecification"))))
    (element (id (node (document "d0") (qualified-name "Requirement Groups::vehicleSpecification::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "Requirement Groups::vehicleSpecification"))) (authored (relationships (typing (reference "Vehicle")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Requirement Groups::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Requirement Definitions::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Groups::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Requirement Usages::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Groups::Engine"))) (kind performSource) (ordinal 0)) (authored-target "Requirement Groups::Engine::generateTorque") (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirement Groups::Engine::generateTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Groups::Engine::clutchPort"))) (kind featureTyping) (ordinal 0)) (authored-target "ClutchPort") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Groups::Engine::generateTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "GenerateTorque") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Groups::engineSpecification"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "Requirement Groups::engineSpecification::engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::drivePowerInterface"))) (kind featureTyping) (ordinal 0)) (authored-target "DrivePowerInterface") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::drivePowerInterface"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "Requirement Groups::engineSpecification::drivePowerInterface::") (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::drivePowerInterface::")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::drivePowerInterface::"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::drivePowerInterface::")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirement Groups::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::torqueGeneration"))) (kind featureTyping) (ordinal 0)) (authored-target "TorqueGeneration") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::torqueGeneration"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "Requirement Groups::engineSpecification::torqueGeneration::") (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::torqueGeneration::")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::torqueGeneration::"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirement Groups::engineSpecification::torqueGeneration::")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Groups::vehicleSpecification"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "Requirement Groups::vehicleSpecification::vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirement Groups::vehicleSpecification::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Groups::vehicleSpecification::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status unresolved)))
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
