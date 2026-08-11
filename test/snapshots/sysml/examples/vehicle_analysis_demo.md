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
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "2ef35b034f91ea3dc526b2ef9ac8b53d0b2dcc7c5aac668ed5d15ab39bc2a75d") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo"))) (kind "package") (name "Vehicle Analysis Demo") (declared-name "Vehicle Analysis Demo") (range (start (line 0) (character 0)) (end (line 0) (character 9023))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 4)) (end (line 1) (character 35))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 19)) (end (line 1) (character 31))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 4)) (end (line 2) (character 26))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 19)) (end (line 2) (character 22))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 4)) (end (line 3) (character 39))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo"))) (authored (membership (kind Import) (visibility "private") (import (reference "USCustomaryUnits::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 19)) (end (line 3) (character 35))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::*#import3"))) (kind "import") (name "*") (declared-name "*") (range (start (line 5) (character 4)) (end (line 5) (character 40))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo"))) (authored (membership (kind Import) (visibility "private") (import (reference "VehicleQuantities::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 5) (character 19)) (end (line 5) (character 36))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::*#import4"))) (kind "import") (name "*") (declared-name "*") (range (start (line 6) (character 4)) (end (line 6) (character 35))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo"))) (authored (membership (kind Import) (visibility "private") (import (reference "VehicleModel::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 6) (character 19)) (end (line 6) (character 31))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::*#import5"))) (kind "import") (name "*") (declared-name "*") (range (start (line 7) (character 4)) (end (line 7) (character 51))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo"))) (authored (membership (kind Import) (visibility "private") (import (reference "FuelEconomyRequirementsModel::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 7) (character 19)) (end (line 7) (character 47))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::*#import6"))) (kind "import") (name "*") (declared-name "*") (range (start (line 8) (character 4)) (end (line 8) (character 36))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo"))) (authored (membership (kind Import) (visibility "private") (import (reference "DynamicsModel::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 8) (character 19)) (end (line 8) (character 32))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::*#import7"))) (kind "import") (name "*") (declared-name "*") (range (start (line 9) (character 4)) (end (line 9) (character 47))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo"))) (authored (membership (kind Import) (visibility "private") (import (reference "FuelEconomyAnalysisModel::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 9) (character 19)) (end (line 9) (character 43))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel"))) (kind "package") (name "DynamicsModel") (declared-name "DynamicsModel") (range (start (line 99) (character 1)) (end (line 99) (character 1566))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Acceleration"))) (kind "calc def") (name "Acceleration") (declared-name "Acceleration") (range (start (line 100) (character 5)) (end (line 100) (character 139))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Acceleration::"))) (kind "return parameter") (name "") (range (start (line 101) (character 6)) (end (line 101) (character 47))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Acceleration"))) (authored (relationships (typing (reference "AccelerationValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Acceleration::m"))) (kind "in out parameter") (name "m") (declared-name "m") (range (start (line 100) (character 48)) (end (line 100) (character 65))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Acceleration"))) (authored (relationships (typing (reference "MassValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Acceleration::p"))) (kind "in out parameter") (name "p") (declared-name "p") (range (start (line 100) (character 29)) (end (line 100) (character 47))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Acceleration"))) (authored (relationships (typing (reference "PowerValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Acceleration::v"))) (kind "in out parameter") (name "v") (declared-name "v") (range (start (line 100) (character 66)) (end (line 100) (character 84))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Acceleration"))) (authored (relationships (typing (reference "SpeedValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Position"))) (kind "calc def") (name "Position") (declared-name "Position") (range (start (line 108) (character 5)) (end (line 108) (character 132))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Position::"))) (kind "return parameter") (name "") (range (start (line 109) (character 6)) (end (line 109) (character 41))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Position"))) (authored (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Position::dt"))) (kind "in out parameter") (name "dt") (declared-name "dt") (range (start (line 108) (character 65)) (end (line 108) (character 83))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Position"))) (authored (relationships (typing (reference "TimeValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Position::v"))) (kind "in out parameter") (name "v") (declared-name "v") (range (start (line 108) (character 46)) (end (line 108) (character 64))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Position"))) (authored (relationships (typing (reference "SpeedValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Position::x0"))) (kind "in out parameter") (name "x0") (declared-name "x0") (range (start (line 108) (character 25)) (end (line 108) (character 45))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Position"))) (authored (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics"))) (kind "action def") (name "StraightLineDynamics") (declared-name "StraightLineDynamics") (range (start (line 129) (character 5)) (end (line 129) (character 621))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics::a_out"))) (kind "in out parameter") (name "a_out") (declared-name "a_out") (range (start (line 137) (character 9)) (end (line 137) (character 39))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics"))) (authored (relationships (typing (reference "AccelerationValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics::delta_t"))) (kind "in out parameter") (name "delta_t") (declared-name "delta_t") (range (start (line 132) (character 9)) (end (line 132) (character 32))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics"))) (authored (relationships (typing (reference "TimeValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics::mass"))) (kind "in out parameter") (name "mass") (declared-name "mass") (range (start (line 131) (character 9)) (end (line 131) (character 29))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics"))) (authored (relationships (typing (reference "MassValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics::power"))) (kind "in out parameter") (name "power") (declared-name "power") (range (start (line 130) (character 9)) (end (line 130) (character 31))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics"))) (authored (relationships (typing (reference "PowerValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics::v_in"))) (kind "in out parameter") (name "v_in") (declared-name "v_in") (range (start (line 134) (character 9)) (end (line 134) (character 30))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics"))) (authored (relationships (typing (reference "SpeedValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics::v_out"))) (kind "in out parameter") (name "v_out") (declared-name "v_out") (range (start (line 136) (character 9)) (end (line 136) (character 32))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics"))) (authored (relationships (typing (reference "SpeedValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics::x_in"))) (kind "in out parameter") (name "x_in") (declared-name "x_in") (range (start (line 133) (character 9)) (end (line 133) (character 31))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics"))) (authored (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics::x_out"))) (kind "in out parameter") (name "x_out") (declared-name "x_out") (range (start (line 135) (character 9)) (end (line 135) (character 33))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics"))) (authored (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamicsEquations"))) (kind "constraint def") (name "StraightLineDynamicsEquations") (declared-name "StraightLineDynamicsEquations") (range (start (line 112) (character 5)) (end (line 112) (character 483))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Velocity"))) (kind "calc def") (name "Velocity") (declared-name "Velocity") (range (start (line 104) (character 5)) (end (line 104) (character 134))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Velocity::"))) (kind "return parameter") (name "") (range (start (line 105) (character 6)) (end (line 105) (character 40))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Velocity"))) (authored (relationships (typing (reference "SpeedValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Velocity::a"))) (kind "in out parameter") (name "a") (declared-name "a") (range (start (line 104) (character 45)) (end (line 104) (character 70))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Velocity"))) (authored (relationships (typing (reference "AccelerationValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Velocity::dt"))) (kind "in out parameter") (name "dt") (declared-name "dt") (range (start (line 104) (character 71)) (end (line 104) (character 89))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Velocity"))) (authored (relationships (typing (reference "TimeValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Velocity::v0"))) (kind "in out parameter") (name "v0") (declared-name "v0") (range (start (line 104) (character 25)) (end (line 104) (character 44))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Velocity"))) (authored (relationships (typing (reference "SpeedValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel"))) (kind "package") (name "FuelEconomyAnalysisModel") (declared-name "FuelEconomyAnalysisModel") (range (start (line 152) (character 1)) (end (line 152) (character 2612))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))) (kind "analysis def") (name "FuelEconomyAnalysis") (declared-name "FuelEconomyAnalysis") (range (start (line 172) (character 2)) (end (line 172) (character 2030))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::calculatedFuelEconomy"))) (kind "analysis result") (name "calculatedFuelEconomy") (declared-name "calculatedFuelEconomy") (range (start (line 176) (character 3)) (end (line 176) (character 57))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))) (authored (relationships (typing (reference "DistancePerVolumeValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::dynamicsAnalysis"))) (kind "action") (name "dynamicsAnalysis") (declared-name "dynamicsAnalysis") (range (start (line 196) (character 3)) (end (line 196) (character 849))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::dynamicsAnalysis::acceleration"))) (kind "in out parameter") (name "acceleration") (declared-name "acceleration") (range (start (line 199) (character 4)) (end (line 199) (character 44))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::dynamicsAnalysis"))) (authored (relationships (typing (reference "acceleration : AccelerationValue[*]") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::dynamicsAnalysis::power"))) (kind "in out parameter") (name "power") (declared-name "power") (range (start (line 198) (character 4)) (end (line 198) (character 30))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::dynamicsAnalysis"))) (authored (relationships (typing (reference "power : PowerValue[*]") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::dynamicsAnalysis::sc"))) (kind "in out parameter") (name "sc") (declared-name "sc") (range (start (line 197) (character 4)) (end (line 197) (character 27))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::dynamicsAnalysis"))) (authored (relationships (typing (reference "NominalScenario") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumptionAnalysis"))) (kind "action") (name "fuelConsumptionAnalysis") (declared-name "fuelConsumptionAnalysis") (range (start (line 224) (character 6)) (end (line 224) (character 478))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumptionAnalysis::acceleration"))) (kind "in out parameter") (name "acceleration") (declared-name "acceleration") (range (start (line 226) (character 10)) (end (line 226) (character 81))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumptionAnalysis"))) (authored (relationships (typing (reference "acceleration : AccelerationValue[*] = dynamicsAnalysis.acceleration") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumptionAnalysis::fuelEconomy"))) (kind "in out parameter") (name "fuelEconomy") (declared-name "fuelEconomy") (range (start (line 227) (character 10)) (end (line 227) (character 75))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumptionAnalysis"))) (authored (relationships (typing (reference "DistancePerVolumeValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumptionAnalysis::power"))) (kind "in out parameter") (name "power") (declared-name "power") (range (start (line 225) (character 10)) (end (line 225) (character 60))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumptionAnalysis"))) (authored (relationships (typing (reference "power : PowerValue[*] = dynamicsAnalysis.power") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelEconomyAnalysisObjective"))) (kind "objective") (name "fuelEconomyAnalysisObjective") (declared-name "fuelEconomyAnalysisObjective") (range (start (line 178) (character 3)) (end (line 178) (character 450))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelEconomyRequirement"))) (kind "requirement") (name "fuelEconomyRequirement") (declared-name "fuelEconomyRequirement") (range (start (line 175) (character 6)) (end (line 175) (character 66))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelEconomyRequirement") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::scenario"))) (kind "attribute") (name "scenario") (declared-name "scenario") (range (start (line 174) (character 3)) (end (line 174) (character 43))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))) (authored (membership (kind Feature)) (relationships (typing (reference "NominalScenario") (range none)) (typing (reference "NominalScenario") (range (start (line 174) (character 27)) (end (line 174) (character 42)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (range (start (line 173) (character 3)) (end (line 173) (character 28))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))) (authored (relationships (typing (reference "Vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario"))) (kind "attribute def") (name "NominalScenario") (declared-name "NominalScenario") (range (start (line 163) (character 2)) (end (line 163) (character 249))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel"))) (authored (membership (kind Owning)) (relationships (typing (reference "SampledFunction") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::TimeStateRecord"))) (kind "attribute def") (name "TimeStateRecord") (declared-name "TimeStateRecord") (range (start (line 164) (character 3)) (end (line 164) (character 126))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario"))) (authored (membership (kind Owning)) (relationships (typing (reference "SamplePair") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::n"))) (kind "attribute") (name "n") (declared-name "n") (range (start (line 169) (character 3)) (end (line 169) (character 31))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario"))) (authored (membership (kind Feature)) (relationships (typing (reference "Natural") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::samples"))) (kind "attribute") (name "samples") (declared-name "samples") (range (start (line 168) (character 3)) (end (line 168) (character 33))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario"))) (authored (membership (kind Feature)) (relationships (typing (reference "TimeStateRecord") (range none)) (redefinition (reference "samples") (range (start (line 168) (character 3)) (end (line 168) (character 14)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::SamplePair"))) (kind "import") (name "SamplePair") (declared-name "SamplePair") (range (start (line 155) (character 5)) (end (line 155) (character 49))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "SampledFunctions::SamplePair") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 155) (character 20)) (end (line 155) (character 48))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::SampledFunction"))) (kind "import") (name "SampledFunction") (declared-name "SampledFunction") (range (start (line 154) (character 5)) (end (line 154) (character 54))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "SampledFunctions::SampledFunction") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 154) (character 20)) (end (line 154) (character 53))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::ScenarioState"))) (kind "attribute def") (name "ScenarioState") (declared-name "ScenarioState") (range (start (line 158) (character 2)) (end (line 158) (character 88))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::ScenarioState::position"))) (kind "attribute") (name "position") (declared-name "position") (range (start (line 159) (character 3)) (end (line 159) (character 26))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::ScenarioState"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::ScenarioState::velocity"))) (kind "attribute") (name "velocity") (declared-name "velocity") (range (start (line 160) (character 3)) (end (line 160) (character 25))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::ScenarioState"))) (authored (membership (kind Feature)) (relationships (typing (reference "SpeedValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::forAll"))) (kind "import") (name "forAll") (declared-name "forAll") (range (start (line 156) (character 5)) (end (line 156) (character 45))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::forAll") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 156) (character 20)) (end (line 156) (character 44))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::size"))) (kind "import") (name "size") (declared-name "size") (range (start (line 153) (character 5)) (end (line 153) (character 44))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::size") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 153) (character 20)) (end (line 153) (character 43))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel"))) (kind "package") (name "FuelEconomyRequirementsModel") (declared-name "FuelEconomyRequirementsModel") (range (start (line 82) (character 1)) (end (line 82) (character 551))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement"))) (kind "requirement def") (name "FuelEconomyRequirement") (declared-name "FuelEconomyRequirement") (range (start (line 83) (character 5)) (end (line 83) (character 255))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement::_requireConstraint_0"))) (kind "require constraint") (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (range (start (line 87) (character 9)) (end (line 87) (character 72))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement::actualFuelEconomy"))) (kind "attribute") (name "actualFuelEconomy") (declared-name "actualFuelEconomy") (range (start (line 84) (character 9)) (end (line 84) (character 62))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement"))) (authored (relationships (typing (reference "DistancePerVolumeValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement::requiredFuelEconomy"))) (kind "attribute") (name "requiredFuelEconomy") (declared-name "requiredFuelEconomy") (range (start (line 85) (character 9)) (end (line 85) (character 64))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement"))) (authored (relationships (typing (reference "DistancePerVolumeValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::cityFuelEconomyRequirement"))) (kind "requirement") (name "cityFuelEconomyRequirement") (declared-name "cityFuelEconomyRequirement") (range (start (line 90) (character 5)) (end (line 90) (character 122))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelEconomyRequirement") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::cityFuelEconomyRequirement::requiredFuelEconomy"))) (kind "attribute") (name "requiredFuelEconomy") (declared-name "requiredFuelEconomy") (range (start (line 91) (character 9)) (end (line 91) (character 44))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::cityFuelEconomyRequirement"))) (authored (relationships (redefinition (reference "requiredFuelEconomy") (range (start (line 91) (character 9)) (end (line 91) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement"))) (kind "requirement") (name "highwayFuelEconomyRequirement") (declared-name "highwayFuelEconomyRequirement") (range (start (line 94) (character 5)) (end (line 94) (character 125))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelEconomyRequirement") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement::requiredFuelEconomy"))) (kind "attribute") (name "requiredFuelEconomy") (declared-name "requiredFuelEconomy") (range (start (line 95) (character 9)) (end (line 95) (character 44))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement"))) (authored (relationships (redefinition (reference "requiredFuelEconomy") (range (start (line 95) (character 9)) (end (line 95) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel"))) (kind "package") (name "VehicleModel") (declared-name "VehicleModel") (range (start (line 30) (character 1)) (end (line 30) (character 1274))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Fuel"))) (kind "item def") (name "Fuel") (declared-name "Fuel") (range (start (line 31) (character 5)) (end (line 31) (character 19))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort"))) (kind "port def") (name "FuelPort") (declared-name "FuelPort") (range (start (line 33) (character 5)) (end (line 33) (character 61))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort::fuel"))) (kind "item") (name "fuel") (declared-name "fuel") (range (start (line 34) (character 9)) (end (line 34) (character 29))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort"))) (authored (membership (kind Feature)) (relationships (typing (reference "Fuel") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort::~FuelPort"))) (kind "conjugated port definition") (name "~FuelPort") (declared-name "~FuelPort") (range (start (line 33) (character 5)) (end (line 33) (character 61))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank"))) (kind "part def") (name "FuelTank") (declared-name "FuelTank") (range (start (line 37) (character 5)) (end (line 37) (character 268))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelInPort"))) (kind "port") (name "fuelInPort") (declared-name "fuelInPort") (range (start (line 42) (character 9)) (end (line 42) (character 37))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank"))) (authored (membership (kind Feature)) (relationships (typing (reference "~FuelPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelLevel"))) (kind "attribute") (name "fuelLevel") (declared-name "fuelLevel") (range (start (line 40) (character 9)) (end (line 40) (character 61))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (typing (reference "Real") (range (start (line 40) (character 31)) (end (line 40) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelOutPort"))) (kind "port") (name "fuelOutPort") (declared-name "fuelOutPort") (range (start (line 43) (character 9)) (end (line 43) (character 37))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelVolume"))) (kind "attribute") (name "fuelVolume") (declared-name "fuelVolume") (range (start (line 39) (character 9)) (end (line 39) (character 44))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank"))) (authored (membership (kind Feature)) (relationships (typing (reference "VolumeValue") (range none)) (typing (reference "VolumeValue") (range (start (line 39) (character 32)) (end (line 39) (character 43)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::volumeMax"))) (kind "attribute") (name "volumeMax") (declared-name "volumeMax") (range (start (line 38) (character 9)) (end (line 38) (character 43))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank"))) (authored (membership (kind Feature)) (relationships (typing (reference "VolumeValue") (range none)) (typing (reference "VolumeValue") (range (start (line 38) (character 31)) (end (line 38) (character 42)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 50) (character 5)) (end (line 50) (character 392))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::cargoMass"))) (kind "attribute") (name "cargoMass") (declared-name "cargoMass") (range (start (line 52) (character 9)) (end (line 52) (character 41))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue") (range none)) (typing (reference "MassValue") (range (start (line 52) (character 31)) (end (line 52) (character 40)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::driveTrainEfficiency"))) (kind "attribute") (name "driveTrainEfficiency") (declared-name "driveTrainEfficiency") (range (start (line 55) (character 9)) (end (line 55) (character 47))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (typing (reference "Real") (range (start (line 55) (character 42)) (end (line 55) (character 46)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelEconomy_city"))) (kind "attribute") (name "fuelEconomy_city") (declared-name "fuelEconomy_city") (range (start (line 57) (character 9)) (end (line 57) (character 61))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "DistancePerVolumeValue") (range none)) (typing (reference "DistancePerVolumeValue") (range (start (line 57) (character 38)) (end (line 57) (character 60)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelEconomy_highway"))) (kind "attribute") (name "fuelEconomy_highway") (declared-name "fuelEconomy_highway") (range (start (line 58) (character 9)) (end (line 58) (character 64))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "DistancePerVolumeValue") (range none)) (typing (reference "DistancePerVolumeValue") (range (start (line 58) (character 41)) (end (line 58) (character 63)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelInPort"))) (kind "port") (name "fuelInPort") (declared-name "fuelInPort") (range (start (line 60) (character 9)) (end (line 60) (character 37))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "~FuelPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 51) (character 9)) (end (line 51) (character 36))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue") (range none)) (typing (reference "MassValue") (range (start (line 51) (character 26)) (end (line 51) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::wheelDiameter"))) (kind "attribute") (name "wheelDiameter") (declared-name "wheelDiameter") (range (start (line 54) (character 9)) (end (line 54) (character 47))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)) (typing (reference "LengthValue") (range (start (line 54) (character 35)) (end (line 54) (character 46)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Wheel"))) (kind "part def") (name "Wheel") (declared-name "Wheel") (range (start (line 46) (character 5)) (end (line 46) (character 71))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Wheel::diameter"))) (kind "attribute") (name "diameter") (declared-name "diameter") (range (start (line 47) (character 9)) (end (line 47) (character 42))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Wheel"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)) (typing (reference "LengthValue") (range (start (line 47) (character 30)) (end (line 47) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1"))) (kind "part") (name "vehicle_c1") (declared-name "vehicle_c1") (range (start (line 63) (character 5)) (end (line 63) (character 396))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 63) (character 23)) (end (line 63) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelInPort"))) (kind "port") (name "fuelInPort") (declared-name "fuelInPort") (range (start (line 64) (character 9)) (end (line 64) (character 73))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "fuelInPort") (range (start (line 64) (character 18)) (end (line 64) (character 28)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelTank"))) (kind "part") (name "fuelTank") (declared-name "fuelTank") (range (start (line 68) (character 9)) (end (line 68) (character 131))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelTank") (range (start (line 68) (character 25)) (end (line 68) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelTank::fuelInPort"))) (kind "port") (name "fuelInPort") (declared-name "fuelInPort") (range (start (line 69) (character 13)) (end (line 69) (character 84))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelTank"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "fuelInPort") (range (start (line 69) (character 22)) (end (line 69) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::wheel"))) (kind "part") (name "wheel") (declared-name "wheel") (range (start (line 76) (character 9)) (end (line 76) (character 86))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel") (range (start (line 76) (character 22)) (end (line 76) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::wheel::diameter"))) (kind "attribute") (name "diameter") (declared-name "diameter") (range (start (line 77) (character 13)) (end (line 77) (character 42))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::wheel"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "diameter") (range (start (line 77) (character 13)) (end (line 77) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities"))) (kind "package") (name "VehicleQuantities") (declared-name "VehicleQuantities") (range (start (line 11) (character 1)) (end (line 11) (character 747))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 12) (character 5)) (end (line 12) (character 34))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 12) (character 20)) (end (line 12) (character 30))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 13) (character 5)) (end (line 13) (character 45))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 13) (character 20)) (end (line 13) (character 41))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit"))) (kind "attribute def") (name "DistancePerVolumeUnit") (declared-name "DistancePerVolumeUnit") (range (start (line 15) (character 5)) (end (line 15) (character 368))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit::distancePF"))) (kind "attribute") (name "distancePF") (declared-name "distancePF") (range (start (line 16) (character 6)) (end (line 16) (character 102))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 18) (character 9)) (end (line 18) (character 95))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 18) (character 23)) (end (line 18) (character 40)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit::volumePF"))) (kind "attribute") (name "volumePF") (declared-name "volumePF") (range (start (line 17) (character 9)) (end (line 17) (character 104))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue"))) (kind "attribute def") (name "DistancePerVolumeValue") (declared-name "DistancePerVolumeValue") (range (start (line 21) (character 5)) (end (line 21) (character 141))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 23) (character 9)) (end (line 23) (character 42))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "DistancePerVolumeUnit") (range none)) (redefinition (reference "mRef") (range (start (line 23) (character 9)) (end (line 23) (character 17)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 22) (character 9)) (end (line 22) (character 24))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 22) (character 9)) (end (line 22) (character 16)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::gallon"))) (kind "attribute def") (name "gallon") (declared-name "gallon") (range (start (line 26) (character 5)) (end (line 26) (character 55))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::mpg"))) (kind "attribute def") (name "mpg") (declared-name "mpg") (range (start (line 27) (character 5)) (end (line 27) (character 59))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities"))) (authored (membership (kind Owning)) (relationships (typing (reference "DistancePerVolumeUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext"))) (kind "part") (name "vehicleFuelEconomyAnalysisContext") (declared-name "vehicleFuelEconomyAnalysisContext") (range (start (line 237) (character 1)) (end (line 237) (character 1896))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::cityScenario"))) (kind "attribute") (name "cityScenario") (declared-name "cityScenario") (range (start (line 263) (character 5)) (end (line 263) (character 46))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext"))) (authored (membership (kind Feature)) (relationships (typing (reference "NominalScenario") (range none)) (typing (reference "NominalScenario") (range (start (line 263) (character 30)) (end (line 263) (character 45)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::highwayScenario"))) (kind "attribute") (name "highwayScenario") (declared-name "highwayScenario") (range (start (line 264) (character 5)) (end (line 264) (character 49))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext"))) (authored (membership (kind Feature)) (relationships (typing (reference "NominalScenario") (range none)) (typing (reference "NominalScenario") (range (start (line 264) (character 33)) (end (line 264) (character 48)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized"))) (kind "part") (name "vehicle_c1_analysized") (declared-name "vehicle_c1_analysized") (range (start (line 278) (character 5)) (end (line 278) (character 238))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle_c1") (range (start (line 278) (character 35)) (end (line 278) (character 45)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized::fuelEconomy_city"))) (kind "attribute") (name "fuelEconomy_city") (declared-name "fuelEconomy_city") (range (start (line 279) (character 9)) (end (line 279) (character 88))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "fuelEconomy_city") (range (start (line 279) (character 23)) (end (line 279) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized::fuelEconomy_highway"))) (kind "attribute") (name "fuelEconomy_highway") (declared-name "fuelEconomy_highway") (range (start (line 280) (character 9)) (end (line 280) (character 94))) (parent (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "fuelEconomy_highway") (range (start (line 280) (character 23)) (end (line 280) (character 42)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 1) (character 19)) (end (line 1) (character 31))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (range (start (line 2) (character 19)) (end (line 2) (character 22))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "USCustomaryUnits::*") (range (start (line 3) (character 19)) (end (line 3) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::*#import3"))) (kind namespaceImport) (ordinal 0)) (authored-target "VehicleQuantities::*") (range (start (line 5) (character 19)) (end (line 5) (character 36))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::*#import4"))) (kind namespaceImport) (ordinal 0)) (authored-target "VehicleModel::*") (range (start (line 6) (character 19)) (end (line 6) (character 31))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::*#import5"))) (kind namespaceImport) (ordinal 0)) (authored-target "FuelEconomyRequirementsModel::*") (range (start (line 7) (character 19)) (end (line 7) (character 47))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::*#import6"))) (kind namespaceImport) (ordinal 0)) (authored-target "DynamicsModel::*") (range (start (line 8) (character 19)) (end (line 8) (character 32))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::*#import7"))) (kind namespaceImport) (ordinal 0)) (authored-target "FuelEconomyAnalysisModel::*") (range (start (line 9) (character 19)) (end (line 9) (character 43))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Acceleration::"))) (kind featureTyping) (ordinal 0)) (authored-target "AccelerationValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Acceleration::m"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Acceleration::p"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Acceleration::v"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Position::"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Position::dt"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Position::v"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Position::x0"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics::a_out"))) (kind featureTyping) (ordinal 0)) (authored-target "AccelerationValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics::delta_t"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics::power"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics::v_in"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics::v_out"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics::x_in"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics::x_out"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Velocity::"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Velocity::a"))) (kind featureTyping) (ordinal 0)) (authored-target "AccelerationValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Velocity::dt"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Velocity::v0"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::calculatedFuelEconomy"))) (kind featureTyping) (ordinal 0)) (authored-target "DistancePerVolumeValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::dynamicsAnalysis::acceleration"))) (kind featureTyping) (ordinal 0)) (authored-target "acceleration : AccelerationValue[*]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::dynamicsAnalysis::power"))) (kind featureTyping) (ordinal 0)) (authored-target "power : PowerValue[*]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::dynamicsAnalysis::sc"))) (kind featureTyping) (ordinal 0)) (authored-target "NominalScenario") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumptionAnalysis::acceleration"))) (kind featureTyping) (ordinal 0)) (authored-target "acceleration : AccelerationValue[*] = dynamicsAnalysis.acceleration") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumptionAnalysis::fuelEconomy"))) (kind featureTyping) (ordinal 0)) (authored-target "DistancePerVolumeValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumptionAnalysis::power"))) (kind featureTyping) (ordinal 0)) (authored-target "power : PowerValue[*] = dynamicsAnalysis.power") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelEconomyRequirement"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelEconomyRequirement") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::scenario"))) (kind featureTyping) (ordinal 0)) (authored-target "NominalScenario") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::scenario"))) (kind featureTyping) (ordinal 1)) (authored-target "NominalScenario") (range (start (line 174) (character 27)) (end (line 174) (character 42))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario"))) (kind featureTyping) (ordinal 0)) (authored-target "SampledFunction") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::SampledFunction")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::TimeStateRecord"))) (kind featureTyping) (ordinal 0)) (authored-target "SamplePair") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::SamplePair")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::n"))) (kind featureTyping) (ordinal 0)) (authored-target "Natural") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::samples"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeStateRecord") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::TimeStateRecord")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::samples"))) (kind redefinition) (ordinal 0)) (authored-target "samples") (range (start (line 168) (character 3)) (end (line 168) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::samples")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::SamplePair"))) (kind membershipImport) (ordinal 0)) (authored-target "SampledFunctions::SamplePair") (range (start (line 155) (character 20)) (end (line 155) (character 48))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::SampledFunction"))) (kind membershipImport) (ordinal 0)) (authored-target "SampledFunctions::SampledFunction") (range (start (line 154) (character 20)) (end (line 154) (character 53))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::ScenarioState::position"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::ScenarioState::velocity"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::forAll"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::forAll") (range (start (line 156) (character 20)) (end (line 156) (character 44))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::size"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::size") (range (start (line 153) (character 20)) (end (line 153) (character 43))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement::actualFuelEconomy"))) (kind featureTyping) (ordinal 0)) (authored-target "DistancePerVolumeValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement::requiredFuelEconomy"))) (kind featureTyping) (ordinal 0)) (authored-target "DistancePerVolumeValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::cityFuelEconomyRequirement"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelEconomyRequirement") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::cityFuelEconomyRequirement::requiredFuelEconomy"))) (kind redefinition) (ordinal 0)) (authored-target "requiredFuelEconomy") (range (start (line 91) (character 9)) (end (line 91) (character 32))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::cityFuelEconomyRequirement::requiredFuelEconomy")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelEconomyRequirement") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement::requiredFuelEconomy"))) (kind redefinition) (ordinal 0)) (authored-target "requiredFuelEconomy") (range (start (line 95) (character 9)) (end (line 95) (character 32))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement::requiredFuelEconomy")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort::fuel"))) (kind featureTyping) (ordinal 0)) (authored-target "Fuel") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Fuel")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelInPort"))) (kind featureTyping) (ordinal 0)) (authored-target "~FuelPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelLevel"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelLevel"))) (kind featureTyping) (ordinal 1)) (authored-target "Real") (range (start (line 40) (character 31)) (end (line 40) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelOutPort"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelVolume"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelVolume"))) (kind featureTyping) (ordinal 1)) (authored-target "VolumeValue") (range (start (line 39) (character 32)) (end (line 39) (character 43))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::volumeMax"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::volumeMax"))) (kind featureTyping) (ordinal 1)) (authored-target "VolumeValue") (range (start (line 38) (character 31)) (end (line 38) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::cargoMass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::cargoMass"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (range (start (line 52) (character 31)) (end (line 52) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::driveTrainEfficiency"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::driveTrainEfficiency"))) (kind featureTyping) (ordinal 1)) (authored-target "Real") (range (start (line 55) (character 42)) (end (line 55) (character 46))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelEconomy_city"))) (kind featureTyping) (ordinal 0)) (authored-target "DistancePerVolumeValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelEconomy_city"))) (kind featureTyping) (ordinal 1)) (authored-target "DistancePerVolumeValue") (range (start (line 57) (character 38)) (end (line 57) (character 60))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelEconomy_highway"))) (kind featureTyping) (ordinal 0)) (authored-target "DistancePerVolumeValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelEconomy_highway"))) (kind featureTyping) (ordinal 1)) (authored-target "DistancePerVolumeValue") (range (start (line 58) (character 41)) (end (line 58) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelInPort"))) (kind featureTyping) (ordinal 0)) (authored-target "~FuelPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::mass"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (range (start (line 51) (character 26)) (end (line 51) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::wheelDiameter"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::wheelDiameter"))) (kind featureTyping) (ordinal 1)) (authored-target "LengthValue") (range (start (line 54) (character 35)) (end (line 54) (character 46))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Wheel::diameter"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Wheel::diameter"))) (kind featureTyping) (ordinal 1)) (authored-target "LengthValue") (range (start (line 47) (character 30)) (end (line 47) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 63) (character 23)) (end (line 63) (character 30))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1"))) (kind bindSource) (ordinal 0)) (authored-target "fuelInPort::fuel") (range (start (line 74) (character 14)) (end (line 74) (character 29))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1"))) (kind bindTarget) (ordinal 0)) (authored-target "fuelTank::fuelInPort::fuel") (range (start (line 74) (character 32)) (end (line 74) (character 56))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelInPort"))) (kind redefinition) (ordinal 0)) (authored-target "fuelInPort") (range (start (line 64) (character 18)) (end (line 64) (character 28))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelInPort")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelTank"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelTank") (range (start (line 68) (character 25)) (end (line 68) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelTank::fuelInPort"))) (kind redefinition) (ordinal 0)) (authored-target "fuelInPort") (range (start (line 69) (character 22)) (end (line 69) (character 32))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelTank::fuelInPort")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::wheel"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (range (start (line 76) (character 22)) (end (line 76) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Wheel")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::wheel::diameter"))) (kind redefinition) (ordinal 0)) (authored-target "diameter") (range (start (line 77) (character 13)) (end (line 77) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::wheel::diameter")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Quantities::*") (range (start (line 12) (character 20)) (end (line 12) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "MeasurementReferences::*") (range (start (line 13) (character 20)) (end (line 13) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit::distancePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 18) (character 23)) (end (line 18) (character 40))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit::volumePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "DistancePerVolumeUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 23) (character 9)) (end (line 23) (character 17))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 22) (character 9)) (end (line 22) (character 16))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::gallon"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::mpg"))) (kind featureTyping) (ordinal 0)) (authored-target "DistancePerVolumeUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext"))) (kind satisfySource) (ordinal 0)) (authored-target "vehicleFuelEconomyRequirementsGroup") (range (start (line 283) (character 13)) (end (line 283) (character 48))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext"))) (kind satisfyTarget) (ordinal 0)) (authored-target "vehicle_c1_analysized") (range (start (line 283) (character 52)) (end (line 283) (character 73))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::cityScenario"))) (kind featureTyping) (ordinal 0)) (authored-target "NominalScenario") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::cityScenario"))) (kind featureTyping) (ordinal 1)) (authored-target "NominalScenario") (range (start (line 263) (character 30)) (end (line 263) (character 45))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::highwayScenario"))) (kind featureTyping) (ordinal 0)) (authored-target "NominalScenario") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::highwayScenario"))) (kind featureTyping) (ordinal 1)) (authored-target "NominalScenario") (range (start (line 264) (character 33)) (end (line 264) (character 48))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle_c1") (range (start (line 278) (character 35)) (end (line 278) (character 45))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized::fuelEconomy_city"))) (kind redefinition) (ordinal 0)) (authored-target "fuelEconomy_city") (range (start (line 279) (character 23)) (end (line 279) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized::fuelEconomy_city")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized::fuelEconomy_highway"))) (kind redefinition) (ordinal 0)) (authored-target "fuelEconomy_highway") (range (start (line 280) (character 23)) (end (line 280) (character 42))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized::fuelEconomy_highway")))))
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
