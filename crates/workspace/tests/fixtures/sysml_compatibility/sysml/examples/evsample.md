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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "EVSample"))) (name "EVSample") (declared-name "EVSample")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "EVSample::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "EVSample::*#import"))) (name "*") (declared-name "*"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "EVSample::Battery"))) (name "Battery") (declared-name "Battery") (declared)
          (contains
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "EVSample::Battery::BatteryInput"))) (name "BatteryInput") (declared-name "BatteryInput") (declared (properties (ordered false) (unique true))) (effective (featuring-type (node (document "d0") (qualified-name "EVSample::Battery")))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "EVSample::Battery::BatteryOutput"))) (name "BatteryOutput") (declared-name "BatteryOutput") (declared (properties (ordered false) (unique true))) (effective (featuring-type (node (document "d0") (qualified-name "EVSample::Battery")))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "EVSample::Battery::BatteryState"))) (name "BatteryState") (declared-name "BatteryState") (declared (properties (ordered false) (unique true))) (effective (featuring-type (node (document "d0") (qualified-name "EVSample::Battery")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "EVSample::Battery::baseVoltage"))) (name "baseVoltage") (declared-name "baseVoltage") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "EVSample::Battery")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "EVSample::Battery::capacity"))) (name "capacity") (declared-name "capacity") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "EVSample::Battery")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "EVSample::Battery::internalResistance"))) (name "internalResistance") (declared-name "internalResistance") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "EVSample::Battery")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "EVSample::Battery::socInit"))) (name "socInit") (declared-name "socInit") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "EVSample::Battery")))))
          )
        )
        (element (kind "analysis def") (id (node (document "d0") (qualified-name "EVSample::EfficiencyAnalysis"))) (name "EfficiencyAnalysis") (declared-name "EfficiencyAnalysis")
          (contains
            (element (kind "objective") (id (node (document "d0") (qualified-name "EVSample::EfficiencyAnalysis::efficiencyAnalysisObjective"))) (name "efficiencyAnalysisObjective") (declared-name "efficiencyAnalysisObjective") (effective (featuring-type (node (document "d0") (qualified-name "EVSample::EfficiencyAnalysis")))))
            (element (kind "requirement") (id (node (document "d0") (qualified-name "EVSample::EfficiencyAnalysis::efficiencyRequirement"))) (name "efficiencyRequirement") (declared-name "efficiencyRequirement") (effective (featuring-type (node (document "d0") (qualified-name "EVSample::EfficiencyAnalysis")))))
            (element (kind "analysis result") (id (node (document "d0") (qualified-name "EVSample::EfficiencyAnalysis::simulatedEfficiency"))) (name "simulatedEfficiency") (declared-name "simulatedEfficiency") (effective (featuring-type (node (document "d0") (qualified-name "EVSample::EfficiencyAnalysis")))))
          )
        )
        (element (kind "requirement def") (id (node (document "d0") (qualified-name "EVSample::EfficiencyRequirement"))) (name "EfficiencyRequirement") (declared-name "EfficiencyRequirement")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "EVSample::EfficiencyRequirement::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "EVSample::EfficiencyRequirement")))))
            (element (kind "require constraint") (id (node (document "d0") (qualified-name "EVSample::EfficiencyRequirement::_requireConstraint_0"))) (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (effective (featuring-type (node (document "d0") (qualified-name "EVSample::EfficiencyRequirement")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "EVSample::EfficiencyRequirement::actualEfficiency"))) (name "actualEfficiency") (declared-name "actualEfficiency") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "EVSample::EfficiencyRequirement")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "EVSample::EfficiencyRequirement::requiredEfficiency"))) (name "requiredEfficiency") (declared-name "requiredEfficiency") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "EVSample::EfficiencyRequirement")))))
          )
        )
        (element (kind "analysis def") (id (node (document "d0") (qualified-name "EVSample::MaxSpeedAnalysis"))) (name "MaxSpeedAnalysis") (declared-name "MaxSpeedAnalysis")
          (contains
            (element (kind "objective") (id (node (document "d0") (qualified-name "EVSample::MaxSpeedAnalysis::maxSpeedAnalysisObjective"))) (name "maxSpeedAnalysisObjective") (declared-name "maxSpeedAnalysisObjective") (effective (featuring-type (node (document "d0") (qualified-name "EVSample::MaxSpeedAnalysis")))))
            (element (kind "requirement") (id (node (document "d0") (qualified-name "EVSample::MaxSpeedAnalysis::maxSpeedRequirement"))) (name "maxSpeedRequirement") (declared-name "maxSpeedRequirement") (effective (featuring-type (node (document "d0") (qualified-name "EVSample::MaxSpeedAnalysis")))))
            (element (kind "analysis result") (id (node (document "d0") (qualified-name "EVSample::MaxSpeedAnalysis::simulatedMaxSpeed"))) (name "simulatedMaxSpeed") (declared-name "simulatedMaxSpeed") (effective (featuring-type (node (document "d0") (qualified-name "EVSample::MaxSpeedAnalysis")))))
          )
        )
        (element (kind "requirement def") (id (node (document "d0") (qualified-name "EVSample::MaxSpeedRequirement"))) (name "MaxSpeedRequirement") (declared-name "MaxSpeedRequirement")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "EVSample::MaxSpeedRequirement::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "EVSample::MaxSpeedRequirement")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "EVSample::MaxSpeedRequirement::actualMaxSpeed"))) (name "actualMaxSpeed") (declared-name "actualMaxSpeed") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "EVSample::MaxSpeedRequirement")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "EVSample::MaxSpeedRequirement::requiredMaxSpeed"))) (name "requiredMaxSpeed") (declared-name "requiredMaxSpeed") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "EVSample::MaxSpeedRequirement")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "EVSample::Motor"))) (name "Motor") (declared-name "Motor") (declared)
          (contains
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "EVSample::Motor::MotorInput"))) (name "MotorInput") (declared-name "MotorInput") (declared (properties (ordered false) (unique true))) (effective (featuring-type (node (document "d0") (qualified-name "EVSample::Motor")))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "EVSample::Motor::MotorOutput"))) (name "MotorOutput") (declared-name "MotorOutput") (declared (properties (ordered false) (unique true))) (effective (featuring-type (node (document "d0") (qualified-name "EVSample::Motor")))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "EVSample::Motor::MotorState"))) (name "MotorState") (declared-name "MotorState") (declared (properties (ordered false) (unique true))) (effective (featuring-type (node (document "d0") (qualified-name "EVSample::Motor")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "EVSample::Motor::motL"))) (name "motL") (declared-name "motL") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "EVSample::Motor")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "EVSample::Motor::motR"))) (name "motR") (declared-name "motR") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "EVSample::Motor")))))
          )
        )
        (element (kind "analysis def") (id (node (document "d0") (qualified-name "EVSample::RangeAnalysis"))) (name "RangeAnalysis") (declared-name "RangeAnalysis")
          (contains
            (element (kind "objective") (id (node (document "d0") (qualified-name "EVSample::RangeAnalysis::rangeAnalysisObjective"))) (name "rangeAnalysisObjective") (declared-name "rangeAnalysisObjective") (effective (featuring-type (node (document "d0") (qualified-name "EVSample::RangeAnalysis")))))
            (element (kind "requirement") (id (node (document "d0") (qualified-name "EVSample::RangeAnalysis::rangeRequirement"))) (name "rangeRequirement") (declared-name "rangeRequirement") (effective (featuring-type (node (document "d0") (qualified-name "EVSample::RangeAnalysis")))))
            (element (kind "analysis result") (id (node (document "d0") (qualified-name "EVSample::RangeAnalysis::simulatedRange"))) (name "simulatedRange") (declared-name "simulatedRange") (effective (featuring-type (node (document "d0") (qualified-name "EVSample::RangeAnalysis")))))
          )
        )
        (element (kind "requirement def") (id (node (document "d0") (qualified-name "EVSample::RangeRequirement"))) (name "RangeRequirement") (declared-name "RangeRequirement")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "EVSample::RangeRequirement::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "EVSample::RangeRequirement")))))
            (element (kind "require constraint") (id (node (document "d0") (qualified-name "EVSample::RangeRequirement::_requireConstraint_0"))) (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (effective (featuring-type (node (document "d0") (qualified-name "EVSample::RangeRequirement")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "EVSample::RangeRequirement::actualRange"))) (name "actualRange") (declared-name "actualRange") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "EVSample::RangeRequirement")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "EVSample::RangeRequirement::requiredRange"))) (name "requiredRange") (declared-name "requiredRange") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "EVSample::RangeRequirement")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "EVSample::Tire"))) (name "Tire") (declared-name "Tire") (declared)
          (contains
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "EVSample::Tire::TireInput"))) (name "TireInput") (declared-name "TireInput") (declared (properties (ordered false) (unique true))) (effective (featuring-type (node (document "d0") (qualified-name "EVSample::Tire")))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "EVSample::Tire::TireOutput"))) (name "TireOutput") (declared-name "TireOutput") (declared (properties (ordered false) (unique true))) (effective (featuring-type (node (document "d0") (qualified-name "EVSample::Tire")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "EVSample::Tire::moment"))) (name "moment") (declared-name "moment") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "EVSample::Tire")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "EVSample::Tire::radius"))) (name "radius") (declared-name "radius") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "EVSample::Tire")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "EVSample::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared)
          (contains
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "EVSample::Vehicle::VehicleInput"))) (name "VehicleInput") (declared-name "VehicleInput") (declared (properties (ordered false) (unique true))) (effective (featuring-type (node (document "d0") (qualified-name "EVSample::Vehicle")))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "EVSample::Vehicle::VehicleOutput"))) (name "VehicleOutput") (declared-name "VehicleOutput") (declared (properties (ordered false) (unique true))) (effective (featuring-type (node (document "d0") (qualified-name "EVSample::Vehicle")))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "EVSample::Vehicle::VehicleState"))) (name "VehicleState") (declared-name "VehicleState") (declared (properties (ordered false) (unique true))) (effective (featuring-type (node (document "d0") (qualified-name "EVSample::Vehicle")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "EVSample::Vehicle::mass"))) (name "mass") (declared-name "mass") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "EVSample::Vehicle")))))
          )
        )
        (element (kind "analysis def") (id (node (document "d0") (qualified-name "EVSample::VehicleAnalysis"))) (name "VehicleAnalysis") (declared-name "VehicleAnalysis")
          (contains
            (element (kind "subject") (id (node (document "d0") (qualified-name "EVSample::VehicleAnalysis::vehicle"))) (name "vehicle") (declared-name "vehicle") (effective (featuring-type (node (document "d0") (qualified-name "EVSample::VehicleAnalysis")))))
            (element (kind "requirement") (id (node (document "d0") (qualified-name "EVSample::VehicleAnalysis::vehicleRequirement"))) (name "vehicleRequirement") (declared-name "vehicleRequirement") (effective (featuring-type (node (document "d0") (qualified-name "EVSample::VehicleAnalysis")))))
          )
        )
        (element (kind "requirement def") (id (node (document "d0") (qualified-name "EVSample::VehicleRequirement"))) (name "VehicleRequirement") (declared-name "VehicleRequirement")
          (contains
            (element (kind "subject") (id (node (document "d0") (qualified-name "EVSample::VehicleRequirement::vehicle"))) (name "vehicle") (declared-name "vehicle") (effective (featuring-type (node (document "d0") (qualified-name "EVSample::VehicleRequirement")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "EVSample::ampere hour"))) (name "ampere hour") (declared-name "ampere hour") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "A")) (expression (kind "featureReference") (reference "h")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "EVSample::ampere hour"))) (role feature-value))))
        (element (kind "part") (id (node (document "d0") (qualified-name "EVSample::largeEVRangeContext"))) (name "largeEVRangeContext") (declared-name "largeEVRangeContext") (declared (properties (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "EVSample::smallEVRangeContext"))) (name "smallEVRangeContext") (declared-name "smallEVRangeContext") (declared (properties (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "EVSample::vehicle"))) (name "vehicle") (declared-name "vehicle") (declared (properties (ordered false)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "EVSample::vehicle::airFrictionCoefficient"))) (name "airFrictionCoefficient") (declared-name "airFrictionCoefficient") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "realLiteral") (literal "0.2")))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "EVSample::Vehicle"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "EVSample::vehicle::airFrictionCoefficient"))) (role feature-value))))
            (element (kind "part") (id (node (document "d0") (qualified-name "EVSample::vehicle::battery"))) (name "battery") (declared-name "battery") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "EVSample::Vehicle"))))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "EVSample::vehicle::battery::baseVoltage"))) (name "baseVoltage") (declared-name "baseVoltage") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 300)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "V")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "EVSample::Battery"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "EVSample::vehicle::battery::baseVoltage"))) (role feature-value))))
                (element (kind "action") (id (node (document "d0") (qualified-name "EVSample::vehicle::battery::batteryBehavior"))) (name "batteryBehavior") (declared-name "batteryBehavior") (declared) (effective (featuring-type (node (document "d0") (qualified-name "EVSample::Battery"))))
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "EVSample::vehicle::battery::batteryBehavior::input"))) (name "input") (declared-name "input") (effective (featuring-type (node (document "d0") (qualified-name "EVSample::Battery")))))
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "EVSample::vehicle::battery::batteryBehavior::output"))) (name "output") (declared-name "output") (effective (featuring-type (node (document "d0") (qualified-name "EVSample::Battery")))))
                  )
                )
                (element (kind "attribute") (id (node (document "d0") (qualified-name "EVSample::vehicle::battery::capacity"))) (name "capacity") (declared-name "capacity") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 50)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "A⋅h")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "EVSample::Battery"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "EVSample::vehicle::battery::capacity"))) (role feature-value))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "EVSample::vehicle::battery::internalResistance"))) (name "internalResistance") (declared-name "internalResistance") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "realLiteral") (literal "1.8")) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "Ω")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "EVSample::Battery"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "EVSample::vehicle::battery::internalResistance"))) (role feature-value))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "EVSample::vehicle::battery::socInit"))) (name "socInit") (declared-name "socInit") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "realLiteral") (literal "0.8")))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "EVSample::Battery"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "EVSample::vehicle::battery::socInit"))) (role feature-value))))
              )
            )
            (element (kind "attribute") (id (node (document "d0") (qualified-name "EVSample::vehicle::efficiency"))) (name "efficiency") (declared-name "efficiency") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "EVSample::Vehicle")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "EVSample::vehicle::mass"))) (name "mass") (declared-name "mass") (declared (properties (ordered false) (unique true)) (feature-value (kind default) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 1000)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "kg")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "EVSample::Vehicle")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "EVSample::vehicle::motor"))) (name "motor") (declared-name "motor") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "EVSample::Vehicle"))))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "EVSample::vehicle::motor::motL"))) (name "motL") (declared-name "motL") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "realLiteral") (literal "0.2")) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "H")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "EVSample::Motor"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "EVSample::vehicle::motor::motL"))) (role feature-value))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "EVSample::vehicle::motor::motR"))) (name "motR") (declared-name "motR") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 4)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "Ω")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "EVSample::Motor"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "EVSample::vehicle::motor::motR"))) (role feature-value))))
                (element (kind "action") (id (node (document "d0") (qualified-name "EVSample::vehicle::motor::motorBehavior"))) (name "motorBehavior") (declared-name "motorBehavior") (declared) (effective (featuring-type (node (document "d0") (qualified-name "EVSample::Motor"))))
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "EVSample::vehicle::motor::motorBehavior::input"))) (name "input") (declared-name "input") (effective (featuring-type (node (document "d0") (qualified-name "EVSample::Motor")))))
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "EVSample::vehicle::motor::motorBehavior::output"))) (name "output") (declared-name "output") (effective (featuring-type (node (document "d0") (qualified-name "EVSample::Motor")))))
                  )
                )
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "EVSample::vehicle::tire"))) (name "tire") (declared-name "tire") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "EVSample::Vehicle"))))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "EVSample::vehicle::tire::moment"))) (name "moment") (declared-name "moment") (declared (properties (ordered false) (unique true)) (feature-value (kind default) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 300)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "kg⋅m²")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "EVSample::Tire")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "EVSample::vehicle::tire::radius"))) (name "radius") (declared-name "radius") (declared (properties (ordered false) (unique true)) (feature-value (kind default) (expression (kind "literalWithUnit") (children (expression (kind "realLiteral") (literal "0.7")) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "m")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "EVSample::Tire")))))
                (element (kind "action") (id (node (document "d0") (qualified-name "EVSample::vehicle::tire::tireBehavior"))) (name "tireBehavior") (declared-name "tireBehavior") (declared) (effective (featuring-type (node (document "d0") (qualified-name "EVSample::Tire"))))
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "EVSample::vehicle::tire::tireBehavior::input"))) (name "input") (declared-name "input") (effective (featuring-type (node (document "d0") (qualified-name "EVSample::Tire")))))
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "EVSample::vehicle::tire::tireBehavior::output"))) (name "output") (declared-name "output") (effective (featuring-type (node (document "d0") (qualified-name "EVSample::Tire")))))
                  )
                )
              )
            )
            (element (kind "action") (id (node (document "d0") (qualified-name "EVSample::vehicle::vehicleBehavior"))) (name "vehicleBehavior") (declared-name "vehicleBehavior") (declared) (effective (featuring-type (node (document "d0") (qualified-name "EVSample::Vehicle"))))
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "EVSample::vehicle::vehicleBehavior::input"))) (name "input") (declared-name "input") (effective (featuring-type (node (document "d0") (qualified-name "EVSample::Vehicle")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "EVSample::vehicle::vehicleBehavior::output"))) (name "output") (declared-name "output") (effective (featuring-type (node (document "d0") (qualified-name "EVSample::Vehicle")))))
              )
            )
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "EVSample::vehicle_compact"))) (name "vehicle_compact") (declared-name "vehicle_compact") (declared (properties (ordered false)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "EVSample::vehicle_compact::mass"))) (name "mass") (declared-name "mass") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 800)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "kg")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "EVSample::vehicle_compact::mass"))) (role feature-value))))
            (element (kind "part") (id (node (document "d0") (qualified-name "EVSample::vehicle_compact::tire"))) (name "tire") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "EVSample::vehicle_compact::tire::moment"))) (name "moment") (declared-name "moment") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 200)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "kg⋅m²")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "EVSample::vehicle_compact::tire::moment"))) (role feature-value))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "EVSample::vehicle_compact::tire::radius"))) (name "radius") (declared-name "radius") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "realLiteral") (literal "0.5")) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "m")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "EVSample::vehicle_compact::tire::radius"))) (role feature-value))))
              )
            )
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "EVSample::vehicle_large"))) (name "vehicle_large") (declared-name "vehicle_large") (declared (properties (ordered false)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "EVSample::vehicle_large::mass"))) (name "mass") (declared-name "mass") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 1100)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "kg")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "EVSample::vehicle_large::mass"))) (role feature-value))))
            (element (kind "part") (id (node (document "d0") (qualified-name "EVSample::vehicle_large::tire"))) (name "tire") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "EVSample::vehicle_large::tire::moment"))) (name "moment") (declared-name "moment") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 300)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "kg⋅m²")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "EVSample::vehicle_large::tire::moment"))) (role feature-value))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "EVSample::vehicle_large::tire::radius"))) (name "radius") (declared-name "radius") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "realLiteral") (literal "0.7")) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "m")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "EVSample::vehicle_large::tire::radius"))) (role feature-value))))
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "EVSample::EfficiencyRequirement::_documentation"))) (to (node (document "d0") (qualified-name "EVSample::EfficiencyRequirement"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "EVSample::MaxSpeedRequirement::_documentation"))) (to (node (document "d0") (qualified-name "EVSample::MaxSpeedRequirement"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "EVSample::RangeRequirement::_documentation"))) (to (node (document "d0") (qualified-name "EVSample::RangeRequirement"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "EVSample::vehicle::tire::tireBehavior::output"))) (to (node (document "d0") (qualified-name "EVSample::vehicle::motor::motorBehavior::input"))) (flow (source-expression "tire::tireBehavior::output") (target-expression "motor::motorBehavior::input")))
    (flow (status resolved) (from (node (document "d0") (qualified-name "EVSample::vehicle::tire::tireBehavior::output"))) (to (node (document "d0") (qualified-name "EVSample::vehicle::vehicleBehavior::input"))) (flow (source-expression "tire::tireBehavior::output") (target-expression "vehicleBehavior::input")))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "EVSample::vehicle::battery::baseVoltage"))) (to (node (document "d0") (qualified-name "EVSample::Battery::baseVoltage"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "EVSample::vehicle::battery::capacity"))) (to (node (document "d0") (qualified-name "EVSample::Battery::capacity"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "EVSample::vehicle::battery::internalResistance"))) (to (node (document "d0") (qualified-name "EVSample::Battery::internalResistance"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "EVSample::vehicle::battery::socInit"))) (to (node (document "d0") (qualified-name "EVSample::Battery::socInit"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "EVSample::vehicle::mass"))) (to (node (document "d0") (qualified-name "EVSample::Vehicle::mass"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "EVSample::vehicle::motor::motL"))) (to (node (document "d0") (qualified-name "EVSample::Motor::motL"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "EVSample::vehicle::motor::motR"))) (to (node (document "d0") (qualified-name "EVSample::Motor::motR"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "EVSample::vehicle::tire::moment"))) (to (node (document "d0") (qualified-name "EVSample::Tire::moment"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "EVSample::vehicle::tire::radius"))) (to (node (document "d0") (qualified-name "EVSample::Tire::radius"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "EVSample::EfficiencyAnalysis"))) (to (node (document "d0") (qualified-name "EVSample::VehicleAnalysis"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "EVSample::EfficiencyRequirement"))) (to (node (document "d0") (qualified-name "EVSample::VehicleRequirement"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "EVSample::MaxSpeedAnalysis"))) (to (node (document "d0") (qualified-name "EVSample::VehicleAnalysis"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "EVSample::MaxSpeedRequirement"))) (to (node (document "d0") (qualified-name "EVSample::VehicleRequirement"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "EVSample::RangeAnalysis"))) (to (node (document "d0") (qualified-name "EVSample::VehicleAnalysis"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "EVSample::RangeRequirement"))) (to (node (document "d0") (qualified-name "EVSample::VehicleRequirement"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "EVSample::VehicleAnalysis"))) (to (node (document "d0") (qualified-name "EVSample::Vehicle"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "EVSample::VehicleRequirement"))) (to (node (document "d0") (qualified-name "EVSample::Vehicle"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "EVSample::VehicleRequirement"))) (to (node (document "d0") (qualified-name "EVSample::VehicleRequirement::vehicle"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "EVSample::vehicle_compact"))) (to (node (document "d0") (qualified-name "EVSample::vehicle"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "EVSample::vehicle_large"))) (to (node (document "d0") (qualified-name "EVSample::vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "EVSample::EfficiencyAnalysis::efficiencyRequirement"))) (to (node (document "d0") (qualified-name "EVSample::EfficiencyRequirement"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "EVSample::MaxSpeedAnalysis::maxSpeedRequirement"))) (to (node (document "d0") (qualified-name "EVSample::MaxSpeedRequirement"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "EVSample::RangeAnalysis::rangeRequirement"))) (to (node (document "d0") (qualified-name "EVSample::RangeRequirement"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "EVSample::VehicleAnalysis::vehicle"))) (to (node (document "d0") (qualified-name "EVSample::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "EVSample::VehicleAnalysis::vehicleRequirement"))) (to (node (document "d0") (qualified-name "EVSample::VehicleRequirement"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "EVSample::VehicleRequirement::vehicle"))) (to (node (document "d0") (qualified-name "EVSample::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "EVSample::vehicle"))) (to (node (document "d0") (qualified-name "EVSample::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "EVSample::vehicle::battery"))) (to (node (document "d0") (qualified-name "EVSample::Battery"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "EVSample::vehicle::battery::batteryBehavior::input"))) (to (node (document "d0") (qualified-name "EVSample::Battery::BatteryInput"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "EVSample::vehicle::battery::batteryBehavior::output"))) (to (node (document "d0") (qualified-name "EVSample::Battery::BatteryOutput"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "EVSample::vehicle::motor"))) (to (node (document "d0") (qualified-name "EVSample::Motor"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "EVSample::vehicle::motor::motorBehavior::input"))) (to (node (document "d0") (qualified-name "EVSample::Motor::MotorInput"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "EVSample::vehicle::motor::motorBehavior::output"))) (to (node (document "d0") (qualified-name "EVSample::Motor::MotorOutput"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "EVSample::vehicle::tire"))) (to (node (document "d0") (qualified-name "EVSample::Tire"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "EVSample::vehicle::tire::tireBehavior::input"))) (to (node (document "d0") (qualified-name "EVSample::Tire::TireInput"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "EVSample::vehicle::tire::tireBehavior::output"))) (to (node (document "d0") (qualified-name "EVSample::Tire::TireOutput"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "EVSample::vehicle::vehicleBehavior::input"))) (to (node (document "d0") (qualified-name "EVSample::Vehicle::VehicleInput"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "EVSample::vehicle::vehicleBehavior::output"))) (to (node (document "d0") (qualified-name "EVSample::Vehicle::VehicleOutput"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/evsample.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 4) (end 2 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 4) (end 3 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 4) (end 5 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 38) (end 10 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 39) (end 14 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 20 38) (end 20 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 28 8) (end 28 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 32 38) (end 32 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 36 39) (end 36 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 40 38) (end 40 48))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 47 8) (end 47 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 52 36) (end 52 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 57 37) (end 57 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 62 36) (end 62 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 71 35) (end 71 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 76 36) (end 76 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 94 8) (end 94 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 95 8) (end 95 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 101 5) (end 101 41))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 155 8) (end 155 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 163 8) (end 163 193))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 170 12) (end 170 37))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 173 12) (end 173 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 174 12) (end 174 213))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 184 12) (end 184 31))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 185 12) (end 185 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 187 12) (end 187 205))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 197 12) (end 197 47))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 198 12) (end 198 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 199 12) (end 199 157))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 210 8) (end 210 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 210 8) (end 210 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 211 8) (end 211 108))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 212 12) (end 212 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 212 12) (end 212 41))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 213 12) (end 213 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 213 12) (end 213 32))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 265 8) (end 265 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 265 8) (end 265 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 266 8) (end 266 108))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 267 12) (end 267 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 267 12) (end 267 41))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 268 12) (end 268 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 268 12) (end 268 32))
      )
    )
  )
)
~~~
