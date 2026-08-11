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
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "e710bdcdb2cac9bda4c96aa9510da9c186e9ec359210a2ec82d9b7690005b8db") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations"))) (kind "package") (name "7b-Variant Configurations") (declared-name "7b-Variant Configurations") (range (start (line 0) (character 0)) (end (line 0) (character 3937))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 37))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations"))) (authored (membership (kind Import) (visibility "private") (import (reference "RequirementsModel::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 33))))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 31))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations"))) (authored (membership (kind Import) (visibility "private") (import (reference "DesignModel::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 27))))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 1)) (end (line 3) (character 38))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations"))) (authored (membership (kind Import) (visibility "private") (import (reference "VariantDefinitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 16)) (end (line 3) (character 34))))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel"))) (kind "package") (name "DesignModel") (declared-name "DesignModel") (range (start (line 12) (character 1)) (end (line 12) (character 1014))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Clutch"))) (kind "part def") (name "Clutch") (declared-name "Clutch") (range (start (line 16) (character 2)) (end (line 16) (character 18))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ClutchPort"))) (kind "port def") (name "ClutchPort") (declared-name "ClutchPort") (range (start (line 22) (character 2)) (end (line 22) (character 22))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ClutchPort::~ClutchPort"))) (kind "conjugated port definition") (name "~ClutchPort") (declared-name "~ClutchPort") (range (start (line 22) (character 2)) (end (line 22) (character 22))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ClutchPort"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Driveshaft"))) (kind "part def") (name "Driveshaft") (declared-name "Driveshaft") (range (start (line 17) (character 2)) (end (line 17) (character 22))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (range (start (line 14) (character 2)) (end (line 14) (character 18))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::FuelCmdPort"))) (kind "port def") (name "FuelCmdPort") (declared-name "FuelCmdPort") (range (start (line 21) (character 2)) (end (line 21) (character 23))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::FuelCmdPort::~FuelCmdPort"))) (kind "conjugated port definition") (name "~FuelCmdPort") (declared-name "~FuelCmdPort") (range (start (line 21) (character 2)) (end (line 21) (character 23))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::FuelCmdPort"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::RearAxleAssembly"))) (kind "part def") (name "RearAxleAssembly") (declared-name "RearAxleAssembly") (range (start (line 18) (character 2)) (end (line 18) (character 28))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_b"))) (kind "port def") (name "ShaftPort_b") (declared-name "ShaftPort_b") (range (start (line 23) (character 2)) (end (line 23) (character 23))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_b::~ShaftPort_b"))) (kind "conjugated port definition") (name "~ShaftPort_b") (declared-name "~ShaftPort_b") (range (start (line 23) (character 2)) (end (line 23) (character 23))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_b"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_c"))) (kind "port def") (name "ShaftPort_c") (declared-name "ShaftPort_c") (range (start (line 24) (character 2)) (end (line 24) (character 23))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_c::~ShaftPort_c"))) (kind "conjugated port definition") (name "~ShaftPort_c") (declared-name "~ShaftPort_c") (range (start (line 24) (character 2)) (end (line 24) (character 23))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_c"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_d"))) (kind "port def") (name "ShaftPort_d") (declared-name "ShaftPort_d") (range (start (line 25) (character 2)) (end (line 25) (character 23))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_d::~ShaftPort_d"))) (kind "conjugated port definition") (name "~ShaftPort_d") (declared-name "~ShaftPort_d") (range (start (line 25) (character 2)) (end (line 25) (character 23))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_d"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Transmission"))) (kind "part def") (name "Transmission") (declared-name "Transmission") (range (start (line 15) (character 2)) (end (line 15) (character 24))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 13) (character 2)) (end (line 13) (character 19))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::VehicleToRoadPort"))) (kind "port def") (name "VehicleToRoadPort") (declared-name "VehicleToRoadPort") (range (start (line 26) (character 2)) (end (line 26) (character 29))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::VehicleToRoadPort::~VehicleToRoadPort"))) (kind "conjugated port definition") (name "~VehicleToRoadPort") (declared-name "~VehicleToRoadPort") (range (start (line 26) (character 2)) (end (line 26) (character 29))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::VehicleToRoadPort"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Wheel"))) (kind "part def") (name "Wheel") (declared-name "Wheel") (range (start (line 19) (character 2)) (end (line 19) (character 17))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::WheelToRoadPort"))) (kind "port def") (name "WheelToRoadPort") (declared-name "WheelToRoadPort") (range (start (line 27) (character 2)) (end (line 27) (character 27))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::WheelToRoadPort::~WheelToRoadPort"))) (kind "conjugated port definition") (name "~WheelToRoadPort") (declared-name "~WheelToRoadPort") (range (start (line 27) (character 2)) (end (line 27) (character 27))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::WheelToRoadPort"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (range (start (line 29) (character 2)) (end (line 29) (character 652))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 29) (character 17)) (end (line 29) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft"))) (kind "part") (name "driveshaft") (declared-name "driveshaft") (range (start (line 44) (character 3)) (end (line 44) (character 113))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Driveshaft") (range (start (line 44) (character 21)) (end (line 44) (character 31)))))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft::shaftPort_b"))) (kind "port") (name "shaftPort_b") (declared-name "shaftPort_b") (range (start (line 45) (character 4)) (end (line 45) (character 35))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft"))) (authored (membership (kind Feature)) (relationships (typing (reference "ShaftPort_b") (range none)))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft::shaftPort_c"))) (kind "port") (name "shaftPort_c") (declared-name "shaftPort_c") (range (start (line 46) (character 4)) (end (line 46) (character 35))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft"))) (authored (membership (kind Feature)) (relationships (typing (reference "ShaftPort_c") (range none)))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine"))) (kind "part") (name "engine") (declared-name "engine") (range (start (line 34) (character 3)) (end (line 34) (character 69))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 34) (character 17)) (end (line 34) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine::fuelCmdPort"))) (kind "port") (name "fuelCmdPort") (declared-name "fuelCmdPort") (range (start (line 35) (character 4)) (end (line 35) (character 35))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelCmdPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::fuelCmdPort"))) (kind "port") (name "fuelCmdPort") (declared-name "fuelCmdPort") (range (start (line 30) (character 3)) (end (line 30) (character 20))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly"))) (kind "part") (name "rearAxleAssembly") (declared-name "rearAxleAssembly") (range (start (line 49) (character 3)) (end (line 49) (character 134))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "RearAxleAssembly") (range (start (line 49) (character 27)) (end (line 49) (character 43)))))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly::rearWheels"))) (kind "part") (name "rearWheels") (declared-name "rearWheels") (range (start (line 50) (character 4)) (end (line 50) (character 83))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel") (range (start (line 50) (character 22)) (end (line 50) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly::rearWheels::wheelToRoadPort"))) (kind "port") (name "wheelToRoadPort") (declared-name "wheelToRoadPort") (range (start (line 51) (character 5)) (end (line 51) (character 44))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly::rearWheels"))) (authored (membership (kind Feature)) (relationships (typing (reference "WheelToRoadPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission"))) (kind "part") (name "transmission") (declared-name "transmission") (range (start (line 38) (character 3)) (end (line 38) (character 115))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Transmission") (range (start (line 38) (character 23)) (end (line 38) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission::clutch"))) (kind "part") (name "clutch") (declared-name "clutch") (range (start (line 39) (character 4)) (end (line 39) (character 69))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission"))) (authored (membership (kind Feature)) (relationships (typing (reference "Clutch") (range (start (line 39) (character 17)) (end (line 39) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission::clutch::clutchPort"))) (kind "port") (name "clutchPort") (declared-name "clutchPort") (range (start (line 40) (character 5)) (end (line 40) (character 34))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission::clutch"))) (authored (membership (kind Feature)) (relationships (typing (reference "ClutchPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::vehicleToRoadPort"))) (kind "port") (name "vehicleToRoadPort") (declared-name "vehicleToRoadPort") (range (start (line 55) (character 3)) (end (line 55) (character 99))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleToRoadPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::vehicleToRoadPort::wheelToRoadPort"))) (kind "port") (name "wheelToRoadPort") (declared-name "wheelToRoadPort") (range (start (line 56) (character 4)) (end (line 56) (character 46))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::vehicleToRoadPort"))) (authored (membership (kind Feature)) (relationships (typing (reference "WheelToRoadPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::RequirementsModel"))) (kind "package") (name "RequirementsModel") (declared-name "RequirementsModel") (range (start (line 6) (character 1)) (end (line 6) (character 227))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::RequirementsModel::EnginePerformanceRequirement"))) (kind "requirement def") (name "EnginePerformanceRequirement") (declared-name "EnginePerformanceRequirement") (range (start (line 7) (character 2)) (end (line 7) (character 47))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::RequirementsModel"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::RequirementsModel::highPerformanceRequirement"))) (kind "requirement") (name "highPerformanceRequirement") (declared-name "highPerformanceRequirement") (range (start (line 8) (character 2)) (end (line 8) (character 72))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::RequirementsModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "EnginePerformanceRequirement") (range none)))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::RequirementsModel::normalPerformanceRequirement"))) (kind "requirement") (name "normalPerformanceRequirement") (declared-name "normalPerformanceRequirement") (range (start (line 9) (character 2)) (end (line 9) (character 74))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::RequirementsModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "EnginePerformanceRequirement") (range none)))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel"))) (kind "package") (name "VariabilityModel") (declared-name "VariabilityModel") (range (start (line 78) (character 1)) (end (line 78) (character 2049))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig"))) (kind "part") (name "anyVehicleConfig") (declared-name "anyVehicleConfig") (range (start (line 79) (character 2)) (end (line 79) (character 1901))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle") (range (start (line 79) (character 27)) (end (line 79) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice"))) (kind "part") (name "engineChoice") (declared-name "engineChoice") (range (start (line 86) (character 3)) (end (line 86) (character 140))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "engine") (range (start (line 86) (character 35)) (end (line 86) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice::4cylEngine"))) (kind "part") (name "4cylEngine") (declared-name "4cylEngine") (range (start (line 87) (character 12)) (end (line 87) (character 45))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice"))) (authored (membership (kind Feature)) (relationships (typing (reference "4CylEngine") (range (start (line 87) (character 32)) (end (line 87) (character 44)))))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice::6cylEngine"))) (kind "part") (name "6cylEngine") (declared-name "6cylEngine") (range (start (line 88) (character 12)) (end (line 88) (character 45))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice"))) (authored (membership (kind Feature)) (relationships (typing (reference "6CylEngine") (range (start (line 88) (character 32)) (end (line 88) (character 44)))))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly"))) (kind "part") (name "rearAxleAssembly") (range (start (line 118) (character 3)) (end (line 118) (character 558))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "rearAxleAssembly") (range (start (line 118) (character 12)) (end (line 118) (character 28)))))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly::rearWheelChoice"))) (kind "part") (name "rearWheelChoice") (declared-name "rearWheelChoice") (range (start (line 119) (character 4)) (end (line 119) (character 155))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "rearWheels") (range (start (line 119) (character 39)) (end (line 119) (character 49)))))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly::rearWheelChoice::narrowRimWheel"))) (kind "part") (name "narrowRimWheel") (declared-name "narrowRimWheel") (range (start (line 120) (character 13)) (end (line 120) (character 50))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly::rearWheelChoice"))) (authored (membership (kind Feature)) (relationships (typing (reference "NarrowRimWheel") (range (start (line 120) (character 35)) (end (line 120) (character 49)))))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly::rearWheelChoice::wideRimWheel"))) (kind "part") (name "wideRimWheel") (declared-name "wideRimWheel") (range (start (line 121) (character 13)) (end (line 121) (character 46))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly::rearWheelChoice"))) (authored (membership (kind Feature)) (relationships (typing (reference "WideRimWheel") (range (start (line 121) (character 33)) (end (line 121) (character 45)))))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice"))) (kind "part") (name "transmissionChoice") (declared-name "transmissionChoice") (range (start (line 100) (character 3)) (end (line 100) (character 384))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "transmission") (range (start (line 100) (character 41)) (end (line 100) (character 53)))))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::automaticTransmission"))) (kind "part") (name "automaticTransmission") (declared-name "automaticTransmission") (range (start (line 106) (character 12)) (end (line 106) (character 167))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice"))) (authored (membership (kind Feature)) (relationships (typing (reference "AutomaticTransmission") (range (start (line 106) (character 41)) (end (line 106) (character 62)))))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::automaticTransmission::clutch"))) (kind "part") (name "clutch") (declared-name "clutch") (range (start (line 107) (character 5)) (end (line 107) (character 96))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::automaticTransmission"))) (authored (membership (kind Feature)) (relationships (typing (reference "AutomaticClutch") (range (start (line 107) (character 23)) (end (line 107) (character 38)))))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::automaticTransmission::clutch::clutchPort"))) (kind "port") (name "clutchPort") (declared-name "clutchPort") (range (start (line 108) (character 6)) (end (line 108) (character 48))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::automaticTransmission::clutch"))) (authored (membership (kind Feature)) (relationships (typing (reference "AutomaticClutchPort") (range none)) (redefinition (reference "clutchPort") (range (start (line 108) (character 15)) (end (line 108) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::manualTransmission"))) (kind "part") (name "manualTransmission") (declared-name "manualTransmission") (range (start (line 101) (character 12)) (end (line 101) (character 155))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice"))) (authored (membership (kind Feature)) (relationships (typing (reference "ManualTransmission") (range (start (line 101) (character 38)) (end (line 101) (character 56)))))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::manualTransmission::clutch"))) (kind "part") (name "clutch") (declared-name "clutch") (range (start (line 102) (character 5)) (end (line 102) (character 90))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::manualTransmission"))) (authored (membership (kind Feature)) (relationships (typing (reference "ManualClutch") (range (start (line 102) (character 23)) (end (line 102) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::manualTransmission::clutch::clutchPort"))) (kind "port") (name "clutchPort") (declared-name "clutchPort") (range (start (line 103) (character 6)) (end (line 103) (character 45))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::manualTransmission::clutch"))) (authored (membership (kind Feature)) (relationships (typing (reference "ManualClutchPort") (range none)) (redefinition (reference "clutchPort") (range (start (line 103) (character 15)) (end (line 103) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::vehicleChoice"))) (kind "part") (name "vehicleChoice") (declared-name "vehicleChoice") (range (start (line 134) (character 2)) (end (line 134) (character 112))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "anyVehicleConfig") (range (start (line 134) (character 34)) (end (line 134) (character 50)))))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::vehicleChoice::vehicle_c1"))) (kind "part") (name "vehicle_c1") (declared-name "vehicle_c1") (range (start (line 135) (character 11)) (end (line 135) (character 27))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::vehicleChoice"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::vehicleChoice::vehicle_c2"))) (kind "part") (name "vehicle_c2") (declared-name "vehicle_c2") (range (start (line 136) (character 11)) (end (line 136) (character 27))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::vehicleChoice"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions"))) (kind "package") (name "VariantDefinitions") (declared-name "VariantDefinitions") (range (start (line 61) (character 1)) (end (line 61) (character 445))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::4CylEngine"))) (kind "part def") (name "4CylEngine") (declared-name "4CylEngine") (range (start (line 62) (character 2)) (end (line 62) (character 34))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Engine") (range (start (line 62) (character 27)) (end (line 62) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::6CylEngine"))) (kind "part def") (name "6CylEngine") (declared-name "6CylEngine") (range (start (line 63) (character 2)) (end (line 63) (character 34))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Engine") (range (start (line 63) (character 27)) (end (line 63) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutch"))) (kind "part def") (name "AutomaticClutch") (declared-name "AutomaticClutch") (range (start (line 69) (character 2)) (end (line 69) (character 37))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Clutch") (range (start (line 69) (character 30)) (end (line 69) (character 36)))))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutchPort"))) (kind "port def") (name "AutomaticClutchPort") (declared-name "AutomaticClutchPort") (range (start (line 72) (character 2)) (end (line 72) (character 45))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ClutchPort") (range (start (line 72) (character 34)) (end (line 72) (character 44)))))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutchPort::~AutomaticClutchPort"))) (kind "conjugated port definition") (name "~AutomaticClutchPort") (declared-name "~AutomaticClutchPort") (range (start (line 72) (character 2)) (end (line 72) (character 45))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutchPort"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticTransmission"))) (kind "part def") (name "AutomaticTransmission") (declared-name "AutomaticTransmission") (range (start (line 66) (character 2)) (end (line 66) (character 49))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Transmission") (range (start (line 66) (character 36)) (end (line 66) (character 48)))))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutch"))) (kind "part def") (name "ManualClutch") (declared-name "ManualClutch") (range (start (line 68) (character 2)) (end (line 68) (character 34))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Clutch") (range (start (line 68) (character 27)) (end (line 68) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutchPort"))) (kind "port def") (name "ManualClutchPort") (declared-name "ManualClutchPort") (range (start (line 71) (character 2)) (end (line 71) (character 42))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ClutchPort") (range (start (line 71) (character 31)) (end (line 71) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutchPort::~ManualClutchPort"))) (kind "conjugated port definition") (name "~ManualClutchPort") (declared-name "~ManualClutchPort") (range (start (line 71) (character 2)) (end (line 71) (character 42))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutchPort"))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualTransmission"))) (kind "part def") (name "ManualTransmission") (declared-name "ManualTransmission") (range (start (line 65) (character 2)) (end (line 65) (character 46))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Transmission") (range (start (line 65) (character 33)) (end (line 65) (character 45)))))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::NarrowRimWheel"))) (kind "part def") (name "NarrowRimWheel") (declared-name "NarrowRimWheel") (range (start (line 74) (character 2)) (end (line 74) (character 35))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Wheel") (range (start (line 74) (character 29)) (end (line 74) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::WideRimWheel"))) (kind "part def") (name "WideRimWheel") (declared-name "WideRimWheel") (range (start (line 75) (character 2)) (end (line 75) (character 33))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Wheel") (range (start (line 75) (character 27)) (end (line 75) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "7b-Variant Configurations::forAll"))) (kind "import") (name "forAll") (declared-name "forAll") (range (start (line 4) (character 1)) (end (line 4) (character 41))) (parent (node (document "d0") (qualified-name "7b-Variant Configurations"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::forAll") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 4) (character 16)) (end (line 4) (character 40))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "RequirementsModel::*") (range (start (line 1) (character 16)) (end (line 1) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::RequirementsModel")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "DesignModel::*") (range (start (line 2) (character 16)) (end (line 2) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "VariantDefinitions::*") (range (start (line 3) (character 16)) (end (line 3) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 29) (character 17)) (end (line 29) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle"))) (kind bindSource) (ordinal 0)) (authored-target "fuelCmdPort") (range (start (line 32) (character 8)) (end (line 32) (character 19))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::fuelCmdPort")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle"))) (kind bindTarget) (ordinal 0)) (authored-target "engine::fuelCmdPort") (range (start (line 32) (character 22)) (end (line 32) (character 40))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine::fuelCmdPort")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft"))) (kind featureTyping) (ordinal 0)) (authored-target "Driveshaft") (range (start (line 44) (character 21)) (end (line 44) (character 31))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Driveshaft")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft::shaftPort_b"))) (kind featureTyping) (ordinal 0)) (authored-target "ShaftPort_b") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_b")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft::shaftPort_c"))) (kind featureTyping) (ordinal 0)) (authored-target "ShaftPort_c") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_c")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 34) (character 17)) (end (line 34) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine::fuelCmdPort"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelCmdPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::FuelCmdPort")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly"))) (kind featureTyping) (ordinal 0)) (authored-target "RearAxleAssembly") (range (start (line 49) (character 27)) (end (line 49) (character 43))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::RearAxleAssembly")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly::rearWheels"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (range (start (line 50) (character 22)) (end (line 50) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Wheel")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::rearAxleAssembly::rearWheels::wheelToRoadPort"))) (kind featureTyping) (ordinal 0)) (authored-target "WheelToRoadPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::WheelToRoadPort")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission"))) (kind featureTyping) (ordinal 0)) (authored-target "Transmission") (range (start (line 38) (character 23)) (end (line 38) (character 35))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Transmission")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission::clutch"))) (kind featureTyping) (ordinal 0)) (authored-target "Clutch") (range (start (line 39) (character 17)) (end (line 39) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Clutch")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::transmission::clutch::clutchPort"))) (kind featureTyping) (ordinal 0)) (authored-target "ClutchPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ClutchPort")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::vehicleToRoadPort"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleToRoadPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::VehicleToRoadPort")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::vehicleToRoadPort::wheelToRoadPort"))) (kind featureTyping) (ordinal 0)) (authored-target "WheelToRoadPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::WheelToRoadPort")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::RequirementsModel::highPerformanceRequirement"))) (kind featureTyping) (ordinal 0)) (authored-target "EnginePerformanceRequirement") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::RequirementsModel::EnginePerformanceRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::RequirementsModel::normalPerformanceRequirement"))) (kind featureTyping) (ordinal 0)) (authored-target "EnginePerformanceRequirement") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::RequirementsModel::EnginePerformanceRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle") (range (start (line 79) (character 27)) (end (line 79) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig"))) (kind satisfySource) (ordinal 0)) (authored-target "engineRqtChoice") (range (start (line 91) (character 11)) (end (line 91) (character 26))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig"))) (kind satisfyTarget) (ordinal 0)) (authored-target "engineChoice") (range (start (line 91) (character 30)) (end (line 91) (character 42))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice"))) (kind redefinition) (ordinal 0)) (authored-target "engine") (range (start (line 86) (character 35)) (end (line 86) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice::4cylEngine"))) (kind featureTyping) (ordinal 0)) (authored-target "4CylEngine") (range (start (line 87) (character 32)) (end (line 87) (character 44))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::4CylEngine")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::engineChoice::6cylEngine"))) (kind featureTyping) (ordinal 0)) (authored-target "6CylEngine") (range (start (line 88) (character 32)) (end (line 88) (character 44))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::6CylEngine")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly"))) (kind redefinition) (ordinal 0)) (authored-target "rearAxleAssembly") (range (start (line 118) (character 12)) (end (line 118) (character 28))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly::rearWheelChoice"))) (kind redefinition) (ordinal 0)) (authored-target "rearWheels") (range (start (line 119) (character 39)) (end (line 119) (character 49))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly::rearWheelChoice::narrowRimWheel"))) (kind featureTyping) (ordinal 0)) (authored-target "NarrowRimWheel") (range (start (line 120) (character 35)) (end (line 120) (character 49))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::NarrowRimWheel")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::rearAxleAssembly::rearWheelChoice::wideRimWheel"))) (kind featureTyping) (ordinal 0)) (authored-target "WideRimWheel") (range (start (line 121) (character 33)) (end (line 121) (character 45))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::WideRimWheel")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice"))) (kind redefinition) (ordinal 0)) (authored-target "transmission") (range (start (line 100) (character 41)) (end (line 100) (character 53))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::automaticTransmission"))) (kind featureTyping) (ordinal 0)) (authored-target "AutomaticTransmission") (range (start (line 106) (character 41)) (end (line 106) (character 62))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticTransmission")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::automaticTransmission::clutch"))) (kind featureTyping) (ordinal 0)) (authored-target "AutomaticClutch") (range (start (line 107) (character 23)) (end (line 107) (character 38))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutch")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::automaticTransmission::clutch::clutchPort"))) (kind featureTyping) (ordinal 0)) (authored-target "AutomaticClutchPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutchPort")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::automaticTransmission::clutch::clutchPort"))) (kind redefinition) (ordinal 0)) (authored-target "clutchPort") (range (start (line 108) (character 15)) (end (line 108) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::automaticTransmission::clutch::clutchPort")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::manualTransmission"))) (kind featureTyping) (ordinal 0)) (authored-target "ManualTransmission") (range (start (line 101) (character 38)) (end (line 101) (character 56))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualTransmission")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::manualTransmission::clutch"))) (kind featureTyping) (ordinal 0)) (authored-target "ManualClutch") (range (start (line 102) (character 23)) (end (line 102) (character 35))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutch")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::manualTransmission::clutch::clutchPort"))) (kind featureTyping) (ordinal 0)) (authored-target "ManualClutchPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutchPort")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::manualTransmission::clutch::clutchPort"))) (kind redefinition) (ordinal 0)) (authored-target "clutchPort") (range (start (line 103) (character 15)) (end (line 103) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig::transmissionChoice::manualTransmission::clutch::clutchPort")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::vehicleChoice"))) (kind subsetting) (ordinal 0)) (authored-target "anyVehicleConfig") (range (start (line 134) (character 34)) (end (line 134) (character 50))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::VariabilityModel::anyVehicleConfig")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::4CylEngine"))) (kind specialization) (ordinal 0)) (authored-target "Engine") (range (start (line 62) (character 27)) (end (line 62) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::6CylEngine"))) (kind specialization) (ordinal 0)) (authored-target "Engine") (range (start (line 63) (character 27)) (end (line 63) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutch"))) (kind specialization) (ordinal 0)) (authored-target "Clutch") (range (start (line 69) (character 30)) (end (line 69) (character 36))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Clutch")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticClutchPort"))) (kind specialization) (ordinal 0)) (authored-target "ClutchPort") (range (start (line 72) (character 34)) (end (line 72) (character 44))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ClutchPort")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::AutomaticTransmission"))) (kind specialization) (ordinal 0)) (authored-target "Transmission") (range (start (line 66) (character 36)) (end (line 66) (character 48))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Transmission")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutch"))) (kind specialization) (ordinal 0)) (authored-target "Clutch") (range (start (line 68) (character 27)) (end (line 68) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Clutch")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualClutchPort"))) (kind specialization) (ordinal 0)) (authored-target "ClutchPort") (range (start (line 71) (character 31)) (end (line 71) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ClutchPort")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::ManualTransmission"))) (kind specialization) (ordinal 0)) (authored-target "Transmission") (range (start (line 65) (character 33)) (end (line 65) (character 45))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Transmission")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::NarrowRimWheel"))) (kind specialization) (ordinal 0)) (authored-target "Wheel") (range (start (line 74) (character 29)) (end (line 74) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Wheel")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::VariantDefinitions::WideRimWheel"))) (kind specialization) (ordinal 0)) (authored-target "Wheel") (range (start (line 75) (character 27)) (end (line 75) (character 32))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Wheel")))))
    (reference (id (source (node (document "d0") (qualified-name "7b-Variant Configurations::forAll"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::forAll") (range (start (line 4) (character 16)) (end (line 4) (character 40))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Driveshaft"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft::shaftPort_b"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_b"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft::shaftPort_b"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft::shaftPort_c"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::ShaftPort_c"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::driveshaft::shaftPort_c"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine::fuelCmdPort"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::FuelCmdPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine::fuelCmdPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind bind) (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::fuelCmdPort"))) (target (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle::engine::fuelCmdPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7b-Variant Configurations::DesignModel::vehicle"))) (kind bindSource) (ordinal 0)) (expression (kind bind) (source "fuelCmdPort") (target "engine::fuelCmdPort") (source-range (start (line 32) (character 8)) (end (line 32) (character 19))) (target-range (start (line 32) (character 22)) (end (line 32) (character 40)))))
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
