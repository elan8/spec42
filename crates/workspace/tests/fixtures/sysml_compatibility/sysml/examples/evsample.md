# META
~~~ini
description=SysML Example (State Space Representation): EVSample
type=file
~~~
# SOURCE
~~~sysml
// State Space Representation EV example
package EVSample {
    private import SI::*;
    private import StateSpaceRepresentation::*;

    attribute <'A⋅h'> 'ampere hour'  : ElectricChargeUnit = A*h;

    part def Vehicle {
        attribute mass :> ISQ::mass;

        attribute def VehicleInput :> Input {
            attribute force :> ISQ::force;
        }

        attribute def VehicleOutput :> Output {
            attribute accel :> ISQ::acceleration;
            attribute velocity :> ISQ::speed;
            attribute distance :> ISQ::distance;
        }

        attribute def VehicleState :> StateSpace {
            attribute velocity :> ISQ::speed;
            attribute distance :> ISQ::distance;
        }
    }

    part def Battery {
        attribute baseVoltage :> ISQ::electricPotential;
        attribute socInit: ScalarValues::Real;
        attribute capacity :> ISQ::electricCharge;
        attribute internalResistance :> ISQ::resistance;

        attribute def BatteryInput :> Input {
            attribute current :> ISQ::electricCurrent;
        }

        attribute def BatteryOutput :> Output {
            attribute voltage :> ISQ::electricPotential;
        }

        attribute def BatteryState :> StateSpace {
            attribute soc: ScalarValues::Real;
        }

    }

    part def Motor {
        torquePerCurrent :> Quantities::scalarQuantities = ISQ::torque / ISQ::electricCurrent;

        attribute motR :> ISQ::resistance;
        attribute motL :> ISQ::inductance;

        attribute def MotorInput :> Input {
            attribute voltage :> ISQ::electricPotential;
            attribute friction :> ISQ::torque;
        }

        attribute def MotorOutput :> Output {
            attribute current :> ISQ::electricCurrent;
            attribute torque :> ISQ::torque;
        }

        attribute def MotorState :> StateSpace {
            attribute current :> ISQ::electricCurrent;
        }
    }

    part def Tire {
        attribute radius :> ISQ::length;
        attribute moment :> ISQ::momentOfInertia;

        attribute def TireInput :> Input {
            attribute torque :> ISQ::torque;
            attribute accel :> ISQ::acceleration;
        }

        attribute def TireOutput :> Output {
            attribute force :> ISQ::force;
            attribute outTorque :> ISQ::torque;
        }
    }

    requirement def VehicleRequirement {
        subject vehicle : Vehicle;
    }

    analysis def VehicleAnalysis {
        subject vehicle : Vehicle;
        requirement vehicleRequirement : VehicleRequirement;
    }


    requirement def RangeRequirement :> VehicleRequirement {
        doc /* The range of EV must be longer than the required spec under the flat road. */
        attribute actualRange : LengthValue;
        attribute requiredRange : LengthValue;

        require constraint { actualRange >= requiredRange }
    }

    analysis def RangeAnalysis :> VehicleAnalysis {
    	return simulatedRange : LengthValue;
    	
        requirement rangeRequirement :>> vehicleRequirement : RangeRequirement;

        objective rangeAnalysisObjective {
            doc /* This analysis is to estimate the range of
                 * the EV by simulating the vehicle driving under the compact vehicle regulation.
                 */
            require rangeRequirement {
                :>> actualRange = simulatedRange;
            }
        }
    }

    requirement def EfficiencyRequirement :> VehicleRequirement {
        doc /* The efficiency of EV must be better than the required spec. */
        attribute actualEfficiency;
        attribute requiredEfficiency;

        require constraint { actualEfficiency >= requiredEfficiency }
    }

    analysis def EfficiencyAnalysis :> VehicleAnalysis {
    	return simulatedEfficiency;
    	
        requirement efficiencyRequirement :>> vehicleRequirement : EfficiencyRequirement;

        objective efficiencyAnalysisObjective {
            require efficiencyRequirement {
                attribute :>> actualEfficiency = simulatedEfficiency;
            }
        }
    }

    requirement def MaxSpeedRequirement :> VehicleRequirement {
        doc /* The maximum speed of EV must be larger than the required spec. */
        attribute actualMaxSpeed :> ISQ::speed;
        attribute requiredMaxSpeed :> ISQ::speed;
    }

    analysis def MaxSpeedAnalysis :> VehicleAnalysis {
    	return simulatedMaxSpeed;
    	
        requirement maxSpeedRequirement :>> vehicleRequirement : MaxSpeedRequirement;

        objective maxSpeedAnalysisObjective {
            require maxSpeedRequirement {
                attribute :>> actualMaxSpeed = simulatedMaxSpeed;
            }
        }
    }


    part vehicle : Vehicle {
        attribute :>> mass default 1000[kg];

        /* airFrictionCoefficient [kg / m] = 1/2 * rho[kg/m^3] * Cd * S[m^2],
         * where rho is air density, S is front projected area. */
        attribute airFrictionCoefficient = 0.2;

        attribute efficiency;

        action vehicleBehavior : ContinuousStateSpaceDynamics {
            in input : VehicleInput;
            out output : VehicleOutput;
            :>> stateSpace : VehicleState;
        }
        
        part battery: Battery {
            :>> baseVoltage = 300[V];
            :>> capacity = 50['A⋅h'];
            :>> socInit = 0.8;
            :>> internalResistance = 1.8['Ω'];
            action batteryBehavior : ContinuousStateSpaceDynamics {
                in input : BatteryInput;
                out output : BatteryOutput;
                :>> stateSpace : BatteryState;
            }
        }

        flow battery.batteryBehavior.output to motor.motorBehavior.input;

        part motor: Motor {
            :>> motR = 4['Ω'];
            :>> motL = 0.2[H];

            action motorBehavior : ContinuousStateSpaceDynamics {
                in input : MotorInput;
                out output : MotorOutput;
                :>> stateSpace : MotorState;
            }
        }

        flow motor.motorBehavior.output to tire.tireBehavior.input;

        part tire: Tire {
            :>> moment default 300['kg⋅m²'];
            :>> radius default 0.7[m];
            action tireBehavior : ContinuousStateSpaceDynamics {
                in input : TireInput;
                out output : TireOutput;
            }
        }

        flow tire.tireBehavior.output to motor.motorBehavior.input;
        flow tire.tireBehavior.output to vehicleBehavior.input;
    }

    part vehicle_compact :> vehicle {
        attribute :>> mass = 800[kg];
        part :>> tire {
            :>> moment = 200['kg⋅m²'];
            :>> radius = 0.5[m];
        }
    }

    part smallEVRangeContext {
        requirement smallEVRequirement : VehicleRequirement {
            doc /* The small EVs must be ligher than 900[kg] */
            subject :>> vehicle = vehicle_compact;
            /*  To comform with the regulation and the battery mass will impact it. */
            assume constraint { vehicle.mass < 900[kg] }
        }

        analysis smallEVAnalysis : VehicleAnalysis {
            subject :>> vehicle :> vehicle_compact;
            requirement :>> vehicleRequirement = smallEVRequirement;
        }

        requirement <C1> rangeRequirementSmall :> smallEVRequirement : RangeRequirement {
            doc /* The small EVs must run longer than 130km */
            attribute :>> requiredRange = 130[km];
        }

        analysis rangeAnalysisSmall :> smallEVAnalysis : RangeAnalysis {
            requirement :>> rangeRequirement = rangeRequirementSmall;
            return simulatedRange = vehicle.vehicleBehavior.output.distance;
        }

        requirement <C2> efficiencyRequirementSmall :> smallEVRequirement : EfficiencyRequirement {
            doc /* The target efficiency of small EVs is 0.9. */
            attribute :>> requiredEfficiency = 0.9;
        }

        analysis efficiencyAnalysisSmall :> smallEVAnalysis : EfficiencyAnalysis {
            requirement :>> efficiencyRequirement = efficiencyRequirementSmall;

            return simulatedEfficiency = vehicle.efficiency;
        }

        requirement <C3> maxSpeedRequirementSmall :> smallEVRequirement : MaxSpeedRequirement {
            doc /* The target maximum speed of small EVs is 130 [km/h]. */
            attribute :>> requiredMaxSpeed = 130 [km/h];
        }

        analysis maxSpeedAnalysisSmall :> smallEVAnalysis : MaxSpeedAnalysis {
            subject;
            requirement :>> maxSpeedRequirement = maxSpeedRequirementSmall;
            out voltage :> ISQ::electricPotential = vehicle.battery.batteryBehavior.output.voltage;
            return simulatedMaxSpeed = vehicle.vehicleBehavior.output.velocity;
        }
    }

    part vehicle_large :> vehicle {
        attribute :>> mass = 1100[kg];
        part :>> tire {
            :>> moment = 300['kg⋅m²'];
            :>> radius = 0.7[m];
        }
    }

    part largeEVRangeContext {
        requirement largeEVRequirement : VehicleRequirement {
            doc /* The large EVs must be ligher than 900[kg] */
            subject :>> vehicle = vehicle_large;
            /*  To comform with the regulation and the battery mass will impact it. */
            assume constraint { vehicle.mass < 1200[kg] }
        }

        analysis largeEVAnalysis : VehicleAnalysis {
            subject :>> vehicle :> vehicle_large;
            requirement :>> vehicleRequirement = largeEVRequirement;
        }

        requirement <L1> rangeRequirementLarge :> largeEVRequirement : RangeRequirement {
            doc /* The large EVs must run longer than 200km */
            attribute :>> requiredRange = 200[km];
        }

        analysis rangeAnalysisLarge :> largeEVAnalysis : RangeAnalysis {
            requirement :>> rangeRequirement = rangeRequirementLarge;
            return simulatedRange = vehicle.vehicleBehavior.output.distance;
        }

        requirement <L2> efficiencyRequirementLarge :> largeEVRequirement : EfficiencyRequirement {
            doc /* The target efficiency of large EVs is 0.8. */
            attribute :>> requiredEfficiency = 0.8;
        }

        analysis efficiencyAnalysisLarge :> largeEVAnalysis : EfficiencyAnalysis {
            requirement :>> efficiencyRequirement = efficiencyRequirementLarge;

            return simulatedEfficiency = vehicle.efficiency;
        }

        requirement <L3> maxSpeedRequirementLarge :> largeEVRequirement : MaxSpeedRequirement {
            doc /* The target maximum speed of large EVs is 140 [km/h]. */
            attribute :>> requiredMaxSpeed = 140 [km/h];
        }

        analysis maxSpeedAnalysisLarge :> largeEVAnalysis : MaxSpeedAnalysis {
            subject;
            requirement :>> maxSpeedRequirement = maxSpeedRequirementLarge;
            out voltage = vehicle.battery.batteryBehavior.output.voltage;
            return simulatedMaxSpeed = vehicle.vehicleBehavior.output.velocity;
        }
    }
}
~~~
# TOKENS
~~~zig
LineComment,
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Star,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
Ident,ColonGt,Ident,ColonColon,Ident,Eq,Ident,ColonColon,Ident,Slash,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwRequirement,KwDef,Ident,OpenCurly,
KwSubject,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAnalysis,KwDef,Ident,OpenCurly,
KwSubject,Ident,Colon,Ident,Semicolon,
KwRequirement,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwRequirement,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,RegularComment,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwRequire,KwConstraint,OpenCurly,Ident,GtEq,Ident,CloseCurly,
CloseCurly,
KwAnalysis,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwReturn,Ident,Colon,Ident,Semicolon,
KwRequirement,Ident,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwObjective,Ident,OpenCurly,
KwDoc,RegularComment,
KwRequire,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwRequirement,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,RegularComment,
KwAttribute,Ident,Semicolon,
KwAttribute,Ident,Semicolon,
KwRequire,KwConstraint,OpenCurly,Ident,GtEq,Ident,CloseCurly,
CloseCurly,
KwAnalysis,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwReturn,Ident,Semicolon,
KwRequirement,Ident,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwObjective,Ident,OpenCurly,
KwRequire,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwRequirement,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,RegularComment,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwAnalysis,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwReturn,Ident,Semicolon,
KwRequirement,Ident,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwObjective,Ident,OpenCurly,
KwRequire,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,KwDefault,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
RegularComment,
KwAttribute,Ident,Eq,DecimalValue,Dot,DecimalValue,Semicolon,
KwAttribute,Ident,Semicolon,
KwAction,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwOut,Ident,Colon,Ident,Semicolon,
ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,UnrestrictedName,CloseSquare,Semicolon,
ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,Semicolon,
ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,UnrestrictedName,CloseSquare,Semicolon,
KwAction,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwOut,Ident,Colon,Ident,Semicolon,
ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwFlow,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,UnrestrictedName,CloseSquare,Semicolon,
ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwAction,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwOut,Ident,Colon,Ident,Semicolon,
ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwFlow,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,KwDefault,DecimalValue,OpenSquare,UnrestrictedName,CloseSquare,Semicolon,
ColonGtGt,Ident,KwDefault,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwAction,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwOut,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwFlow,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwFlow,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwPart,ColonGtGt,Ident,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,UnrestrictedName,CloseSquare,Semicolon,
ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,OpenCurly,
KwRequirement,Ident,Colon,Ident,OpenCurly,
KwDoc,RegularComment,
KwSubject,ColonGtGt,Ident,Eq,Ident,Semicolon,
RegularComment,
KwAssume,KwConstraint,OpenCurly,Ident,Dot,Ident,OpenAngle,DecimalValue,OpenSquare,Ident,CloseSquare,CloseCurly,
CloseCurly,
KwAnalysis,Ident,Colon,Ident,OpenCurly,
KwSubject,ColonGtGt,Ident,ColonGt,Ident,Semicolon,
KwRequirement,ColonGtGt,Ident,Eq,Ident,Semicolon,
CloseCurly,
KwRequirement,OpenAngle,Ident,CloseAngle,Ident,ColonGt,Ident,Colon,Ident,OpenCurly,
KwDoc,RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
KwAnalysis,Ident,ColonGt,Ident,Colon,Ident,OpenCurly,
KwRequirement,ColonGtGt,Ident,Eq,Ident,Semicolon,
KwReturn,Ident,Eq,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwRequirement,OpenAngle,Ident,CloseAngle,Ident,ColonGt,Ident,Colon,Ident,OpenCurly,
KwDoc,RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,Semicolon,
CloseCurly,
KwAnalysis,Ident,ColonGt,Ident,Colon,Ident,OpenCurly,
KwRequirement,ColonGtGt,Ident,Eq,Ident,Semicolon,
KwReturn,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwRequirement,OpenAngle,Ident,CloseAngle,Ident,ColonGt,Ident,Colon,Ident,OpenCurly,
KwDoc,RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,Slash,Ident,CloseSquare,Semicolon,
CloseCurly,
KwAnalysis,Ident,ColonGt,Ident,Colon,Ident,OpenCurly,
KwSubject,Semicolon,
KwRequirement,ColonGtGt,Ident,Eq,Ident,Semicolon,
KwOut,Ident,ColonGt,Ident,ColonColon,Ident,Eq,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwReturn,Ident,Eq,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwPart,ColonGtGt,Ident,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,UnrestrictedName,CloseSquare,Semicolon,
ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,OpenCurly,
KwRequirement,Ident,Colon,Ident,OpenCurly,
KwDoc,RegularComment,
KwSubject,ColonGtGt,Ident,Eq,Ident,Semicolon,
RegularComment,
KwAssume,KwConstraint,OpenCurly,Ident,Dot,Ident,OpenAngle,DecimalValue,OpenSquare,Ident,CloseSquare,CloseCurly,
CloseCurly,
KwAnalysis,Ident,Colon,Ident,OpenCurly,
KwSubject,ColonGtGt,Ident,ColonGt,Ident,Semicolon,
KwRequirement,ColonGtGt,Ident,Eq,Ident,Semicolon,
CloseCurly,
KwRequirement,OpenAngle,Ident,CloseAngle,Ident,ColonGt,Ident,Colon,Ident,OpenCurly,
KwDoc,RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
KwAnalysis,Ident,ColonGt,Ident,Colon,Ident,OpenCurly,
KwRequirement,ColonGtGt,Ident,Eq,Ident,Semicolon,
KwReturn,Ident,Eq,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwRequirement,OpenAngle,Ident,CloseAngle,Ident,ColonGt,Ident,Colon,Ident,OpenCurly,
KwDoc,RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,Semicolon,
CloseCurly,
KwAnalysis,Ident,ColonGt,Ident,Colon,Ident,OpenCurly,
KwRequirement,ColonGtGt,Ident,Eq,Ident,Semicolon,
KwReturn,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwRequirement,OpenAngle,Ident,CloseAngle,Ident,ColonGt,Ident,Colon,Ident,OpenCurly,
KwDoc,RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,Slash,Ident,CloseSquare,Semicolon,
CloseCurly,
KwAnalysis,Ident,ColonGt,Ident,Colon,Ident,OpenCurly,
KwSubject,Semicolon,
KwRequirement,ColonGtGt,Ident,Eq,Ident,Semicolon,
KwOut,Ident,Eq,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwReturn,Ident,Eq,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (line_comment)
  (package_def 'EVSample'
    (import_decl private 'SI::*')
    (import_decl private 'StateSpaceRepresentation::*')
    (attribute_usage ''ampere hour'' : 'ElectricChargeUnit' value)
    (part_def 'Vehicle'
      (attribute_usage 'mass' :> 'ISQ::mass')
      (attribute_def 'VehicleInput' :> 'Input'
        (attribute_usage 'force' :> 'ISQ::force'))
      (attribute_def 'VehicleOutput' :> 'Output'
        (attribute_usage 'accel' :> 'ISQ::acceleration')
        (attribute_usage 'velocity' :> 'ISQ::speed')
        (attribute_usage 'distance' :> 'ISQ::distance'))
      (attribute_def 'VehicleState' :> 'StateSpace'
        (attribute_usage 'velocity' :> 'ISQ::speed')
        (attribute_usage 'distance' :> 'ISQ::distance')))
    (part_def 'Battery'
      (attribute_usage 'baseVoltage' :> 'ISQ::electricPotential')
      (attribute_usage 'socInit' : 'ScalarValues::Real')
      (attribute_usage 'capacity' :> 'ISQ::electricCharge')
      (attribute_usage 'internalResistance' :> 'ISQ::resistance')
      (attribute_def 'BatteryInput' :> 'Input'
        (attribute_usage 'current' :> 'ISQ::electricCurrent'))
      (attribute_def 'BatteryOutput' :> 'Output'
        (attribute_usage 'voltage' :> 'ISQ::electricPotential'))
      (attribute_def 'BatteryState' :> 'StateSpace'
        (attribute_usage 'soc' : 'ScalarValues::Real')))
    (part_def 'Motor'
      (default_ref_usage 'torquePerCurrent' :> 'Quantities::scalarQuantities' value)
      (attribute_usage 'motR' :> 'ISQ::resistance')
      (attribute_usage 'motL' :> 'ISQ::inductance')
      (attribute_def 'MotorInput' :> 'Input'
        (attribute_usage 'voltage' :> 'ISQ::electricPotential')
        (attribute_usage 'friction' :> 'ISQ::torque'))
      (attribute_def 'MotorOutput' :> 'Output'
        (attribute_usage 'current' :> 'ISQ::electricCurrent')
        (attribute_usage 'torque' :> 'ISQ::torque'))
      (attribute_def 'MotorState' :> 'StateSpace'
        (attribute_usage 'current' :> 'ISQ::electricCurrent')))
    (part_def 'Tire'
      (attribute_usage 'radius' :> 'ISQ::length')
      (attribute_usage 'moment' :> 'ISQ::momentOfInertia')
      (attribute_def 'TireInput' :> 'Input'
        (attribute_usage 'torque' :> 'ISQ::torque')
        (attribute_usage 'accel' :> 'ISQ::acceleration'))
      (attribute_def 'TireOutput' :> 'Output'
        (attribute_usage 'force' :> 'ISQ::force')
        (attribute_usage 'outTorque' :> 'ISQ::torque')))
    (requirement_def 'VehicleRequirement'
      (sysml_decl 'vehicle' : 'Vehicle'))
    (analysis_case_def 'VehicleAnalysis'
      (sysml_decl 'vehicle' : 'Vehicle')
      (requirement_usage 'vehicleRequirement' : 'VehicleRequirement'))
    (requirement_def 'RangeRequirement' :> 'VehicleRequirement'
      (documentation)
      (attribute_usage 'actualRange' : 'LengthValue')
      (attribute_usage 'requiredRange' : 'LengthValue')
      (sysml_decl
        (result_expr_member)))
    (analysis_case_def 'RangeAnalysis' :> 'VehicleAnalysis'
      (return_member)
      (requirement_usage 'rangeRequirement' :>> 'vehicleRequirement' : 'RangeRequirement')
      (objective_member))
    (requirement_def 'EfficiencyRequirement' :> 'VehicleRequirement'
      (documentation)
      (attribute_usage 'actualEfficiency')
      (attribute_usage 'requiredEfficiency')
      (sysml_decl
        (result_expr_member)))
    (analysis_case_def 'EfficiencyAnalysis' :> 'VehicleAnalysis'
      (return_member)
      (requirement_usage 'efficiencyRequirement' :>> 'vehicleRequirement' : 'EfficiencyRequirement')
      (objective_member))
    (requirement_def 'MaxSpeedRequirement' :> 'VehicleRequirement'
      (documentation)
      (attribute_usage 'actualMaxSpeed' :> 'ISQ::speed')
      (attribute_usage 'requiredMaxSpeed' :> 'ISQ::speed'))
    (analysis_case_def 'MaxSpeedAnalysis' :> 'VehicleAnalysis'
      (return_member)
      (requirement_usage 'maxSpeedRequirement' :>> 'vehicleRequirement' : 'MaxSpeedRequirement')
      (objective_member))
    (part_usage 'vehicle' : 'Vehicle'
      (attribute_usage :>> 'mass' value)
      (comment)
      (attribute_usage 'airFrictionCoefficient' value)
      (attribute_usage 'efficiency')
      (action_usage 'vehicleBehavior' : 'ContinuousStateSpaceDynamics'
        (default_ref_usage in 'input' : 'VehicleInput')
        (default_ref_usage out 'output' : 'VehicleOutput')
        (default_ref_usage :>> 'stateSpace' : 'VehicleState'))
      (part_usage 'battery' : 'Battery'
        (default_ref_usage :>> 'baseVoltage' value)
        (default_ref_usage :>> 'capacity' value)
        (default_ref_usage :>> 'socInit' value)
        (default_ref_usage :>> 'internalResistance' value)
        (action_usage 'batteryBehavior' : 'ContinuousStateSpaceDynamics'
          (default_ref_usage in 'input' : 'BatteryInput')
          (default_ref_usage out 'output' : 'BatteryOutput')
          (default_ref_usage :>> 'stateSpace' : 'BatteryState')))
      (flow_usage 'battery')
      (part_usage 'motor' : 'Motor'
        (default_ref_usage :>> 'motR' value)
        (default_ref_usage :>> 'motL' value)
        (action_usage 'motorBehavior' : 'ContinuousStateSpaceDynamics'
          (default_ref_usage in 'input' : 'MotorInput')
          (default_ref_usage out 'output' : 'MotorOutput')
          (default_ref_usage :>> 'stateSpace' : 'MotorState')))
      (flow_usage 'motor')
      (part_usage 'tire' : 'Tire'
        (default_ref_usage :>> 'moment' value)
        (default_ref_usage :>> 'radius' value)
        (action_usage 'tireBehavior' : 'ContinuousStateSpaceDynamics'
          (default_ref_usage in 'input' : 'TireInput')
          (default_ref_usage out 'output' : 'TireOutput')))
      (flow_usage 'tire')
      (flow_usage 'tire'))
    (part_usage 'vehicle_compact' :> 'vehicle'
      (attribute_usage :>> 'mass' value)
      (part_usage :>> 'tire'
        (default_ref_usage :>> 'moment' value)
        (default_ref_usage :>> 'radius' value)))
    (part_usage 'smallEVRangeContext'
      (requirement_usage 'smallEVRequirement' : 'VehicleRequirement'
        (documentation)
        (sysml_decl :>> 'vehicle' value)
        (comment)
        (sysml_decl
          (result_expr_member)))
      (sysml_decl 'smallEVAnalysis' : 'VehicleAnalysis'
        (sysml_decl :>> 'vehicle' :> 'vehicle_compact')
        (requirement_usage :>> 'vehicleRequirement' value))
      (requirement_usage 'rangeRequirementSmall' :> 'smallEVRequirement' : 'RangeRequirement'
        (documentation)
        (attribute_usage :>> 'requiredRange' value))
      (sysml_decl 'rangeAnalysisSmall' :> 'smallEVAnalysis' : 'RangeAnalysis'
        (requirement_usage :>> 'rangeRequirement' value)
        (return_member))
      (requirement_usage 'efficiencyRequirementSmall' :> 'smallEVRequirement' : 'EfficiencyRequirement'
        (documentation)
        (attribute_usage :>> 'requiredEfficiency' value))
      (sysml_decl 'efficiencyAnalysisSmall' :> 'smallEVAnalysis' : 'EfficiencyAnalysis'
        (requirement_usage :>> 'efficiencyRequirement' value)
        (return_member))
      (requirement_usage 'maxSpeedRequirementSmall' :> 'smallEVRequirement' : 'MaxSpeedRequirement'
        (documentation)
        (attribute_usage :>> 'requiredMaxSpeed' value))
      (sysml_decl 'maxSpeedAnalysisSmall' :> 'smallEVAnalysis' : 'MaxSpeedAnalysis'
        (sysml_decl)
        (requirement_usage :>> 'maxSpeedRequirement' value)
        (default_ref_usage out 'voltage' :> 'ISQ::electricPotential' value)
        (return_member)))
    (part_usage 'vehicle_large' :> 'vehicle'
      (attribute_usage :>> 'mass' value)
      (part_usage :>> 'tire'
        (default_ref_usage :>> 'moment' value)
        (default_ref_usage :>> 'radius' value)))
    (part_usage 'largeEVRangeContext'
      (requirement_usage 'largeEVRequirement' : 'VehicleRequirement'
        (documentation)
        (sysml_decl :>> 'vehicle' value)
        (comment)
        (sysml_decl
          (result_expr_member)))
      (sysml_decl 'largeEVAnalysis' : 'VehicleAnalysis'
        (sysml_decl :>> 'vehicle' :> 'vehicle_large')
        (requirement_usage :>> 'vehicleRequirement' value))
      (requirement_usage 'rangeRequirementLarge' :> 'largeEVRequirement' : 'RangeRequirement'
        (documentation)
        (attribute_usage :>> 'requiredRange' value))
      (sysml_decl 'rangeAnalysisLarge' :> 'largeEVAnalysis' : 'RangeAnalysis'
        (requirement_usage :>> 'rangeRequirement' value)
        (return_member))
      (requirement_usage 'efficiencyRequirementLarge' :> 'largeEVRequirement' : 'EfficiencyRequirement'
        (documentation)
        (attribute_usage :>> 'requiredEfficiency' value))
      (sysml_decl 'efficiencyAnalysisLarge' :> 'largeEVAnalysis' : 'EfficiencyAnalysis'
        (requirement_usage :>> 'efficiencyRequirement' value)
        (return_member))
      (requirement_usage 'maxSpeedRequirementLarge' :> 'largeEVRequirement' : 'MaxSpeedRequirement'
        (documentation)
        (attribute_usage :>> 'requiredMaxSpeed' value))
      (sysml_decl 'maxSpeedAnalysisLarge' :> 'largeEVAnalysis' : 'MaxSpeedAnalysis'
        (sysml_decl)
        (requirement_usage :>> 'maxSpeedRequirement' value)
        (default_ref_usage out 'voltage' value)
        (return_member)))))
