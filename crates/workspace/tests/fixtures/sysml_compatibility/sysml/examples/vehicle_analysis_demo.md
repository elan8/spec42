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
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
ColonGtGt,Ident,Colon,Ident,Semicolon,
ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,Eq,DecimalValue,Dot,DecimalValue,Star,UnrestrictedName,StarStar,DecimalValue,Semicolon,
KwAttribute,Ident,Colon,Ident,Eq,UnrestrictedName,Slash,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwItem,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,OpenCurly,
KwOut,KwItem,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwPort,Ident,Colon,Tilde,Ident,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwPort,Ident,Colon,Tilde,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPort,ColonGtGt,Ident,OpenCurly,
KwIn,KwItem,ColonGtGt,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPort,ColonGtGt,Ident,OpenCurly,
KwIn,KwItem,ColonGtGt,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwBind,Ident,Dot,Ident,Eq,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
ColonGtGt,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwRequirement,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwRequire,KwConstraint,OpenCurly,Ident,GtEq,Ident,CloseCurly,
CloseCurly,
KwRequirement,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
KwRequirement,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwCalc,KwDef,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,KwIn,Ident,Colon,Ident,Semicolon,KwIn,Ident,Colon,Ident,Semicolon,
KwReturn,Colon,Ident,Eq,Ident,Slash,OpenParen,Ident,Star,Ident,CloseParen,Semicolon,
CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,KwIn,Ident,Colon,Ident,Semicolon,KwIn,Ident,Colon,Ident,Semicolon,
KwReturn,Colon,Ident,Eq,Ident,Plus,Ident,Star,Ident,Semicolon,
CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,KwIn,Ident,Colon,Ident,Semicolon,KwIn,Ident,Colon,Ident,Semicolon,
KwReturn,Colon,Ident,Eq,Ident,Plus,Ident,Star,Ident,Semicolon,
CloseCurly,
KwConstraint,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Eq,OpenParen,Ident,Plus,Ident,CloseParen,Slash,DecimalValue,Semicolon,
Ident,EqEq,Ident,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Ampersand,
Ident,EqEq,Ident,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Ampersand,
Ident,EqEq,Ident,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,
CloseCurly,
KwAction,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwOut,Ident,Colon,Ident,Semicolon,
KwOut,Ident,Colon,Ident,Semicolon,
KwOut,Ident,Colon,Ident,Semicolon,
KwAssert,KwConstraint,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAttribute,KwDef,Ident,OpenCurly,
Ident,Colon,Ident,Semicolon,
Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
Ident,Colon,Ident,ColonGtGt,Ident,Semicolon,
Ident,Colon,Ident,ColonGtGt,Ident,Semicolon,
CloseCurly,
ColonGtGt,Ident,Colon,Ident,Semicolon,
Ident,Colon,Ident,Eq,Ident,OpenParen,Ident,CloseParen,Semicolon,
CloseCurly,
KwAnalysis,KwDef,Ident,OpenCurly,
KwSubject,Ident,Colon,Ident,Semicolon,
KwIn,KwAttribute,Ident,Colon,Ident,Semicolon,
KwIn,KwRequirement,Ident,Colon,Ident,Semicolon,
KwReturn,Ident,Colon,Ident,Semicolon,
KwObjective,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAssume,KwConstraint,OpenCurly,
Ident,Dot,Ident,EqEq,DecimalValue,OpenSquare,UnrestrictedName,CloseSquare,Ampersand,
Ident,Dot,Ident,EqEq,DecimalValue,Dot,DecimalValue,
CloseCurly,
KwRequire,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwAction,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwOut,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Semicolon,
KwOut,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Semicolon,
RegularComment,
KwAssert,KwConstraint,Ident,OpenCurly,
OpenParen,DecimalValue,DotDot,Ident,Dot,Ident,Minus,DecimalValue,CloseParen,Arrow,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,
KwPrivate,Ident,Colon,Ident,ColonColon,Ident,Eq,
Ident,Dot,Ident,Hash,OpenParen,Ident,CloseParen,Semicolon,
KwPrivate,Ident,Colon,Ident,ColonColon,Ident,Eq,
Ident,Dot,Ident,Hash,OpenParen,Ident,Plus,DecimalValue,CloseParen,Semicolon,
Ident,OpenParen,
Ident,Eq,Ident,Hash,OpenParen,Ident,CloseParen,Comma,
Ident,Eq,Ident,Dot,Ident,Comma,
Ident,Eq,Ident,Dot,Ident,Minus,Ident,Dot,Ident,Comma,
Ident,Eq,Ident,Dot,Ident,Dot,Ident,Comma,
Ident,Eq,Ident,Dot,Ident,Dot,Ident,Comma,
Ident,Eq,Ident,Dot,Ident,Dot,Ident,Comma,
Ident,Eq,Ident,Dot,Ident,Dot,Ident,Comma,
Ident,Eq,Ident,Hash,OpenParen,Ident,CloseParen,
CloseParen,
CloseCurly,
CloseCurly,
CloseCurly,
KwAction,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Eq,Ident,Dot,Ident,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Eq,Ident,Dot,Ident,Semicolon,
KwOut,Ident,Colon,Ident,Eq,Ident,Semicolon,
RegularComment,
CloseCurly,
CloseCurly,
CloseCurly,
KwPart,Ident,OpenCurly,
KwRequirement,Ident,OpenCurly,
KwSubject,Ident,Colon,Ident,Semicolon,
KwRequirement,Ident,ColonGt,Ident,OpenCurly,
KwDoc,RegularComment,
ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwAssume,KwConstraint,OpenCurly,Ident,Dot,Ident,EqEq,DecimalValue,OpenSquare,Ident,CloseSquare,CloseCurly,
CloseCurly,
KwRequirement,Ident,ColonGt,Ident,OpenCurly,
KwDoc,RegularComment,
ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwAssume,KwConstraint,OpenCurly,Ident,Dot,Ident,EqEq,DecimalValue,OpenSquare,Ident,CloseSquare,CloseCurly,
CloseCurly,
CloseCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAnalysis,Ident,Colon,Ident,OpenCurly,
KwSubject,Ident,Eq,Ident,Semicolon,
KwIn,KwAttribute,Ident,Eq,Ident,Semicolon,
KwIn,KwRequirement,Ident,Eq,Ident,Semicolon,
CloseCurly,
KwAnalysis,Ident,Colon,Ident,OpenCurly,
KwSubject,Ident,Eq,Ident,Semicolon,
KwIn,KwAttribute,Ident,Eq,Ident,Semicolon,
KwIn,KwRequirement,Ident,Eq,Ident,Semicolon,
CloseCurly,
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwSatisfy,Ident,KwBy,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Vehicle Analysis Demo''
    (import_decl private 'ScalarValues::*')
    (import_decl private 'ISQ::*')
    (import_decl private 'USCustomaryUnits::*')
    (import_decl private 'VehicleQuantities::*')
    (import_decl private 'VehicleModel::*')
    (import_decl private 'FuelEconomyRequirementsModel::*')
    (import_decl private 'DynamicsModel::*')
    (import_decl private 'FuelEconomyAnalysisModel::*')
    (package_def 'VehicleQuantities'
      (import_decl private 'Quantities::*')
      (import_decl private 'MeasurementReferences::*')
      (attribute_def 'DistancePerVolumeUnit' :> 'DerivedUnit'
        (attribute_usage private 'distancePF' : 'QuantityPowerFactor' multiplicity
          (default_ref_usage :>> 'quantity' value)
          (default_ref_usage :>> 'exponent' value))
        (attribute_usage private 'volumePF' : 'QuantityPowerFactor' multiplicity
          (default_ref_usage :>> 'quantity' value)
          (default_ref_usage :>> 'exponent' value))
        (attribute_usage :>> 'quantityDimension'
          (default_ref_usage :>> 'quantityPowerFactors' value)))
      (attribute_def 'DistancePerVolumeValue' :> 'ScalarQuantityValue'
        (default_ref_usage :>> 'num' : 'Real')
        (default_ref_usage :>> 'mRef' : 'DistancePerVolumeUnit'))
      (attribute_usage 'gallon' : 'VolumeUnit' value)
      (attribute_usage 'mpg' : 'DistancePerVolumeUnit' value))
    (package_def 'VehicleModel'
      (item_def 'Fuel')
      (port_def 'FuelPort'
        (item_usage out 'fuel' : 'Fuel'))
      (part_def 'FuelTank'
        (attribute_usage 'volumeMax' : 'VolumeValue')
        (attribute_usage 'fuelVolume' : 'VolumeValue')
        (attribute_usage 'fuelLevel' : 'Real' value)
        (port_usage 'fuelInPort' : ~'FuelPort')
        (port_usage 'fuelOutPort' : 'FuelPort'))
      (part_def 'Wheel'
        (attribute_usage 'diameter' : 'LengthValue'))
      (part_def 'Vehicle'
        (attribute_usage 'mass' : 'MassValue')
        (attribute_usage 'cargoMass' : 'MassValue')
        (attribute_usage 'wheelDiameter' : 'LengthValue')
        (attribute_usage 'driveTrainEfficiency' : 'Real')
        (attribute_usage 'fuelEconomy_city' : 'DistancePerVolumeValue')
        (attribute_usage 'fuelEconomy_highway' : 'DistancePerVolumeValue')
        (port_usage 'fuelInPort' : ~'FuelPort'))
      (part_usage 'vehicle_c1' : 'Vehicle'
        (port_usage :>> 'fuelInPort'
          (item_usage in :>> 'fuel'))
        (part_usage 'fuelTank' : 'FuelTank'
          (port_usage :>> 'fuelInPort'
            (item_usage in :>> 'fuel')))
        (binding_as_usage
          (connector_end)
          (connector_end))
        (part_usage 'wheel' : 'Wheel' multiplicity
          (default_ref_usage :>> 'diameter' value))))
    (package_def 'FuelEconomyRequirementsModel'
      (requirement_def 'FuelEconomyRequirement'
        (attribute_usage 'actualFuelEconomy' : 'DistancePerVolumeValue')
        (attribute_usage 'requiredFuelEconomy' : 'DistancePerVolumeValue')
        (sysml_decl
          (result_expr_member)))
      (requirement_usage 'cityFuelEconomyRequirement' : 'FuelEconomyRequirement'
        (default_ref_usage :>> 'requiredFuelEconomy' value))
      (requirement_usage 'highwayFuelEconomyRequirement' : 'FuelEconomyRequirement'
        (default_ref_usage :>> 'requiredFuelEconomy' value)))
    (package_def 'DynamicsModel'
      (calc_def 'Acceleration'
        (default_ref_usage in 'p' : 'PowerValue')
        (default_ref_usage in 'm' : 'MassValue')
        (default_ref_usage in 'v' : 'SpeedValue')
        (return_member))
      (calc_def 'Velocity'
        (default_ref_usage in 'v0' : 'SpeedValue')
        (default_ref_usage in 'a' : 'AccelerationValue')
        (default_ref_usage in 'dt' : 'TimeValue')
        (return_member))
      (calc_def 'Position'
        (default_ref_usage in 'x0' : 'LengthValue')
        (default_ref_usage in 'v' : 'SpeedValue')
        (default_ref_usage in 'dt' : 'TimeValue')
        (return_member))
      (constraint_def 'StraightLineDynamicsEquations'
        (default_ref_usage in 'p' : 'PowerValue')
        (default_ref_usage in 'm' : 'MassValue')
        (default_ref_usage in 'dt' : 'TimeValue')
        (default_ref_usage in 'x_i' : 'LengthValue')
        (default_ref_usage in 'v_i' : 'SpeedValue')
        (default_ref_usage in 'x_f' : 'LengthValue')
        (default_ref_usage in 'v_f' : 'SpeedValue')
        (default_ref_usage in 'a' : 'AccelerationValue')
        (attribute_usage 'v_avg' : 'SpeedValue' value)
        (result_expr_member))
      (action_def 'StraightLineDynamics'
        (default_ref_usage in 'power' : 'PowerValue')
        (default_ref_usage in 'mass' : 'MassValue')
        (default_ref_usage in 'delta_t' : 'TimeValue')
        (default_ref_usage in 'x_in' : 'LengthValue')
        (default_ref_usage in 'v_in' : 'SpeedValue')
        (default_ref_usage out 'x_out' : 'LengthValue')
        (default_ref_usage out 'v_out' : 'SpeedValue')
        (default_ref_usage out 'a_out' : 'AccelerationValue')
        (sysml_decl 'dynamics' : 'StraightLineDynamicsEquations'
          (default_ref_usage in 'p' value)
          (default_ref_usage in 'm' value)
          (default_ref_usage in 'dt' value)
          (default_ref_usage in 'x_i' value)
          (default_ref_usage in 'v_i' value)
          (default_ref_usage in 'x_f' value)
          (default_ref_usage in 'v_f' value)
          (default_ref_usage in 'a' value))))
    (package_def 'FuelEconomyAnalysisModel'
      (import_decl private 'SequenceFunctions::size')
      (import_decl private 'SampledFunctions::SampledFunction')
      (import_decl private 'SampledFunctions::SamplePair')
      (import_decl private 'ControlFunctions::forAll')
      (attribute_def 'ScenarioState'
        (default_ref_usage 'position' : 'LengthValue')
        (default_ref_usage 'velocity' : 'SpeedValue'))
      (attribute_def 'NominalScenario' :> 'SampledFunction'
        (attribute_def 'TimeStateRecord' :> 'SamplePair'
          (default_ref_usage 't' : 'TimeValue' :>> 'domainValue')
          (default_ref_usage 's' : 'ScenarioState' :>> 'rangeValue'))
        (default_ref_usage :>> 'samples' : 'TimeStateRecord')
        (default_ref_usage 'n' : 'Natural' value))
      (analysis_case_def 'FuelEconomyAnalysis'
        (sysml_decl 'vehicle' : 'Vehicle')
        (attribute_usage in 'scenario' : 'NominalScenario')
        (requirement_usage in 'fuelEconomyRequirement' : 'FuelEconomyRequirement')
        (return_member)
        (objective_member)
        (action_usage 'dynamicsAnalysis'
          (default_ref_usage in 'sc' : 'NominalScenario')
          (default_ref_usage out 'power' : 'PowerValue' multiplicity)
          (default_ref_usage out 'acceleration' : 'AccelerationValue' multiplicity)
          (comment)
          (sysml_decl 'straightLineDynamics'
            (result_expr_member)))
        (action_usage 'fuelConsumptionAnalysis'
          (default_ref_usage in 'power' : 'PowerValue' multiplicity value)
          (default_ref_usage in 'acceleration' : 'AccelerationValue' multiplicity value)
          (default_ref_usage out 'fuelEconomy' : 'DistancePerVolumeValue' value)
          (comment))))
    (part_usage 'vehicleFuelEconomyAnalysisContext'
      (requirement_usage 'vehicleFuelEconomyRequirementsGroup'
        (sysml_decl 'vehicle' : 'Vehicle')
        (requirement_usage 'vehicleFuelEconomyRequirement_city' :> 'cityFuelEconomyRequirement'
          (documentation)
          (default_ref_usage :>> 'actualFuelEconomy' value)
          (sysml_decl
            (result_expr_member)))
        (requirement_usage 'vehicleFuelEconomyRequirement_highway' :> 'highwayFuelEconomyRequirement'
          (documentation)
          (default_ref_usage :>> 'actualFuelEconomy' value)
          (sysml_decl
            (result_expr_member))))
      (attribute_usage 'cityScenario' : 'NominalScenario')
      (attribute_usage 'highwayScenario' : 'NominalScenario')
      (sysml_decl 'cityFuelEconomyAnalysis' : 'FuelEconomyAnalysis'
        (sysml_decl 'vehicle' value)
        (attribute_usage in 'scenario' value)
        (requirement_usage in 'fuelEconomyRequirement' value))
      (sysml_decl 'highwayFuelEconomyAnalysis' : 'FuelEconomyAnalysis'
        (sysml_decl 'vehicle' value)
        (attribute_usage in 'scenario' value)
        (requirement_usage in 'fuelEconomyRequirement' value))
      (part_usage 'vehicle_c1_analysized' :> 'vehicle_c1'
        (attribute_usage :>> 'fuelEconomy_city' value)
        (attribute_usage :>> 'fuelEconomy_highway' value))
      (sysml_decl 'vehicleFuelEconomyRequirementsGroup'))))
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
            private attribute distancePF : QuantityPowerFactor [1] {
                :>> quantity = isq.L;
                :>> exponent = 1;
            }
            private attribute volumePF : QuantityPowerFactor [1] {
                :>> quantity = isq.L;
                :>> exponent = -3;
            }
            attribute :>> quantityDimension {
                :>> quantityPowerFactors = (distancePF, volumePF);
            }
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
            out item fuel : Fuel;
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

            part wheel : Wheel [4] {
                :>> diameter = wheelDiameter;
            }
        }
    }

    package FuelEconomyRequirementsModel {
        requirement def FuelEconomyRequirement {
            attribute actualFuelEconomy : DistancePerVolumeValue;
            attribute requiredFuelEconomy : DistancePerVolumeValue;

            require constraint {
                = actualFuelEconomy >= requiredFuelEconomy;
            }
        }

        requirement cityFuelEconomyRequirement : FuelEconomyRequirement {
            :>> requiredFuelEconomy = 25 [mpg];
        }

        requirement highwayFuelEconomyRequirement : FuelEconomyRequirement {
            :>> requiredFuelEconomy = 30 [mpg];
        }
    }

    package DynamicsModel {
        calc def Acceleration {
            in p : PowerValue;
            in m : MassValue;
            in v : SpeedValue;
            return : AccelerationValue = p / (m * v);
        }

        calc def Velocity {
            in v0 : SpeedValue;
            in a : AccelerationValue;
            in dt : TimeValue;
            return : SpeedValue = v0 + a * dt;
        }

        calc def Position {
            in x0 : LengthValue;
            in v : SpeedValue;
            in dt : TimeValue;
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

            = a == Acceleration(p, m, v_avg) & v_f == Velocity(v_i, a, dt) & x_f == Position(x_i, v_avg, dt);
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
            subject vehicle : Vehicle;
            in attribute scenario : NominalScenario;
            in requirement fuelEconomyRequirement : FuelEconomyRequirement;
            return calculatedFuelEconomy : DistancePerVolumeValue;

            objective fuelEconomyAnalysisObjective {
                doc /*
				 * The objective of this analysis is to determine whether the
				 * current vehicle design configuration can satisfy the fuel
				 * economy requirement.
				 */

                assume constraint {
                    = vehicle.wheelDiameter == 33['in'] & vehicle.driveTrainEfficiency == 0.4;
                }

                require constraint fuelEconomyRequirement {
                    :>> actualFuelEconomy = calculatedFuelEconomy;
                }
            }

            action dynamicsAnalysis {
                in sc : NominalScenario;
                out power : PowerValue [*];
                out acceleration : AccelerationValue [*];
                /*
				 * Solve for the required engine power as a function of time
				 * to support the scenarios.
				 */
                assert constraint straightLineDynamics {
                    = (1 .. sc.n - 1)->forAll {in i: Integer;
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
					};
                }
            }

            action fuelConsumptionAnalysis {
                in power : PowerValue [*] = dynamicsAnalysis.power;
                in acceleration : AccelerationValue [*] = dynamicsAnalysis.acceleration;
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

                assume constraint {
                    = vehicle.cargoMass == 1000[lb];
                }
            }

            requirement vehicleFuelEconomyRequirement_highway :> highwayFuelEconomyRequirement {
                doc /* The vehicle shall provide a fuel economy that is greater than or equal to
	             * 30 miles per gallon for the nominal highway driving scenarios.
	             */

                :>> actualFuelEconomy = vehicle.fuelEconomy_highway;

                assume constraint {
                    = vehicle.cargoMass == 1000[lb];
                }
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
# EXPECTED
~~~
semantic.duplicate_name 'vehicleFuelEconomyRequirementsGroup'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'VolumeValue'
semantic.unresolved_name 'VolumeValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'SampledFunction'
semantic.unresolved_name 'SamplePair'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'domainValue'
semantic.unresolved_name 'rangeValue'
semantic.unresolved_name 'samples'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'actualFuelEconomy'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'AccelerationValue'
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'vehicleFuelEconomyRequirementsGroup'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'VolumeValue'
semantic.unresolved_name 'VolumeValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'SampledFunction'
semantic.unresolved_name 'SamplePair'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'domainValue'
semantic.unresolved_name 'rangeValue'
semantic.unresolved_name 'samples'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'actualFuelEconomy'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'AccelerationValue'
~~~
# SMG
~~~
(model
  (namespace
    (package 'Vehicle Analysis Demo'
      (namespace_import private -> 'ScalarValues'[unresolved])
      (namespace_import private -> 'ISQ'[unresolved])
      (namespace_import private -> 'USCustomaryUnits'[unresolved])
      (namespace_import private -> 'Vehicle Analysis Demo::VehicleQuantities'[package])
      (namespace_import private -> 'Vehicle Analysis Demo::VehicleModel'[package])
      (namespace_import private -> 'Vehicle Analysis Demo::FuelEconomyRequirementsModel'[package])
      (namespace_import private -> 'Vehicle Analysis Demo::DynamicsModel'[package])
      (namespace_import private -> 'Vehicle Analysis Demo::FuelEconomyAnalysisModel'[package])
      (package 'VehicleQuantities'
        (namespace_import private -> 'Quantities'[unresolved])
        (namespace_import private -> 'MeasurementReferences'[unresolved])
        (attribute_def 'DistancePerVolumeUnit' :> 'DerivedUnit'[unresolved]
          (attribute_usage composite 'distancePF' : 'QuantityPowerFactor'[unresolved]
            (multiplicity_range [1])
            (reference_usage reference :>> 'quantity'[unresolved]
              (feature_value (=)))
            (reference_usage reference :>> 'exponent'[unresolved]
              (feature_value (=))))
          (attribute_usage composite 'volumePF' : 'QuantityPowerFactor'[unresolved]
            (multiplicity_range [1])
            (reference_usage reference :>> 'quantity'[unresolved]
              (feature_value (=)))
            (reference_usage reference :>> 'exponent'[unresolved]
              (feature_value (=))))
          (attribute_usage composite :>> 'quantityDimension'[unresolved]
            (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
              (feature_value (=)))))
        (attribute_def 'DistancePerVolumeValue' :> 'ScalarQuantityValue'[unresolved]
          (reference_usage reference :>> 'num'[unresolved] : 'Real'[unresolved])
          (reference_usage reference :>> 'mRef'[unresolved] : 'Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit'[attribute_def]))
        (attribute_usage 'gallon' : 'VolumeUnit'[unresolved]
          (feature_value (=)))
        (attribute_usage 'mpg' : 'Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit'[attribute_def]
          (feature_value (=))))
      (package 'VehicleModel'
        (item_def 'Fuel')
        (port_def 'FuelPort'
          (item_usage out 'fuel' : 'Vehicle Analysis Demo::VehicleModel::Fuel'[item_def]))
        (part_def 'FuelTank'
          (attribute_usage composite 'volumeMax' : 'VolumeValue'[unresolved])
          (attribute_usage composite 'fuelVolume' : 'VolumeValue'[unresolved])
          (attribute_usage composite 'fuelLevel' : 'Real'[unresolved]
            (feature_value (=)))
          (port_usage composite 'fuelInPort' : 'Vehicle Analysis Demo::VehicleModel::FuelPort'[port_def] ~ 'Vehicle Analysis Demo::VehicleModel::FuelPort'[port_def])
          (port_usage composite 'fuelOutPort' : 'Vehicle Analysis Demo::VehicleModel::FuelPort'[port_def]))
        (part_def 'Wheel'
          (attribute_usage composite 'diameter' : 'LengthValue'[unresolved]))
        (part_def 'Vehicle'
          (attribute_usage composite 'mass' : 'MassValue'[unresolved])
          (attribute_usage composite 'cargoMass' : 'MassValue'[unresolved])
          (attribute_usage composite 'wheelDiameter' : 'LengthValue'[unresolved])
          (attribute_usage composite 'driveTrainEfficiency' : 'Real'[unresolved])
          (attribute_usage composite 'fuelEconomy_city' : 'Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue'[attribute_def])
          (attribute_usage composite 'fuelEconomy_highway' : 'Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue'[attribute_def])
          (port_usage composite 'fuelInPort' : 'Vehicle Analysis Demo::VehicleModel::FuelPort'[port_def] ~ 'Vehicle Analysis Demo::VehicleModel::FuelPort'[port_def]))
        (part_usage 'vehicle_c1' : 'Vehicle Analysis Demo::VehicleModel::Vehicle'[part_def]
          (port_usage composite :>> 'Vehicle Analysis Demo::VehicleModel::Vehicle::fuelInPort'[port_usage]
            (item_usage in :>> 'Vehicle Analysis Demo::VehicleModel::FuelPort::fuel'[item_usage]))
          (part_usage composite 'fuelTank' : 'Vehicle Analysis Demo::VehicleModel::FuelTank'[part_def]
            (port_usage composite :>> 'Vehicle Analysis Demo::VehicleModel::FuelTank::fuelInPort'[port_usage]
              (item_usage in :>> 'Vehicle Analysis Demo::VehicleModel::FuelPort::fuel'[item_usage])))
          (binding_connector_def
            (connector_end 'fuelInPort.fuel')
            (connector_end 'fuelTank.fuelInPort.fuel'))
          (part_usage composite 'wheel' : 'Vehicle Analysis Demo::VehicleModel::Wheel'[part_def]
            (multiplicity_range [4])
            (reference_usage reference :>> 'Vehicle Analysis Demo::VehicleModel::Wheel::diameter'[attribute_usage]
              (feature_value (=))))))
      (package 'FuelEconomyRequirementsModel'
        (requirement_def 'FuelEconomyRequirement'
          (attribute_usage composite 'actualFuelEconomy' : 'Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue'[attribute_def])
          (attribute_usage composite 'requiredFuelEconomy' : 'Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue'[attribute_def])
          (require_constraint_usage composite
            (result_expr_membership)))
        (requirement_usage 'cityFuelEconomyRequirement' : 'Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement'[requirement_def]
          (reference_usage reference :>> 'Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement::requiredFuelEconomy'[attribute_usage]
            (feature_value (=))))
        (requirement_usage 'highwayFuelEconomyRequirement' : 'Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement'[requirement_def]
          (reference_usage reference :>> 'Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement::requiredFuelEconomy'[attribute_usage]
            (feature_value (=)))))
      (package 'DynamicsModel'
        (calculation_def 'Acceleration'
          (reference_usage in reference 'p' : 'PowerValue'[unresolved])
          (reference_usage in reference 'm' : 'MassValue'[unresolved])
          (reference_usage in reference 'v' : 'SpeedValue'[unresolved])
          (return_parameter_membership
            (feature_def out : 'AccelerationValue'[unresolved]
              (feature_value (=)))))
        (calculation_def 'Velocity'
          (reference_usage in reference 'v0' : 'SpeedValue'[unresolved])
          (reference_usage in reference 'a' : 'AccelerationValue'[unresolved])
          (reference_usage in reference 'dt' : 'TimeValue'[unresolved])
          (return_parameter_membership
            (feature_def out : 'SpeedValue'[unresolved]
              (feature_value (=)))))
        (calculation_def 'Position'
          (reference_usage in reference 'x0' : 'LengthValue'[unresolved])
          (reference_usage in reference 'v' : 'SpeedValue'[unresolved])
          (reference_usage in reference 'dt' : 'TimeValue'[unresolved])
          (return_parameter_membership
            (feature_def out : 'LengthValue'[unresolved]
              (feature_value (=)))))
        (constraint_def 'StraightLineDynamicsEquations'
          (reference_usage in reference 'p' : 'PowerValue'[unresolved])
          (reference_usage in reference 'm' : 'MassValue'[unresolved])
          (reference_usage in reference 'dt' : 'TimeValue'[unresolved])
          (reference_usage in reference 'x_i' : 'LengthValue'[unresolved])
          (reference_usage in reference 'v_i' : 'SpeedValue'[unresolved])
          (reference_usage in reference 'x_f' : 'LengthValue'[unresolved])
          (reference_usage in reference 'v_f' : 'SpeedValue'[unresolved])
          (reference_usage in reference 'a' : 'AccelerationValue'[unresolved])
          (attribute_usage composite 'v_avg' : 'SpeedValue'[unresolved]
            (feature_value (=)))
          (result_expr_membership))
        (action_def 'StraightLineDynamics'
          (reference_usage in reference 'power' : 'PowerValue'[unresolved])
          (reference_usage in reference 'mass' : 'MassValue'[unresolved])
          (reference_usage in reference 'delta_t' : 'TimeValue'[unresolved])
          (reference_usage in reference 'x_in' : 'LengthValue'[unresolved])
          (reference_usage in reference 'v_in' : 'SpeedValue'[unresolved])
          (reference_usage out reference 'x_out' : 'LengthValue'[unresolved])
          (reference_usage out reference 'v_out' : 'SpeedValue'[unresolved])
          (reference_usage out reference 'a_out' : 'AccelerationValue'[unresolved])
          (assert_constraint_usage 'dynamics' : 'Vehicle Analysis Demo::DynamicsModel::StraightLineDynamicsEquations'[constraint_def]
            (reference_usage in reference 'p'
              (feature_value (=)))
            (reference_usage in reference 'm'
              (feature_value (=)))
            (reference_usage in reference 'dt'
              (feature_value (=)))
            (reference_usage in reference 'x_i'
              (feature_value (=)))
            (reference_usage in reference 'v_i'
              (feature_value (=)))
            (reference_usage in reference 'x_f'
              (feature_value (=)))
            (reference_usage in reference 'v_f'
              (feature_value (=)))
            (reference_usage in reference 'a'
              (feature_value (=))))))
      (package 'FuelEconomyAnalysisModel'
        (membership_import private -> 'SequenceFunctions::size'[unresolved])
        (membership_import private -> 'SampledFunctions::SampledFunction'[unresolved])
        (membership_import private -> 'SampledFunctions::SamplePair'[unresolved])
        (membership_import private -> 'ControlFunctions::forAll'[unresolved])
        (attribute_def 'ScenarioState'
          (reference_usage reference 'position' : 'LengthValue'[unresolved])
          (reference_usage reference 'velocity' : 'SpeedValue'[unresolved]))
        (attribute_def 'NominalScenario' :> 'SampledFunction'[unresolved]
          (attribute_def 'TimeStateRecord' :> 'SamplePair'[unresolved]
            (reference_usage reference 't' : 'TimeValue'[unresolved] :>> 'domainValue'[unresolved])
            (reference_usage reference 's' : 'Vehicle Analysis Demo::FuelEconomyAnalysisModel::ScenarioState'[attribute_def] :>> 'rangeValue'[unresolved]))
          (reference_usage reference :>> 'samples'[unresolved] : 'Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::TimeStateRecord'[attribute_def])
          (reference_usage reference 'n' : 'Natural'[unresolved]
            (feature_value (=))))
        (analysis_case_def 'FuelEconomyAnalysis'
          (subject_membership in 'vehicle' : 'Vehicle Analysis Demo::VehicleModel::Vehicle'[part_def])
          (attribute_usage in 'scenario' : 'Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario'[attribute_def])
          (requirement_usage in 'fuelEconomyRequirement' : 'Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement'[requirement_def])
          (return_parameter_membership
            (feature_def out 'calculatedFuelEconomy' : 'Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue'[attribute_def]))
          (objective_membership composite 'fuelEconomyAnalysisObjective'
            (documentation)
            (assume_constraint_usage composite
              (result_expr_membership))
            (require_constraint_usage composite 'fuelEconomyRequirement'
              (reference_usage reference :>> 'actualFuelEconomy'[unresolved]
                (feature_value (=)))))
          (action_usage composite 'dynamicsAnalysis'
            (reference_usage in reference 'sc' : 'Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario'[attribute_def])
            (reference_usage out reference 'power' : 'PowerValue'[unresolved]
              (multiplicity_range [*]))
            (reference_usage out reference 'acceleration' : 'AccelerationValue'[unresolved]
              (multiplicity_range [*]))
            (assert_constraint_usage 'straightLineDynamics'
              (result_expr_membership)))
          (action_usage composite 'fuelConsumptionAnalysis'
            (reference_usage in reference 'power' : 'PowerValue'[unresolved]
              (multiplicity_range [*])
              (feature_value (=)))
            (reference_usage in reference 'acceleration' : 'AccelerationValue'[unresolved]
              (multiplicity_range [*])
              (feature_value (=)))
            (reference_usage out reference 'fuelEconomy' : 'Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue'[attribute_def]
              (feature_value (=))))))
      (part_usage 'vehicleFuelEconomyAnalysisContext'
        (requirement_usage composite 'vehicleFuelEconomyRequirementsGroup'
          (subject_membership in 'vehicle' : 'Vehicle Analysis Demo::VehicleModel::Vehicle'[part_def])
          (requirement_usage composite 'vehicleFuelEconomyRequirement_city' :> 'Vehicle Analysis Demo::FuelEconomyRequirementsModel::cityFuelEconomyRequirement'[requirement_usage]
            (documentation)
            (reference_usage reference :>> 'Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement::actualFuelEconomy'[attribute_usage]
              (feature_value (=)))
            (assume_constraint_usage composite
              (result_expr_membership)))
          (requirement_usage composite 'vehicleFuelEconomyRequirement_highway' :> 'Vehicle Analysis Demo::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement'[requirement_usage]
            (documentation)
            (reference_usage reference :>> 'Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement::actualFuelEconomy'[attribute_usage]
              (feature_value (=)))
            (assume_constraint_usage composite
              (result_expr_membership))))
        (attribute_usage composite 'cityScenario' : 'Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario'[attribute_def])
        (attribute_usage composite 'highwayScenario' : 'Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario'[attribute_def])
        (analysis_case_usage composite 'cityFuelEconomyAnalysis' : 'Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis'[analysis_case_def]
          (subject_membership in 'vehicle'
            (feature_value (=)))
          (attribute_usage in 'scenario'
            (feature_value (=)))
          (requirement_usage in 'fuelEconomyRequirement'
            (feature_value (=))))
        (analysis_case_usage composite 'highwayFuelEconomyAnalysis' : 'Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis'[analysis_case_def]
          (subject_membership in 'vehicle'
            (feature_value (=)))
          (attribute_usage in 'scenario'
            (feature_value (=)))
          (requirement_usage in 'fuelEconomyRequirement'
            (feature_value (=))))
        (part_usage composite 'vehicle_c1_analysized' :> 'Vehicle Analysis Demo::VehicleModel::vehicle_c1'[part_usage]
          (attribute_usage composite :>> 'Vehicle Analysis Demo::VehicleModel::Vehicle::fuelEconomy_city'[attribute_usage]
            (feature_value (=)))
          (attribute_usage composite :>> 'Vehicle Analysis Demo::VehicleModel::Vehicle::fuelEconomy_highway'[attribute_usage]
            (feature_value (=))))
        (satisfy_requirement_usage 'vehicleFuelEconomyRequirementsGroup' by 'Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized'[part_usage])))))
~~~
