# META
~~~ini
description=SysML Validation (08-Requirements): 8-Requirements
type=file
~~~
# SOURCE
~~~sysml
package '8-Requirements' {
	private import ScalarValues::Real;
	private import ISQ::*;
	private import SI::*;
	public import 'Vehicle Usages'::*;
	public import 'Vehicle Requirements'::*;
	
	package 'Vehicle Definitions' {
		part def Vehicle {
			attribute mass: MassValue;
			attribute fuelLevel: Real;
			attribute fuelTankCapacity: Real;
		}
		
		part def Engine {
			port drivePwrPort: DrivePwrPort;
			perform action 'generate torque': 'Generate Torque';
		}
		
		part def Transmission {
			port clutchPort: ClutchPort;
		}
		
		port def DrivePwrPort;
		port def ClutchPort;
		
		interface def EngineToTransmissionInterface {
			end drivePwrPort: DrivePwrPort;
			end clutchPort: ClutchPort;
		}
	
		action def 'Generate Torque';
	}
	
	package 'Vehicle Usages' {
		public import 'Vehicle Definitions'::*;
		
		action 'provide power' {
			action 'generate torque' { /* ... */ }
			//...
		}
		
		part vehicle1_c1: Vehicle {
			attribute :>> mass = 2000 [kg];
			perform 'provide power';
				
			part engine_v1: Engine {
				port :>> drivePwrPort;
				perform 'provide power'.'generate torque' :>> 'generate torque';
			}
			
			part transmission: Transmission {
				port :>> clutchPort;
			}
			
			interface engineToTransmission: EngineToTransmissionInterface
				connect engine_v1.drivePwrPort to transmission.clutchPort;
		}
		
		part vehicle1_c2: Vehicle {
			attribute :>> mass = 2500 [kg];
		}
	}
	
	package 'Vehicle Requirements' {	
		public import 'Vehicle Definitions'::*;
	
		requirement def <'1'> MassLimitationRequirement {
			/*
			 * The optional requirement ID  of this requirement ('1') is given after the keyword "id" (using name syntax).
			 * Every requirement is parameterized by a "subject". The "subject" of this requirement is implicitly "Anything".
			 */
		
			// The requirement text is given by the documentation in the requirement def body.
			doc /* The actual mass shall be less than or equal to the required mass. */
			
			attribute massActual: MassValue;
			attribute massReqd: MassValue;
			
			require constraint {
				/*
				 * A constraint can be used to formalize a requirement.
				 */
				 massActual <= massReqd 
			 }
		}
		
		requirement def <'2'> ReliabilityRequirement;
		
		requirement <'1.1'> vehicleMass1: MassLimitationRequirement {
			doc /* The vehicle mass shall be less than or equal to 2000 kg when the fuel tank is full. */
			
			subject vehicle : Vehicle {
				/*
				 * The subject of this requirement is redefined to be a "Vehicle".
				 */
			}
			
			attribute :>> massActual: MassValue = vehicle.mass {
				/*
				 * This redefinition binds the vehicle mass to the actual mass.
				 */
			}
			
			attribute :>> massReqd = 2000 [kg] {
				/*
				 * This redefinition sets the required mass to 2000 kg.
				 */
			}
			
			assume constraint fuelConstraint {
				/*
				 * A constraint can also be used to specify an assumption.
				 */
			
				doc /* full fuel tank */
				vehicle.fuelLevel >= vehicle.fuelTankCapacity
			}
		}
			
		requirement <'2.1'> vehicleMass2: MassLimitationRequirement {
			doc /* The vehicle mass shall be less than or equal to 2500 kg when the fuel tank is empty. */
			
			subject vehicle : Vehicle;
			
			attribute :>> massActual: MassValue = vehicle.mass;
			attribute :>> massReqd = 2500 [kg];
		
			assume constraint fuelConstraint {
				doc /* empty fuel tank */
				vehicle.fuelLevel == 0.0
			}
		}
		
		requirement <'2.2'> vehicleReliability2: ReliabilityRequirement {
			subject vehicle : Vehicle;
		}
			
		requirement <'3.1'> drivePowerInterface {
			doc /* The engine shall transfer its generated torque to the transmission via the clutch interface. */
			subject drivePwrPort: DrivePwrPort;
		}
		
		requirement <'3.2'> torqueGeneration {
			doc /* The engine shall generate torque as a function of RPM as shown in Table 1. */
			subject generateTorque: 'Generate Torque';
		}
			
	}
	
	part 'vehicle1_c1 Specification Context' {
		private import 'vehicle1-c1 Specification'::*;
		private import 'engine-v1 Specification'::*;
		
		requirement 'vehicle1-c1 Specification' {
		doc
		/*
		 * This models a "requirement group" as a requirement that references other requirements.
		 */
		
			subject vehicle : Vehicle;
			requirement references vehicleMass1 {
				/*
				 * This is a reference to a requirement defined outside the group.
				 * By default, the subject of the requirement is bound to that of the group.
				 */				
			}
			// ...
		}
		
		requirement 'engine-v1 Specification' {
			subject engine : Engine;
			/* 
			 * Here the subjects of the referenced requirements are defined to be specific properties of the
			 * subject of the group.
			 */
			require torqueGeneration {
				in :>> generateTorque = engine.'generate torque';
			}
			require drivePowerInterface {
				in :>> drivePwrPort = engine.drivePwrPort; 
			}
		}
		
		satisfy 'vehicle1-c1 Specification' by vehicle1_c1 {
			/*
			 * This asserts that if the assumptions of 'vehicle1-c1 Specification' are true with 'vehicle_c1' as
			 * the subject, then the required constraints are also true.
			 */
		}
		satisfy 'engine-v1 Specification' by vehicle1_c1.engine_v1;
	}
	
	part 'vehicle1_c2 Specification Context' {
		private import 'vehicle1-c2 Specification'::*;
		
		requirement 'vehicle1-c2 Specification' {
			subject vehicle : Vehicle;
			require vehicleMass2;
			require vehicleReliability2;
		}
		
		satisfy 'vehicle1-c2 Specification' by vehicle1_c2;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/8_requirements.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 19) (end 9 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 24) (end 10 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 31) (end 11 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 15 3) (end 15 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 16 3) (end 16 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 20 3) (end 20 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 23 2) (end 23 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 24 2) (end 24 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 26 2) (end 29 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 31 2) (end 31 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 37 2) (end 40 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 43 17) (end 43 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 44 3) (end 44 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 47 4) (end 47 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 48 4) (end 48 68))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 52 4) (end 52 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 55 3) (end 56 62))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 60 17) (end 60 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 67 2) (end 85 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 87 2) (end 87 47))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 89 2) (end 118 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 120 2) (end 132 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 134 2) (end 136 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 138 2) (end 141 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 143 2) (end 146 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 151 17) (end 151 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 152 17) (end 152 45))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 154 2) (end 168 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 170 2) (end 182 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 184 2) (end 189 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 190 2) (end 190 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 194 17) (end 194 47))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 196 2) (end 200 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 202 2) (end 202 53))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:760d4cca14b7207e41c47f068c6e60ee38e86987fb47fca722a89b7f9b5ae549") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/8_requirements.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Real") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/8_requirements.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQ") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/8_requirements.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "SI") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/8_requirements.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "Vehicle Usages") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/8_requirements.md") (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "Vehicle Requirements") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Definitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Definitions::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Definitions::Transmission"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::fuelLevel"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::fuelTankCapacity"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue"))))
    (declaration (id (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Requirements"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/8_requirements.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "Vehicle Definitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Usages"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/8_requirements.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "Vehicle Definitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/8_requirements.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "mass"))))
    (declaration (id (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::transmission"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Transmission"))))
    (declaration (id (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c2"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/8_requirements.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "mass"))))
    (declaration (id (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::vehicle1_c1 Specification Context"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/8_requirements.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "vehicle1-c1 Specification") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/8_requirements.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "engine-v1 Specification") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::vehicle1_c2 Specification Context"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/8_requirements.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "vehicle1-c2 Specification") (import (shape namespace) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/8_requirements.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQ")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/8_requirements.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "SI")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/8_requirements.md") (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Vehicle Usages")
      (outcome (status resolved) (target (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Usages")))))
    (reference (id (source (node (document "memory://snapshot/8_requirements.md") (anonymous (kind import) (ordinal 4))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Vehicle Requirements")
      (outcome (status resolved) (target (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Requirements")))))
    (reference (id (source (node (document "memory://snapshot/8_requirements.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::fuelLevel"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::fuelTankCapacity"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/8_requirements.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Vehicle Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Definitions")))))
    (reference (id (source (node (document "memory://snapshot/8_requirements.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Vehicle Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Definitions")))))
    (reference (id (source (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/8_requirements.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "mass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Definitions::Engine")))))
    (reference (id (source (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::transmission"))) (kind featureTyping) (ordinal 0))
      (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Definitions::Transmission")))))
    (reference (id (source (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c2"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/8_requirements.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "mass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/8_requirements.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "vehicle1-c1 Specification")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/8_requirements.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "engine-v1 Specification")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/8_requirements.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "vehicle1-c2 Specification")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1"))) (target (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1"))) (target (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Definitions::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::transmission"))) (target (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Definitions::Transmission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::transmission"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c2"))) (target (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c2"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/8_requirements.md") (range (start 2 16) (end 2 22)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/8_requirements.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQ")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/8_requirements.md") (range (start 3 16) (end 3 21)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/8_requirements.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "SI")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/8_requirements.md") (range (start 4 15) (end 4 34)) (probe (position 4 15))
    (reference (id (source (node (document "memory://snapshot/8_requirements.md") (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0) (authored-target "Vehicle Usages")
      (outcome (status resolved) (target (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Usages")))))
  )
  (query (document "memory://snapshot/8_requirements.md") (range (start 5 15) (end 5 40)) (probe (position 5 15))
    (reference (id (source (node (document "memory://snapshot/8_requirements.md") (anonymous (kind import) (ordinal 4))))) (kind namespaceImport) (ordinal 0) (authored-target "Vehicle Requirements")
      (outcome (status resolved) (target (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Requirements")))))
  )
  (query (document "memory://snapshot/8_requirements.md") (range (start 1 16) (end 1 34)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/8_requirements.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/8_requirements.md") (range (start 10 24) (end 10 28)) (probe (position 10 24))
    (reference (id (source (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::fuelLevel"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/8_requirements.md") (range (start 11 31) (end 11 35)) (probe (position 11 31))
    (reference (id (source (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::fuelTankCapacity"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/8_requirements.md") (range (start 9 19) (end 9 28)) (probe (position 9 19))
    (reference (id (source (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle::mass"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/8_requirements.md") (range (start 65 16) (end 65 40)) (probe (position 65 16))
    (reference (id (source (node (document "memory://snapshot/8_requirements.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Vehicle Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Definitions")))))
  )
  (query (document "memory://snapshot/8_requirements.md") (range (start 35 16) (end 35 40)) (probe (position 35 16))
    (reference (id (source (node (document "memory://snapshot/8_requirements.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Vehicle Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Definitions")))))
  )
  (query (document "memory://snapshot/8_requirements.md") (range (start 42 20) (end 42 27)) (probe (position 42 20))
    (reference (id (source (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle")))))
  )
  (query (document "memory://snapshot/8_requirements.md") (range (start 43 17) (end 43 21)) (probe (position 43 17))
    (reference (id (source (node (document "memory://snapshot/8_requirements.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "mass")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/8_requirements.md") (range (start 46 19) (end 46 25)) (probe (position 46 19))
    (reference (id (source (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::engine_v1"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Definitions::Engine")))))
  )
  (query (document "memory://snapshot/8_requirements.md") (range (start 51 22) (end 51 34)) (probe (position 51 22))
    (reference (id (source (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c1::transmission"))) (kind featureTyping) (ordinal 0) (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Definitions::Transmission")))))
  )
  (query (document "memory://snapshot/8_requirements.md") (range (start 59 20) (end 59 27)) (probe (position 59 20))
    (reference (id (source (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Usages::vehicle1_c2"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/8_requirements.md") (qualified-name "8-Requirements::Vehicle Definitions::Vehicle")))))
  )
  (query (document "memory://snapshot/8_requirements.md") (range (start 60 17) (end 60 21)) (probe (position 60 17))
    (reference (id (source (node (document "memory://snapshot/8_requirements.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "mass")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/8_requirements.md") (range (start 151 17) (end 151 47)) (probe (position 151 17))
    (reference (id (source (node (document "memory://snapshot/8_requirements.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "vehicle1-c1 Specification")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/8_requirements.md") (range (start 152 17) (end 152 45)) (probe (position 152 17))
    (reference (id (source (node (document "memory://snapshot/8_requirements.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "engine-v1 Specification")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/8_requirements.md") (range (start 194 17) (end 194 47)) (probe (position 194 17))
    (reference (id (source (node (document "memory://snapshot/8_requirements.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "vehicle1-c2 Specification")
      (outcome (status unresolved)))
  )
)
~~~