~~~
# FORMAT
~~~sysml
// State Space Representation EV example
package EVSample {
    private import SI::*;
    private import StateSpaceRepresentation::*;

    attribute <'A⋅h'> 'ampere hour' : ElectricChargeUnit = A*h;

    part def Vehicle {
        attribute mass :> ISQ::mass;

        attribute def VehicleInput :> Input {
            attribute force :> ISQ::force;
        }

        attribute def VehicleOutput :> Output {
            attribute accel :> ISQ::acceleration;
            attribute velocity :> ISQ::speed;
            attribute distance :> ISQ::distance;
        }

        attribute def VehicleState :> StateSpace {
            attribute velocity :> ISQ::speed;
            attribute distance :> ISQ::distance;
        }
    }

    part def Battery {
        attribute baseVoltage :> ISQ::electricPotential;
        attribute socInit : ScalarValues::Real;
        attribute capacity :> ISQ::electricCharge;
        attribute internalResistance :> ISQ::resistance;

        attribute def BatteryInput :> Input {
            attribute current :> ISQ::electricCurrent;
        }

        attribute def BatteryOutput :> Output {
            attribute voltage :> ISQ::electricPotential;
        }

        attribute def BatteryState :> StateSpace {
            attribute soc : ScalarValues::Real;
        }
    }

    part def Motor {
        torquePerCurrent :> Quantities::scalarQuantities = ISQ::torque / ISQ::electricCurrent;

        attribute motR :> ISQ::resistance;
        attribute motL :> ISQ::inductance;

        attribute def MotorInput :> Input {
            attribute voltage :> ISQ::electricPotential;
            attribute friction :> ISQ::torque;
        }

        attribute def MotorOutput :> Output {
            attribute current :> ISQ::electricCurrent;
            attribute torque :> ISQ::torque;
        }

        attribute def MotorState :> StateSpace {
            attribute current :> ISQ::electricCurrent;
        }
    }

    part def Tire {
        attribute radius :> ISQ::length;
        attribute moment :> ISQ::momentOfInertia;

        attribute def TireInput :> Input {
            attribute torque :> ISQ::torque;
            attribute accel :> ISQ::acceleration;
        }

        attribute def TireOutput :> Output {
            attribute force :> ISQ::force;
            attribute outTorque :> ISQ::torque;
        }
    }

    requirement def VehicleRequirement {
        subject vehicle : Vehicle;
    }

    analysis def VehicleAnalysis {
        subject vehicle : Vehicle;
        requirement vehicleRequirement : VehicleRequirement;
    }

    requirement def RangeRequirement :> VehicleRequirement {
        doc /* The range of EV must be longer than the required spec under the flat road. */
        attribute actualRange : LengthValue;
        attribute requiredRange : LengthValue;

        require constraint {
            = actualRange >= requiredRange;
        }
    }

    analysis def RangeAnalysis :> VehicleAnalysis {
        return simulatedRange : LengthValue;

        requirement rangeRequirement :>> vehicleRequirement : RangeRequirement;

        objective rangeAnalysisObjective {
            doc /* This analysis is to estimate the range of
                 * the EV by simulating the vehicle driving under the compact vehicle regulation.
                 */
            require constraint rangeRequirement {
                :>> actualRange = simulatedRange;
            }
        }
    }

    requirement def EfficiencyRequirement :> VehicleRequirement {
        doc /* The efficiency of EV must be better than the required spec. */
        attribute actualEfficiency;
        attribute requiredEfficiency;

        require constraint {
            = actualEfficiency >= requiredEfficiency;
        }
    }

    analysis def EfficiencyAnalysis :> VehicleAnalysis {
        return simulatedEfficiency;

        requirement efficiencyRequirement :>> vehicleRequirement : EfficiencyRequirement;

        objective efficiencyAnalysisObjective {
            require constraint efficiencyRequirement {
                attribute :>> actualEfficiency = simulatedEfficiency;
            }
        }
    }

    requirement def MaxSpeedRequirement :> VehicleRequirement {
        doc /* The maximum speed of EV must be larger than the required spec. */
        attribute actualMaxSpeed :> ISQ::speed;
        attribute requiredMaxSpeed :> ISQ::speed;
    }

    analysis def MaxSpeedAnalysis :> VehicleAnalysis {
        return simulatedMaxSpeed;

        requirement maxSpeedRequirement :>> vehicleRequirement : MaxSpeedRequirement;

        objective maxSpeedAnalysisObjective {
            require constraint maxSpeedRequirement {
                attribute :>> actualMaxSpeed = simulatedMaxSpeed;
            }
        }
    }

    part vehicle : Vehicle {
        attribute :>> mass default = 1000[kg];

        /* airFrictionCoefficient [kg / m] = 1/2 * rho[kg/m^3] * Cd * S[m^2],
         * where rho is air density, S is front projected area. */
        attribute airFrictionCoefficient = 0.2;

        attribute efficiency;

        action vehicleBehavior : ContinuousStateSpaceDynamics {
            in input : VehicleInput;
            out output : VehicleOutput;
            :>> stateSpace : VehicleState;
        }

        part battery : Battery {
            :>> baseVoltage = 300[V];
            :>> capacity = 50['A⋅h'];
            :>> socInit = 0.8;
            :>> internalResistance = 1.8['Ω'];
            action batteryBehavior : ContinuousStateSpaceDynamics {
                in input : BatteryInput;
                out output : BatteryOutput;
                :>> stateSpace : BatteryState;
            }
        }

        flow battery;

        part motor : Motor {
            :>> motR = 4['Ω'];
            :>> motL = 0.2[H];

            action motorBehavior : ContinuousStateSpaceDynamics {
                in input : MotorInput;
                out output : MotorOutput;
                :>> stateSpace : MotorState;
            }
        }

        flow motor;

        part tire : Tire {
            :>> moment default = 300['kg⋅m²'];
            :>> radius default = 0.7[m];
            action tireBehavior : ContinuousStateSpaceDynamics {
                in input : TireInput;
                out output : TireOutput;
            }
        }

        flow tire;
        flow tire;
    }

    part vehicle_compact :> vehicle {
        attribute :>> mass = 800[kg];
        part :>> tire {
            :>> moment = 200['kg⋅m²'];
            :>> radius = 0.5[m];
        }
    }

    part smallEVRangeContext {
        requirement smallEVRequirement : VehicleRequirement {
            doc /* The small EVs must be ligher than 900[kg] */
            subject :>> vehicle = vehicle_compact;
            /*  To comform with the regulation and the battery mass will impact it. */
            assume constraint {
                = vehicle.mass < 900[kg];
            }
        }

        analysis smallEVAnalysis : VehicleAnalysis {
            subject :>> vehicle :> vehicle_compact;
            requirement :>> vehicleRequirement = smallEVRequirement;
        }

        requirement <C1> rangeRequirementSmall :> smallEVRequirement : RangeRequirement {
            doc /* The small EVs must run longer than 130km */
            attribute :>> requiredRange = 130[km];
        }

        analysis rangeAnalysisSmall :> smallEVAnalysis : RangeAnalysis {
            requirement :>> rangeRequirement = rangeRequirementSmall;
            return simulatedRange = vehicle.vehicleBehavior.output.distance;
        }

        requirement <C2> efficiencyRequirementSmall :> smallEVRequirement : EfficiencyRequirement {
            doc /* The target efficiency of small EVs is 0.9. */
            attribute :>> requiredEfficiency = 0.9;
        }

        analysis efficiencyAnalysisSmall :> smallEVAnalysis : EfficiencyAnalysis {
            requirement :>> efficiencyRequirement = efficiencyRequirementSmall;

            return simulatedEfficiency = vehicle.efficiency;
        }

        requirement <C3> maxSpeedRequirementSmall :> smallEVRequirement : MaxSpeedRequirement {
            doc /* The target maximum speed of small EVs is 130 [km/h]. */
            attribute :>> requiredMaxSpeed = 130 [km/h];
        }

        analysis maxSpeedAnalysisSmall :> smallEVAnalysis : MaxSpeedAnalysis {
            subject;
            requirement :>> maxSpeedRequirement = maxSpeedRequirementSmall;
            out voltage :> ISQ::electricPotential = vehicle.battery.batteryBehavior.output.voltage;
            return simulatedMaxSpeed = vehicle.vehicleBehavior.output.velocity;
        }
    }

    part vehicle_large :> vehicle {
        attribute :>> mass = 1100[kg];
        part :>> tire {
            :>> moment = 300['kg⋅m²'];
            :>> radius = 0.7[m];
        }
    }

    part largeEVRangeContext {
        requirement largeEVRequirement : VehicleRequirement {
            doc /* The large EVs must be ligher than 900[kg] */
            subject :>> vehicle = vehicle_large;
            /*  To comform with the regulation and the battery mass will impact it. */
            assume constraint {
                = vehicle.mass < 1200[kg];
            }
        }

        analysis largeEVAnalysis : VehicleAnalysis {
            subject :>> vehicle :> vehicle_large;
            requirement :>> vehicleRequirement = largeEVRequirement;
        }

        requirement <L1> rangeRequirementLarge :> largeEVRequirement : RangeRequirement {
            doc /* The large EVs must run longer than 200km */
            attribute :>> requiredRange = 200[km];
        }

        analysis rangeAnalysisLarge :> largeEVAnalysis : RangeAnalysis {
            requirement :>> rangeRequirement = rangeRequirementLarge;
            return simulatedRange = vehicle.vehicleBehavior.output.distance;
        }

        requirement <L2> efficiencyRequirementLarge :> largeEVRequirement : EfficiencyRequirement {
            doc /* The target efficiency of large EVs is 0.8. */
            attribute :>> requiredEfficiency = 0.8;
        }

        analysis efficiencyAnalysisLarge :> largeEVAnalysis : EfficiencyAnalysis {
            requirement :>> efficiencyRequirement = efficiencyRequirementLarge;

            return simulatedEfficiency = vehicle.efficiency;
        }

        requirement <L3> maxSpeedRequirementLarge :> largeEVRequirement : MaxSpeedRequirement {
            doc /* The target maximum speed of large EVs is 140 [km/h]. */
            attribute :>> requiredMaxSpeed = 140 [km/h];
        }

        analysis maxSpeedAnalysisLarge :> largeEVAnalysis : MaxSpeedAnalysis {
            subject;
            requirement :>> maxSpeedRequirement = maxSpeedRequirementLarge;
            out voltage = vehicle.battery.batteryBehavior.output.voltage;
            return simulatedMaxSpeed = vehicle.vehicleBehavior.output.velocity;
        }
    }
}
~~~
# EXPECTED
~~~
semantic.duplicate_name 'battery'
semantic.duplicate_name 'motor'
semantic.duplicate_name 'tire'
semantic.duplicate_name 'tire'
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.unresolved_name 'ElectricChargeUnit'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'Input'
semantic.unresolved_name 'ISQ::force'
semantic.unresolved_name 'Output'
semantic.unresolved_name 'ISQ::acceleration'
semantic.unresolved_name 'ISQ::speed'
semantic.unresolved_name 'ISQ::distance'
semantic.unresolved_name 'StateSpace'
semantic.unresolved_name 'ISQ::speed'
semantic.unresolved_name 'ISQ::distance'
semantic.unresolved_name 'ISQ::electricPotential'
semantic.unresolved_name 'ScalarValues::Real'
semantic.unresolved_name 'ISQ::electricCharge'
semantic.unresolved_name 'ISQ::resistance'
semantic.unresolved_name 'Input'
semantic.unresolved_name 'ISQ::electricCurrent'
semantic.unresolved_name 'Output'
semantic.unresolved_name 'ISQ::electricPotential'
semantic.unresolved_name 'StateSpace'
semantic.unresolved_name 'ScalarValues::Real'
semantic.unresolved_name 'Quantities::scalarQuantities'
semantic.unresolved_name 'ISQ::resistance'
semantic.unresolved_name 'ISQ::inductance'
semantic.unresolved_name 'Input'
semantic.unresolved_name 'ISQ::electricPotential'
semantic.unresolved_name 'ISQ::torque'
semantic.unresolved_name 'Output'
semantic.unresolved_name 'ISQ::electricCurrent'
semantic.unresolved_name 'ISQ::torque'
semantic.unresolved_name 'StateSpace'
semantic.unresolved_name 'ISQ::electricCurrent'
semantic.unresolved_name 'ISQ::length'
semantic.unresolved_name 'ISQ::momentOfInertia'
semantic.unresolved_name 'Input'
semantic.unresolved_name 'ISQ::torque'
semantic.unresolved_name 'ISQ::acceleration'
semantic.unresolved_name 'Output'
semantic.unresolved_name 'ISQ::force'
semantic.unresolved_name 'ISQ::torque'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'actualRange'
semantic.unresolved_name 'actualEfficiency'
semantic.unresolved_name 'ISQ::speed'
semantic.unresolved_name 'ISQ::speed'
semantic.unresolved_name 'actualMaxSpeed'
semantic.unresolved_name 'ContinuousStateSpaceDynamics'
semantic.unresolved_name 'stateSpace'
semantic.unresolved_name 'ContinuousStateSpaceDynamics'
semantic.unresolved_name 'stateSpace'
semantic.unresolved_name 'ContinuousStateSpaceDynamics'
semantic.unresolved_name 'stateSpace'
semantic.unresolved_name 'ContinuousStateSpaceDynamics'
semantic.unresolved_name 'ISQ::electricPotential'
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'battery'
semantic.duplicate_name 'motor'
semantic.duplicate_name 'tire'
semantic.duplicate_name 'tire'
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.unresolved_name 'ElectricChargeUnit'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'Input'
semantic.unresolved_name 'ISQ::force'
semantic.unresolved_name 'Output'
semantic.unresolved_name 'ISQ::acceleration'
semantic.unresolved_name 'ISQ::speed'
semantic.unresolved_name 'ISQ::distance'
semantic.unresolved_name 'StateSpace'
semantic.unresolved_name 'ISQ::speed'
semantic.unresolved_name 'ISQ::distance'
semantic.unresolved_name 'ISQ::electricPotential'
semantic.unresolved_name 'ScalarValues::Real'
semantic.unresolved_name 'ISQ::electricCharge'
semantic.unresolved_name 'ISQ::resistance'
semantic.unresolved_name 'Input'
semantic.unresolved_name 'ISQ::electricCurrent'
semantic.unresolved_name 'Output'
semantic.unresolved_name 'ISQ::electricPotential'
semantic.unresolved_name 'StateSpace'
semantic.unresolved_name 'ScalarValues::Real'
semantic.unresolved_name 'Quantities::scalarQuantities'
semantic.unresolved_name 'ISQ::resistance'
semantic.unresolved_name 'ISQ::inductance'
semantic.unresolved_name 'Input'
semantic.unresolved_name 'ISQ::electricPotential'
semantic.unresolved_name 'ISQ::torque'
semantic.unresolved_name 'Output'
semantic.unresolved_name 'ISQ::electricCurrent'
semantic.unresolved_name 'ISQ::torque'
semantic.unresolved_name 'StateSpace'
semantic.unresolved_name 'ISQ::electricCurrent'
semantic.unresolved_name 'ISQ::length'
semantic.unresolved_name 'ISQ::momentOfInertia'
semantic.unresolved_name 'Input'
semantic.unresolved_name 'ISQ::torque'
semantic.unresolved_name 'ISQ::acceleration'
semantic.unresolved_name 'Output'
semantic.unresolved_name 'ISQ::force'
semantic.unresolved_name 'ISQ::torque'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'actualRange'
semantic.unresolved_name 'actualEfficiency'
semantic.unresolved_name 'ISQ::speed'
semantic.unresolved_name 'ISQ::speed'
semantic.unresolved_name 'actualMaxSpeed'
semantic.unresolved_name 'ContinuousStateSpaceDynamics'
semantic.unresolved_name 'stateSpace'
semantic.unresolved_name 'ContinuousStateSpaceDynamics'
semantic.unresolved_name 'stateSpace'
semantic.unresolved_name 'ContinuousStateSpaceDynamics'
semantic.unresolved_name 'stateSpace'
semantic.unresolved_name 'ContinuousStateSpaceDynamics'
semantic.unresolved_name 'ISQ::electricPotential'
~~~
# SMG
~~~
(model
  (namespace
    (package 'EVSample'
      (namespace_import private -> 'SI'[unresolved])
      (namespace_import private -> 'StateSpaceRepresentation'[unresolved])
      (attribute_usage 'ampere hour' : 'ElectricChargeUnit'[unresolved]
        (feature_value (=)))
      (part_def 'Vehicle'
        (attribute_usage composite 'mass' :> 'ISQ::mass'[unresolved])
        (attribute_def 'VehicleInput' :> 'Input'[unresolved]
          (attribute_usage composite 'force' :> 'ISQ::force'[unresolved]))
        (attribute_def 'VehicleOutput' :> 'Output'[unresolved]
          (attribute_usage composite 'accel' :> 'ISQ::acceleration'[unresolved])
          (attribute_usage composite 'velocity' :> 'ISQ::speed'[unresolved])
          (attribute_usage composite 'distance' :> 'ISQ::distance'[unresolved]))
        (attribute_def 'VehicleState' :> 'StateSpace'[unresolved]
          (attribute_usage composite 'velocity' :> 'ISQ::speed'[unresolved])
          (attribute_usage composite 'distance' :> 'ISQ::distance'[unresolved])))
      (part_def 'Battery'
        (attribute_usage composite 'baseVoltage' :> 'ISQ::electricPotential'[unresolved])
        (attribute_usage composite 'socInit' : 'ScalarValues::Real'[unresolved])
        (attribute_usage composite 'capacity' :> 'ISQ::electricCharge'[unresolved])
        (attribute_usage composite 'internalResistance' :> 'ISQ::resistance'[unresolved])
        (attribute_def 'BatteryInput' :> 'Input'[unresolved]
          (attribute_usage composite 'current' :> 'ISQ::electricCurrent'[unresolved]))
        (attribute_def 'BatteryOutput' :> 'Output'[unresolved]
          (attribute_usage composite 'voltage' :> 'ISQ::electricPotential'[unresolved]))
        (attribute_def 'BatteryState' :> 'StateSpace'[unresolved]
          (attribute_usage composite 'soc' : 'ScalarValues::Real'[unresolved])))
      (part_def 'Motor'
        (reference_usage reference 'torquePerCurrent' :> 'Quantities::scalarQuantities'[unresolved]
          (feature_value (=)))
        (attribute_usage composite 'motR' :> 'ISQ::resistance'[unresolved])
        (attribute_usage composite 'motL' :> 'ISQ::inductance'[unresolved])
        (attribute_def 'MotorInput' :> 'Input'[unresolved]
          (attribute_usage composite 'voltage' :> 'ISQ::electricPotential'[unresolved])
          (attribute_usage composite 'friction' :> 'ISQ::torque'[unresolved]))
        (attribute_def 'MotorOutput' :> 'Output'[unresolved]
          (attribute_usage composite 'current' :> 'ISQ::electricCurrent'[unresolved])
          (attribute_usage composite 'torque' :> 'ISQ::torque'[unresolved]))
        (attribute_def 'MotorState' :> 'StateSpace'[unresolved]
          (attribute_usage composite 'current' :> 'ISQ::electricCurrent'[unresolved])))
      (part_def 'Tire'
        (attribute_usage composite 'radius' :> 'ISQ::length'[unresolved])
        (attribute_usage composite 'moment' :> 'ISQ::momentOfInertia'[unresolved])
        (attribute_def 'TireInput' :> 'Input'[unresolved]
          (attribute_usage composite 'torque' :> 'ISQ::torque'[unresolved])
          (attribute_usage composite 'accel' :> 'ISQ::acceleration'[unresolved]))
        (attribute_def 'TireOutput' :> 'Output'[unresolved]
          (attribute_usage composite 'force' :> 'ISQ::force'[unresolved])
          (attribute_usage composite 'outTorque' :> 'ISQ::torque'[unresolved])))
      (requirement_def 'VehicleRequirement'
        (subject_membership in 'vehicle' : 'EVSample::Vehicle'[part_def]))
      (analysis_case_def 'VehicleAnalysis'
        (subject_membership in 'vehicle' : 'EVSample::Vehicle'[part_def])
        (requirement_usage composite 'vehicleRequirement' : 'EVSample::VehicleRequirement'[requirement_def]))
      (requirement_def 'RangeRequirement' :> 'EVSample::VehicleRequirement'[requirement_def]
        (documentation)
        (attribute_usage composite 'actualRange' : 'LengthValue'[unresolved])
        (attribute_usage composite 'requiredRange' : 'LengthValue'[unresolved])
        (require_constraint_usage composite
          (result_expr_membership)))
      (analysis_case_def 'RangeAnalysis' :> 'EVSample::VehicleAnalysis'[analysis_case_def]
        (return_parameter_membership
          (feature_def out 'simulatedRange' : 'LengthValue'[unresolved]))
        (requirement_usage composite 'rangeRequirement' :>> 'EVSample::VehicleAnalysis::vehicleRequirement'[requirement_usage] : 'EVSample::RangeRequirement'[requirement_def])
        (objective_membership composite 'rangeAnalysisObjective'
          (documentation)
          (require_constraint_usage composite 'rangeRequirement'
            (reference_usage reference :>> 'actualRange'[unresolved]
              (feature_value (=))))))
      (requirement_def 'EfficiencyRequirement' :> 'EVSample::VehicleRequirement'[requirement_def]
        (documentation)
        (attribute_usage composite 'actualEfficiency')
        (attribute_usage composite 'requiredEfficiency')
        (require_constraint_usage composite
          (result_expr_membership)))
      (analysis_case_def 'EfficiencyAnalysis' :> 'EVSample::VehicleAnalysis'[analysis_case_def]
        (return_parameter_membership
          (feature_def out 'simulatedEfficiency'))
        (requirement_usage composite 'efficiencyRequirement' :>> 'EVSample::VehicleAnalysis::vehicleRequirement'[requirement_usage] : 'EVSample::EfficiencyRequirement'[requirement_def])
        (objective_membership composite 'efficiencyAnalysisObjective'
          (require_constraint_usage composite 'efficiencyRequirement'
            (attribute_usage :>> 'actualEfficiency'[unresolved]
              (feature_value (=))))))
      (requirement_def 'MaxSpeedRequirement' :> 'EVSample::VehicleRequirement'[requirement_def]
        (documentation)
        (attribute_usage composite 'actualMaxSpeed' :> 'ISQ::speed'[unresolved])
        (attribute_usage composite 'requiredMaxSpeed' :> 'ISQ::speed'[unresolved]))
      (analysis_case_def 'MaxSpeedAnalysis' :> 'EVSample::VehicleAnalysis'[analysis_case_def]
        (return_parameter_membership
          (feature_def out 'simulatedMaxSpeed'))
        (requirement_usage composite 'maxSpeedRequirement' :>> 'EVSample::VehicleAnalysis::vehicleRequirement'[requirement_usage] : 'EVSample::MaxSpeedRequirement'[requirement_def])
        (objective_membership composite 'maxSpeedAnalysisObjective'
          (require_constraint_usage composite 'maxSpeedRequirement'
            (attribute_usage :>> 'actualMaxSpeed'[unresolved]
              (feature_value (=))))))
      (part_usage 'vehicle' : 'EVSample::Vehicle'[part_def]
        (attribute_usage composite :>> 'EVSample::Vehicle::mass'[attribute_usage]
          (feature_value (default =)))
        (attribute_usage composite 'airFrictionCoefficient'
          (feature_value (=)))
        (attribute_usage composite 'efficiency')
        (action_usage composite 'vehicleBehavior' : 'ContinuousStateSpaceDynamics'[unresolved]
          (reference_usage in reference 'input' : 'EVSample::Vehicle::VehicleInput'[attribute_def])
          (reference_usage out reference 'output' : 'EVSample::Vehicle::VehicleOutput'[attribute_def])
          (reference_usage reference :>> 'stateSpace'[unresolved] : 'EVSample::Vehicle::VehicleState'[attribute_def]))
        (part_usage composite 'battery' : 'EVSample::Battery'[part_def]
          (reference_usage reference :>> 'EVSample::Battery::baseVoltage'[attribute_usage]
            (feature_value (=)))
          (reference_usage reference :>> 'EVSample::Battery::capacity'[attribute_usage]
            (feature_value (=)))
          (reference_usage reference :>> 'EVSample::Battery::socInit'[attribute_usage]
            (feature_value (=)))
          (reference_usage reference :>> 'EVSample::Battery::internalResistance'[attribute_usage]
            (feature_value (=)))
          (action_usage composite 'batteryBehavior' : 'ContinuousStateSpaceDynamics'[unresolved]
            (reference_usage in reference 'input' : 'EVSample::Battery::BatteryInput'[attribute_def])
            (reference_usage out reference 'output' : 'EVSample::Battery::BatteryOutput'[attribute_def])
            (reference_usage reference :>> 'stateSpace'[unresolved] : 'EVSample::Battery::BatteryState'[attribute_def])))
        (flow_usage composite 'battery')
        (part_usage composite 'motor' : 'EVSample::Motor'[part_def]
          (reference_usage reference :>> 'EVSample::Motor::motR'[attribute_usage]
            (feature_value (=)))
          (reference_usage reference :>> 'EVSample::Motor::motL'[attribute_usage]
            (feature_value (=)))
          (action_usage composite 'motorBehavior' : 'ContinuousStateSpaceDynamics'[unresolved]
            (reference_usage in reference 'input' : 'EVSample::Motor::MotorInput'[attribute_def])
            (reference_usage out reference 'output' : 'EVSample::Motor::MotorOutput'[attribute_def])
            (reference_usage reference :>> 'stateSpace'[unresolved] : 'EVSample::Motor::MotorState'[attribute_def])))
        (flow_usage composite 'motor')
        (part_usage composite 'tire' : 'EVSample::Tire'[part_def]
          (reference_usage reference :>> 'EVSample::Tire::moment'[attribute_usage]
            (feature_value (default =)))
          (reference_usage reference :>> 'EVSample::Tire::radius'[attribute_usage]
            (feature_value (default =)))
          (action_usage composite 'tireBehavior' : 'ContinuousStateSpaceDynamics'[unresolved]
            (reference_usage in reference 'input' : 'EVSample::Tire::TireInput'[attribute_def])
            (reference_usage out reference 'output' : 'EVSample::Tire::TireOutput'[attribute_def])))
        (flow_usage composite 'tire')
        (flow_usage composite 'tire'))
      (part_usage 'vehicle_compact' :> 'EVSample::vehicle'[part_usage]
        (attribute_usage composite :>> ''[attribute_usage]
          (feature_value (=)))
        (part_usage composite :>> 'EVSample::vehicle::tire'[part_usage]
          (reference_usage reference :>> ''[reference_usage]
            (feature_value (=)))
          (reference_usage reference :>> ''[reference_usage]
            (feature_value (=)))))
      (part_usage 'smallEVRangeContext'
        (requirement_usage composite 'smallEVRequirement' : 'EVSample::VehicleRequirement'[requirement_def]
          (documentation)
          (subject_membership in :>> 'EVSample::VehicleRequirement::vehicle'[subject_membership]
            (feature_value (=)))
          (assume_constraint_usage composite
            (result_expr_membership)))
        (analysis_case_usage composite 'smallEVAnalysis' : 'EVSample::VehicleAnalysis'[analysis_case_def]
          (subject_membership in :>> 'EVSample::VehicleAnalysis::vehicle'[subject_membership] :> 'EVSample::vehicle_compact'[part_usage])
          (requirement_usage composite :>> 'EVSample::VehicleAnalysis::vehicleRequirement'[requirement_usage]
            (feature_value (=))))
        (requirement_usage composite 'rangeRequirementSmall' :> 'EVSample::smallEVRangeContext::smallEVRequirement'[requirement_usage] : 'EVSample::RangeRequirement'[requirement_def]
          (documentation)
          (attribute_usage composite :>> 'EVSample::RangeRequirement::requiredRange'[attribute_usage]
            (feature_value (=))))
        (analysis_case_usage composite 'rangeAnalysisSmall' :> 'EVSample::smallEVRangeContext::smallEVAnalysis'[analysis_case_usage] : 'EVSample::RangeAnalysis'[analysis_case_def]
          (requirement_usage composite :>> 'EVSample::RangeAnalysis::rangeRequirement'[requirement_usage]
            (feature_value (=)))
          (return_parameter_membership
            (feature_def out 'simulatedRange'
              (feature_value (=)))))
        (requirement_usage composite 'efficiencyRequirementSmall' :> 'EVSample::smallEVRangeContext::smallEVRequirement'[requirement_usage] : 'EVSample::EfficiencyRequirement'[requirement_def]
          (documentation)
          (attribute_usage composite :>> 'EVSample::EfficiencyRequirement::requiredEfficiency'[attribute_usage]
            (feature_value (=))))
        (analysis_case_usage composite 'efficiencyAnalysisSmall' :> 'EVSample::smallEVRangeContext::smallEVAnalysis'[analysis_case_usage] : 'EVSample::EfficiencyAnalysis'[analysis_case_def]
          (requirement_usage composite :>> 'EVSample::EfficiencyAnalysis::efficiencyRequirement'[requirement_usage]
            (feature_value (=)))
          (return_parameter_membership
            (feature_def out 'simulatedEfficiency'
              (feature_value (=)))))
        (requirement_usage composite 'maxSpeedRequirementSmall' :> 'EVSample::smallEVRangeContext::smallEVRequirement'[requirement_usage] : 'EVSample::MaxSpeedRequirement'[requirement_def]
          (documentation)
          (attribute_usage composite :>> 'EVSample::MaxSpeedRequirement::requiredMaxSpeed'[attribute_usage]
            (feature_value (=))))
        (analysis_case_usage composite 'maxSpeedAnalysisSmall' :> 'EVSample::smallEVRangeContext::smallEVAnalysis'[analysis_case_usage] : 'EVSample::MaxSpeedAnalysis'[analysis_case_def]
          (subject_membership in :>> ''[subject_membership][implied])
          (requirement_usage composite :>> 'EVSample::MaxSpeedAnalysis::maxSpeedRequirement'[requirement_usage]
            (feature_value (=)))
          (reference_usage out reference 'voltage' :> 'ISQ::electricPotential'[unresolved]
            (feature_value (=)))
          (return_parameter_membership
            (feature_def out 'simulatedMaxSpeed'
              (feature_value (=))))))
      (part_usage 'vehicle_large' :> 'EVSample::vehicle'[part_usage]
        (attribute_usage composite :>> ''[attribute_usage]
          (feature_value (=)))
        (part_usage composite :>> 'EVSample::vehicle::tire'[part_usage]
          (reference_usage reference :>> ''[reference_usage]
            (feature_value (=)))
          (reference_usage reference :>> ''[reference_usage]
            (feature_value (=)))))
      (part_usage 'largeEVRangeContext'
        (requirement_usage composite 'largeEVRequirement' : 'EVSample::VehicleRequirement'[requirement_def]
          (documentation)
          (subject_membership in :>> 'EVSample::VehicleRequirement::vehicle'[subject_membership]
            (feature_value (=)))
          (assume_constraint_usage composite
            (result_expr_membership)))
        (analysis_case_usage composite 'largeEVAnalysis' : 'EVSample::VehicleAnalysis'[analysis_case_def]
          (subject_membership in :>> 'EVSample::VehicleAnalysis::vehicle'[subject_membership] :> 'EVSample::vehicle_large'[part_usage])
          (requirement_usage composite :>> 'EVSample::VehicleAnalysis::vehicleRequirement'[requirement_usage]
            (feature_value (=))))
        (requirement_usage composite 'rangeRequirementLarge' :> 'EVSample::largeEVRangeContext::largeEVRequirement'[requirement_usage] : 'EVSample::RangeRequirement'[requirement_def]
          (documentation)
          (attribute_usage composite :>> 'EVSample::RangeRequirement::requiredRange'[attribute_usage]
            (feature_value (=))))
        (analysis_case_usage composite 'rangeAnalysisLarge' :> 'EVSample::largeEVRangeContext::largeEVAnalysis'[analysis_case_usage] : 'EVSample::RangeAnalysis'[analysis_case_def]
          (requirement_usage composite :>> 'EVSample::RangeAnalysis::rangeRequirement'[requirement_usage]
            (feature_value (=)))
          (return_parameter_membership
            (feature_def out 'simulatedRange'
              (feature_value (=)))))
        (requirement_usage composite 'efficiencyRequirementLarge' :> 'EVSample::largeEVRangeContext::largeEVRequirement'[requirement_usage] : 'EVSample::EfficiencyRequirement'[requirement_def]
          (documentation)
          (attribute_usage composite :>> 'EVSample::EfficiencyRequirement::requiredEfficiency'[attribute_usage]
            (feature_value (=))))
        (analysis_case_usage composite 'efficiencyAnalysisLarge' :> 'EVSample::largeEVRangeContext::largeEVAnalysis'[analysis_case_usage] : 'EVSample::EfficiencyAnalysis'[analysis_case_def]
          (requirement_usage composite :>> 'EVSample::EfficiencyAnalysis::efficiencyRequirement'[requirement_usage]
            (feature_value (=)))
          (return_parameter_membership
            (feature_def out 'simulatedEfficiency'
              (feature_value (=)))))
        (requirement_usage composite 'maxSpeedRequirementLarge' :> 'EVSample::largeEVRangeContext::largeEVRequirement'[requirement_usage] : 'EVSample::MaxSpeedRequirement'[requirement_def]
          (documentation)
          (attribute_usage composite :>> 'EVSample::MaxSpeedRequirement::requiredMaxSpeed'[attribute_usage]
            (feature_value (=))))
        (analysis_case_usage composite 'maxSpeedAnalysisLarge' :> 'EVSample::largeEVRangeContext::largeEVAnalysis'[analysis_case_usage] : 'EVSample::MaxSpeedAnalysis'[analysis_case_def]
          (subject_membership in :>> ''[subject_membership][implied])
          (requirement_usage composite :>> 'EVSample::MaxSpeedAnalysis::maxSpeedRequirement'[requirement_usage]
            (feature_value (=)))
          (reference_usage out reference 'voltage'
            (feature_value (=)))
          (return_parameter_membership
            (feature_def out 'simulatedMaxSpeed'
              (feature_value (=)))))))))
~~~
