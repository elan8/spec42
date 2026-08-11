# META
~~~ini
description=SysML Example (Analysis): Vehicle Analysis Demo
type=file
~~~
# SOURCE
~~~sysml
package 'Vehicle Analysis Demo' {
    private import ScalarValues::*;
    private import ISQ::*;
    private import USCustomaryUnits::*;
	    
    private import VehicleQuantities::*;
    private import VehicleModel::*;
    private import FuelEconomyRequirementsModel::*;
    private import DynamicsModel::*;
    private import FuelEconomyAnalysisModel::*;	
	
	package VehicleQuantities {
    	private import Quantities::*;
    	private import MeasurementReferences::*;

	    attribute def DistancePerVolumeUnit :> DerivedUnit {
	    	private attribute distancePF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
	        private attribute volumePF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
	        attribute :>> quantityDimension { :>> quantityPowerFactors = (distancePF, volumePF); }
	    }

	    attribute def DistancePerVolumeValue :> ScalarQuantityValue {
	        :>> num : Real;
	        :>> mRef : DistancePerVolumeUnit;
	    }
	    
	    attribute gallon : VolumeUnit = 231.0 * 'in' ** 3;
	    attribute mpg : DistancePerVolumeUnit = 'mi' / gallon;
	}
	
	package VehicleModel {	    
	    item def Fuel;
	        
	    port def FuelPort {
	        out item fuel: Fuel;
	    }
	    
	    part def FuelTank {
	        attribute volumeMax : VolumeValue;
	        attribute fuelVolume : VolumeValue;
	        attribute fuelLevel : Real = fuelVolume / volumeMax;
	        
	        port fuelInPort : ~FuelPort;
	        port fuelOutPort : FuelPort;
	    }
	    
	    part def Wheel {
	        attribute diameter : LengthValue;
	    }
	
	    part def Vehicle {
	        attribute mass : MassValue;
	        attribute cargoMass : MassValue;
	        
	        attribute wheelDiameter : LengthValue;
	        attribute driveTrainEfficiency : Real;
	        
	        attribute fuelEconomy_city : DistancePerVolumeValue;
	        attribute fuelEconomy_highway : DistancePerVolumeValue;
	
	        port fuelInPort : ~FuelPort;
	    }
	    
	    part vehicle_c1 : Vehicle {
	        port :>> fuelInPort {
	            in item :>> fuel; 
	        }
	
	        part fuelTank : FuelTank {
	            port :>> fuelInPort {
	                in item :>> fuel;
	            }
	        }
	
	        bind fuelInPort.fuel = fuelTank.fuelInPort.fuel;
	
	        part wheel : Wheel[4] {
	            :>> diameter = wheelDiameter;
	        }
	    }
	}
	
	package FuelEconomyRequirementsModel {
	    requirement def FuelEconomyRequirement {
	        attribute actualFuelEconomy : DistancePerVolumeValue;
	        attribute requiredFuelEconomy : DistancePerVolumeValue;
	
	        require constraint { actualFuelEconomy >= requiredFuelEconomy }
	    }
	
	    requirement cityFuelEconomyRequirement : FuelEconomyRequirement {
	        :>> requiredFuelEconomy = 25 [mpg];
	    }
	
	    requirement highwayFuelEconomyRequirement : FuelEconomyRequirement {
	        :>> requiredFuelEconomy = 30 [mpg];
	    }
	}
	
	package DynamicsModel {
	    calc def Acceleration { in p : PowerValue; in m : MassValue; in v : SpeedValue;
	    	return : AccelerationValue = p / (m * v);
	    }
	    
	    calc def Velocity { in v0 : SpeedValue; in a : AccelerationValue; in dt : TimeValue;
	    	return : SpeedValue = v0 + a * dt;
		}
		    
	    calc def Position { in x0 : LengthValue; in v : SpeedValue; in dt : TimeValue;
	    	return : LengthValue = x0 + v * dt;
	    }
	    
	    constraint def StraightLineDynamicsEquations {
	        in p : PowerValue;
	        in m : MassValue;
	        in dt : TimeValue;
	        in x_i : LengthValue;
	        in v_i : SpeedValue;
	        in x_f : LengthValue;
	        in v_f : SpeedValue;
	        in a : AccelerationValue;
	    
	        attribute v_avg : SpeedValue = (v_i + v_f)/2;
	
	        a == Acceleration(p, m, v_avg) &
	        v_f == Velocity(v_i, a, dt) &
	        x_f == Position(x_i, v_avg, dt)
	    }
	    
	    action def StraightLineDynamics {
	        in power : PowerValue;
	        in mass : MassValue;
	        in delta_t : TimeValue;
	        in x_in : LengthValue;
	        in v_in : SpeedValue;
	        out x_out : LengthValue;
	        out v_out : SpeedValue;
	        out a_out : AccelerationValue;
	    
	        assert constraint dynamics : StraightLineDynamicsEquations {
	            in p = power;
	            in m = mass;
	            in dt = delta_t;
	            in x_i = x_in;
	            in v_i = v_in;
	            in x_f = x_out;
	            in v_f = v_out;
	            in a = a_out;
	        }
	    }
	}
	
	package FuelEconomyAnalysisModel {
	    private import SequenceFunctions::size;
	    private import SampledFunctions::SampledFunction;
	    private import SampledFunctions::SamplePair;
	    private import ControlFunctions::forAll;
	
		attribute def ScenarioState {
			position : LengthValue;
			velocity : SpeedValue;
		}
		
		attribute def NominalScenario :> SampledFunction {
			attribute def TimeStateRecord :> SamplePair {
				t : TimeValue :>> domainValue;
				s : ScenarioState :>> rangeValue;
			}
			:>> samples : TimeStateRecord;
			n : Natural = size(samples);
		}
		
		analysis def FuelEconomyAnalysis { 
			subject vehicle: Vehicle;
			in attribute scenario : NominalScenario;
			in requirement fuelEconomyRequirement : FuelEconomyRequirement;
			return calculatedFuelEconomy : DistancePerVolumeValue;
			
			objective fuelEconomyAnalysisObjective {
				doc 
				/*
				 * The objective of this analysis is to determine whether the
				 * current vehicle design configuration can satisfy the fuel
				 * economy requirement.
				 */
				
				assume constraint {
					vehicle.wheelDiameter == 33 ['in'] &
					vehicle.driveTrainEfficiency == 0.4
				}
				
				require fuelEconomyRequirement { 
					:>> actualFuelEconomy = calculatedFuelEconomy;
				}
			}
		
			action dynamicsAnalysis {
				in sc: NominalScenario;
				out power : PowerValue[*];
				out acceleration : AccelerationValue[*];
				/*
				 * Solve for the required engine power as a function of time
				 * to support the scenarios.
				 */
				assert constraint straightLineDynamics {
					(1..sc.n-1)->forAll {in i: Integer;
						private thisSample : NominalScenario::TimeStateRecord = 
							sc.samples#(i);
						private nextSample : NominalScenario::TimeStateRecord = 
							sc.samples#(i+1);
						StraightLineDynamicsEquations (
							p = power#(i),
							m = vehicle.mass,
							dt = nextSample.t - thisSample.t,
							x_i = thisSample.s.position,
							v_i = thisSample.s.velocity,
							x_f = nextSample.s.position,
							v_f = nextSample.s.velocity,
							a = acceleration#(i)                    
						)
					}
				}
			}

		    action fuelConsumptionAnalysis {
		        in power : PowerValue[*] = dynamicsAnalysis.power;
		        in acceleration : AccelerationValue[*] = dynamicsAnalysis.acceleration;
		        out fuelEconomy : DistancePerVolumeValue = calculatedFuelEconomy;
		        /*
		         * Solve the engine equations to determine how much fuel is
		         * consumed. The engine RPM is a function of the speed of the
		         * vehicle and the gear state.
		         */
	        }
	    }
	}
	
	part vehicleFuelEconomyAnalysisContext {
	    requirement vehicleFuelEconomyRequirementsGroup {
	        subject vehicle : Vehicle;
	
	        requirement vehicleFuelEconomyRequirement_city :> cityFuelEconomyRequirement {
	            doc /* The vehicle shall provide a fuel economy that is greater than or equal to
	             * 25 miles per gallon for the nominal city driving scenarios.
	             */
	
	            :>> actualFuelEconomy = vehicle.fuelEconomy_city;
	
	            assume constraint { vehicle.cargoMass == 1000 [lb] }
	        }
	
	        requirement vehicleFuelEconomyRequirement_highway :> highwayFuelEconomyRequirement {
	            doc /* The vehicle shall provide a fuel economy that is greater than or equal to
	             * 30 miles per gallon for the nominal highway driving scenarios.
	             */
	
	            :>> actualFuelEconomy = vehicle.fuelEconomy_highway;
	
	            assume constraint { vehicle.cargoMass == 1000 [lb] }
	        }
	
	    }
	    
	    attribute cityScenario : NominalScenario;
	    attribute highwayScenario : NominalScenario;
	
	    analysis cityFuelEconomyAnalysis : FuelEconomyAnalysis {
	        subject vehicle = vehicle_c1;
	        in attribute scenario = cityScenario;
	        in requirement fuelEconomyRequirement = cityFuelEconomyRequirement;
	    }
	
	    analysis highwayFuelEconomyAnalysis : FuelEconomyAnalysis {
	        subject vehicle = vehicle_c1;
	        in attribute scenario = highwayScenario;
	        in requirement fuelEconomyRequirement = highwayFuelEconomyRequirement;
	    }
	
	    part vehicle_c1_analysized :> vehicle_c1 {
	        attribute :>> fuelEconomy_city = cityFuelEconomyAnalysis.calculatedFuelEconomy;
	        attribute :>> fuelEconomy_highway = highwayFuelEconomyAnalysis.calculatedFuelEconomy;
	    }
	
	    satisfy vehicleFuelEconomyRequirementsGroup by vehicle_c1_analysized;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "vehicle_analysis_demo.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 19) (end 1 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 19) (end 2 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 19) (end 3 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 20) (end 12 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 13 20) (end 13 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 5) (end 15 368))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 6) (end 16 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 17 9) (end 17 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 5) (end 21 141))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 22 9) (end 22 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 26 5) (end 26 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 38 9) (end 38 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 38 31) (end 38 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 39 9) (end 39 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 39 32) (end 39 43))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 40 9) (end 40 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 40 9) (end 40 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 40 31) (end 40 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 47 9) (end 47 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 47 30) (end 47 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 51 9) (end 51 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 51 26) (end 51 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 52 9) (end 52 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 52 31) (end 52 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 54 9) (end 54 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 54 35) (end 54 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 55 9) (end 55 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 55 42) (end 55 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 74 14) (end 74 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 74 32) (end 74 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 100 29) (end 100 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 100 48) (end 100 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 100 66) (end 100 84))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 101 6) (end 101 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 104 25) (end 104 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 104 45) (end 104 70))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 104 71) (end 104 89))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 105 6) (end 105 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 108 25) (end 108 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 108 46) (end 108 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 108 65) (end 108 83))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 109 6) (end 109 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 130 9) (end 130 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 131 9) (end 131 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 132 9) (end 132 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 133 9) (end 133 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 134 9) (end 134 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 135 9) (end 135 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 136 9) (end 136 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 137 9) (end 137 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 153 20) (end 153 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 154 20) (end 154 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 155 20) (end 155 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 156 20) (end 156 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 159 3) (end 159 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 160 3) (end 160 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 169 3) (end 169 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 198 4) (end 198 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 199 4) (end 199 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 225 10) (end 225 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 226 10) (end 226 81))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 283 13) (end 283 48))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "2ef35b034f91ea3dc526b2ef9ac8b53d0b2dcc7c5aac668ed5d15ab39bc2a75d") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo"))) (kind "package") (name "Vehicle Analysis Demo") (declared-name "Vehicle Analysis Demo"))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::*#import2"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo"))) (authored (membership (kind Import) (visibility "private") (import (reference "USCustomaryUnits::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::*#import3"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo"))) (authored (membership (kind Import) (visibility "private") (import (reference "VehicleQuantities::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::*#import4"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo"))) (authored (membership (kind Import) (visibility "private") (import (reference "VehicleModel::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::*#import5"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo"))) (authored (membership (kind Import) (visibility "private") (import (reference "FuelEconomyRequirementsModel::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::*#import6"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo"))) (authored (membership (kind Import) (visibility "private") (import (reference "DynamicsModel::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::*#import7"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo"))) (authored (membership (kind Import) (visibility "private") (import (reference "FuelEconomyAnalysisModel::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel"))) (kind "package") (name "DynamicsModel") (declared-name "DynamicsModel") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Acceleration"))) (kind "calc def") (name "Acceleration") (declared-name "Acceleration") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Acceleration::"))) (kind "return parameter") (name "") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Acceleration"))) (authored (relationships (typing (reference "AccelerationValue")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Acceleration::m"))) (kind "in out parameter") (name "m") (declared-name "m") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Acceleration"))) (authored (relationships (typing (reference "MassValue")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Acceleration::p"))) (kind "in out parameter") (name "p") (declared-name "p") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Acceleration"))) (authored (relationships (typing (reference "PowerValue")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Acceleration::v"))) (kind "in out parameter") (name "v") (declared-name "v") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Acceleration"))) (authored (relationships (typing (reference "SpeedValue")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Position"))) (kind "calc def") (name "Position") (declared-name "Position") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Position::"))) (kind "return parameter") (name "") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Position"))) (authored (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Position::dt"))) (kind "in out parameter") (name "dt") (declared-name "dt") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Position"))) (authored (relationships (typing (reference "TimeValue")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Position::v"))) (kind "in out parameter") (name "v") (declared-name "v") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Position"))) (authored (relationships (typing (reference "SpeedValue")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Position::x0"))) (kind "in out parameter") (name "x0") (declared-name "x0") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Position"))) (authored (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics"))) (kind "action def") (name "StraightLineDynamics") (declared-name "StraightLineDynamics") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics::a_out"))) (kind "in out parameter") (name "a_out") (declared-name "a_out") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics"))) (authored (relationships (typing (reference "AccelerationValue")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics::delta_t"))) (kind "in out parameter") (name "delta_t") (declared-name "delta_t") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics"))) (authored (relationships (typing (reference "TimeValue")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics::mass"))) (kind "in out parameter") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics"))) (authored (relationships (typing (reference "MassValue")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics::power"))) (kind "in out parameter") (name "power") (declared-name "power") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics"))) (authored (relationships (typing (reference "PowerValue")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics::v_in"))) (kind "in out parameter") (name "v_in") (declared-name "v_in") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics"))) (authored (relationships (typing (reference "SpeedValue")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics::v_out"))) (kind "in out parameter") (name "v_out") (declared-name "v_out") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics"))) (authored (relationships (typing (reference "SpeedValue")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics::x_in"))) (kind "in out parameter") (name "x_in") (declared-name "x_in") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics"))) (authored (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics::x_out"))) (kind "in out parameter") (name "x_out") (declared-name "x_out") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics"))) (authored (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamicsEquations"))) (kind "constraint def") (name "StraightLineDynamicsEquations") (declared-name "StraightLineDynamicsEquations") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Velocity"))) (kind "calc def") (name "Velocity") (declared-name "Velocity") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Velocity::"))) (kind "return parameter") (name "") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Velocity"))) (authored (relationships (typing (reference "SpeedValue")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Velocity::a"))) (kind "in out parameter") (name "a") (declared-name "a") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Velocity"))) (authored (relationships (typing (reference "AccelerationValue")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Velocity::dt"))) (kind "in out parameter") (name "dt") (declared-name "dt") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Velocity"))) (authored (relationships (typing (reference "TimeValue")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Velocity::v0"))) (kind "in out parameter") (name "v0") (declared-name "v0") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Velocity"))) (authored (relationships (typing (reference "SpeedValue")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel"))) (kind "package") (name "FuelEconomyAnalysisModel") (declared-name "FuelEconomyAnalysisModel") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))) (kind "analysis def") (name "FuelEconomyAnalysis") (declared-name "FuelEconomyAnalysis") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::calculatedFuelEconomy"))) (kind "analysis result") (name "calculatedFuelEconomy") (declared-name "calculatedFuelEconomy") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))) (authored (relationships (typing (reference "DistancePerVolumeValue")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::dynamicsAnalysis"))) (kind "action") (name "dynamicsAnalysis") (declared-name "dynamicsAnalysis") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::dynamicsAnalysis::acceleration"))) (kind "in out parameter") (name "acceleration") (declared-name "acceleration") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::dynamicsAnalysis"))) (authored (relationships (typing (reference "acceleration : AccelerationValue[*]")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::dynamicsAnalysis::power"))) (kind "in out parameter") (name "power") (declared-name "power") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::dynamicsAnalysis"))) (authored (relationships (typing (reference "power : PowerValue[*]")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::dynamicsAnalysis::sc"))) (kind "in out parameter") (name "sc") (declared-name "sc") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::dynamicsAnalysis"))) (authored (relationships (typing (reference "NominalScenario")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumptionAnalysis"))) (kind "action") (name "fuelConsumptionAnalysis") (declared-name "fuelConsumptionAnalysis") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumptionAnalysis::acceleration"))) (kind "in out parameter") (name "acceleration") (declared-name "acceleration") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumptionAnalysis"))) (authored (relationships (typing (reference "acceleration : AccelerationValue[*] = dynamicsAnalysis.acceleration")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumptionAnalysis::fuelEconomy"))) (kind "in out parameter") (name "fuelEconomy") (declared-name "fuelEconomy") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumptionAnalysis"))) (authored (relationships (typing (reference "DistancePerVolumeValue")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumptionAnalysis::power"))) (kind "in out parameter") (name "power") (declared-name "power") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumptionAnalysis"))) (authored (relationships (typing (reference "power : PowerValue[*] = dynamicsAnalysis.power")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelEconomyAnalysisObjective"))) (kind "objective") (name "fuelEconomyAnalysisObjective") (declared-name "fuelEconomyAnalysisObjective") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelEconomyRequirement"))) (kind "requirement") (name "fuelEconomyRequirement") (declared-name "fuelEconomyRequirement") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelEconomyRequirement")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::scenario"))) (kind "attribute") (name "scenario") (declared-name "scenario") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))) (authored (membership (kind Feature)) (relationships (typing (reference "NominalScenario")) (typing (reference "NominalScenario")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))) (authored (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario"))) (kind "attribute def") (name "NominalScenario") (declared-name "NominalScenario") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel"))) (authored (membership (kind Owning)) (relationships (typing (reference "SampledFunction")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::TimeStateRecord"))) (kind "attribute def") (name "TimeStateRecord") (declared-name "TimeStateRecord") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario"))) (authored (membership (kind Owning)) (relationships (typing (reference "SamplePair")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::n"))) (kind "attribute") (name "n") (declared-name "n") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario"))) (authored (membership (kind Feature)) (relationships (typing (reference "Natural")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::samples"))) (kind "attribute") (name "samples") (declared-name "samples") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario"))) (authored (membership (kind Feature)) (relationships (typing (reference "TimeStateRecord")) (redefinition (reference "samples")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::SamplePair"))) (kind "import") (name "SamplePair") (declared-name "SamplePair") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "SampledFunctions::SamplePair") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::SampledFunction"))) (kind "import") (name "SampledFunction") (declared-name "SampledFunction") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "SampledFunctions::SampledFunction") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::ScenarioState"))) (kind "attribute def") (name "ScenarioState") (declared-name "ScenarioState") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::ScenarioState::position"))) (kind "attribute") (name "position") (declared-name "position") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::ScenarioState"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::ScenarioState::velocity"))) (kind "attribute") (name "velocity") (declared-name "velocity") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::ScenarioState"))) (authored (membership (kind Feature)) (relationships (typing (reference "SpeedValue")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::forAll"))) (kind "import") (name "forAll") (declared-name "forAll") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::forAll") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::size"))) (kind "import") (name "size") (declared-name "size") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::size") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel"))) (kind "package") (name "FuelEconomyRequirementsModel") (declared-name "FuelEconomyRequirementsModel") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement"))) (kind "requirement def") (name "FuelEconomyRequirement") (declared-name "FuelEconomyRequirement") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement::_requireConstraint_0"))) (kind "require constraint") (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement::actualFuelEconomy"))) (kind "attribute") (name "actualFuelEconomy") (declared-name "actualFuelEconomy") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement"))) (authored (relationships (typing (reference "DistancePerVolumeValue")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement::requiredFuelEconomy"))) (kind "attribute") (name "requiredFuelEconomy") (declared-name "requiredFuelEconomy") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement"))) (authored (relationships (typing (reference "DistancePerVolumeValue")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::cityFuelEconomyRequirement"))) (kind "requirement") (name "cityFuelEconomyRequirement") (declared-name "cityFuelEconomyRequirement") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelEconomyRequirement")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::cityFuelEconomyRequirement::requiredFuelEconomy"))) (kind "attribute") (name "requiredFuelEconomy") (declared-name "requiredFuelEconomy") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::cityFuelEconomyRequirement"))) (authored (relationships (redefinition (reference "requiredFuelEconomy")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement"))) (kind "requirement") (name "highwayFuelEconomyRequirement") (declared-name "highwayFuelEconomyRequirement") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelEconomyRequirement")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement::requiredFuelEconomy"))) (kind "attribute") (name "requiredFuelEconomy") (declared-name "requiredFuelEconomy") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement"))) (authored (relationships (redefinition (reference "requiredFuelEconomy")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel"))) (kind "package") (name "VehicleModel") (declared-name "VehicleModel") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Fuel"))) (kind "item def") (name "Fuel") (declared-name "Fuel") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort"))) (kind "port def") (name "FuelPort") (declared-name "FuelPort") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort::fuel"))) (kind "item") (name "fuel") (declared-name "fuel") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort"))) (authored (membership (kind Feature)) (relationships (typing (reference "Fuel")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort::~FuelPort"))) (kind "conjugated port definition") (name "~FuelPort") (declared-name "~FuelPort") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank"))) (kind "part def") (name "FuelTank") (declared-name "FuelTank") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelInPort"))) (kind "port") (name "fuelInPort") (declared-name "fuelInPort") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank"))) (authored (membership (kind Feature)) (relationships (typing (reference "~FuelPort")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelLevel"))) (kind "attribute") (name "fuelLevel") (declared-name "fuelLevel") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelOutPort"))) (kind "port") (name "fuelOutPort") (declared-name "fuelOutPort") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelPort")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelVolume"))) (kind "attribute") (name "fuelVolume") (declared-name "fuelVolume") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank"))) (authored (membership (kind Feature)) (relationships (typing (reference "VolumeValue")) (typing (reference "VolumeValue")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::volumeMax"))) (kind "attribute") (name "volumeMax") (declared-name "volumeMax") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank"))) (authored (membership (kind Feature)) (relationships (typing (reference "VolumeValue")) (typing (reference "VolumeValue")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::cargoMass"))) (kind "attribute") (name "cargoMass") (declared-name "cargoMass") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue")) (typing (reference "MassValue")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::driveTrainEfficiency"))) (kind "attribute") (name "driveTrainEfficiency") (declared-name "driveTrainEfficiency") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelEconomy_city"))) (kind "attribute") (name "fuelEconomy_city") (declared-name "fuelEconomy_city") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "DistancePerVolumeValue")) (typing (reference "DistancePerVolumeValue")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelEconomy_highway"))) (kind "attribute") (name "fuelEconomy_highway") (declared-name "fuelEconomy_highway") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "DistancePerVolumeValue")) (typing (reference "DistancePerVolumeValue")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelInPort"))) (kind "port") (name "fuelInPort") (declared-name "fuelInPort") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "~FuelPort")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue")) (typing (reference "MassValue")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::wheelDiameter"))) (kind "attribute") (name "wheelDiameter") (declared-name "wheelDiameter") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")) (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Wheel"))) (kind "part def") (name "Wheel") (declared-name "Wheel") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Wheel::diameter"))) (kind "attribute") (name "diameter") (declared-name "diameter") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Wheel"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")) (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1"))) (kind "part") (name "vehicle_c1") (declared-name "vehicle_c1") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelInPort"))) (kind "port") (name "fuelInPort") (declared-name "fuelInPort") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "fuelInPort")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelTank"))) (kind "part") (name "fuelTank") (declared-name "fuelTank") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelTank")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelTank::fuelInPort"))) (kind "port") (name "fuelInPort") (declared-name "fuelInPort") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelTank"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "fuelInPort")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::wheel"))) (kind "part") (name "wheel") (declared-name "wheel") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::wheel::diameter"))) (kind "attribute") (name "diameter") (declared-name "diameter") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::wheel"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "diameter")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities"))) (kind "package") (name "VehicleQuantities") (declared-name "VehicleQuantities") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit"))) (kind "attribute def") (name "DistancePerVolumeUnit") (declared-name "DistancePerVolumeUnit") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit::distancePF"))) (kind "attribute") (name "distancePF") (declared-name "distancePF") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit::volumePF"))) (kind "attribute") (name "volumePF") (declared-name "volumePF") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue"))) (kind "attribute def") (name "DistancePerVolumeValue") (declared-name "DistancePerVolumeValue") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "DistancePerVolumeUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::gallon"))) (kind "attribute def") (name "gallon") (declared-name "gallon") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeUnit")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::mpg"))) (kind "attribute def") (name "mpg") (declared-name "mpg") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities"))) (authored (membership (kind Owning)) (relationships (typing (reference "DistancePerVolumeUnit")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext"))) (kind "part") (name "vehicleFuelEconomyAnalysisContext") (declared-name "vehicleFuelEconomyAnalysisContext") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::cityScenario"))) (kind "attribute") (name "cityScenario") (declared-name "cityScenario") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext"))) (authored (membership (kind Feature)) (relationships (typing (reference "NominalScenario")) (typing (reference "NominalScenario")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::highwayScenario"))) (kind "attribute") (name "highwayScenario") (declared-name "highwayScenario") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext"))) (authored (membership (kind Feature)) (relationships (typing (reference "NominalScenario")) (typing (reference "NominalScenario")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized"))) (kind "part") (name "vehicle_c1_analysized") (declared-name "vehicle_c1_analysized") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle_c1")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized::fuelEconomy_city"))) (kind "attribute") (name "fuelEconomy_city") (declared-name "fuelEconomy_city") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "fuelEconomy_city")))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized::fuelEconomy_highway"))) (kind "attribute") (name "fuelEconomy_highway") (declared-name "fuelEconomy_highway") (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "fuelEconomy_highway")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "USCustomaryUnits::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::*#import3"))) (kind namespaceImport) (ordinal 0)) (authored-target "VehicleQuantities::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::*#import4"))) (kind namespaceImport) (ordinal 0)) (authored-target "VehicleModel::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::*#import5"))) (kind namespaceImport) (ordinal 0)) (authored-target "FuelEconomyRequirementsModel::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::*#import6"))) (kind namespaceImport) (ordinal 0)) (authored-target "DynamicsModel::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::*#import7"))) (kind namespaceImport) (ordinal 0)) (authored-target "FuelEconomyAnalysisModel::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Acceleration::"))) (kind featureTyping) (ordinal 0)) (authored-target "AccelerationValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Acceleration::m"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Acceleration::p"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Acceleration::v"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Position::"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Position::dt"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Position::v"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Position::x0"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics::a_out"))) (kind featureTyping) (ordinal 0)) (authored-target "AccelerationValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics::delta_t"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics::power"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics::v_in"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics::v_out"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics::x_in"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics::x_out"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Velocity::"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Velocity::a"))) (kind featureTyping) (ordinal 0)) (authored-target "AccelerationValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Velocity::dt"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Velocity::v0"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::calculatedFuelEconomy"))) (kind featureTyping) (ordinal 0)) (authored-target "DistancePerVolumeValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::dynamicsAnalysis::acceleration"))) (kind featureTyping) (ordinal 0)) (authored-target "acceleration : AccelerationValue[*]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::dynamicsAnalysis::power"))) (kind featureTyping) (ordinal 0)) (authored-target "power : PowerValue[*]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::dynamicsAnalysis::sc"))) (kind featureTyping) (ordinal 0)) (authored-target "NominalScenario") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumptionAnalysis::acceleration"))) (kind featureTyping) (ordinal 0)) (authored-target "acceleration : AccelerationValue[*] = dynamicsAnalysis.acceleration") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumptionAnalysis::fuelEconomy"))) (kind featureTyping) (ordinal 0)) (authored-target "DistancePerVolumeValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumptionAnalysis::power"))) (kind featureTyping) (ordinal 0)) (authored-target "power : PowerValue[*] = dynamicsAnalysis.power") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelEconomyRequirement"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelEconomyRequirement") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::scenario"))) (kind featureTyping) (ordinal 0)) (authored-target "NominalScenario") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::scenario"))) (kind featureTyping) (ordinal 1)) (authored-target "NominalScenario") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario"))) (kind featureTyping) (ordinal 0)) (authored-target "SampledFunction") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::SampledFunction")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::TimeStateRecord"))) (kind featureTyping) (ordinal 0)) (authored-target "SamplePair") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::SamplePair")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::n"))) (kind featureTyping) (ordinal 0)) (authored-target "Natural") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::samples"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeStateRecord") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::TimeStateRecord")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::samples"))) (kind redefinition) (ordinal 0)) (authored-target "samples") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::samples")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::SamplePair"))) (kind membershipImport) (ordinal 0)) (authored-target "SampledFunctions::SamplePair") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::SampledFunction"))) (kind membershipImport) (ordinal 0)) (authored-target "SampledFunctions::SampledFunction") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::ScenarioState::position"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::ScenarioState::velocity"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::forAll"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::forAll") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::size"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::size") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement::actualFuelEconomy"))) (kind featureTyping) (ordinal 0)) (authored-target "DistancePerVolumeValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement::requiredFuelEconomy"))) (kind featureTyping) (ordinal 0)) (authored-target "DistancePerVolumeValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::cityFuelEconomyRequirement"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelEconomyRequirement") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::cityFuelEconomyRequirement::requiredFuelEconomy"))) (kind redefinition) (ordinal 0)) (authored-target "requiredFuelEconomy") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::cityFuelEconomyRequirement::requiredFuelEconomy")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelEconomyRequirement") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement::requiredFuelEconomy"))) (kind redefinition) (ordinal 0)) (authored-target "requiredFuelEconomy") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement::requiredFuelEconomy")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort::fuel"))) (kind featureTyping) (ordinal 0)) (authored-target "Fuel") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Fuel")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelInPort"))) (kind featureTyping) (ordinal 0)) (authored-target "~FuelPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelLevel"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelLevel"))) (kind featureTyping) (ordinal 1)) (authored-target "Real") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelOutPort"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelVolume"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelVolume"))) (kind featureTyping) (ordinal 1)) (authored-target "VolumeValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::volumeMax"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::volumeMax"))) (kind featureTyping) (ordinal 1)) (authored-target "VolumeValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::cargoMass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::cargoMass"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::driveTrainEfficiency"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::driveTrainEfficiency"))) (kind featureTyping) (ordinal 1)) (authored-target "Real") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelEconomy_city"))) (kind featureTyping) (ordinal 0)) (authored-target "DistancePerVolumeValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelEconomy_city"))) (kind featureTyping) (ordinal 1)) (authored-target "DistancePerVolumeValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelEconomy_highway"))) (kind featureTyping) (ordinal 0)) (authored-target "DistancePerVolumeValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelEconomy_highway"))) (kind featureTyping) (ordinal 1)) (authored-target "DistancePerVolumeValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelInPort"))) (kind featureTyping) (ordinal 0)) (authored-target "~FuelPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::mass"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::wheelDiameter"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::wheelDiameter"))) (kind featureTyping) (ordinal 1)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Wheel::diameter"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Wheel::diameter"))) (kind featureTyping) (ordinal 1)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1"))) (kind bindSource) (ordinal 0)) (authored-target "fuelInPort::fuel") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1"))) (kind bindTarget) (ordinal 0)) (authored-target "fuelTank::fuelInPort::fuel") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelInPort"))) (kind redefinition) (ordinal 0)) (authored-target "fuelInPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelInPort")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelTank"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelTank") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelTank::fuelInPort"))) (kind redefinition) (ordinal 0)) (authored-target "fuelInPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelTank::fuelInPort")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::wheel"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Wheel")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::wheel::diameter"))) (kind redefinition) (ordinal 0)) (authored-target "diameter") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::wheel::diameter")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Quantities::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "MeasurementReferences::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit::distancePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit::volumePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "DistancePerVolumeUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::gallon"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::mpg"))) (kind featureTyping) (ordinal 0)) (authored-target "DistancePerVolumeUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext"))) (kind satisfySource) (ordinal 0)) (authored-target "vehicleFuelEconomyRequirementsGroup") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext"))) (kind satisfyTarget) (ordinal 0)) (authored-target "vehicle_c1_analysized") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::cityScenario"))) (kind featureTyping) (ordinal 0)) (authored-target "NominalScenario") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::cityScenario"))) (kind featureTyping) (ordinal 1)) (authored-target "NominalScenario") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::highwayScenario"))) (kind featureTyping) (ordinal 0)) (authored-target "NominalScenario") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::highwayScenario"))) (kind featureTyping) (ordinal 1)) (authored-target "NominalScenario") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle_c1") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized::fuelEconomy_city"))) (kind redefinition) (ordinal 0)) (authored-target "fuelEconomy_city") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized::fuelEconomy_city")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized::fuelEconomy_highway"))) (kind redefinition) (ordinal 0)) (authored-target "fuelEconomy_highway") (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized::fuelEconomy_highway")))))
  )
  (relationships
    (relationship (kind subject) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::calculatedFuelEconomy"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::calculatedFuelEconomy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::dynamicsAnalysis::sc"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::dynamicsAnalysis::sc"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumptionAnalysis::fuelEconomy"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumptionAnalysis::fuelEconomy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelEconomyRequirement"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelEconomyRequirement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::scenario"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::scenario"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::scenario"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::scenario"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::vehicle"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::SampledFunction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::TimeStateRecord"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::SamplePair"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::TimeStateRecord"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::samples"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::TimeStateRecord"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::samples"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::samples"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::samples"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::samples"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement::actualFuelEconomy"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement::actualFuelEconomy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement::requiredFuelEconomy"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement::requiredFuelEconomy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::cityFuelEconomyRequirement"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::cityFuelEconomyRequirement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::cityFuelEconomyRequirement::requiredFuelEconomy"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::cityFuelEconomyRequirement::requiredFuelEconomy"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::cityFuelEconomyRequirement::requiredFuelEconomy"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement::requiredFuelEconomy"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement::requiredFuelEconomy"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement::requiredFuelEconomy"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort::fuel"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Fuel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort::fuel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelInPort"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelInPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelOutPort"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelOutPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelEconomy_city"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelEconomy_city"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelEconomy_city"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelEconomy_city"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelEconomy_highway"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelEconomy_highway"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelEconomy_highway"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelEconomy_highway"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelInPort"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelInPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelInPort"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelInPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelInPort"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelTank"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelTank"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelTank::fuelInPort"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelTank::fuelInPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelTank::fuelInPort"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::wheel"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Wheel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::wheel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::wheel::diameter"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::wheel::diameter"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::wheel::diameter"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue::mRef"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue::mRef"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue::num"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::mpg"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::mpg"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::cityScenario"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::cityScenario"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::cityScenario"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::cityScenario"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::highwayScenario"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::highwayScenario"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::highwayScenario"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::highwayScenario"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized::fuelEconomy_city"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized::fuelEconomy_city"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized::fuelEconomy_city"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized::fuelEconomy_highway"))) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized::fuelEconomy_highway"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized::fuelEconomy_highway"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamicsEquations")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumptionAnalysis::fuelEconomy")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelEconomyRequirement")) (expression (status "incomplete") (error "expression is incomplete")) (analysis (status "incomplete")))
    (node (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement")) (expression (status "incomplete") (error "expression is incomplete")) (analysis (status "incomplete")))
    (node (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement::_requireConstraint_0")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::cityFuelEconomyRequirement")) (expression (status "incomplete") (error "expression is incomplete")) (analysis (status "incomplete")))
    (node (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::cityFuelEconomyRequirement::requiredFuelEconomy")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement")) (expression (status "incomplete") (error "expression is incomplete")) (analysis (status "incomplete")))
    (node (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement::requiredFuelEconomy")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelLevel")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::wheel::diameter")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::gallon")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::mpg")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized::fuelEconomy_city")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized::fuelEconomy_highway")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 2 19) (end 2 22)) (probe (position 2 19))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQ::*")
        (range (start 2 19) (end 2 22))
        (outcome (status unresolved))
      )
    )
    (query (range (start 40 31) (end 40 35)) (probe (position 40 31))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelLevel"))
        (kind featureTyping) (ordinal 1) (authored-target "Real")
        (range (start 40 31) (end 40 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 55 42) (end 55 46)) (probe (position 55 42))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::driveTrainEfficiency"))
        (kind featureTyping) (ordinal 1) (authored-target "Real")
        (range (start 55 42) (end 55 46))
        (outcome (status unresolved))
      )
    )
    (query (range (start 76 22) (end 76 27)) (probe (position 76 22))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::wheel"))
        (kind featureTyping) (ordinal 0) (authored-target "Wheel")
        (range (start 76 22) (end 76 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Wheel") (range (start 46 5) (end 46 71)))
        )
      )
    )
    (query (range (start 22 9) (end 22 16)) (probe (position 22 9))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 22 9) (end 22 16))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue::num") (range (start 22 9) (end 22 24)))
        )
      )
    )
    (query (range (start 63 23) (end 63 30)) (probe (position 63 23))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 63 23) (end 63 30))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle") (range (start 50 5) (end 50 392)))
        )
      )
    )
    (query (range (start 23 9) (end 23 17)) (probe (position 23 9))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 23 9) (end 23 17))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue::mRef") (range (start 23 9) (end 23 42)))
        )
      )
    )
    (query (range (start 68 25) (end 68 33)) (probe (position 68 25))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelTank"))
        (kind featureTyping) (ordinal 0) (authored-target "FuelTank")
        (range (start 68 25) (end 68 33))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank") (range (start 37 5) (end 37 268)))
        )
      )
    )
    (query (range (start 51 26) (end 51 35)) (probe (position 51 26))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::mass"))
        (kind featureTyping) (ordinal 1) (authored-target "MassValue")
        (range (start 51 26) (end 51 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 52 31) (end 52 40)) (probe (position 52 31))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::cargoMass"))
        (kind featureTyping) (ordinal 1) (authored-target "MassValue")
        (range (start 52 31) (end 52 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 12 20) (end 12 30)) (probe (position 12 20))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Quantities::*")
        (range (start 12 20) (end 12 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 64 18) (end 64 28)) (probe (position 64 18))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelInPort"))
        (kind redefinition) (ordinal 0) (authored-target "fuelInPort")
        (range (start 64 18) (end 64 28))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelInPort") (range (start 64 9) (end 64 73)))
        )
      )
    )
    (query (range (start 69 22) (end 69 32)) (probe (position 69 22))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelTank::fuelInPort"))
        (kind redefinition) (ordinal 0) (authored-target "fuelInPort")
        (range (start 69 22) (end 69 32))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelTank::fuelInPort") (range (start 69 13) (end 69 84)))
        )
      )
    )
    (query (range (start 278 35) (end 278 45)) (probe (position 278 35))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized"))
        (kind subsetting) (ordinal 0) (authored-target "vehicle_c1")
        (range (start 278 35) (end 278 45))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1") (range (start 63 5) (end 63 396)))
        )
      )
    )
    (query (range (start 38 31) (end 38 42)) (probe (position 38 31))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::volumeMax"))
        (kind featureTyping) (ordinal 1) (authored-target "VolumeValue")
        (range (start 38 31) (end 38 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 39 32) (end 39 43)) (probe (position 39 32))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelVolume"))
        (kind featureTyping) (ordinal 1) (authored-target "VolumeValue")
        (range (start 39 32) (end 39 43))
        (outcome (status unresolved))
      )
    )
    (query (range (start 47 30) (end 47 41)) (probe (position 47 30))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Wheel::diameter"))
        (kind featureTyping) (ordinal 1) (authored-target "LengthValue")
        (range (start 47 30) (end 47 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 54 35) (end 54 46)) (probe (position 54 35))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::wheelDiameter"))
        (kind featureTyping) (ordinal 1) (authored-target "LengthValue")
        (range (start 54 35) (end 54 46))
        (outcome (status unresolved))
      )
    )
    (query (range (start 168 3) (end 168 14)) (probe (position 168 3))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::samples"))
        (kind redefinition) (ordinal 0) (authored-target "samples")
        (range (start 168 3) (end 168 14))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::samples") (range (start 168 3) (end 168 33)))
        )
      )
    )
    (query (range (start 1 19) (end 1 31)) (probe (position 1 19))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 1 19) (end 1 31))
        (outcome (status unresolved))
      )
    )
    (query (range (start 6 19) (end 6 31)) (probe (position 6 19))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::*#import4"))
        (kind namespaceImport) (ordinal 0) (authored-target "VehicleModel::*")
        (range (start 6 19) (end 6 31))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel") (range (start 30 1) (end 30 1274)))
        )
      )
    )
    (query (range (start 77 13) (end 77 25)) (probe (position 77 13))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::wheel::diameter"))
        (kind redefinition) (ordinal 0) (authored-target "diameter")
        (range (start 77 13) (end 77 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::wheel::diameter") (range (start 77 13) (end 77 42)))
        )
      )
    )
    (query (range (start 8 19) (end 8 32)) (probe (position 8 19))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::*#import6"))
        (kind namespaceImport) (ordinal 0) (authored-target "DynamicsModel::*")
        (range (start 8 19) (end 8 32))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel") (range (start 99 1) (end 99 1566)))
        )
      )
    )
    (query (range (start 74 14) (end 74 29)) (probe (position 74 14))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1"))
        (kind bindSource) (ordinal 0) (authored-target "fuelInPort::fuel")
        (range (start 74 14) (end 74 29))
        (outcome (status unresolved))
      )
    )
    (query (range (start 174 27) (end 174 42)) (probe (position 174 27))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::scenario"))
        (kind featureTyping) (ordinal 1) (authored-target "NominalScenario")
        (range (start 174 27) (end 174 42))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario") (range (start 163 2) (end 163 249)))
        )
      )
    )
    (query (range (start 263 30) (end 263 45)) (probe (position 263 30))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::cityScenario"))
        (kind featureTyping) (ordinal 1) (authored-target "NominalScenario")
        (range (start 263 30) (end 263 45))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario") (range (start 163 2) (end 163 249)))
        )
      )
    )
    (query (range (start 264 33) (end 264 48)) (probe (position 264 33))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::highwayScenario"))
        (kind featureTyping) (ordinal 1) (authored-target "NominalScenario")
        (range (start 264 33) (end 264 48))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario") (range (start 163 2) (end 163 249)))
        )
      )
    )
    (query (range (start 3 19) (end 3 35)) (probe (position 3 19))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::*#import2"))
        (kind namespaceImport) (ordinal 0) (authored-target "USCustomaryUnits::*")
        (range (start 3 19) (end 3 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 279 23) (end 279 39)) (probe (position 279 23))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized::fuelEconomy_city"))
        (kind redefinition) (ordinal 0) (authored-target "fuelEconomy_city")
        (range (start 279 23) (end 279 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized::fuelEconomy_city") (range (start 279 9) (end 279 88)))
        )
      )
    )
    (query (range (start 5 19) (end 5 36)) (probe (position 5 19))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::*#import3"))
        (kind namespaceImport) (ordinal 0) (authored-target "VehicleQuantities::*")
        (range (start 5 19) (end 5 36))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities") (range (start 11 1) (end 11 747)))
        )
      )
    )
    (query (range (start 18 23) (end 18 40)) (probe (position 18 23))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 18 23) (end 18 40))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit::quantityDimension") (range (start 18 9) (end 18 95)))
        )
      )
    )
    (query (range (start 280 23) (end 280 42)) (probe (position 280 23))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized::fuelEconomy_highway"))
        (kind redefinition) (ordinal 0) (authored-target "fuelEconomy_highway")
        (range (start 280 23) (end 280 42))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized::fuelEconomy_highway") (range (start 280 9) (end 280 94)))
        )
      )
    )
    (query (range (start 13 20) (end 13 41)) (probe (position 13 20))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "MeasurementReferences::*")
        (range (start 13 20) (end 13 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 283 52) (end 283 73)) (probe (position 283 52))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext"))
        (kind satisfyTarget) (ordinal 0) (authored-target "vehicle_c1_analysized")
        (range (start 283 52) (end 283 73))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized") (range (start 278 5) (end 278 238)))
        )
      )
    )
    (query (range (start 57 38) (end 57 60)) (probe (position 57 38))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelEconomy_city"))
        (kind featureTyping) (ordinal 1) (authored-target "DistancePerVolumeValue")
        (range (start 57 38) (end 57 60))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue") (range (start 21 5) (end 21 141)))
        )
      )
    )
    (query (range (start 58 41) (end 58 63)) (probe (position 58 41))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelEconomy_highway"))
        (kind featureTyping) (ordinal 1) (authored-target "DistancePerVolumeValue")
        (range (start 58 41) (end 58 63))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue") (range (start 21 5) (end 21 141)))
        )
      )
    )
    (query (range (start 91 9) (end 91 32)) (probe (position 91 9))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::cityFuelEconomyRequirement::requiredFuelEconomy"))
        (kind redefinition) (ordinal 0) (authored-target "requiredFuelEconomy")
        (range (start 91 9) (end 91 32))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::cityFuelEconomyRequirement::requiredFuelEconomy") (range (start 91 9) (end 91 44)))
        )
      )
    )
    (query (range (start 95 9) (end 95 32)) (probe (position 95 9))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement::requiredFuelEconomy"))
        (kind redefinition) (ordinal 0) (authored-target "requiredFuelEconomy")
        (range (start 95 9) (end 95 32))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement::requiredFuelEconomy") (range (start 95 9) (end 95 44)))
        )
      )
    )
    (query (range (start 153 20) (end 153 43)) (probe (position 153 20))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::size"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::size")
        (range (start 153 20) (end 153 43))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 19) (end 9 43)) (probe (position 9 19))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::*#import7"))
        (kind namespaceImport) (ordinal 0) (authored-target "FuelEconomyAnalysisModel::*")
        (range (start 9 19) (end 9 43))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel") (range (start 152 1) (end 152 2612)))
        )
      )
    )
    (query (range (start 74 32) (end 74 56)) (probe (position 74 32))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1"))
        (kind bindTarget) (ordinal 0) (authored-target "fuelTank::fuelInPort::fuel")
        (range (start 74 32) (end 74 56))
        (outcome (status unresolved))
      )
    )
    (query (range (start 156 20) (end 156 44)) (probe (position 156 20))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::forAll"))
        (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::forAll")
        (range (start 156 20) (end 156 44))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 19) (end 7 47)) (probe (position 7 19))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::*#import5"))
        (kind namespaceImport) (ordinal 0) (authored-target "FuelEconomyRequirementsModel::*")
        (range (start 7 19) (end 7 47))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel") (range (start 82 1) (end 82 551)))
        )
      )
    )
    (query (range (start 155 20) (end 155 48)) (probe (position 155 20))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::SamplePair"))
        (kind membershipImport) (ordinal 0) (authored-target "SampledFunctions::SamplePair")
        (range (start 155 20) (end 155 48))
        (outcome (status unresolved))
      )
    )
    (query (range (start 154 20) (end 154 53)) (probe (position 154 20))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::SampledFunction"))
        (kind membershipImport) (ordinal 0) (authored-target "SampledFunctions::SampledFunction")
        (range (start 154 20) (end 154 53))
        (outcome (status unresolved))
      )
    )
    (query (range (start 283 13) (end 283 48)) (probe (position 283 13))
      (reference
        (source (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext"))
        (kind satisfySource) (ordinal 0) (authored-target "vehicleFuelEconomyRequirementsGroup")
        (range (start 283 13) (end 283 48))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
