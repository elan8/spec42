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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo"))) (name "Vehicle Analysis Demo") (declared-name "Vehicle Analysis Demo")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::*#import2"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::*#import3"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::*#import4"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::*#import5"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::*#import6"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::*#import7"))) (name "*") (declared-name "*"))
        (element (kind "package") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel"))) (name "DynamicsModel") (declared-name "DynamicsModel")
          (contains
            (element (kind "calc def") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Acceleration"))) (name "Acceleration") (declared-name "Acceleration")
              (contains
                (element (kind "return parameter") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Acceleration::"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Acceleration")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Acceleration::m"))) (name "m") (declared-name "m") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Acceleration")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Acceleration::p"))) (name "p") (declared-name "p") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Acceleration")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Acceleration::v"))) (name "v") (declared-name "v") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Acceleration")))))
              )
            )
            (element (kind "calc def") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Position"))) (name "Position") (declared-name "Position")
              (contains
                (element (kind "return parameter") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Position::"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Position")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Position::dt"))) (name "dt") (declared-name "dt") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Position")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Position::v"))) (name "v") (declared-name "v") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Position")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Position::x0"))) (name "x0") (declared-name "x0") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Position")))))
              )
            )
            (element (kind "action def") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics"))) (name "StraightLineDynamics") (declared-name "StraightLineDynamics")
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics::a_out"))) (name "a_out") (declared-name "a_out") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics::delta_t"))) (name "delta_t") (declared-name "delta_t") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics::mass"))) (name "mass") (declared-name "mass") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics::power"))) (name "power") (declared-name "power") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics::v_in"))) (name "v_in") (declared-name "v_in") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics::v_out"))) (name "v_out") (declared-name "v_out") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics::x_in"))) (name "x_in") (declared-name "x_in") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics::x_out"))) (name "x_out") (declared-name "x_out") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics")))))
              )
            )
            (element (kind "constraint def") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamicsEquations"))) (name "StraightLineDynamicsEquations") (declared-name "StraightLineDynamicsEquations") (declared (own-expression (expression (kind "featureReference") (reference "attribute")))) (evaluation (expression (status "unresolved") (error "expression has an unresolved reference"))))
            (element (kind "calc def") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Velocity"))) (name "Velocity") (declared-name "Velocity")
              (contains
                (element (kind "return parameter") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Velocity::"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Velocity")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Velocity::a"))) (name "a") (declared-name "a") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Velocity")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Velocity::dt"))) (name "dt") (declared-name "dt") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Velocity")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Velocity::v0"))) (name "v0") (declared-name "v0") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Velocity")))))
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel"))) (name "FuelEconomyAnalysisModel") (declared-name "FuelEconomyAnalysisModel")
          (contains
            (element (kind "analysis def") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))) (name "FuelEconomyAnalysis") (declared-name "FuelEconomyAnalysis")
              (contains
                (element (kind "analysis result") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::calculatedFuelEconomy"))) (name "calculatedFuelEconomy") (declared-name "calculatedFuelEconomy") (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis")))))
                (element (kind "action") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::dynamicsAnalysis"))) (name "dynamicsAnalysis") (declared-name "dynamicsAnalysis") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))))
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::dynamicsAnalysis::acceleration"))) (name "acceleration") (declared-name "acceleration") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis")))))
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::dynamicsAnalysis::power"))) (name "power") (declared-name "power") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis")))))
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::dynamicsAnalysis::sc"))) (name "sc") (declared-name "sc") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis")))))
                  )
                )
                (element (kind "action") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumptionAnalysis"))) (name "fuelConsumptionAnalysis") (declared-name "fuelConsumptionAnalysis") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))))
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumptionAnalysis::acceleration"))) (name "acceleration") (declared-name "acceleration") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis")))))
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumptionAnalysis::fuelEconomy"))) (name "fuelEconomy") (declared-name "fuelEconomy") (declared (properties (direction "out")) (own-expression (expression (kind "featureReference") (reference "calculatedFuelEconomy")))) (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis")))) (evaluation (expression (status "incomplete") (error "expression is incomplete"))))
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumptionAnalysis::power"))) (name "power") (declared-name "power") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis")))))
                  )
                )
                (element (kind "objective") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelEconomyAnalysisObjective"))) (name "fuelEconomyAnalysisObjective") (declared-name "fuelEconomyAnalysisObjective") (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis")))))
                (element (kind "requirement") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelEconomyRequirement"))) (name "fuelEconomyRequirement") (declared-name "fuelEconomyRequirement") (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis")))) (evaluation (expression (status "incomplete") (error "expression is incomplete")) (analysis (status "incomplete"))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::scenario"))) (name "scenario") (declared-name "scenario") (declared (properties (direction "in") (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis")))))
                (element (kind "subject") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::vehicle"))) (name "vehicle") (declared-name "vehicle") (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis")))))
              )
            )
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario"))) (name "NominalScenario") (declared-name "NominalScenario") (declared (properties (ordered false) (unique true)))
              (contains
                (element (kind "attribute def") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::TimeStateRecord"))) (name "TimeStateRecord") (declared-name "TimeStateRecord") (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::n"))) (name "n") (declared-name "n") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::samples"))) (name "samples") (declared-name "samples") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario")))))
              )
            )
            (element (kind "import") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::SamplePair"))) (name "SamplePair") (declared-name "SamplePair"))
            (element (kind "import") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::SampledFunction"))) (name "SampledFunction") (declared-name "SampledFunction"))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::ScenarioState"))) (name "ScenarioState") (declared-name "ScenarioState") (declared (properties (ordered false) (unique true)))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::ScenarioState::position"))) (name "position") (declared-name "position") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::ScenarioState")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::ScenarioState::velocity"))) (name "velocity") (declared-name "velocity") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::ScenarioState")))))
              )
            )
            (element (kind "import") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::forAll"))) (name "forAll") (declared-name "forAll"))
            (element (kind "import") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::size"))) (name "size") (declared-name "size"))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel"))) (name "FuelEconomyRequirementsModel") (declared-name "FuelEconomyRequirementsModel")
          (contains
            (element (kind "requirement def") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement"))) (name "FuelEconomyRequirement") (declared-name "FuelEconomyRequirement") (evaluation (expression (status "incomplete") (error "expression is incomplete")) (analysis (status "incomplete")))
              (contains
                (element (kind "require constraint") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement::_requireConstraint_0"))) (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (declared (own-expression (expression (kind "binary") (operator ">=") (children (expression (kind "featureReference") (reference "actualFuelEconomy")) (expression (kind "featureReference") (reference "requiredFuelEconomy")))))) (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement")))) (evaluation (expression (status "incomplete") (error "expression is incomplete"))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement::actualFuelEconomy"))) (name "actualFuelEconomy") (declared-name "actualFuelEconomy") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement::requiredFuelEconomy"))) (name "requiredFuelEconomy") (declared-name "requiredFuelEconomy") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement")))))
              )
            )
            (element (kind "requirement") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::cityFuelEconomyRequirement"))) (name "cityFuelEconomyRequirement") (declared-name "cityFuelEconomyRequirement") (evaluation (expression (status "incomplete") (error "expression is incomplete")) (analysis (status "incomplete")))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::cityFuelEconomyRequirement::requiredFuelEconomy"))) (name "requiredFuelEconomy") (declared-name "requiredFuelEconomy") (declared (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal (integer 25))) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "mpg")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::cityFuelEconomyRequirement::requiredFuelEconomy"))) (role feature-value))) (evaluation (expression (status "unsupported") (error "declared expression form is not supported"))))
              )
            )
            (element (kind "requirement") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement"))) (name "highwayFuelEconomyRequirement") (declared-name "highwayFuelEconomyRequirement") (evaluation (expression (status "incomplete") (error "expression is incomplete")) (analysis (status "incomplete")))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement::requiredFuelEconomy"))) (name "requiredFuelEconomy") (declared-name "requiredFuelEconomy") (declared (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal (integer 30))) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "mpg")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement::requiredFuelEconomy"))) (role feature-value))) (evaluation (expression (status "unsupported") (error "declared expression form is not supported"))))
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel"))) (name "VehicleModel") (declared-name "VehicleModel")
          (contains
            (element (kind "item def") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Fuel"))) (name "Fuel") (declared-name "Fuel"))
            (element (kind "port def") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort"))) (name "FuelPort") (declared-name "FuelPort")
              (contains
                (element (kind "item") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort::fuel"))) (name "fuel") (declared-name "fuel") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort")))))
                (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort::~FuelPort"))) (name "~FuelPort") (declared-name "~FuelPort") (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort")))))
              )
            )
            (element (kind "part def") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank"))) (name "FuelTank") (declared-name "FuelTank") (declared)
              (contains
                (element (kind "port") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelInPort"))) (name "fuelInPort") (declared-name "fuelInPort") (declared (properties (conjugated true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelLevel"))) (name "fuelLevel") (declared-name "fuelLevel") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "fuelVolume")) (expression (kind "featureReference") (reference "volumeMax")))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelLevel"))) (role feature-value))) (evaluation (expression (status "incomplete") (error "expression is incomplete"))))
                (element (kind "port") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelOutPort"))) (name "fuelOutPort") (declared-name "fuelOutPort") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelVolume"))) (name "fuelVolume") (declared-name "fuelVolume") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::volumeMax"))) (name "volumeMax") (declared-name "volumeMax") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank")))))
              )
            )
            (element (kind "part def") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared)
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::cargoMass"))) (name "cargoMass") (declared-name "cargoMass") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::driveTrainEfficiency"))) (name "driveTrainEfficiency") (declared-name "driveTrainEfficiency") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelEconomy_city"))) (name "fuelEconomy_city") (declared-name "fuelEconomy_city") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelEconomy_highway"))) (name "fuelEconomy_highway") (declared-name "fuelEconomy_highway") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle")))))
                (element (kind "port") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelInPort"))) (name "fuelInPort") (declared-name "fuelInPort") (declared (properties (conjugated true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::mass"))) (name "mass") (declared-name "mass") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::wheelDiameter"))) (name "wheelDiameter") (declared-name "wheelDiameter") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle")))))
              )
            )
            (element (kind "part def") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Wheel"))) (name "Wheel") (declared-name "Wheel") (declared)
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Wheel::diameter"))) (name "diameter") (declared-name "diameter") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Wheel")))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1"))) (name "vehicle_c1") (declared-name "vehicle_c1") (declared (properties (ordered false)))
              (contains
                (element (kind "port") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelInPort"))) (name "fuelInPort") (declared-name "fuelInPort") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle")))))
                (element (kind "part") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelTank"))) (name "fuelTank") (declared-name "fuelTank") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle"))))
                  (contains
                    (element (kind "port") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelTank::fuelInPort"))) (name "fuelInPort") (declared-name "fuelInPort") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank")))))
                  )
                )
                (element (kind "part") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::wheel"))) (name "wheel") (declared-name "wheel") (declared (properties (ordered false)) (multiplicity (lower 4) (upper 4) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle"))))
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::wheel::diameter"))) (name "diameter") (declared-name "diameter") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "featureReference") (reference "wheelDiameter")))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Wheel"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::wheel::diameter"))) (role feature-value))) (evaluation (expression (status "incomplete") (error "expression is incomplete"))))
                  )
                )
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities"))) (name "VehicleQuantities") (declared-name "VehicleQuantities")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::*"))) (name "*") (declared-name "*"))
            (element (kind "import") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::*#import"))) (name "*") (declared-name "*"))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit"))) (name "DistancePerVolumeUnit") (declared-name "DistancePerVolumeUnit") (declared (properties (ordered false) (unique true)))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit::distancePF"))) (name "distancePF") (declared-name "distancePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit::volumePF"))) (name "volumePF") (declared-name "volumePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit")))))
              )
            )
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue"))) (name "DistancePerVolumeValue") (declared-name "DistancePerVolumeValue") (declared (properties (ordered false) (unique true)))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue")))))
              )
            )
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::gallon"))) (name "gallon") (declared-name "gallon") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "**") (children (expression (kind "binary") (operator "*") (children (expression (kind "realLiteral") (literal (real "231.0"))) (expression (kind "featureReference") (reference "in")))) (expression (kind "integerLiteral") (literal (integer 3))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::gallon"))) (role feature-value))) (evaluation (expression (status "unresolved") (error "expression has an unresolved reference"))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::mpg"))) (name "mpg") (declared-name "mpg") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "mi")) (expression (kind "featureReference") (reference "gallon")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::mpg"))) (role feature-value))) (evaluation (expression (status "unresolved") (error "expression has an unresolved reference"))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext"))) (name "vehicleFuelEconomyAnalysisContext") (declared-name "vehicleFuelEconomyAnalysisContext") (declared (properties (ordered false)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::cityScenario"))) (name "cityScenario") (declared-name "cityScenario") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::highwayScenario"))) (name "highwayScenario") (declared-name "highwayScenario") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
            (element (kind "part") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized"))) (name "vehicle_c1_analysized") (declared-name "vehicle_c1_analysized") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized::fuelEconomy_city"))) (name "fuelEconomy_city") (declared-name "fuelEconomy_city") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "memberAccess") (reference "calculatedFuelEconomy") (children (expression (kind "featureReference") (reference "cityFuelEconomyAnalysis")))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized::fuelEconomy_city"))) (role feature-value))) (evaluation (expression (status "unresolved") (error "expression has an unresolved reference"))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized::fuelEconomy_highway"))) (name "fuelEconomy_highway") (declared-name "fuelEconomy_highway") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "memberAccess") (reference "calculatedFuelEconomy") (children (expression (kind "featureReference") (reference "highwayFuelEconomyAnalysis")))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized::fuelEconomy_highway"))) (role feature-value))) (evaluation (expression (status "unresolved") (error "expression has an unresolved reference"))))
              )
            )
          )
        )
      )
    )
    (element (kind "diagnostic") (id (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::unresolved_satisfy_source"))) (name "unresolved_satisfy_source") (declared-name "unresolved_satisfy_source"))
  )
  (relationships
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort::~FuelPort"))) (to (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort"))) (provenance authored))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::cityFuelEconomyRequirement::requiredFuelEconomy"))) (to (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement::requiredFuelEconomy"))) (provenance authored))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement::requiredFuelEconomy"))) (to (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement::requiredFuelEconomy"))) (provenance authored))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelInPort"))) (to (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelInPort"))) (provenance authored))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelTank::fuelInPort"))) (to (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelInPort"))) (provenance authored))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::wheel::diameter"))) (to (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Wheel::diameter"))) (provenance authored))
    (subject (status resolved) (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))) (to (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::calculatedFuelEconomy"))) (to (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::dynamicsAnalysis::sc"))) (to (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumptionAnalysis::fuelEconomy"))) (to (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelEconomyRequirement"))) (to (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::scenario"))) (to (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::vehicle"))) (to (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::samples"))) (to (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::TimeStateRecord"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement::actualFuelEconomy"))) (to (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement::requiredFuelEconomy"))) (to (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::cityFuelEconomyRequirement"))) (to (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement"))) (to (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort::fuel"))) (to (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Fuel"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelInPort"))) (to (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort::~FuelPort"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelOutPort"))) (to (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelEconomy_city"))) (to (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelEconomy_highway"))) (to (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelInPort"))) (to (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort::~FuelPort"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1"))) (to (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelTank"))) (to (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::wheel"))) (to (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Wheel"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue::mRef"))) (to (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::mpg"))) (to (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::cityScenario"))) (to (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::highwayScenario"))) (to (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
    (bind (status pending-expression) (document "d0") (source-expression "fuelInPort::fuel") (target-expression "fuelTank::fuelInPort::fuel") (container-prefix "Vehicle Analysis Demo::VehicleModel::vehicle_c1"))
    (satisfy (status pending-expression) (document "d0") (source-expression "vehicleFuelEconomyRequirementsGroup") (target-expression "vehicle_c1_analysized") (container-prefix "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext"))
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Acceleration"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Position"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamics"))) (status missing-prerequisite) (target "Actions::Action"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::StraightLineDynamicsEquations"))) (status missing-prerequisite) (target "Constraints::ConstraintCheck"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::DynamicsModel::Velocity"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))) (status missing-prerequisite) (target "AnalysisCases::AnalysisCase"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::dynamicsAnalysis"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumptionAnalysis"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelEconomyAnalysisObjective"))) (status missing-prerequisite) (target "Requirements::requirementChecks"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelEconomyRequirement"))) (status missing-prerequisite) (target "Requirements::requirementChecks"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::FuelEconomyAnalysis::scenario"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::TimeStateRecord"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::n"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::NominalScenario::samples"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::ScenarioState"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::ScenarioState::position"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyAnalysisModel::ScenarioState::velocity"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement"))) (status missing-prerequisite) (target "Requirements::RequirementCheck"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement::_requireConstraint_0"))) (status missing-prerequisite) (target "Constraints::constraintChecks"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement::actualFuelEconomy"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::FuelEconomyRequirement::requiredFuelEconomy"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::cityFuelEconomyRequirement"))) (status missing-prerequisite) (target "Requirements::requirementChecks"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::cityFuelEconomyRequirement::requiredFuelEconomy"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement"))) (status missing-prerequisite) (target "Requirements::requirementChecks"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement::requiredFuelEconomy"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Fuel"))) (status missing-prerequisite) (target "Items::Item"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort"))) (status missing-prerequisite) (target "Ports::Port"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort::fuel"))) (status missing-prerequisite) (target "Items::items"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelPort::~FuelPort"))) (status missing-prerequisite) (target "Ports::Port"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelInPort"))) (status missing-prerequisite) (target "Ports::ports"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelLevel"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelOutPort"))) (status missing-prerequisite) (target "Ports::ports"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::fuelVolume"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::FuelTank::volumeMax"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::cargoMass"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::driveTrainEfficiency"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelEconomy_city"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelEconomy_highway"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::fuelInPort"))) (status missing-prerequisite) (target "Ports::ports"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::mass"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Vehicle::wheelDiameter"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Wheel"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::Wheel::diameter"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelInPort"))) (status missing-prerequisite) (target "Ports::ports"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelTank"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::fuelTank::fuelInPort"))) (status missing-prerequisite) (target "Ports::ports"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::wheel"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleModel::vehicle_c1::wheel::diameter"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit::distancePF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeUnit::volumePF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::DistancePerVolumeValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::gallon"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::VehicleQuantities::mpg"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::cityScenario"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::highwayScenario"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized::fuelEconomy_city"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Analysis Demo::vehicleFuelEconomyAnalysisContext::vehicle_c1_analysized::fuelEconomy_highway"))) (status missing-prerequisite) (target "Base::dataValues"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/vehicle_analysis_demo.md"
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
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 18 9) (end 18 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 5) (end 21 141))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 22 9) (end 22 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 22 9) (end 22 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 23 9) (end 23 42))
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
        (range (start 39 9) (end 39 44))
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
        (range (start 47 9) (end 47 42))
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
        (range (start 52 9) (end 52 41))
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
        (range (start 55 9) (end 55 47))
      )
      (diagnostic
        (severity error)
        (code "unresolved_pending_expression_relationship")
        (source "semantic")
        (range (start 74 14) (end 74 29))
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
        (range (start 163 2) (end 163 249))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 164 3) (end 164 126))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 168 3) (end 168 33))
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
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 279 9) (end 279 88))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 280 9) (end 280 94))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_satisfy_source")
        (source "semantic")
        (range (start 283 13) (end 283 48))
      )
    )
  )
)
~~~
