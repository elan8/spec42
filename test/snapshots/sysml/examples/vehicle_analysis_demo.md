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
  (document "memory://snapshot/vehicle_analysis_demo.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 19) (end 1 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 19) (end 2 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 19) (end 3 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 20) (end 12 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 13 20) (end 13 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 15 44) (end 15 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 36) (end 16 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 16 65) (end 16 73))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 16 87) (end 16 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 17 37) (end 17 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 17 66) (end 17 74))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 17 88) (end 17 96))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 18 23) (end 18 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 18 47) (end 18 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 21 45) (end 21 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 22 13) (end 22 16))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 22 19) (end 22 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 23 13) (end 23 17))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 26 24) (end 26 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_port_definition_member")
        (source "semantic")
        (range (start 34 9) (end 34 29))
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
        (range (start 39 32) (end 39 43))
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
        (range (start 47 30) (end 47 41))
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
        (range (start 52 31) (end 52 40))
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
        (range (start 55 42) (end 55 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 64 18) (end 64 28))
      )
      (diagnostic
        (severity error)
        (code "recovered_port_body_element")
        (source "parser")
        (range (start 65 13) (end 66 9))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 65 13) (end 66 9))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 69 22) (end 69 32))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 74 9) (end 74 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 77 17) (end 77 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 87 9) (end 87 72))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 91 13) (end 91 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 95 13) (end 95 32))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 100 5) (end 102 6))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 104 5) (end 106 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 108 5) (end 110 6))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 112 5) (end 127 6))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 130 9) (end 130 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 131 9) (end 131 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 132 9) (end 132 32))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 133 9) (end 133 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 134 9) (end 134 30))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 135 9) (end 135 33))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 136 9) (end 136 32))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 137 9) (end 137 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 139 9) (end 148 10))
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
        (range (start 159 14) (end 159 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 160 14) (end 160 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 163 35) (end 163 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 164 36) (end 164 46))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 165 4) (end 166 4))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 166 4) (end 167 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 168 7) (end 168 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 169 7) (end 169 14))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 173 3) (end 173 28))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 175 3) (end 175 66))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 176 3) (end 176 57))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 178 3) (end 194 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 196 3) (end 222 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 224 6) (end 233 10))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 239 9) (end 239 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 246 17) (end 246 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 248 13) (end 248 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 256 17) (end 256 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 258 13) (end 258 65))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 266 5) (end 270 6))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 272 5) (end 276 6))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 279 23) (end 279 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 280 23) (end 280 42))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 283 5) (end 283 74))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:eb40a694a564d325af4a54cd01459502344bc37ccf6688f758713c7b53aa958b") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQ") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "USCustomaryUnits") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "VehicleQuantities") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "VehicleModel") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind import) (ordinal 5))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "FuelEconomyRequirementsModel") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind import) (ordinal 6))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "DynamicsModel") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind import) (ordinal 7))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "FuelEconomyAnalysisModel") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::DynamicsModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::size") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SampledFunctions::SampledFunction") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SampledFunctions::SamplePair") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlFunctions::forAll") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))) (kind analysis-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::scenario"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "NominalScenario"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SampledFunction"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TimeStateRecord")) (redefinition (reference "samples"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::TimeStateRecord"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SamplePair"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::n"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Natural"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::ScenarioState"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::ScenarioState::position"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::ScenarioState::velocity"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpeedValue"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement::actualFuelEconomy"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DistancePerVolumeValue"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement::requiredFuelEconomy"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DistancePerVolumeValue"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::cityFuelEconomyRequirement"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelEconomyRequirement"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "requiredFuelEconomy"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelEconomyRequirement"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "requiredFuelEconomy"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::Fuel"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelInPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelPort") (conjugated true))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelLevel"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelOutPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelPort"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelVolume"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VolumeValue"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::volumeMax"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VolumeValue"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::cargoMass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::driveTrainEfficiency"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelEconomy_city"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DistancePerVolumeValue"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelEconomy_highway"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DistancePerVolumeValue"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelInPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelPort") (conjugated true))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::wheelDiameter"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::Wheel"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::Wheel::diameter"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind port) (ordinal 0))))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "fuelInPort"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelTank"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelTank"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind port) (ordinal 0))))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "fuelInPort"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::wheel"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "diameter"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleQuantities"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Quantities") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "MeasurementReferences") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DerivedUnit"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit::distancePF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit::volumePF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DistancePerVolumeUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::gallon"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "VolumeUnit"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::mpg"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "DistancePerVolumeUnit"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::cityScenario"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "NominalScenario"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::highwayScenario"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "NominalScenario"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicleFuelEconomyRequirementsGroup"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "cityFuelEconomyRequirement"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "actualFuelEconomy"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "highwayFuelEconomyRequirement"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "actualFuelEconomy"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "vehicle_c1"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "fuelEconomy_city"))))
    (declaration (id (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "fuelEconomy_highway"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQ")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "USCustomaryUnits")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0))
      (authored-target "VehicleQuantities")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleQuantities")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind import) (ordinal 4))))) (kind namespaceImport) (ordinal 0))
      (authored-target "VehicleModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind import) (ordinal 5))))) (kind namespaceImport) (ordinal 0))
      (authored-target "FuelEconomyRequirementsModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind import) (ordinal 6))))) (kind namespaceImport) (ordinal 0))
      (authored-target "DynamicsModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::DynamicsModel")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind import) (ordinal 7))))) (kind namespaceImport) (ordinal 0))
      (authored-target "FuelEconomyAnalysisModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::size")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "SampledFunctions::SampledFunction")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "SampledFunctions::SamplePair")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlFunctions::forAll")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::scenario"))) (kind featureTyping) (ordinal 0))
      (authored-target "NominalScenario")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario"))) (kind specialization) (ordinal 0))
      (authored-target "SampledFunction")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "TimeStateRecord")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::TimeStateRecord")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "samples")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::TimeStateRecord"))) (kind specialization) (ordinal 0))
      (authored-target "SamplePair")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::n"))) (kind featureTyping) (ordinal 0))
      (authored-target "Natural")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::ScenarioState::position"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::ScenarioState::velocity"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpeedValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement::actualFuelEconomy"))) (kind featureTyping) (ordinal 0))
      (authored-target "DistancePerVolumeValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement::requiredFuelEconomy"))) (kind featureTyping) (ordinal 0))
      (authored-target "DistancePerVolumeValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::cityFuelEconomyRequirement"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelEconomyRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "requiredFuelEconomy")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelEconomyRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "requiredFuelEconomy")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelInPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelLevel"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelOutPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelVolume"))) (kind featureTyping) (ordinal 0))
      (authored-target "VolumeValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::volumeMax"))) (kind featureTyping) (ordinal 0))
      (authored-target "VolumeValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::cargoMass"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::driveTrainEfficiency"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelEconomy_city"))) (kind featureTyping) (ordinal 0))
      (authored-target "DistancePerVolumeValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelEconomy_highway"))) (kind featureTyping) (ordinal 0))
      (authored-target "DistancePerVolumeValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelInPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::wheelDiameter"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::Wheel::diameter"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "fuelInPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelTank"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelTank")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "fuelInPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::wheel"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::Wheel")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "diameter")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Quantities")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "MeasurementReferences")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit"))) (kind specialization) (ordinal 0))
      (authored-target "DerivedUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit::distancePF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit::volumePF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "DistancePerVolumeUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::gallon"))) (kind featureTyping) (ordinal 0))
      (authored-target "VolumeUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::mpg"))) (kind featureTyping) (ordinal 0))
      (authored-target "DistancePerVolumeUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::cityScenario"))) (kind featureTyping) (ordinal 0))
      (authored-target "NominalScenario")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::highwayScenario"))) (kind featureTyping) (ordinal 0))
      (authored-target "NominalScenario")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city"))) (kind subsetting) (ordinal 0))
      (authored-target "cityFuelEconomyRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::cityFuelEconomyRequirement")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "actualFuelEconomy")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway"))) (kind subsetting) (ordinal 0))
      (authored-target "highwayFuelEconomyRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "actualFuelEconomy")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized"))) (kind subsetting) (ordinal 0))
      (authored-target "vehicle_c1")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "fuelEconomy_city")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "fuelEconomy_highway")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::scenario"))) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::scenario"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::TimeStateRecord"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement::actualFuelEconomy"))) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement::actualFuelEconomy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement::requiredFuelEconomy"))) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement::requiredFuelEconomy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::cityFuelEconomyRequirement"))) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::cityFuelEconomyRequirement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement"))) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelInPort"))) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelInPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelOutPort"))) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelOutPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelEconomy_city"))) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelEconomy_city"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelEconomy_highway"))) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelEconomy_highway"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelInPort"))) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelInPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1"))) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelTank"))) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelTank"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::wheel"))) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::wheel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::mpg"))) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::mpg"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::cityScenario"))) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::cityScenario"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::highwayScenario"))) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::highwayScenario"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city"))) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::cityFuelEconomyRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway"))) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized"))) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized"))) (kind subsetting) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 1 19) (end 1 34)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 2 19) (end 2 25)) (probe (position 2 19))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQ")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 3 19) (end 3 38)) (probe (position 3 19))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "USCustomaryUnits")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 5 19) (end 5 39)) (probe (position 5 19))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0) (authored-target "VehicleQuantities")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleQuantities")))))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 6 19) (end 6 34)) (probe (position 6 19))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind import) (ordinal 4))))) (kind namespaceImport) (ordinal 0) (authored-target "VehicleModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel")))))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 7 19) (end 7 50)) (probe (position 7 19))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind import) (ordinal 5))))) (kind namespaceImport) (ordinal 0) (authored-target "FuelEconomyRequirementsModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel")))))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 8 19) (end 8 35)) (probe (position 8 19))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind import) (ordinal 6))))) (kind namespaceImport) (ordinal 0) (authored-target "DynamicsModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::DynamicsModel")))))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 9 19) (end 9 46)) (probe (position 9 19))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind import) (ordinal 7))))) (kind namespaceImport) (ordinal 0) (authored-target "FuelEconomyAnalysisModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel")))))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 153 20) (end 153 43)) (probe (position 153 20))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::size")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 154 20) (end 154 53)) (probe (position 154 20))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "SampledFunctions::SampledFunction")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 155 20) (end 155 48)) (probe (position 155 20))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "SampledFunctions::SamplePair")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 156 20) (end 156 44)) (probe (position 156 20))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::forAll")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 174 27) (end 174 42)) (probe (position 174 27))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::scenario"))) (kind featureTyping) (ordinal 0) (authored-target "NominalScenario")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario")))))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 163 35) (end 163 50)) (probe (position 163 35))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario"))) (kind specialization) (ordinal 0) (authored-target "SampledFunction")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 168 17) (end 168 32)) (probe (position 168 17))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "TimeStateRecord")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::TimeStateRecord")))))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 168 7) (end 168 14)) (probe (position 168 7))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "samples")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 164 36) (end 164 46)) (probe (position 164 36))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::TimeStateRecord"))) (kind specialization) (ordinal 0) (authored-target "SamplePair")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 169 7) (end 169 14)) (probe (position 169 7))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::n"))) (kind featureTyping) (ordinal 0) (authored-target "Natural")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 159 14) (end 159 25)) (probe (position 159 14))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::ScenarioState::position"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 160 14) (end 160 24)) (probe (position 160 14))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::ScenarioState::velocity"))) (kind featureTyping) (ordinal 0) (authored-target "SpeedValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 84 39) (end 84 61)) (probe (position 84 39))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement::actualFuelEconomy"))) (kind featureTyping) (ordinal 0) (authored-target "DistancePerVolumeValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue")))))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 85 41) (end 85 63)) (probe (position 85 41))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement::requiredFuelEconomy"))) (kind featureTyping) (ordinal 0) (authored-target "DistancePerVolumeValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue")))))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 90 46) (end 90 68)) (probe (position 90 46))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::cityFuelEconomyRequirement"))) (kind featureTyping) (ordinal 0) (authored-target "FuelEconomyRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement")))))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 91 13) (end 91 32)) (probe (position 91 13))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "requiredFuelEconomy")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 94 49) (end 94 71)) (probe (position 94 49))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement"))) (kind featureTyping) (ordinal 0) (authored-target "FuelEconomyRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement")))))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 95 13) (end 95 32)) (probe (position 95 13))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "requiredFuelEconomy")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 42 28) (end 42 36)) (probe (position 42 28))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelInPort"))) (kind featureTyping) (ordinal 0) (authored-target "FuelPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort")))))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 40 31) (end 40 35)) (probe (position 40 31))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelLevel"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 43 28) (end 43 36)) (probe (position 43 28))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelOutPort"))) (kind featureTyping) (ordinal 0) (authored-target "FuelPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort")))))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 39 32) (end 39 43)) (probe (position 39 32))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelVolume"))) (kind featureTyping) (ordinal 0) (authored-target "VolumeValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 38 31) (end 38 42)) (probe (position 38 31))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::volumeMax"))) (kind featureTyping) (ordinal 0) (authored-target "VolumeValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 52 31) (end 52 40)) (probe (position 52 31))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::cargoMass"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 55 42) (end 55 46)) (probe (position 55 42))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::driveTrainEfficiency"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 57 38) (end 57 60)) (probe (position 57 38))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelEconomy_city"))) (kind featureTyping) (ordinal 0) (authored-target "DistancePerVolumeValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue")))))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 58 41) (end 58 63)) (probe (position 58 41))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelEconomy_highway"))) (kind featureTyping) (ordinal 0) (authored-target "DistancePerVolumeValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue")))))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 60 28) (end 60 36)) (probe (position 60 28))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelInPort"))) (kind featureTyping) (ordinal 0) (authored-target "FuelPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort")))))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 51 26) (end 51 35)) (probe (position 51 26))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::mass"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 54 35) (end 54 46)) (probe (position 54 35))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::wheelDiameter"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 47 30) (end 47 41)) (probe (position 47 30))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::Wheel::diameter"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 63 23) (end 63 30)) (probe (position 63 23))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle")))))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 64 18) (end 64 28)) (probe (position 64 18))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "fuelInPort")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 68 25) (end 68 33)) (probe (position 68 25))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelTank"))) (kind featureTyping) (ordinal 0) (authored-target "FuelTank")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank")))))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 69 22) (end 69 32)) (probe (position 69 22))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "fuelInPort")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 76 22) (end 76 27)) (probe (position 76 22))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::wheel"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::Wheel")))))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 77 17) (end 77 25)) (probe (position 77 17))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "diameter")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 12 20) (end 12 33)) (probe (position 12 20))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Quantities")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 13 20) (end 13 44)) (probe (position 13 20))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "MeasurementReferences")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 15 44) (end 15 55)) (probe (position 15 44))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit"))) (kind specialization) (ordinal 0) (authored-target "DerivedUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 18 23) (end 18 40)) (probe (position 18 23))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 18 47) (end 18 67)) (probe (position 18 47))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 16 36) (end 16 55)) (probe (position 16 36))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit::distancePF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 16 65) (end 16 73)) (probe (position 16 65))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 16 87) (end 16 95)) (probe (position 16 87))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 17 37) (end 17 56)) (probe (position 17 37))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit::volumePF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 17 66) (end 17 74)) (probe (position 17 66))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 17 88) (end 17 96)) (probe (position 17 88))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 21 45) (end 21 64)) (probe (position 21 45))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 22 19) (end 22 23)) (probe (position 22 19))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 23 20) (end 23 41)) (probe (position 23 20))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "DistancePerVolumeUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit")))))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 22 13) (end 22 16)) (probe (position 22 13))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 23 13) (end 23 17)) (probe (position 23 13))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 26 24) (end 26 34)) (probe (position 26 24))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::gallon"))) (kind featureTyping) (ordinal 0) (authored-target "VolumeUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 27 21) (end 27 42)) (probe (position 27 21))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::mpg"))) (kind featureTyping) (ordinal 0) (authored-target "DistancePerVolumeUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit")))))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 263 30) (end 263 45)) (probe (position 263 30))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::cityScenario"))) (kind featureTyping) (ordinal 0) (authored-target "NominalScenario")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario")))))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 264 33) (end 264 48)) (probe (position 264 33))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::highwayScenario"))) (kind featureTyping) (ordinal 0) (authored-target "NominalScenario")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario")))))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 241 59) (end 241 85)) (probe (position 241 59))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city"))) (kind subsetting) (ordinal 0) (authored-target "cityFuelEconomyRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::cityFuelEconomyRequirement")))))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 246 17) (end 246 34)) (probe (position 246 17))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "actualFuelEconomy")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 251 62) (end 251 91)) (probe (position 251 62))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway"))) (kind subsetting) (ordinal 0) (authored-target "highwayFuelEconomyRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement")))))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 256 17) (end 256 34)) (probe (position 256 17))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "actualFuelEconomy")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 278 35) (end 278 45)) (probe (position 278 35))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized"))) (kind subsetting) (ordinal 0) (authored-target "vehicle_c1")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_analysis_demo.md") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1")))))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 279 23) (end 279 39)) (probe (position 279 23))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "fuelEconomy_city")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_analysis_demo.md") (range (start 280 23) (end 280 42)) (probe (position 280 23))
    (reference (id (source (node (document "memory://snapshot/vehicle_analysis_demo.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "fuelEconomy_highway")
      (outcome (status unresolved)))
  )
)
~~~
