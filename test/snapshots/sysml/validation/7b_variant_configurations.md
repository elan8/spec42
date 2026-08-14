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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 32 22) (end 32 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 94 26) (end 94 69))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 95 21) (end 95 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 97 21) (end 97 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 114 21) (end 114 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 114 74) (end 114 112))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 115 21) (end 115 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 115 74) (end 115 115))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 118 12) (end 118 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 125 25) (end 125 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 127 25) (end 127 51))
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
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:85f8155a8001806fd987774d95b107e5b13e3eecce5c068304dc9c9d8fa574bb") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "RequirementsModel") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (anonymous (kind import) (ordinal 1)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "DesignModel") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (anonymous (kind import) (ordinal 2)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "VariantDefinitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (anonymous (kind import) (ordinal 3)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlFunctions::forAll") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Clutch"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::ClutchPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Driveshaft"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::FuelCmdPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::RearAxleAssembly"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_b"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_c"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_d"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Transmission"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::VehicleToRoadPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Wheel"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::WheelToRoadPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "DesignModel")) (named (kind part) (name "vehicle")) (anonymous (kind bind) (ordinal 0)))))) (kind bind) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindSource (reference "fuelCmdPort")) (memberAccessOperand (reference "engine::fuelCmdPort"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Driveshaft"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft::shaftPort_b"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShaftPort_b"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft::shaftPort_c"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ShaftPort_c"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine::fuelCmdPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelCmdPort"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::fuelCmdPort"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "RearAxleAssembly"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly::rearWheels"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 2) (upper 2))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly::rearWheels::wheelToRoadPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "WheelToRoadPort"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Transmission"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission::clutch"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Clutch"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission::clutch::clutchPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ClutchPort"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::vehicleToRoadPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleToRoadPort"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::vehicleToRoadPort::wheelToRoadPort"))) (kind port) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 2) (upper 2))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "WheelToRoadPort"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::RequirementsModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::RequirementsModel::EnginePerformanceRequirement"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::RequirementsModel::highPerformanceRequirement"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "EnginePerformanceRequirement"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::RequirementsModel::normalPerformanceRequirement"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "EnginePerformanceRequirement"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "vehicle"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind satisfy) (ordinal 0)))))) (kind satisfy) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (satisfySource (reference "engineRqtChoice")) (satisfyTarget (reference "engineChoice"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind part) (ordinal 0)))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "rearAxleAssembly"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind part) (ordinal 0)) (named (kind constraint) (name "engine-wheel selection constraint")))))) (kind constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "engineChoice")) (expressionOperand (reference "engineChoice::4cylEngine")) (expressionOperand (reference "rearWheelChoice")) (expressionOperand (reference "engineChoice")) (expressionOperand (reference "engineChoice::6cylEngine")) (expressionOperand (reference "rearWheelChoice"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind part) (ordinal 0)) (named (kind part) (name "rearWheelChoice")))))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers variation)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "rearWheels"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine choice constraint"))) (kind constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "engineRqtChoice")) (expressionOperand (reference "engineRqtChoice::highPerformanceRequirement")) (expressionOperand (reference "engineChoice")) (expressionOperand (reference "engineChoice::6cylEngine")) (expressionOperand (reference "engineChoice")) (expressionOperand (reference "engineChoice::4cylEngine"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine-transmission selection constraint"))) (kind constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "engineChoice")) (expressionOperand (reference "engineChoice::4cylEngine")) (expressionOperand (reference "transmissionChoice")) (expressionOperand (reference "transmissionChoice::manualTransmission")) (expressionOperand (reference "engineChoice")) (expressionOperand (reference "engineChoice::6cylEngine")) (expressionOperand (reference "transmissionChoice")) (expressionOperand (reference "transmissionChoice::automaticTransmission"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers variation)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "engine"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineRqtChoice"))) (kind requirement) (membership (kind feature) (visibility default)) (facts (modifiers variation)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "EnginePerformanceRequirement")) (variant (reference "highPerformanceRequirement")) (variant (reference "normalPerformanceRequirement"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers variation)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "transmission"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::vehicleChoice"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers variation)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "anyVehicleConfig"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::4CylEngine"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::6CylEngine"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutch"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Clutch"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutchPort"))) (kind port-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ClutchPort"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticTransmission"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Transmission"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutch"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Clutch"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutchPort"))) (kind port-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ClutchPort"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualTransmission"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Transmission"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::NarrowRimWheel"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Wheel"))))
    (declaration (id (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::WideRimWheel"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Wheel"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "RequirementsModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::RequirementsModel")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (anonymous (kind import) (ordinal 1)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "DesignModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (anonymous (kind import) (ordinal 2)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "VariantDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (anonymous (kind import) (ordinal 3)))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlFunctions::forAll")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "DesignModel")) (named (kind part) (name "vehicle")) (anonymous (kind bind) (ordinal 0)))))) (kind bindSource) (ordinal 0))
      (authored-target "fuelCmdPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::fuelCmdPort")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "DesignModel")) (named (kind part) (name "vehicle")) (anonymous (kind bind) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "engine::fuelCmdPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft"))) (kind featureTyping) (ordinal 0))
      (authored-target "Driveshaft")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Driveshaft")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft::shaftPort_b"))) (kind featureTyping) (ordinal 0))
      (authored-target "ShaftPort_b")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_b")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft::shaftPort_c"))) (kind featureTyping) (ordinal 0))
      (authored-target "ShaftPort_c")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_c")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Engine")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine::fuelCmdPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelCmdPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::FuelCmdPort")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly"))) (kind featureTyping) (ordinal 0))
      (authored-target "RearAxleAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::RearAxleAssembly")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly::rearWheels"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Wheel")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly::rearWheels::wheelToRoadPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "WheelToRoadPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::WheelToRoadPort")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission"))) (kind featureTyping) (ordinal 0))
      (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Transmission")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission::clutch"))) (kind featureTyping) (ordinal 0))
      (authored-target "Clutch")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Clutch")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission::clutch::clutchPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "ClutchPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::ClutchPort")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::vehicleToRoadPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleToRoadPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::VehicleToRoadPort")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::vehicleToRoadPort::wheelToRoadPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "WheelToRoadPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::WheelToRoadPort")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::RequirementsModel::highPerformanceRequirement"))) (kind featureTyping) (ordinal 0))
      (authored-target "EnginePerformanceRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::RequirementsModel::EnginePerformanceRequirement")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::RequirementsModel::normalPerformanceRequirement"))) (kind featureTyping) (ordinal 0))
      (authored-target "EnginePerformanceRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::RequirementsModel::EnginePerformanceRequirement")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig"))) (kind subsetting) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind part) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "rearAxleAssembly")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind satisfy) (ordinal 0)))))) (kind satisfySource) (ordinal 0))
      (authored-target "engineRqtChoice")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineRqtChoice")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind satisfy) (ordinal 0)))))) (kind satisfyTarget) (ordinal 0))
      (authored-target "engineChoice")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind part) (ordinal 0)) (named (kind constraint) (name "engine-wheel selection constraint")))))) (kind expressionOperand) (ordinal 0))
      (authored-target "engineChoice")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind part) (ordinal 0)) (named (kind constraint) (name "engine-wheel selection constraint")))))) (kind expressionOperand) (ordinal 1))
      (authored-target "engineChoice::4cylEngine")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind part) (ordinal 0)) (named (kind constraint) (name "engine-wheel selection constraint")))))) (kind expressionOperand) (ordinal 2))
      (authored-target "rearWheelChoice")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind part) (ordinal 0)) (named (kind part) (name "rearWheelChoice"))))))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind part) (ordinal 0)) (named (kind constraint) (name "engine-wheel selection constraint")))))) (kind expressionOperand) (ordinal 3))
      (authored-target "engineChoice")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind part) (ordinal 0)) (named (kind constraint) (name "engine-wheel selection constraint")))))) (kind expressionOperand) (ordinal 4))
      (authored-target "engineChoice::6cylEngine")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind part) (ordinal 0)) (named (kind constraint) (name "engine-wheel selection constraint")))))) (kind expressionOperand) (ordinal 5))
      (authored-target "rearWheelChoice")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind part) (ordinal 0)) (named (kind part) (name "rearWheelChoice"))))))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind part) (ordinal 0)) (named (kind part) (name "rearWheelChoice")))))) (kind redefinition) (ordinal 0))
      (authored-target "rearWheels")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine choice constraint"))) (kind expressionOperand) (ordinal 0))
      (authored-target "engineRqtChoice")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineRqtChoice")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine choice constraint"))) (kind expressionOperand) (ordinal 1))
      (authored-target "engineRqtChoice::highPerformanceRequirement")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine choice constraint"))) (kind expressionOperand) (ordinal 2))
      (authored-target "engineChoice")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine choice constraint"))) (kind expressionOperand) (ordinal 3))
      (authored-target "engineChoice::6cylEngine")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine choice constraint"))) (kind expressionOperand) (ordinal 4))
      (authored-target "engineChoice")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine choice constraint"))) (kind expressionOperand) (ordinal 5))
      (authored-target "engineChoice::4cylEngine")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine-transmission selection constraint"))) (kind expressionOperand) (ordinal 0))
      (authored-target "engineChoice")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine-transmission selection constraint"))) (kind expressionOperand) (ordinal 1))
      (authored-target "engineChoice::4cylEngine")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine-transmission selection constraint"))) (kind expressionOperand) (ordinal 2))
      (authored-target "transmissionChoice")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine-transmission selection constraint"))) (kind expressionOperand) (ordinal 3))
      (authored-target "transmissionChoice::manualTransmission")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine-transmission selection constraint"))) (kind expressionOperand) (ordinal 4))
      (authored-target "engineChoice")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine-transmission selection constraint"))) (kind expressionOperand) (ordinal 5))
      (authored-target "engineChoice::6cylEngine")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine-transmission selection constraint"))) (kind expressionOperand) (ordinal 6))
      (authored-target "transmissionChoice")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine-transmission selection constraint"))) (kind expressionOperand) (ordinal 7))
      (authored-target "transmissionChoice::automaticTransmission")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice"))) (kind redefinition) (ordinal 0))
      (authored-target "engine")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineRqtChoice"))) (kind featureTyping) (ordinal 0))
      (authored-target "EnginePerformanceRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::RequirementsModel::EnginePerformanceRequirement")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineRqtChoice"))) (kind variant) (ordinal 0))
      (authored-target "highPerformanceRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::RequirementsModel::highPerformanceRequirement")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineRqtChoice"))) (kind variant) (ordinal 1))
      (authored-target "normalPerformanceRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::RequirementsModel::normalPerformanceRequirement")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice"))) (kind redefinition) (ordinal 0))
      (authored-target "transmission")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::vehicleChoice"))) (kind subsetting) (ordinal 0))
      (authored-target "anyVehicleConfig")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::4CylEngine"))) (kind specialization) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Engine")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::6CylEngine"))) (kind specialization) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Engine")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutch"))) (kind specialization) (ordinal 0))
      (authored-target "Clutch")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Clutch")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutchPort"))) (kind specialization) (ordinal 0))
      (authored-target "ClutchPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::ClutchPort")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticTransmission"))) (kind specialization) (ordinal 0))
      (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Transmission")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutch"))) (kind specialization) (ordinal 0))
      (authored-target "Clutch")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Clutch")))))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutchPort"))) (kind specialization) (ordinal 0))
      (authored-target "ClutchPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::ClutchPort")))))
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
    (relationship (kind bindSource) (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "DesignModel")) (named (kind part) (name "vehicle")) (anonymous (kind bind) (ordinal 0)))))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::fuelCmdPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "DesignModel")) (named (kind part) (name "vehicle")) (anonymous (kind bind) (ordinal 0)))))) (kind bindSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Driveshaft"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft::shaftPort_b"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_b"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft::shaftPort_b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft::shaftPort_c"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_c"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft::shaftPort_c"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine::fuelCmdPort"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::FuelCmdPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine::fuelCmdPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::RearAxleAssembly"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly::rearWheels"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly::rearWheels"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly::rearWheels::wheelToRoadPort"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::WheelToRoadPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly::rearWheels::wheelToRoadPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Transmission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission::clutch"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Clutch"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission::clutch"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission::clutch::clutchPort"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::ClutchPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission::clutch::clutchPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::vehicleToRoadPort"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::VehicleToRoadPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::vehicleToRoadPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::vehicleToRoadPort::wheelToRoadPort"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::WheelToRoadPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::vehicleToRoadPort::wheelToRoadPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::RequirementsModel::highPerformanceRequirement"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::RequirementsModel::EnginePerformanceRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::RequirementsModel::highPerformanceRequirement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::RequirementsModel::normalPerformanceRequirement"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::RequirementsModel::EnginePerformanceRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::RequirementsModel::normalPerformanceRequirement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig"))) (kind subsetting) (ordinal 0)))
    (relationship (kind satisfySource) (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind satisfy) (ordinal 0)))))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineRqtChoice"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind satisfy) (ordinal 0)))))) (kind satisfySource) (ordinal 0)))
    (relationship (kind satisfyTarget) (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind satisfy) (ordinal 0)))))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind satisfy) (ordinal 0)))))) (kind satisfyTarget) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind part) (ordinal 0)) (named (kind constraint) (name "engine-wheel selection constraint")))))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind part) (ordinal 0)) (named (kind constraint) (name "engine-wheel selection constraint")))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind part) (ordinal 0)) (named (kind constraint) (name "engine-wheel selection constraint")))))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind part) (ordinal 0)) (named (kind part) (name "rearWheelChoice")))))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind part) (ordinal 0)) (named (kind constraint) (name "engine-wheel selection constraint")))))) (kind expressionOperand) (ordinal 2)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind part) (ordinal 0)) (named (kind constraint) (name "engine-wheel selection constraint")))))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind part) (ordinal 0)) (named (kind constraint) (name "engine-wheel selection constraint")))))) (kind expressionOperand) (ordinal 3)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind part) (ordinal 0)) (named (kind constraint) (name "engine-wheel selection constraint")))))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind part) (ordinal 0)) (named (kind part) (name "rearWheelChoice")))))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind part) (ordinal 0)) (named (kind constraint) (name "engine-wheel selection constraint")))))) (kind expressionOperand) (ordinal 5)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine choice constraint"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineRqtChoice"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine choice constraint"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine choice constraint"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine choice constraint"))) (kind expressionOperand) (ordinal 2)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine choice constraint"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine choice constraint"))) (kind expressionOperand) (ordinal 4)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine-transmission selection constraint"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine-transmission selection constraint"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine-transmission selection constraint"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine-transmission selection constraint"))) (kind expressionOperand) (ordinal 2)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine-transmission selection constraint"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine-transmission selection constraint"))) (kind expressionOperand) (ordinal 4)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine-transmission selection constraint"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine-transmission selection constraint"))) (kind expressionOperand) (ordinal 6)))
    (relationship (kind typing) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineRqtChoice"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::RequirementsModel::EnginePerformanceRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineRqtChoice"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind variant) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineRqtChoice"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::RequirementsModel::highPerformanceRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineRqtChoice"))) (kind variant) (ordinal 0)))
    (relationship (kind variant) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineRqtChoice"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::RequirementsModel::normalPerformanceRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineRqtChoice"))) (kind variant) (ordinal 1)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::vehicleChoice"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::vehicleChoice"))) (kind subsetting) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::4CylEngine"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::4CylEngine"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::6CylEngine"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::6CylEngine"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutch"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Clutch"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutch"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutchPort"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::ClutchPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutchPort"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticTransmission"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Transmission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticTransmission"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutch"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Clutch"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutch"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutchPort"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::ClutchPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutchPort"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualTransmission"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Transmission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualTransmission"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::NarrowRimWheel"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::NarrowRimWheel"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::WideRimWheel"))) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::WideRimWheel"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind part) (ordinal 0)) (named (kind constraint) (name "engine-wheel selection constraint")))))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine-transmission selection constraint"))) (value (kind unresolved-operand)))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 1 16) (end 1 36)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0) (authored-target "RequirementsModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::RequirementsModel")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 2 16) (end 2 30)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (anonymous (kind import) (ordinal 1)))))) (kind namespaceImport) (ordinal 0) (authored-target "DesignModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 3 16) (end 3 37)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (anonymous (kind import) (ordinal 2)))))) (kind namespaceImport) (ordinal 0) (authored-target "VariantDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 4 16) (end 4 40)) (probe (position 4 16))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (anonymous (kind import) (ordinal 3)))))) (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::forAll")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 29 17) (end 29 24)) (probe (position 29 17))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Vehicle")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 32 8) (end 32 19)) (probe (position 32 8))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "DesignModel")) (named (kind part) (name "vehicle")) (anonymous (kind bind) (ordinal 0)))))) (kind bindSource) (ordinal 0) (authored-target "fuelCmdPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::fuelCmdPort")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 32 22) (end 32 40)) (probe (position 32 22))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "DesignModel")) (named (kind part) (name "vehicle")) (anonymous (kind bind) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 0) (authored-target "engine::fuelCmdPort")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 44 21) (end 44 31)) (probe (position 44 21))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft"))) (kind featureTyping) (ordinal 0) (authored-target "Driveshaft")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Driveshaft")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 45 23) (end 45 34)) (probe (position 45 23))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft::shaftPort_b"))) (kind featureTyping) (ordinal 0) (authored-target "ShaftPort_b")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_b")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 46 23) (end 46 34)) (probe (position 46 23))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft::shaftPort_c"))) (kind featureTyping) (ordinal 0) (authored-target "ShaftPort_c")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_c")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 34 17) (end 34 23)) (probe (position 34 17))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Engine")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 35 23) (end 35 34)) (probe (position 35 23))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine::fuelCmdPort"))) (kind featureTyping) (ordinal 0) (authored-target "FuelCmdPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::FuelCmdPort")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 49 27) (end 49 43)) (probe (position 49 27))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly"))) (kind featureTyping) (ordinal 0) (authored-target "RearAxleAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::RearAxleAssembly")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 50 22) (end 50 27)) (probe (position 50 22))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly::rearWheels"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Wheel")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 51 28) (end 51 43)) (probe (position 51 28))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly::rearWheels::wheelToRoadPort"))) (kind featureTyping) (ordinal 0) (authored-target "WheelToRoadPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::WheelToRoadPort")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 38 23) (end 38 35)) (probe (position 38 23))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission"))) (kind featureTyping) (ordinal 0) (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Transmission")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 39 17) (end 39 23)) (probe (position 39 17))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission::clutch"))) (kind featureTyping) (ordinal 0) (authored-target "Clutch")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Clutch")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 40 23) (end 40 33)) (probe (position 40 23))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission::clutch::clutchPort"))) (kind featureTyping) (ordinal 0) (authored-target "ClutchPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::ClutchPort")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 55 28) (end 55 45)) (probe (position 55 28))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::vehicleToRoadPort"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleToRoadPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::VehicleToRoadPort")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 56 27) (end 56 42)) (probe (position 56 27))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::vehicleToRoadPort::wheelToRoadPort"))) (kind featureTyping) (ordinal 0) (authored-target "WheelToRoadPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::WheelToRoadPort")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 8 43) (end 8 71)) (probe (position 8 43))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::RequirementsModel::highPerformanceRequirement"))) (kind featureTyping) (ordinal 0) (authored-target "EnginePerformanceRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::RequirementsModel::EnginePerformanceRequirement")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 9 45) (end 9 73)) (probe (position 9 45))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::RequirementsModel::normalPerformanceRequirement"))) (kind featureTyping) (ordinal 0) (authored-target "EnginePerformanceRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::RequirementsModel::EnginePerformanceRequirement")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 79 27) (end 79 34)) (probe (position 79 27))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig"))) (kind subsetting) (ordinal 0) (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::vehicle")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 118 12) (end 118 28)) (probe (position 118 12))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind part) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "rearAxleAssembly")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 91 11) (end 91 26)) (probe (position 91 11))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind satisfy) (ordinal 0)))))) (kind satisfySource) (ordinal 0) (authored-target "engineRqtChoice")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineRqtChoice")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 91 30) (end 91 42)) (probe (position 91 30))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind satisfy) (ordinal 0)))))) (kind satisfyTarget) (ordinal 0) (authored-target "engineChoice")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 125 9) (end 125 21)) (probe (position 125 9))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind part) (ordinal 0)) (named (kind constraint) (name "engine-wheel selection constraint")))))) (kind expressionOperand) (ordinal 0) (authored-target "engineChoice")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 125 25) (end 125 51)) (probe (position 125 25))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind part) (ordinal 0)) (named (kind constraint) (name "engine-wheel selection constraint")))))) (kind expressionOperand) (ordinal 1) (authored-target "engineChoice::4cylEngine")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 126 9) (end 126 24)) (probe (position 126 9))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind part) (ordinal 0)) (named (kind constraint) (name "engine-wheel selection constraint")))))) (kind expressionOperand) (ordinal 2) (authored-target "rearWheelChoice")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind part) (ordinal 0)) (named (kind part) (name "rearWheelChoice"))))))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 127 9) (end 127 21)) (probe (position 127 9))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind part) (ordinal 0)) (named (kind constraint) (name "engine-wheel selection constraint")))))) (kind expressionOperand) (ordinal 3) (authored-target "engineChoice")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 127 25) (end 127 51)) (probe (position 127 25))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind part) (ordinal 0)) (named (kind constraint) (name "engine-wheel selection constraint")))))) (kind expressionOperand) (ordinal 4) (authored-target "engineChoice::6cylEngine")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 128 9) (end 128 24)) (probe (position 128 9))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind part) (ordinal 0)) (named (kind constraint) (name "engine-wheel selection constraint")))))) (kind expressionOperand) (ordinal 5) (authored-target "rearWheelChoice")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind part) (ordinal 0)) (named (kind part) (name "rearWheelChoice"))))))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 119 39) (end 119 49)) (probe (position 119 39))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (path (named (kind package) (name "7b-Variant Configurations")) (named (kind package) (name "VariabilityModel")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind part) (ordinal 0)) (named (kind part) (name "rearWheelChoice")))))) (kind redefinition) (ordinal 0) (authored-target "rearWheels")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 94 7) (end 94 22)) (probe (position 94 7))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine choice constraint"))) (kind expressionOperand) (ordinal 0) (authored-target "engineRqtChoice")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineRqtChoice")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 94 26) (end 94 69)) (probe (position 94 26))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine choice constraint"))) (kind expressionOperand) (ordinal 1) (authored-target "engineRqtChoice::highPerformanceRequirement")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 95 5) (end 95 17)) (probe (position 95 5))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine choice constraint"))) (kind expressionOperand) (ordinal 2) (authored-target "engineChoice")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 95 21) (end 95 47)) (probe (position 95 21))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine choice constraint"))) (kind expressionOperand) (ordinal 3) (authored-target "engineChoice::6cylEngine")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 97 5) (end 97 17)) (probe (position 97 5))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine choice constraint"))) (kind expressionOperand) (ordinal 4) (authored-target "engineChoice")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 97 21) (end 97 47)) (probe (position 97 21))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine choice constraint"))) (kind expressionOperand) (ordinal 5) (authored-target "engineChoice::4cylEngine")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 114 5) (end 114 17)) (probe (position 114 5))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine-transmission selection constraint"))) (kind expressionOperand) (ordinal 0) (authored-target "engineChoice")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 114 21) (end 114 47)) (probe (position 114 21))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine-transmission selection constraint"))) (kind expressionOperand) (ordinal 1) (authored-target "engineChoice::4cylEngine")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 114 52) (end 114 70)) (probe (position 114 52))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine-transmission selection constraint"))) (kind expressionOperand) (ordinal 2) (authored-target "transmissionChoice")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 114 74) (end 114 112)) (probe (position 114 74))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine-transmission selection constraint"))) (kind expressionOperand) (ordinal 3) (authored-target "transmissionChoice::manualTransmission")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 115 5) (end 115 17)) (probe (position 115 5))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine-transmission selection constraint"))) (kind expressionOperand) (ordinal 4) (authored-target "engineChoice")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 115 21) (end 115 47)) (probe (position 115 21))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine-transmission selection constraint"))) (kind expressionOperand) (ordinal 5) (authored-target "engineChoice::6cylEngine")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 115 52) (end 115 70)) (probe (position 115 52))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine-transmission selection constraint"))) (kind expressionOperand) (ordinal 6) (authored-target "transmissionChoice")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 115 74) (end 115 115)) (probe (position 115 74))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engine-transmission selection constraint"))) (kind expressionOperand) (ordinal 7) (authored-target "transmissionChoice::automaticTransmission")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 86 35) (end 86 41)) (probe (position 86 35))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice"))) (kind redefinition) (ordinal 0) (authored-target "engine")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 81 43) (end 81 71)) (probe (position 81 43))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineRqtChoice"))) (kind featureTyping) (ordinal 0) (authored-target "EnginePerformanceRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::RequirementsModel::EnginePerformanceRequirement")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 82 12) (end 82 38)) (probe (position 82 12))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineRqtChoice"))) (kind variant) (ordinal 0) (authored-target "highPerformanceRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::RequirementsModel::highPerformanceRequirement")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 83 12) (end 83 40)) (probe (position 83 12))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineRqtChoice"))) (kind variant) (ordinal 1) (authored-target "normalPerformanceRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::RequirementsModel::normalPerformanceRequirement")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 100 41) (end 100 53)) (probe (position 100 41))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice"))) (kind redefinition) (ordinal 0) (authored-target "transmission")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 134 34) (end 134 50)) (probe (position 134 34))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::vehicleChoice"))) (kind subsetting) (ordinal 0) (authored-target "anyVehicleConfig")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig")))))
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
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 72 34) (end 72 44)) (probe (position 72 34))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutchPort"))) (kind specialization) (ordinal 0) (authored-target "ClutchPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::ClutchPort")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 66 36) (end 66 48)) (probe (position 66 36))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticTransmission"))) (kind specialization) (ordinal 0) (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Transmission")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 68 27) (end 68 33)) (probe (position 68 27))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutch"))) (kind specialization) (ordinal 0) (authored-target "Clutch")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::Clutch")))))
  )
  (query (document "memory://snapshot/7b_variant_configurations.md") (range (start 71 31) (end 71 41)) (probe (position 71 31))
    (reference (id (source (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutchPort"))) (kind specialization) (ordinal 0) (authored-target "ClutchPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/7b_variant_configurations.md") (qualified-name "7b-Variant Configurations::DesignModel::ClutchPort")))))
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
