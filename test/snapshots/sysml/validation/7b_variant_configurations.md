# META
~~~ini
description=SysML Validation (07-Variant Configuration): 7b-Variant Configurations
type=file
~~~
# SOURCE
~~~sysml
package '7b-Variant Configurations' {
	private import RequirementsModel::*;
	private import DesignModel::*;
	private import VariantDefinitions::*;
	private import ControlFunctions::forAll;
	
	package RequirementsModel {
		requirement def EnginePerformanceRequirement;
		requirement highPerformanceRequirement : EnginePerformanceRequirement;
		requirement normalPerformanceRequirement : EnginePerformanceRequirement;
	}
	
	package DesignModel {
		part def Vehicle;
		part def Engine;
		part def Transmission;
		part def Clutch;
		part def Driveshaft;
		part def RearAxleAssembly;
		part def Wheel;
		
		port def FuelCmdPort;
		port def ClutchPort;
		port def ShaftPort_b;
		port def ShaftPort_c;
		port def ShaftPort_d;
		port def VehicleToRoadPort;
		port def WheelToRoadPort;
		
		part vehicle : Vehicle {
			port fuelCmdPort;
			
			bind fuelCmdPort = engine.fuelCmdPort;
			
			part engine : Engine[1] {
				port fuelCmdPort : FuelCmdPort;
			}
			
			part transmission : Transmission[1] {
				part clutch: Clutch[1] {
					port clutchPort : ClutchPort;
				}
			}
			
			part driveshaft : Driveshaft[1] {
				port shaftPort_b : ShaftPort_b;
				port shaftPort_c : ShaftPort_c;
			}
			
			part rearAxleAssembly : RearAxleAssembly {
				part rearWheels : Wheel[2] {
					port wheelToRoadPort : WheelToRoadPort;
				}
			}
			
			port vehicleToRoadPort : VehicleToRoadPort {
				port wheelToRoadPort : WheelToRoadPort[2];
			}
		}
	}
	
	package VariantDefinitions {
		part def '4CylEngine' :> Engine;
		part def '6CylEngine' :> Engine;
		
		part def ManualTransmission :> Transmission;
		part def AutomaticTransmission :> Transmission;
		
		part def ManualClutch :> Clutch;
		part def AutomaticClutch :> Clutch;
		
		port def ManualClutchPort :> ClutchPort;
		port def AutomaticClutchPort :> ClutchPort;
		
		part def NarrowRimWheel :> Wheel;
		part def WideRimWheel :> Wheel;		
	}
	
	package VariabilityModel {
		part anyVehicleConfig :> vehicle {
			
			variation requirement engineRqtChoice : EnginePerformanceRequirement {
				variant highPerformanceRequirement;
				variant normalPerformanceRequirement;
			}
			
			variation part engineChoice :>> engine {
				variant part '4cylEngine' : '4CylEngine';
				variant part '6cylEngine' : '6CylEngine';
			}
			
			satisfy engineRqtChoice by engineChoice;
			
			assert constraint 'engine choice constraint' {
				if engineRqtChoice == engineRqtChoice::highPerformanceRequirement? 
					engineChoice == engineChoice::'6cylEngine' 
				else
					engineChoice == engineChoice::'4cylEngine'
			}
			
			variation part transmissionChoice :>> transmission {
				variant part manualTransmission : ManualTransmission {
					part :>> clutch : ManualClutch {
						port :>> clutchPort : ManualClutchPort;
					}
				}
				variant part automaticTransmission : AutomaticTransmission {
					part :>> clutch : AutomaticClutch {
						port :>> clutchPort : AutomaticClutchPort;
					}
				}
			}
			
			assert constraint 'engine-transmission selection constraint' {
				(engineChoice == engineChoice::'4cylEngine' and transmissionChoice == transmissionChoice::manualTransmission) xor
				(engineChoice == engineChoice::'6cylEngine' and transmissionChoice == transmissionChoice::automaticTransmission)
			}
			
			part :>> rearAxleAssembly {
				variation part rearWheelChoice :>> rearWheels {
					variant part narrowRimWheel : NarrowRimWheel;
					variant part wideRimWheel : WideRimWheel;
				}
			
    			assert constraint 'engine-wheel selection constraint' {
    				(engineChoice == engineChoice::'4cylEngine' and 
    					rearWheelChoice->forAll {in ref w; w == rearWheelChoice::narrowRimWheel}) xor
    				(engineChoice == engineChoice::'6cylEngine' and 
    					rearWheelChoice->forAll {in ref w; w == rearWheelChoice::wideRimWheel})
    			}
            }
			
		}
		
		variation part vehicleChoice :> anyVehicleConfig {
			variant part vehicle_c1;
			variant part vehicle_c2;
		}	
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/7b_variant_configurations.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 4 16) (end 4 40))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 7 2) (end 7 47))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 8 2) (end 8 72))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 9 2) (end 9 74))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 21 2) (end 21 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 22 2) (end 22 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 23 2) (end 23 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 24 2) (end 24 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 25 2) (end 25 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 26 2) (end 26 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 27 2) (end 27 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 30 3) (end 30 20))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 32 3) (end 32 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 35 4) (end 35 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 40 5) (end 40 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 45 4) (end 45 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 46 4) (end 46 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 51 5) (end 51 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 55 3) (end 57 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 71 2) (end 71 42))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 72 2) (end 72 45))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 79 27) (end 79 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 81 3) (end 84 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 86 35) (end 86 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 87 4) (end 87 45))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 88 4) (end 88 45))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 91 3) (end 91 43))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 93 3) (end 98 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 100 41) (end 100 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 101 4) (end 105 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 106 4) (end 110 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 113 3) (end 116 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 118 12) (end 118 28))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 119 39) (end 119 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 120 5) (end 120 50))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 121 5) (end 121 46))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 124 7) (end 129 8))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 134 34) (end 134 50))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 135 3) (end 135 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 136 3) (end 136 27))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:85f8155a8001806fd987774d95b107e5b13e3eecce5c068304dc9c9d8fa574bb") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "RequirementsModel") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "DesignModel") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "VariantDefinitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlFunctions::forAll") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Clutch"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Driveshaft"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::RearAxleAssembly"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Transmission"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Wheel"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Driveshaft"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "RearAxleAssembly"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly::rearWheels"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Transmission"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission::clutch"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Clutch"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::RequirementsModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "vehicle"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "rearAxleAssembly"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::::rearWheelChoice"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "rearWheels"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "engine"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "transmission"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::vehicleChoice"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "anyVehicleConfig"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::4CylEngine"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::6CylEngine"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutch"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Clutch"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticTransmission"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Transmission"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutch"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Clutch"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualTransmission"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Transmission"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::NarrowRimWheel"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Wheel"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::WideRimWheel"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Wheel"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "RequirementsModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::RequirementsModel")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "DesignModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "VariantDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlFunctions::forAll")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft"))) (kind featureTyping) (ordinal 0))
      (authored-target "Driveshaft")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Driveshaft")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Engine")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly"))) (kind featureTyping) (ordinal 0))
      (authored-target "RearAxleAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::RearAxleAssembly")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly::rearWheels"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Wheel")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission"))) (kind featureTyping) (ordinal 0))
      (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Transmission")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission::clutch"))) (kind featureTyping) (ordinal 0))
      (authored-target "Clutch")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Clutch")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig"))) (kind subsetting) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "rearAxleAssembly")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::::rearWheelChoice"))) (kind redefinition) (ordinal 0))
      (authored-target "rearWheels")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice"))) (kind redefinition) (ordinal 0))
      (authored-target "engine")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice"))) (kind redefinition) (ordinal 0))
      (authored-target "transmission")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::vehicleChoice"))) (kind subsetting) (ordinal 0))
      (authored-target "anyVehicleConfig")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::4CylEngine"))) (kind specialization) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Engine")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::6CylEngine"))) (kind specialization) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Engine")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutch"))) (kind specialization) (ordinal 0))
      (authored-target "Clutch")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Clutch")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticTransmission"))) (kind specialization) (ordinal 0))
      (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Transmission")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutch"))) (kind specialization) (ordinal 0))
      (authored-target "Clutch")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Clutch")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualTransmission"))) (kind specialization) (ordinal 0))
      (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Transmission")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::NarrowRimWheel"))) (kind specialization) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Wheel")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::WideRimWheel"))) (kind specialization) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Wheel")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Driveshaft"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::RearAxleAssembly"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly::rearWheels"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly::rearWheels"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Transmission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission::clutch"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Clutch"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission::clutch"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::4CylEngine"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::4CylEngine"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::6CylEngine"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::6CylEngine"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutch"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Clutch"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutch"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticTransmission"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Transmission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticTransmission"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutch"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Clutch"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutch"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualTransmission"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Transmission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualTransmission"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::NarrowRimWheel"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::NarrowRimWheel"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::WideRimWheel"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::WideRimWheel"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 1 16) (end 1 36)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "RequirementsModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::RequirementsModel")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 2 16) (end 2 30)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "DesignModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 3 16) (end 3 37)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "VariantDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 4 16) (end 4 40)) (probe (position 4 16))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::forAll")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 29 17) (end 29 24)) (probe (position 29 17))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Vehicle")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 44 21) (end 44 31)) (probe (position 44 21))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft"))) (kind featureTyping) (ordinal 0) (authored-target "Driveshaft")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Driveshaft")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 34 17) (end 34 23)) (probe (position 34 17))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Engine")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 49 27) (end 49 43)) (probe (position 49 27))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly"))) (kind featureTyping) (ordinal 0) (authored-target "RearAxleAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::RearAxleAssembly")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 50 22) (end 50 27)) (probe (position 50 22))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly::rearWheels"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Wheel")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 38 23) (end 38 35)) (probe (position 38 23))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission"))) (kind featureTyping) (ordinal 0) (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Transmission")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 39 17) (end 39 23)) (probe (position 39 17))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission::clutch"))) (kind featureTyping) (ordinal 0) (authored-target "Clutch")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Clutch")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 79 27) (end 79 34)) (probe (position 79 27))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig"))) (kind subsetting) (ordinal 0) (authored-target "vehicle")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 118 12) (end 118 28)) (probe (position 118 12))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "rearAxleAssembly")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 119 39) (end 119 49)) (probe (position 119 39))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::::rearWheelChoice"))) (kind redefinition) (ordinal 0) (authored-target "rearWheels")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 86 35) (end 86 41)) (probe (position 86 35))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice"))) (kind redefinition) (ordinal 0) (authored-target "engine")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 100 41) (end 100 53)) (probe (position 100 41))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice"))) (kind redefinition) (ordinal 0) (authored-target "transmission")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 134 34) (end 134 50)) (probe (position 134 34))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::vehicleChoice"))) (kind subsetting) (ordinal 0) (authored-target "anyVehicleConfig")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 62 27) (end 62 33)) (probe (position 62 27))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::4CylEngine"))) (kind specialization) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Engine")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 63 27) (end 63 33)) (probe (position 63 27))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::6CylEngine"))) (kind specialization) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Engine")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 69 30) (end 69 36)) (probe (position 69 30))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutch"))) (kind specialization) (ordinal 0) (authored-target "Clutch")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Clutch")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 66 36) (end 66 48)) (probe (position 66 36))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticTransmission"))) (kind specialization) (ordinal 0) (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Transmission")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 68 27) (end 68 33)) (probe (position 68 27))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutch"))) (kind specialization) (ordinal 0) (authored-target "Clutch")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Clutch")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 65 33) (end 65 45)) (probe (position 65 33))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualTransmission"))) (kind specialization) (ordinal 0) (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Transmission")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 74 29) (end 74 34)) (probe (position 74 29))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::NarrowRimWheel"))) (kind specialization) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Wheel")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 75 27) (end 75 32)) (probe (position 75 27))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::WideRimWheel"))) (kind specialization) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Wheel")))))
  )
)
~~~
