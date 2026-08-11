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
  (document "7b_variant_configurations.md"
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
        (range (start 86 35) (end 86 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 91 11) (end 91 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 100 41) (end 100 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 119 39) (end 119 49))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "e710bdcdb2cac9bda4c96aa9510da9c186e9ec359210a2ec82d9b7690005b8db") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations"))) (kind "package") (name "7b-Variant Configurations") (declared-name "7b-Variant Configurations"))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "7b-Variant Configurations"))) (authored (membership (kind Import) (visibility "private") (import (reference "RequirementsModel::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "7b-Variant Configurations"))) (authored (membership (kind Import) (visibility "private") (import (reference "DesignModel::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::*#import2"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "7b-Variant Configurations"))) (authored (membership (kind Import) (visibility "private") (import (reference "VariantDefinitions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel"))) (kind "package") (name "DesignModel") (declared-name "DesignModel") (parent (node (document "d0") (qualified-name "7b-Variant Configurations"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Clutch"))) (kind "part def") (name "Clutch") (declared-name "Clutch") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ClutchPort"))) (kind "port def") (name "ClutchPort") (declared-name "ClutchPort") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ClutchPort::~ClutchPort"))) (kind "conjugated port definition") (name "~ClutchPort") (declared-name "~ClutchPort") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ClutchPort"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Driveshaft"))) (kind "part def") (name "Driveshaft") (declared-name "Driveshaft") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::FuelCmdPort"))) (kind "port def") (name "FuelCmdPort") (declared-name "FuelCmdPort") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::FuelCmdPort::~FuelCmdPort"))) (kind "conjugated port definition") (name "~FuelCmdPort") (declared-name "~FuelCmdPort") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::FuelCmdPort"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::RearAxleAssembly"))) (kind "part def") (name "RearAxleAssembly") (declared-name "RearAxleAssembly") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_b"))) (kind "port def") (name "ShaftPort_b") (declared-name "ShaftPort_b") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_b::~ShaftPort_b"))) (kind "conjugated port definition") (name "~ShaftPort_b") (declared-name "~ShaftPort_b") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_b"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_c"))) (kind "port def") (name "ShaftPort_c") (declared-name "ShaftPort_c") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_c::~ShaftPort_c"))) (kind "conjugated port definition") (name "~ShaftPort_c") (declared-name "~ShaftPort_c") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_c"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_d"))) (kind "port def") (name "ShaftPort_d") (declared-name "ShaftPort_d") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_d::~ShaftPort_d"))) (kind "conjugated port definition") (name "~ShaftPort_d") (declared-name "~ShaftPort_d") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_d"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Transmission"))) (kind "part def") (name "Transmission") (declared-name "Transmission") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::VehicleToRoadPort"))) (kind "port def") (name "VehicleToRoadPort") (declared-name "VehicleToRoadPort") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::VehicleToRoadPort::~VehicleToRoadPort"))) (kind "conjugated port definition") (name "~VehicleToRoadPort") (declared-name "~VehicleToRoadPort") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::VehicleToRoadPort"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Wheel"))) (kind "part def") (name "Wheel") (declared-name "Wheel") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::WheelToRoadPort"))) (kind "port def") (name "WheelToRoadPort") (declared-name "WheelToRoadPort") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::WheelToRoadPort::~WheelToRoadPort"))) (kind "conjugated port definition") (name "~WheelToRoadPort") (declared-name "~WheelToRoadPort") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::WheelToRoadPort"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft"))) (kind "part") (name "driveshaft") (declared-name "driveshaft") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Driveshaft")))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft::shaftPort_b"))) (kind "port") (name "shaftPort_b") (declared-name "shaftPort_b") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft"))) (authored (membership (kind Feature)) (relationships (typing (reference "ShaftPort_b")))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft::shaftPort_c"))) (kind "port") (name "shaftPort_c") (declared-name "shaftPort_c") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft"))) (authored (membership (kind Feature)) (relationships (typing (reference "ShaftPort_c")))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine"))) (kind "part") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine::fuelCmdPort"))) (kind "port") (name "fuelCmdPort") (declared-name "fuelCmdPort") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelCmdPort")))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::fuelCmdPort"))) (kind "port") (name "fuelCmdPort") (declared-name "fuelCmdPort") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly"))) (kind "part") (name "rearAxleAssembly") (declared-name "rearAxleAssembly") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "RearAxleAssembly")))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly::rearWheels"))) (kind "part") (name "rearWheels") (declared-name "rearWheels") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel")))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly::rearWheels::wheelToRoadPort"))) (kind "port") (name "wheelToRoadPort") (declared-name "wheelToRoadPort") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly::rearWheels"))) (authored (membership (kind Feature)) (relationships (typing (reference "WheelToRoadPort")))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission"))) (kind "part") (name "transmission") (declared-name "transmission") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Transmission")))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission::clutch"))) (kind "part") (name "clutch") (declared-name "clutch") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission"))) (authored (membership (kind Feature)) (relationships (typing (reference "Clutch")))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission::clutch::clutchPort"))) (kind "port") (name "clutchPort") (declared-name "clutchPort") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission::clutch"))) (authored (membership (kind Feature)) (relationships (typing (reference "ClutchPort")))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::vehicleToRoadPort"))) (kind "port") (name "vehicleToRoadPort") (declared-name "vehicleToRoadPort") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleToRoadPort")))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::vehicleToRoadPort::wheelToRoadPort"))) (kind "port") (name "wheelToRoadPort") (declared-name "wheelToRoadPort") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::vehicleToRoadPort"))) (authored (membership (kind Feature)) (relationships (typing (reference "WheelToRoadPort")))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::RequirementsModel"))) (kind "package") (name "RequirementsModel") (declared-name "RequirementsModel") (parent (node (document "d0") (qualified-name "7b-Variant Configurations"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::RequirementsModel::EnginePerformanceRequirement"))) (kind "requirement def") (name "EnginePerformanceRequirement") (declared-name "EnginePerformanceRequirement") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::RequirementsModel"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::RequirementsModel::highPerformanceRequirement"))) (kind "requirement") (name "highPerformanceRequirement") (declared-name "highPerformanceRequirement") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::RequirementsModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "EnginePerformanceRequirement")))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::RequirementsModel::normalPerformanceRequirement"))) (kind "requirement") (name "normalPerformanceRequirement") (declared-name "normalPerformanceRequirement") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::RequirementsModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "EnginePerformanceRequirement")))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel"))) (kind "package") (name "VariabilityModel") (declared-name "VariabilityModel") (parent (node (document "d0") (qualified-name "7b-Variant Configurations"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig"))) (kind "part") (name "anyVehicleConfig") (declared-name "anyVehicleConfig") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle")))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice"))) (kind "part") (name "engineChoice") (declared-name "engineChoice") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "engine")))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice::4cylEngine"))) (kind "part") (name "4cylEngine") (declared-name "4cylEngine") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice"))) (authored (membership (kind Feature)) (relationships (typing (reference "4CylEngine")))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice::6cylEngine"))) (kind "part") (name "6cylEngine") (declared-name "6cylEngine") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice"))) (authored (membership (kind Feature)) (relationships (typing (reference "6CylEngine")))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly"))) (kind "part") (name "rearAxleAssembly") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "rearAxleAssembly")))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly::rearWheelChoice"))) (kind "part") (name "rearWheelChoice") (declared-name "rearWheelChoice") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "rearWheels")))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly::rearWheelChoice::narrowRimWheel"))) (kind "part") (name "narrowRimWheel") (declared-name "narrowRimWheel") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly::rearWheelChoice"))) (authored (membership (kind Feature)) (relationships (typing (reference "NarrowRimWheel")))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly::rearWheelChoice::wideRimWheel"))) (kind "part") (name "wideRimWheel") (declared-name "wideRimWheel") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly::rearWheelChoice"))) (authored (membership (kind Feature)) (relationships (typing (reference "WideRimWheel")))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice"))) (kind "part") (name "transmissionChoice") (declared-name "transmissionChoice") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "transmission")))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::automaticTransmission"))) (kind "part") (name "automaticTransmission") (declared-name "automaticTransmission") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice"))) (authored (membership (kind Feature)) (relationships (typing (reference "AutomaticTransmission")))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::automaticTransmission::clutch"))) (kind "part") (name "clutch") (declared-name "clutch") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::automaticTransmission"))) (authored (membership (kind Feature)) (relationships (typing (reference "AutomaticClutch")))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::automaticTransmission::clutch::clutchPort"))) (kind "port") (name "clutchPort") (declared-name "clutchPort") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::automaticTransmission::clutch"))) (authored (membership (kind Feature)) (relationships (typing (reference "AutomaticClutchPort")) (redefinition (reference "clutchPort")))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::manualTransmission"))) (kind "part") (name "manualTransmission") (declared-name "manualTransmission") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice"))) (authored (membership (kind Feature)) (relationships (typing (reference "ManualTransmission")))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::manualTransmission::clutch"))) (kind "part") (name "clutch") (declared-name "clutch") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::manualTransmission"))) (authored (membership (kind Feature)) (relationships (typing (reference "ManualClutch")))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::manualTransmission::clutch::clutchPort"))) (kind "port") (name "clutchPort") (declared-name "clutchPort") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::manualTransmission::clutch"))) (authored (membership (kind Feature)) (relationships (typing (reference "ManualClutchPort")) (redefinition (reference "clutchPort")))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::vehicleChoice"))) (kind "part") (name "vehicleChoice") (declared-name "vehicleChoice") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "anyVehicleConfig")))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::vehicleChoice::vehicle_c1"))) (kind "part") (name "vehicle_c1") (declared-name "vehicle_c1") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::vehicleChoice"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::vehicleChoice::vehicle_c2"))) (kind "part") (name "vehicle_c2") (declared-name "vehicle_c2") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::vehicleChoice"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions"))) (kind "package") (name "VariantDefinitions") (declared-name "VariantDefinitions") (parent (node (document "d0") (qualified-name "7b-Variant Configurations"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::4CylEngine"))) (kind "part def") (name "4CylEngine") (declared-name "4CylEngine") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::6CylEngine"))) (kind "part def") (name "6CylEngine") (declared-name "6CylEngine") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutch"))) (kind "part def") (name "AutomaticClutch") (declared-name "AutomaticClutch") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Clutch")))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutchPort"))) (kind "port def") (name "AutomaticClutchPort") (declared-name "AutomaticClutchPort") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ClutchPort")))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutchPort::~AutomaticClutchPort"))) (kind "conjugated port definition") (name "~AutomaticClutchPort") (declared-name "~AutomaticClutchPort") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutchPort"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticTransmission"))) (kind "part def") (name "AutomaticTransmission") (declared-name "AutomaticTransmission") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Transmission")))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutch"))) (kind "part def") (name "ManualClutch") (declared-name "ManualClutch") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Clutch")))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutchPort"))) (kind "port def") (name "ManualClutchPort") (declared-name "ManualClutchPort") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ClutchPort")))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutchPort::~ManualClutchPort"))) (kind "conjugated port definition") (name "~ManualClutchPort") (declared-name "~ManualClutchPort") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutchPort"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualTransmission"))) (kind "part def") (name "ManualTransmission") (declared-name "ManualTransmission") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Transmission")))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::NarrowRimWheel"))) (kind "part def") (name "NarrowRimWheel") (declared-name "NarrowRimWheel") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Wheel")))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::WideRimWheel"))) (kind "part def") (name "WideRimWheel") (declared-name "WideRimWheel") (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Wheel")))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::forAll"))) (kind "import") (name "forAll") (declared-name "forAll") (parent (node (document "d0") (qualified-name "7b-Variant Configurations"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::forAll") (origin Import) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "RequirementsModel::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::RequirementsModel")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "DesignModel::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "VariantDefinitions::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle"))) (kind bindSource) (ordinal 0)) (authored-target "fuelCmdPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::fuelCmdPort")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle"))) (kind bindTarget) (ordinal 0)) (authored-target "engine::fuelCmdPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine::fuelCmdPort")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft"))) (kind featureTyping) (ordinal 0)) (authored-target "Driveshaft") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Driveshaft")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft::shaftPort_b"))) (kind featureTyping) (ordinal 0)) (authored-target "ShaftPort_b") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_b")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft::shaftPort_c"))) (kind featureTyping) (ordinal 0)) (authored-target "ShaftPort_c") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_c")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine::fuelCmdPort"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelCmdPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::FuelCmdPort")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly"))) (kind featureTyping) (ordinal 0)) (authored-target "RearAxleAssembly") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::RearAxleAssembly")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly::rearWheels"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Wheel")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly::rearWheels::wheelToRoadPort"))) (kind featureTyping) (ordinal 0)) (authored-target "WheelToRoadPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::WheelToRoadPort")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission"))) (kind featureTyping) (ordinal 0)) (authored-target "Transmission") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Transmission")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission::clutch"))) (kind featureTyping) (ordinal 0)) (authored-target "Clutch") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Clutch")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission::clutch::clutchPort"))) (kind featureTyping) (ordinal 0)) (authored-target "ClutchPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ClutchPort")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::vehicleToRoadPort"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleToRoadPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::VehicleToRoadPort")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::vehicleToRoadPort::wheelToRoadPort"))) (kind featureTyping) (ordinal 0)) (authored-target "WheelToRoadPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::WheelToRoadPort")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::RequirementsModel::highPerformanceRequirement"))) (kind featureTyping) (ordinal 0)) (authored-target "EnginePerformanceRequirement") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::RequirementsModel::EnginePerformanceRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::RequirementsModel::normalPerformanceRequirement"))) (kind featureTyping) (ordinal 0)) (authored-target "EnginePerformanceRequirement") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::RequirementsModel::EnginePerformanceRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig"))) (kind satisfySource) (ordinal 0)) (authored-target "engineRqtChoice") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig"))) (kind satisfyTarget) (ordinal 0)) (authored-target "engineChoice") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice"))) (kind redefinition) (ordinal 0)) (authored-target "engine") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice::4cylEngine"))) (kind featureTyping) (ordinal 0)) (authored-target "4CylEngine") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::4CylEngine")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice::6cylEngine"))) (kind featureTyping) (ordinal 0)) (authored-target "6CylEngine") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::6CylEngine")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly"))) (kind redefinition) (ordinal 0)) (authored-target "rearAxleAssembly") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly::rearWheelChoice"))) (kind redefinition) (ordinal 0)) (authored-target "rearWheels") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly::rearWheelChoice::narrowRimWheel"))) (kind featureTyping) (ordinal 0)) (authored-target "NarrowRimWheel") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::NarrowRimWheel")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly::rearWheelChoice::wideRimWheel"))) (kind featureTyping) (ordinal 0)) (authored-target "WideRimWheel") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::WideRimWheel")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice"))) (kind redefinition) (ordinal 0)) (authored-target "transmission") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::automaticTransmission"))) (kind featureTyping) (ordinal 0)) (authored-target "AutomaticTransmission") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticTransmission")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::automaticTransmission::clutch"))) (kind featureTyping) (ordinal 0)) (authored-target "AutomaticClutch") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutch")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::automaticTransmission::clutch::clutchPort"))) (kind featureTyping) (ordinal 0)) (authored-target "AutomaticClutchPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutchPort")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::automaticTransmission::clutch::clutchPort"))) (kind redefinition) (ordinal 0)) (authored-target "clutchPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::automaticTransmission::clutch::clutchPort")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::manualTransmission"))) (kind featureTyping) (ordinal 0)) (authored-target "ManualTransmission") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualTransmission")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::manualTransmission::clutch"))) (kind featureTyping) (ordinal 0)) (authored-target "ManualClutch") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutch")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::manualTransmission::clutch::clutchPort"))) (kind featureTyping) (ordinal 0)) (authored-target "ManualClutchPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutchPort")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::manualTransmission::clutch::clutchPort"))) (kind redefinition) (ordinal 0)) (authored-target "clutchPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::manualTransmission::clutch::clutchPort")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::vehicleChoice"))) (kind subsetting) (ordinal 0)) (authored-target "anyVehicleConfig") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::4CylEngine"))) (kind specialization) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::6CylEngine"))) (kind specialization) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutch"))) (kind specialization) (ordinal 0)) (authored-target "Clutch") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Clutch")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutchPort"))) (kind specialization) (ordinal 0)) (authored-target "ClutchPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ClutchPort")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticTransmission"))) (kind specialization) (ordinal 0)) (authored-target "Transmission") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Transmission")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutch"))) (kind specialization) (ordinal 0)) (authored-target "Clutch") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Clutch")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutchPort"))) (kind specialization) (ordinal 0)) (authored-target "ClutchPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ClutchPort")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualTransmission"))) (kind specialization) (ordinal 0)) (authored-target "Transmission") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Transmission")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::NarrowRimWheel"))) (kind specialization) (ordinal 0)) (authored-target "Wheel") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Wheel")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::WideRimWheel"))) (kind specialization) (ordinal 0)) (authored-target "Wheel") (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Wheel")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::forAll"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::forAll") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Driveshaft"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft::shaftPort_b"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_b"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft::shaftPort_b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft::shaftPort_c"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_c"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft::shaftPort_c"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine::fuelCmdPort"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::FuelCmdPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine::fuelCmdPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind bind) (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::fuelCmdPort"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine::fuelCmdPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle"))) (kind bindSource) (ordinal 0)) (expression (kind bind) (source "fuelCmdPort") (target "engine::fuelCmdPort")))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::RearAxleAssembly"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly::rearWheels"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Wheel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly::rearWheels"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly::rearWheels::wheelToRoadPort"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::WheelToRoadPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly::rearWheels::wheelToRoadPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Transmission"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission::clutch"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Clutch"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission::clutch"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission::clutch::clutchPort"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ClutchPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission::clutch::clutchPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::vehicleToRoadPort"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::VehicleToRoadPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::vehicleToRoadPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::vehicleToRoadPort::wheelToRoadPort"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::WheelToRoadPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::vehicleToRoadPort::wheelToRoadPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "7b-Variant Configurations::RequirementsModel::highPerformanceRequirement"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::RequirementsModel::EnginePerformanceRequirement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::RequirementsModel::highPerformanceRequirement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "7b-Variant Configurations::RequirementsModel::normalPerformanceRequirement"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::RequirementsModel::EnginePerformanceRequirement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::RequirementsModel::normalPerformanceRequirement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice::4cylEngine"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::4CylEngine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice::4cylEngine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice::6cylEngine"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::6CylEngine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice::6cylEngine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly::rearWheelChoice::narrowRimWheel"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::NarrowRimWheel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly::rearWheelChoice::narrowRimWheel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly::rearWheelChoice::wideRimWheel"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::WideRimWheel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly::rearWheelChoice::wideRimWheel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::automaticTransmission"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticTransmission"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::automaticTransmission"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::automaticTransmission::clutch"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutch"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::automaticTransmission::clutch"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::automaticTransmission::clutch::clutchPort"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutchPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::automaticTransmission::clutch::clutchPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::automaticTransmission::clutch::clutchPort"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::automaticTransmission::clutch::clutchPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::automaticTransmission::clutch::clutchPort"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::manualTransmission"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualTransmission"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::manualTransmission"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::manualTransmission::clutch"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutch"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::manualTransmission::clutch"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::manualTransmission::clutch::clutchPort"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutchPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::manualTransmission::clutch::clutchPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::manualTransmission::clutch::clutchPort"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::manualTransmission::clutch::clutchPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::manualTransmission::clutch::clutchPort"))) (kind redefinition) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::vehicleChoice"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::vehicleChoice"))) (kind subsetting) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::4CylEngine"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::4CylEngine"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::6CylEngine"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::6CylEngine"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutch"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Clutch"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutch"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutchPort"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ClutchPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutchPort"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticTransmission"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Transmission"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticTransmission"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutch"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Clutch"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutch"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutchPort"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ClutchPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutchPort"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualTransmission"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Transmission"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualTransmission"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::NarrowRimWheel"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Wheel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::NarrowRimWheel"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::WideRimWheel"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Wheel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::WideRimWheel"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 50 22) (end 50 27)) (probe (position 50 22))
      (reference
        (source (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly::rearWheels"))
        (kind featureTyping) (ordinal 0) (authored-target "Wheel")
        (range (start 50 22) (end 50 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Wheel") (range (start 19 2) (end 19 17)))
        )
      )
    )
    (query (range (start 74 29) (end 74 34)) (probe (position 74 29))
      (reference
        (source (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::NarrowRimWheel"))
        (kind specialization) (ordinal 0) (authored-target "Wheel")
        (range (start 74 29) (end 74 34))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Wheel") (range (start 19 2) (end 19 17)))
        )
      )
    )
    (query (range (start 75 27) (end 75 32)) (probe (position 75 27))
      (reference
        (source (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::WideRimWheel"))
        (kind specialization) (ordinal 0) (authored-target "Wheel")
        (range (start 75 27) (end 75 32))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Wheel") (range (start 19 2) (end 19 17)))
        )
      )
    )
    (query (range (start 34 17) (end 34 23)) (probe (position 34 17))
      (reference
        (source (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine"))
        (kind featureTyping) (ordinal 0) (authored-target "Engine")
        (range (start 34 17) (end 34 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Engine") (range (start 14 2) (end 14 18)))
        )
      )
    )
    (query (range (start 39 17) (end 39 23)) (probe (position 39 17))
      (reference
        (source (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission::clutch"))
        (kind featureTyping) (ordinal 0) (authored-target "Clutch")
        (range (start 39 17) (end 39 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Clutch") (range (start 16 2) (end 16 18)))
        )
      )
    )
    (query (range (start 62 27) (end 62 33)) (probe (position 62 27))
      (reference
        (source (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::4CylEngine"))
        (kind specialization) (ordinal 0) (authored-target "Engine")
        (range (start 62 27) (end 62 33))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Engine") (range (start 14 2) (end 14 18)))
        )
      )
    )
    (query (range (start 63 27) (end 63 33)) (probe (position 63 27))
      (reference
        (source (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::6CylEngine"))
        (kind specialization) (ordinal 0) (authored-target "Engine")
        (range (start 63 27) (end 63 33))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Engine") (range (start 14 2) (end 14 18)))
        )
      )
    )
    (query (range (start 68 27) (end 68 33)) (probe (position 68 27))
      (reference
        (source (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutch"))
        (kind specialization) (ordinal 0) (authored-target "Clutch")
        (range (start 68 27) (end 68 33))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Clutch") (range (start 16 2) (end 16 18)))
        )
      )
    )
    (query (range (start 69 30) (end 69 36)) (probe (position 69 30))
      (reference
        (source (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutch"))
        (kind specialization) (ordinal 0) (authored-target "Clutch")
        (range (start 69 30) (end 69 36))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Clutch") (range (start 16 2) (end 16 18)))
        )
      )
    )
    (query (range (start 86 35) (end 86 41)) (probe (position 86 35))
      (reference
        (source (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice"))
        (kind redefinition) (ordinal 0) (authored-target "engine")
        (range (start 86 35) (end 86 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 29 17) (end 29 24)) (probe (position 29 17))
      (reference
        (source (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 29 17) (end 29 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Vehicle") (range (start 13 2) (end 13 19)))
        )
      )
    )
    (query (range (start 79 27) (end 79 34)) (probe (position 79 27))
      (reference
        (source (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig"))
        (kind subsetting) (ordinal 0) (authored-target "vehicle")
        (range (start 79 27) (end 79 34))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle") (range (start 29 2) (end 29 652)))
        )
      )
    )
    (query (range (start 44 21) (end 44 31)) (probe (position 44 21))
      (reference
        (source (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft"))
        (kind featureTyping) (ordinal 0) (authored-target "Driveshaft")
        (range (start 44 21) (end 44 31))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Driveshaft") (range (start 17 2) (end 17 22)))
        )
      )
    )
    (query (range (start 71 31) (end 71 41)) (probe (position 71 31))
      (reference
        (source (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutchPort"))
        (kind specialization) (ordinal 0) (authored-target "ClutchPort")
        (range (start 71 31) (end 71 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ClutchPort") (range (start 22 2) (end 22 22)))
        )
      )
    )
    (query (range (start 72 34) (end 72 44)) (probe (position 72 34))
      (reference
        (source (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutchPort"))
        (kind specialization) (ordinal 0) (authored-target "ClutchPort")
        (range (start 72 34) (end 72 44))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ClutchPort") (range (start 22 2) (end 22 22)))
        )
      )
    )
    (query (range (start 103 15) (end 103 25)) (probe (position 103 15))
      (reference
        (source (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::manualTransmission::clutch::clutchPort"))
        (kind redefinition) (ordinal 0) (authored-target "clutchPort")
        (range (start 103 15) (end 103 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::manualTransmission::clutch::clutchPort") (range (start 103 6) (end 103 45)))
        )
      )
    )
    (query (range (start 108 15) (end 108 25)) (probe (position 108 15))
      (reference
        (source (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::automaticTransmission::clutch::clutchPort"))
        (kind redefinition) (ordinal 0) (authored-target "clutchPort")
        (range (start 108 15) (end 108 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::automaticTransmission::clutch::clutchPort") (range (start 108 6) (end 108 48)))
        )
      )
    )
    (query (range (start 119 39) (end 119 49)) (probe (position 119 39))
      (reference
        (source (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly::rearWheelChoice"))
        (kind redefinition) (ordinal 0) (authored-target "rearWheels")
        (range (start 119 39) (end 119 49))
        (outcome (status unresolved))
      )
    )
    (query (range (start 2 16) (end 2 27)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "7b-Variant Configurations::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "DesignModel::*")
        (range (start 2 16) (end 2 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7b-Variant Configurations::DesignModel") (range (start 12 1) (end 12 1014)))
        )
      )
    )
    (query (range (start 32 8) (end 32 19)) (probe (position 32 8))
      (reference
        (source (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle"))
        (kind bindSource) (ordinal 0) (authored-target "fuelCmdPort")
        (range (start 32 8) (end 32 19))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::fuelCmdPort") (range (start 30 3) (end 30 20)))
        )
      )
    )
    (query (range (start 38 23) (end 38 35)) (probe (position 38 23))
      (reference
        (source (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission"))
        (kind featureTyping) (ordinal 0) (authored-target "Transmission")
        (range (start 38 23) (end 38 35))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Transmission") (range (start 15 2) (end 15 24)))
        )
      )
    )
    (query (range (start 65 33) (end 65 45)) (probe (position 65 33))
      (reference
        (source (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualTransmission"))
        (kind specialization) (ordinal 0) (authored-target "Transmission")
        (range (start 65 33) (end 65 45))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Transmission") (range (start 15 2) (end 15 24)))
        )
      )
    )
    (query (range (start 66 36) (end 66 48)) (probe (position 66 36))
      (reference
        (source (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticTransmission"))
        (kind specialization) (ordinal 0) (authored-target "Transmission")
        (range (start 66 36) (end 66 48))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Transmission") (range (start 15 2) (end 15 24)))
        )
      )
    )
    (query (range (start 87 32) (end 87 44)) (probe (position 87 32))
      (reference
        (source (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice::4cylEngine"))
        (kind featureTyping) (ordinal 0) (authored-target "4CylEngine")
        (range (start 87 32) (end 87 44))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::4CylEngine") (range (start 62 2) (end 62 34)))
        )
      )
    )
    (query (range (start 88 32) (end 88 44)) (probe (position 88 32))
      (reference
        (source (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice::6cylEngine"))
        (kind featureTyping) (ordinal 0) (authored-target "6CylEngine")
        (range (start 88 32) (end 88 44))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::6CylEngine") (range (start 63 2) (end 63 34)))
        )
      )
    )
    (query (range (start 91 30) (end 91 42)) (probe (position 91 30))
      (reference
        (source (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig"))
        (kind satisfyTarget) (ordinal 0) (authored-target "engineChoice")
        (range (start 91 30) (end 91 42))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice") (range (start 86 3) (end 86 140)))
        )
      )
    )
    (query (range (start 100 41) (end 100 53)) (probe (position 100 41))
      (reference
        (source (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice"))
        (kind redefinition) (ordinal 0) (authored-target "transmission")
        (range (start 100 41) (end 100 53))
        (outcome (status unresolved))
      )
    )
    (query (range (start 102 23) (end 102 35)) (probe (position 102 23))
      (reference
        (source (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::manualTransmission::clutch"))
        (kind featureTyping) (ordinal 0) (authored-target "ManualClutch")
        (range (start 102 23) (end 102 35))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutch") (range (start 68 2) (end 68 34)))
        )
      )
    )
    (query (range (start 121 33) (end 121 45)) (probe (position 121 33))
      (reference
        (source (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly::rearWheelChoice::wideRimWheel"))
        (kind featureTyping) (ordinal 0) (authored-target "WideRimWheel")
        (range (start 121 33) (end 121 45))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::WideRimWheel") (range (start 75 2) (end 75 33)))
        )
      )
    )
    (query (range (start 120 35) (end 120 49)) (probe (position 120 35))
      (reference
        (source (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly::rearWheelChoice::narrowRimWheel"))
        (kind featureTyping) (ordinal 0) (authored-target "NarrowRimWheel")
        (range (start 120 35) (end 120 49))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::NarrowRimWheel") (range (start 74 2) (end 74 35)))
        )
      )
    )
    (query (range (start 91 11) (end 91 26)) (probe (position 91 11))
      (reference
        (source (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig"))
        (kind satisfySource) (ordinal 0) (authored-target "engineRqtChoice")
        (range (start 91 11) (end 91 26))
        (outcome (status unresolved))
      )
    )
    (query (range (start 107 23) (end 107 38)) (probe (position 107 23))
      (reference
        (source (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::automaticTransmission::clutch"))
        (kind featureTyping) (ordinal 0) (authored-target "AutomaticClutch")
        (range (start 107 23) (end 107 38))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutch") (range (start 69 2) (end 69 37)))
        )
      )
    )
    (query (range (start 49 27) (end 49 43)) (probe (position 49 27))
      (reference
        (source (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly"))
        (kind featureTyping) (ordinal 0) (authored-target "RearAxleAssembly")
        (range (start 49 27) (end 49 43))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::RearAxleAssembly") (range (start 18 2) (end 18 28)))
        )
      )
    )
    (query (range (start 118 12) (end 118 28)) (probe (position 118 12))
      (reference
        (source (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly"))
        (kind redefinition) (ordinal 0) (authored-target "rearAxleAssembly")
        (range (start 118 12) (end 118 28))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly") (range (start 118 3) (end 118 558)))
        )
      )
    )
    (query (range (start 134 34) (end 134 50)) (probe (position 134 34))
      (reference
        (source (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::vehicleChoice"))
        (kind subsetting) (ordinal 0) (authored-target "anyVehicleConfig")
        (range (start 134 34) (end 134 50))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig") (range (start 79 2) (end 79 1901)))
        )
      )
    )
    (query (range (start 1 16) (end 1 33)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "7b-Variant Configurations::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "RequirementsModel::*")
        (range (start 1 16) (end 1 33))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7b-Variant Configurations::RequirementsModel") (range (start 6 1) (end 6 227)))
        )
      )
    )
    (query (range (start 3 16) (end 3 34)) (probe (position 3 16))
      (reference
        (source (document "d0") (qualified-name "7b-Variant Configurations::*#import2"))
        (kind namespaceImport) (ordinal 0) (authored-target "VariantDefinitions::*")
        (range (start 3 16) (end 3 34))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions") (range (start 61 1) (end 61 445)))
        )
      )
    )
    (query (range (start 32 22) (end 32 40)) (probe (position 32 22))
      (reference
        (source (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle"))
        (kind bindTarget) (ordinal 0) (authored-target "engine::fuelCmdPort")
        (range (start 32 22) (end 32 40))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine::fuelCmdPort") (range (start 35 4) (end 35 35)))
        )
      )
    )
    (query (range (start 101 38) (end 101 56)) (probe (position 101 38))
      (reference
        (source (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::manualTransmission"))
        (kind featureTyping) (ordinal 0) (authored-target "ManualTransmission")
        (range (start 101 38) (end 101 56))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualTransmission") (range (start 65 2) (end 65 46)))
        )
      )
    )
    (query (range (start 106 41) (end 106 62)) (probe (position 106 41))
      (reference
        (source (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::automaticTransmission"))
        (kind featureTyping) (ordinal 0) (authored-target "AutomaticTransmission")
        (range (start 106 41) (end 106 62))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticTransmission") (range (start 66 2) (end 66 49)))
        )
      )
    )
    (query (range (start 4 16) (end 4 40)) (probe (position 4 16))
      (reference
        (source (document "d0") (qualified-name "7b-Variant Configurations::forAll"))
        (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::forAll")
        (range (start 4 16) (end 4 40))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
