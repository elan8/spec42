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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "evsample.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 19) (end 2 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 19) (end 3 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 4) (end 5 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 8 26) (end 8 35))
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 27 33) (end 27 55))
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
        (range (start 28 27) (end 28 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 29 30) (end 29 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 30 40) (end 30 55))
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 49 26) (end 49 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 50 26) (end 50 41))
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 68 28) (end 68 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 69 28) (end 69 48))
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 137 36) (end 137 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 138 38) (end 138 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 163 8) (end 163 193))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 164 12) (end 164 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 165 12) (end 165 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 174 12) (end 174 213))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 175 16) (end 175 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 176 16) (end 176 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 187 12) (end 187 205))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 188 16) (end 188 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 189 16) (end 189 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 199 12) (end 199 157))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 200 16) (end 200 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 201 16) (end 201 40))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "77eca7bc7cd516a9002535214fdafc0fd1686c7eb10d60184972c0f981c0cf3c") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "EVSample"))) (kind "package") (name "EVSample") (declared-name "EVSample") (range (start (line 1) (character 0)) (end (line 1) (character 10960))))
    (element (id (node (document "d0") (qualified-name "EVSample::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 4)) (end (line 2) (character 25))) (parent (node (document "d0") (qualified-name "EVSample"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 19)) (end (line 2) (character 21))))))
    (element (id (node (document "d0") (qualified-name "EVSample::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 4)) (end (line 3) (character 47))) (parent (node (document "d0") (qualified-name "EVSample"))) (authored (membership (kind Import) (visibility "private") (import (reference "StateSpaceRepresentation::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 19)) (end (line 3) (character 43))))))
    (element (id (node (document "d0") (qualified-name "EVSample::Battery"))) (kind "part def") (name "Battery") (declared-name "Battery") (range (start (line 26) (character 4)) (end (line 26) (character 578))) (parent (node (document "d0") (qualified-name "EVSample"))))
    (element (id (node (document "d0") (qualified-name "EVSample::Battery::BatteryInput"))) (kind "attribute def") (name "BatteryInput") (declared-name "BatteryInput") (range (start (line 32) (character 8)) (end (line 32) (character 110))) (parent (node (document "d0") (qualified-name "EVSample::Battery"))) (authored (membership (kind Owning)) (relationships (typing (reference "Input") (range (start (line 32) (character 38)) (end (line 32) (character 43)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::Battery::BatteryOutput"))) (kind "attribute def") (name "BatteryOutput") (declared-name "BatteryOutput") (range (start (line 36) (character 8)) (end (line 36) (character 114))) (parent (node (document "d0") (qualified-name "EVSample::Battery"))) (authored (membership (kind Owning)) (relationships (typing (reference "Output") (range (start (line 36) (character 39)) (end (line 36) (character 45)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::Battery::BatteryState"))) (kind "attribute def") (name "BatteryState") (declared-name "BatteryState") (range (start (line 40) (character 8)) (end (line 40) (character 107))) (parent (node (document "d0") (qualified-name "EVSample::Battery"))) (authored (membership (kind Owning)) (relationships (typing (reference "StateSpace") (range (start (line 40) (character 38)) (end (line 40) (character 48)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::Battery::baseVoltage"))) (kind "attribute") (name "baseVoltage") (declared-name "baseVoltage") (range (start (line 27) (character 8)) (end (line 27) (character 56))) (parent (node (document "d0") (qualified-name "EVSample::Battery"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::electricPotential") (range (start (line 27) (character 33)) (end (line 27) (character 55)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::Battery::capacity"))) (kind "attribute") (name "capacity") (declared-name "capacity") (range (start (line 29) (character 8)) (end (line 29) (character 50))) (parent (node (document "d0") (qualified-name "EVSample::Battery"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::electricCharge") (range (start (line 29) (character 30)) (end (line 29) (character 49)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::Battery::internalResistance"))) (kind "attribute") (name "internalResistance") (declared-name "internalResistance") (range (start (line 30) (character 8)) (end (line 30) (character 56))) (parent (node (document "d0") (qualified-name "EVSample::Battery"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::resistance") (range (start (line 30) (character 40)) (end (line 30) (character 55)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::Battery::socInit"))) (kind "attribute") (name "socInit") (declared-name "socInit") (range (start (line 28) (character 8)) (end (line 28) (character 46))) (parent (node (document "d0") (qualified-name "EVSample::Battery"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (typing (reference "ScalarValues::Real") (range (start (line 28) (character 27)) (end (line 28) (character 45)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::EfficiencyAnalysis"))) (kind "analysis def") (name "EfficiencyAnalysis") (declared-name "EfficiencyAnalysis") (range (start (line 123) (character 4)) (end (line 123) (character 378))) (parent (node (document "d0") (qualified-name "EVSample"))) (authored (membership (kind Owning)) (relationships (specializes (reference "VehicleAnalysis") (range (start (line 123) (character 39)) (end (line 123) (character 54)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::EfficiencyAnalysis::efficiencyAnalysisObjective"))) (kind "objective") (name "efficiencyAnalysisObjective") (declared-name "efficiencyAnalysisObjective") (range (start (line 128) (character 8)) (end (line 128) (character 185))) (parent (node (document "d0") (qualified-name "EVSample::EfficiencyAnalysis"))))
    (element (id (node (document "d0") (qualified-name "EVSample::EfficiencyAnalysis::efficiencyRequirement"))) (kind "requirement") (name "efficiencyRequirement") (declared-name "efficiencyRequirement") (range (start (line 126) (character 8)) (end (line 126) (character 89))) (parent (node (document "d0") (qualified-name "EVSample::EfficiencyAnalysis"))) (authored (membership (kind Feature)) (relationships (typing (reference "EfficiencyRequirement") (range none)))))
    (element (id (node (document "d0") (qualified-name "EVSample::EfficiencyAnalysis::simulatedEfficiency"))) (kind "analysis result") (name "simulatedEfficiency") (declared-name "simulatedEfficiency") (range (start (line 124) (character 5)) (end (line 124) (character 32))) (parent (node (document "d0") (qualified-name "EVSample::EfficiencyAnalysis"))))
    (element (id (node (document "d0") (qualified-name "EVSample::EfficiencyRequirement"))) (kind "requirement def") (name "EfficiencyRequirement") (declared-name "EfficiencyRequirement") (range (start (line 115) (character 4)) (end (line 115) (character 294))) (parent (node (document "d0") (qualified-name "EVSample"))) (authored (membership (kind Owning)) (relationships (specializes (reference "VehicleRequirement") (range (start (line 115) (character 45)) (end (line 115) (character 63)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::EfficiencyRequirement::_documentation"))) (kind "documentation") (name "") (range (start (line 115) (character 4)) (end (line 115) (character 294))) (parent (node (document "d0") (qualified-name "EVSample::EfficiencyRequirement"))))
    (element (id (node (document "d0") (qualified-name "EVSample::EfficiencyRequirement::_requireConstraint_0"))) (kind "require constraint") (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (range (start (line 120) (character 8)) (end (line 120) (character 69))) (parent (node (document "d0") (qualified-name "EVSample::EfficiencyRequirement"))))
    (element (id (node (document "d0") (qualified-name "EVSample::EfficiencyRequirement::actualEfficiency"))) (kind "attribute") (name "actualEfficiency") (declared-name "actualEfficiency") (range (start (line 117) (character 8)) (end (line 117) (character 35))) (parent (node (document "d0") (qualified-name "EVSample::EfficiencyRequirement"))))
    (element (id (node (document "d0") (qualified-name "EVSample::EfficiencyRequirement::requiredEfficiency"))) (kind "attribute") (name "requiredEfficiency") (declared-name "requiredEfficiency") (range (start (line 118) (character 8)) (end (line 118) (character 37))) (parent (node (document "d0") (qualified-name "EVSample::EfficiencyRequirement"))))
    (element (id (node (document "d0") (qualified-name "EVSample::MaxSpeedAnalysis"))) (kind "analysis def") (name "MaxSpeedAnalysis") (declared-name "MaxSpeedAnalysis") (range (start (line 141) (character 4)) (end (line 141) (character 362))) (parent (node (document "d0") (qualified-name "EVSample"))) (authored (membership (kind Owning)) (relationships (specializes (reference "VehicleAnalysis") (range (start (line 141) (character 37)) (end (line 141) (character 52)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::MaxSpeedAnalysis::maxSpeedAnalysisObjective"))) (kind "objective") (name "maxSpeedAnalysisObjective") (declared-name "maxSpeedAnalysisObjective") (range (start (line 146) (character 8)) (end (line 146) (character 177))) (parent (node (document "d0") (qualified-name "EVSample::MaxSpeedAnalysis"))))
    (element (id (node (document "d0") (qualified-name "EVSample::MaxSpeedAnalysis::maxSpeedRequirement"))) (kind "requirement") (name "maxSpeedRequirement") (declared-name "maxSpeedRequirement") (range (start (line 144) (character 8)) (end (line 144) (character 85))) (parent (node (document "d0") (qualified-name "EVSample::MaxSpeedAnalysis"))) (authored (membership (kind Feature)) (relationships (typing (reference "MaxSpeedRequirement") (range none)))))
    (element (id (node (document "d0") (qualified-name "EVSample::MaxSpeedAnalysis::simulatedMaxSpeed"))) (kind "analysis result") (name "simulatedMaxSpeed") (declared-name "simulatedMaxSpeed") (range (start (line 142) (character 5)) (end (line 142) (character 30))) (parent (node (document "d0") (qualified-name "EVSample::MaxSpeedAnalysis"))))
    (element (id (node (document "d0") (qualified-name "EVSample::MaxSpeedRequirement"))) (kind "requirement def") (name "MaxSpeedRequirement") (declared-name "MaxSpeedRequirement") (range (start (line 135) (character 4)) (end (line 135) (character 248))) (parent (node (document "d0") (qualified-name "EVSample"))) (authored (membership (kind Owning)) (relationships (specializes (reference "VehicleRequirement") (range (start (line 135) (character 43)) (end (line 135) (character 61)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::MaxSpeedRequirement::_documentation"))) (kind "documentation") (name "") (range (start (line 135) (character 4)) (end (line 135) (character 248))) (parent (node (document "d0") (qualified-name "EVSample::MaxSpeedRequirement"))))
    (element (id (node (document "d0") (qualified-name "EVSample::MaxSpeedRequirement::actualMaxSpeed"))) (kind "attribute") (name "actualMaxSpeed") (declared-name "actualMaxSpeed") (range (start (line 137) (character 8)) (end (line 137) (character 47))) (parent (node (document "d0") (qualified-name "EVSample::MaxSpeedRequirement"))) (authored (relationships (subsetting (reference "ISQ::speed") (range (start (line 137) (character 36)) (end (line 137) (character 46)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::MaxSpeedRequirement::requiredMaxSpeed"))) (kind "attribute") (name "requiredMaxSpeed") (declared-name "requiredMaxSpeed") (range (start (line 138) (character 8)) (end (line 138) (character 49))) (parent (node (document "d0") (qualified-name "EVSample::MaxSpeedRequirement"))) (authored (relationships (subsetting (reference "ISQ::speed") (range (start (line 138) (character 38)) (end (line 138) (character 48)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::Motor"))) (kind "part def") (name "Motor") (declared-name "Motor") (range (start (line 46) (character 4)) (end (line 46) (character 639))) (parent (node (document "d0") (qualified-name "EVSample"))))
    (element (id (node (document "d0") (qualified-name "EVSample::Motor::MotorInput"))) (kind "attribute def") (name "MotorInput") (declared-name "MotorInput") (range (start (line 52) (character 8)) (end (line 52) (character 157))) (parent (node (document "d0") (qualified-name "EVSample::Motor"))) (authored (membership (kind Owning)) (relationships (typing (reference "Input") (range (start (line 52) (character 36)) (end (line 52) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::Motor::MotorOutput"))) (kind "attribute def") (name "MotorOutput") (declared-name "MotorOutput") (range (start (line 57) (character 8)) (end (line 57) (character 155))) (parent (node (document "d0") (qualified-name "EVSample::Motor"))) (authored (membership (kind Owning)) (relationships (typing (reference "Output") (range (start (line 57) (character 37)) (end (line 57) (character 43)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::Motor::MotorState"))) (kind "attribute def") (name "MotorState") (declared-name "MotorState") (range (start (line 62) (character 8)) (end (line 62) (character 113))) (parent (node (document "d0") (qualified-name "EVSample::Motor"))) (authored (membership (kind Owning)) (relationships (typing (reference "StateSpace") (range (start (line 62) (character 36)) (end (line 62) (character 46)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::Motor::motL"))) (kind "attribute") (name "motL") (declared-name "motL") (range (start (line 50) (character 8)) (end (line 50) (character 42))) (parent (node (document "d0") (qualified-name "EVSample::Motor"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::inductance") (range (start (line 50) (character 26)) (end (line 50) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::Motor::motR"))) (kind "attribute") (name "motR") (declared-name "motR") (range (start (line 49) (character 8)) (end (line 49) (character 42))) (parent (node (document "d0") (qualified-name "EVSample::Motor"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::resistance") (range (start (line 49) (character 26)) (end (line 49) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::RangeAnalysis"))) (kind "analysis def") (name "RangeAnalysis") (declared-name "RangeAnalysis") (range (start (line 100) (character 4)) (end (line 100) (character 521))) (parent (node (document "d0") (qualified-name "EVSample"))) (authored (membership (kind Owning)) (relationships (specializes (reference "VehicleAnalysis") (range (start (line 100) (character 34)) (end (line 100) (character 49)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::RangeAnalysis::rangeAnalysisObjective"))) (kind "objective") (name "rangeAnalysisObjective") (declared-name "rangeAnalysisObjective") (range (start (line 105) (character 8)) (end (line 105) (character 334))) (parent (node (document "d0") (qualified-name "EVSample::RangeAnalysis"))))
    (element (id (node (document "d0") (qualified-name "EVSample::RangeAnalysis::rangeRequirement"))) (kind "requirement") (name "rangeRequirement") (declared-name "rangeRequirement") (range (start (line 103) (character 8)) (end (line 103) (character 79))) (parent (node (document "d0") (qualified-name "EVSample::RangeAnalysis"))) (authored (membership (kind Feature)) (relationships (typing (reference "RangeRequirement") (range none)))))
    (element (id (node (document "d0") (qualified-name "EVSample::RangeAnalysis::simulatedRange"))) (kind "analysis result") (name "simulatedRange") (declared-name "simulatedRange") (range (start (line 101) (character 5)) (end (line 101) (character 41))) (parent (node (document "d0") (qualified-name "EVSample::RangeAnalysis"))) (authored (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "EVSample::RangeRequirement"))) (kind "requirement def") (name "RangeRequirement") (declared-name "RangeRequirement") (range (start (line 92) (character 4)) (end (line 92) (character 312))) (parent (node (document "d0") (qualified-name "EVSample"))) (authored (membership (kind Owning)) (relationships (specializes (reference "VehicleRequirement") (range (start (line 92) (character 40)) (end (line 92) (character 58)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::RangeRequirement::_documentation"))) (kind "documentation") (name "") (range (start (line 92) (character 4)) (end (line 92) (character 312))) (parent (node (document "d0") (qualified-name "EVSample::RangeRequirement"))))
    (element (id (node (document "d0") (qualified-name "EVSample::RangeRequirement::_requireConstraint_0"))) (kind "require constraint") (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (range (start (line 97) (character 8)) (end (line 97) (character 59))) (parent (node (document "d0") (qualified-name "EVSample::RangeRequirement"))))
    (element (id (node (document "d0") (qualified-name "EVSample::RangeRequirement::actualRange"))) (kind "attribute") (name "actualRange") (declared-name "actualRange") (range (start (line 94) (character 8)) (end (line 94) (character 44))) (parent (node (document "d0") (qualified-name "EVSample::RangeRequirement"))) (authored (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "EVSample::RangeRequirement::requiredRange"))) (kind "attribute") (name "requiredRange") (declared-name "requiredRange") (range (start (line 95) (character 8)) (end (line 95) (character 46))) (parent (node (document "d0") (qualified-name "EVSample::RangeRequirement"))) (authored (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "EVSample::Tire"))) (kind "part def") (name "Tire") (declared-name "Tire") (range (start (line 67) (character 4)) (end (line 67) (character 412))) (parent (node (document "d0") (qualified-name "EVSample"))))
    (element (id (node (document "d0") (qualified-name "EVSample::Tire::TireInput"))) (kind "attribute def") (name "TireInput") (declared-name "TireInput") (range (start (line 71) (character 8)) (end (line 71) (character 147))) (parent (node (document "d0") (qualified-name "EVSample::Tire"))) (authored (membership (kind Owning)) (relationships (typing (reference "Input") (range (start (line 71) (character 35)) (end (line 71) (character 40)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::Tire::TireOutput"))) (kind "attribute def") (name "TireOutput") (declared-name "TireOutput") (range (start (line 76) (character 8)) (end (line 76) (character 145))) (parent (node (document "d0") (qualified-name "EVSample::Tire"))) (authored (membership (kind Owning)) (relationships (typing (reference "Output") (range (start (line 76) (character 36)) (end (line 76) (character 42)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::Tire::moment"))) (kind "attribute") (name "moment") (declared-name "moment") (range (start (line 69) (character 8)) (end (line 69) (character 49))) (parent (node (document "d0") (qualified-name "EVSample::Tire"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::momentOfInertia") (range (start (line 69) (character 28)) (end (line 69) (character 48)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::Tire::radius"))) (kind "attribute") (name "radius") (declared-name "radius") (range (start (line 68) (character 8)) (end (line 68) (character 40))) (parent (node (document "d0") (qualified-name "EVSample::Tire"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::length") (range (start (line 68) (character 28)) (end (line 68) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 7) (character 4)) (end (line 7) (character 526))) (parent (node (document "d0") (qualified-name "EVSample"))))
    (element (id (node (document "d0") (qualified-name "EVSample::Vehicle::VehicleInput"))) (kind "attribute def") (name "VehicleInput") (declared-name "VehicleInput") (range (start (line 10) (character 8)) (end (line 10) (character 98))) (parent (node (document "d0") (qualified-name "EVSample::Vehicle"))) (authored (membership (kind Owning)) (relationships (typing (reference "Input") (range (start (line 10) (character 38)) (end (line 10) (character 43)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::Vehicle::VehicleOutput"))) (kind "attribute def") (name "VehicleOutput") (declared-name "VehicleOutput") (range (start (line 14) (character 8)) (end (line 14) (character 202))) (parent (node (document "d0") (qualified-name "EVSample::Vehicle"))) (authored (membership (kind Owning)) (relationships (typing (reference "Output") (range (start (line 14) (character 39)) (end (line 14) (character 45)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::Vehicle::VehicleState"))) (kind "attribute def") (name "VehicleState") (declared-name "VehicleState") (range (start (line 20) (character 8)) (end (line 20) (character 155))) (parent (node (document "d0") (qualified-name "EVSample::Vehicle"))) (authored (membership (kind Owning)) (relationships (typing (reference "StateSpace") (range (start (line 20) (character 38)) (end (line 20) (character 48)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::Vehicle::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 8) (character 8)) (end (line 8) (character 36))) (parent (node (document "d0") (qualified-name "EVSample::Vehicle"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::mass") (range (start (line 8) (character 26)) (end (line 8) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::VehicleAnalysis"))) (kind "analysis def") (name "VehicleAnalysis") (declared-name "VehicleAnalysis") (range (start (line 86) (character 4)) (end (line 86) (character 136))) (parent (node (document "d0") (qualified-name "EVSample"))))
    (element (id (node (document "d0") (qualified-name "EVSample::VehicleAnalysis::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (range (start (line 87) (character 8)) (end (line 87) (character 34))) (parent (node (document "d0") (qualified-name "EVSample::VehicleAnalysis"))) (authored (relationships (typing (reference "Vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "EVSample::VehicleAnalysis::vehicleRequirement"))) (kind "requirement") (name "vehicleRequirement") (declared-name "vehicleRequirement") (range (start (line 88) (character 8)) (end (line 88) (character 60))) (parent (node (document "d0") (qualified-name "EVSample::VehicleAnalysis"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleRequirement") (range none)))))
    (element (id (node (document "d0") (qualified-name "EVSample::VehicleRequirement"))) (kind "requirement def") (name "VehicleRequirement") (declared-name "VehicleRequirement") (range (start (line 82) (character 4)) (end (line 82) (character 81))) (parent (node (document "d0") (qualified-name "EVSample"))) (authored (membership (kind Owning)) (relationships (subject (reference "EVSample::VehicleRequirement::vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "EVSample::VehicleRequirement::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (range (start (line 83) (character 8)) (end (line 83) (character 34))) (parent (node (document "d0") (qualified-name "EVSample::VehicleRequirement"))) (authored (relationships (typing (reference "Vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "EVSample::ampere hour"))) (kind "attribute def") (name "ampere hour") (declared-name "ampere hour") (range (start (line 5) (character 4)) (end (line 5) (character 66))) (parent (node (document "d0") (qualified-name "EVSample"))) (authored (membership (kind Owning)) (relationships (typing (reference "ElectricChargeUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "EVSample::largeEVRangeContext"))) (kind "part") (name "largeEVRangeContext") (declared-name "largeEVRangeContext") (range (start (line 272) (character 4)) (end (line 272) (character 2039))) (parent (node (document "d0") (qualified-name "EVSample"))))
    (element (id (node (document "d0") (qualified-name "EVSample::smallEVRangeContext"))) (kind "part") (name "smallEVRangeContext") (declared-name "smallEVRangeContext") (range (start (line 217) (character 4)) (end (line 217) (character 2068))) (parent (node (document "d0") (qualified-name "EVSample"))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (range (start (line 154) (character 4)) (end (line 154) (character 1789))) (parent (node (document "d0") (qualified-name "EVSample"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 154) (character 19)) (end (line 154) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::airFrictionCoefficient"))) (kind "attribute") (name "airFrictionCoefficient") (declared-name "airFrictionCoefficient") (range (start (line 159) (character 8)) (end (line 159) (character 47))) (parent (node (document "d0") (qualified-name "EVSample::vehicle"))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::battery"))) (kind "part") (name "battery") (declared-name "battery") (range (start (line 169) (character 8)) (end (line 169) (character 412))) (parent (node (document "d0") (qualified-name "EVSample::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Battery") (range (start (line 169) (character 22)) (end (line 169) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::battery::baseVoltage"))) (kind "attribute") (name "baseVoltage") (declared-name "baseVoltage") (range (start (line 170) (character 12)) (end (line 170) (character 37))) (parent (node (document "d0") (qualified-name "EVSample::vehicle::battery"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseVoltage") (range (start (line 170) (character 12)) (end (line 170) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::battery::batteryBehavior"))) (kind "action") (name "batteryBehavior") (declared-name "batteryBehavior") (range (start (line 174) (character 12)) (end (line 174) (character 213))) (parent (node (document "d0") (qualified-name "EVSample::vehicle::battery"))) (authored (membership (kind Feature)) (relationships (typing (reference "ContinuousStateSpaceDynamics") (range none)))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::battery::batteryBehavior::input"))) (kind "in out parameter") (name "input") (declared-name "input") (range (start (line 175) (character 16)) (end (line 175) (character 40))) (parent (node (document "d0") (qualified-name "EVSample::vehicle::battery::batteryBehavior"))) (authored (relationships (typing (reference "BatteryInput") (range none)))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::battery::batteryBehavior::output"))) (kind "in out parameter") (name "output") (declared-name "output") (range (start (line 176) (character 16)) (end (line 176) (character 43))) (parent (node (document "d0") (qualified-name "EVSample::vehicle::battery::batteryBehavior"))) (authored (relationships (typing (reference "BatteryOutput") (range none)))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::battery::capacity"))) (kind "attribute") (name "capacity") (declared-name "capacity") (range (start (line 171) (character 12)) (end (line 171) (character 39))) (parent (node (document "d0") (qualified-name "EVSample::vehicle::battery"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "capacity") (range (start (line 171) (character 12)) (end (line 171) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::battery::internalResistance"))) (kind "attribute") (name "internalResistance") (declared-name "internalResistance") (range (start (line 173) (character 12)) (end (line 173) (character 47))) (parent (node (document "d0") (qualified-name "EVSample::vehicle::battery"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "internalResistance") (range (start (line 173) (character 12)) (end (line 173) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::battery::socInit"))) (kind "attribute") (name "socInit") (declared-name "socInit") (range (start (line 172) (character 12)) (end (line 172) (character 30))) (parent (node (document "d0") (qualified-name "EVSample::vehicle::battery"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "socInit") (range (start (line 172) (character 12)) (end (line 172) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::efficiency"))) (kind "attribute") (name "efficiency") (declared-name "efficiency") (range (start (line 161) (character 8)) (end (line 161) (character 29))) (parent (node (document "d0") (qualified-name "EVSample::vehicle"))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 155) (character 8)) (end (line 155) (character 44))) (parent (node (document "d0") (qualified-name "EVSample::vehicle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "mass") (range (start (line 155) (character 22)) (end (line 155) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::motor"))) (kind "part") (name "motor") (declared-name "motor") (range (start (line 183) (character 8)) (end (line 183) (character 307))) (parent (node (document "d0") (qualified-name "EVSample::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Motor") (range (start (line 183) (character 20)) (end (line 183) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::motor::motL"))) (kind "attribute") (name "motL") (declared-name "motL") (range (start (line 185) (character 12)) (end (line 185) (character 30))) (parent (node (document "d0") (qualified-name "EVSample::vehicle::motor"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "motL") (range (start (line 185) (character 12)) (end (line 185) (character 20)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::motor::motR"))) (kind "attribute") (name "motR") (declared-name "motR") (range (start (line 184) (character 12)) (end (line 184) (character 31))) (parent (node (document "d0") (qualified-name "EVSample::vehicle::motor"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "motR") (range (start (line 184) (character 12)) (end (line 184) (character 20)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::motor::motorBehavior"))) (kind "action") (name "motorBehavior") (declared-name "motorBehavior") (range (start (line 187) (character 12)) (end (line 187) (character 205))) (parent (node (document "d0") (qualified-name "EVSample::vehicle::motor"))) (authored (membership (kind Feature)) (relationships (typing (reference "ContinuousStateSpaceDynamics") (range none)))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::motor::motorBehavior::input"))) (kind "in out parameter") (name "input") (declared-name "input") (range (start (line 188) (character 16)) (end (line 188) (character 38))) (parent (node (document "d0") (qualified-name "EVSample::vehicle::motor::motorBehavior"))) (authored (relationships (typing (reference "MotorInput") (range none)))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::motor::motorBehavior::output"))) (kind "in out parameter") (name "output") (declared-name "output") (range (start (line 189) (character 16)) (end (line 189) (character 41))) (parent (node (document "d0") (qualified-name "EVSample::vehicle::motor::motorBehavior"))) (authored (relationships (typing (reference "MotorOutput") (range none)))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::tire"))) (kind "part") (name "tire") (declared-name "tire") (range (start (line 196) (character 8)) (end (line 196) (character 280))) (parent (node (document "d0") (qualified-name "EVSample::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Tire") (range (start (line 196) (character 19)) (end (line 196) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::tire::moment"))) (kind "attribute") (name "moment") (declared-name "moment") (range (start (line 197) (character 12)) (end (line 197) (character 47))) (parent (node (document "d0") (qualified-name "EVSample::vehicle::tire"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "moment") (range (start (line 197) (character 12)) (end (line 197) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::tire::radius"))) (kind "attribute") (name "radius") (declared-name "radius") (range (start (line 198) (character 12)) (end (line 198) (character 38))) (parent (node (document "d0") (qualified-name "EVSample::vehicle::tire"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "radius") (range (start (line 198) (character 12)) (end (line 198) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::tire::tireBehavior"))) (kind "action") (name "tireBehavior") (declared-name "tireBehavior") (range (start (line 199) (character 12)) (end (line 199) (character 157))) (parent (node (document "d0") (qualified-name "EVSample::vehicle::tire"))) (authored (membership (kind Feature)) (relationships (typing (reference "ContinuousStateSpaceDynamics") (range none)))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::tire::tireBehavior::input"))) (kind "in out parameter") (name "input") (declared-name "input") (range (start (line 200) (character 16)) (end (line 200) (character 37))) (parent (node (document "d0") (qualified-name "EVSample::vehicle::tire::tireBehavior"))) (authored (relationships (typing (reference "TireInput") (range none)))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::tire::tireBehavior::output"))) (kind "in out parameter") (name "output") (declared-name "output") (range (start (line 201) (character 16)) (end (line 201) (character 40))) (parent (node (document "d0") (qualified-name "EVSample::vehicle::tire::tireBehavior"))) (authored (relationships (typing (reference "TireOutput") (range none)))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::vehicleBehavior"))) (kind "action") (name "vehicleBehavior") (declared-name "vehicleBehavior") (range (start (line 163) (character 8)) (end (line 163) (character 193))) (parent (node (document "d0") (qualified-name "EVSample::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "ContinuousStateSpaceDynamics") (range none)))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::vehicleBehavior::input"))) (kind "in out parameter") (name "input") (declared-name "input") (range (start (line 164) (character 12)) (end (line 164) (character 36))) (parent (node (document "d0") (qualified-name "EVSample::vehicle::vehicleBehavior"))) (authored (relationships (typing (reference "VehicleInput") (range none)))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::vehicleBehavior::output"))) (kind "in out parameter") (name "output") (declared-name "output") (range (start (line 165) (character 12)) (end (line 165) (character 39))) (parent (node (document "d0") (qualified-name "EVSample::vehicle::vehicleBehavior"))) (authored (relationships (typing (reference "VehicleOutput") (range none)))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle_compact"))) (kind "part") (name "vehicle_compact") (declared-name "vehicle_compact") (range (start (line 209) (character 4)) (end (line 209) (character 190))) (parent (node (document "d0") (qualified-name "EVSample"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle") (range (start (line 209) (character 28)) (end (line 209) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle_compact::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 210) (character 8)) (end (line 210) (character 37))) (parent (node (document "d0") (qualified-name "EVSample::vehicle_compact"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "mass") (range (start (line 210) (character 22)) (end (line 210) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle_compact::tire"))) (kind "part") (name "tire") (range (start (line 211) (character 8)) (end (line 211) (character 108))) (parent (node (document "d0") (qualified-name "EVSample::vehicle_compact"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "tire") (range (start (line 211) (character 17)) (end (line 211) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle_compact::tire::moment"))) (kind "attribute") (name "moment") (declared-name "moment") (range (start (line 212) (character 12)) (end (line 212) (character 41))) (parent (node (document "d0") (qualified-name "EVSample::vehicle_compact::tire"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "moment") (range (start (line 212) (character 12)) (end (line 212) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle_compact::tire::radius"))) (kind "attribute") (name "radius") (declared-name "radius") (range (start (line 213) (character 12)) (end (line 213) (character 32))) (parent (node (document "d0") (qualified-name "EVSample::vehicle_compact::tire"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "radius") (range (start (line 213) (character 12)) (end (line 213) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle_large"))) (kind "part") (name "vehicle_large") (declared-name "vehicle_large") (range (start (line 264) (character 4)) (end (line 264) (character 189))) (parent (node (document "d0") (qualified-name "EVSample"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle") (range (start (line 264) (character 26)) (end (line 264) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle_large::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 265) (character 8)) (end (line 265) (character 38))) (parent (node (document "d0") (qualified-name "EVSample::vehicle_large"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "mass") (range (start (line 265) (character 22)) (end (line 265) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle_large::tire"))) (kind "part") (name "tire") (range (start (line 266) (character 8)) (end (line 266) (character 108))) (parent (node (document "d0") (qualified-name "EVSample::vehicle_large"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "tire") (range (start (line 266) (character 17)) (end (line 266) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle_large::tire::moment"))) (kind "attribute") (name "moment") (declared-name "moment") (range (start (line 267) (character 12)) (end (line 267) (character 41))) (parent (node (document "d0") (qualified-name "EVSample::vehicle_large::tire"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "moment") (range (start (line 267) (character 12)) (end (line 267) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle_large::tire::radius"))) (kind "attribute") (name "radius") (declared-name "radius") (range (start (line 268) (character 12)) (end (line 268) (character 32))) (parent (node (document "d0") (qualified-name "EVSample::vehicle_large::tire"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "radius") (range (start (line 268) (character 12)) (end (line 268) (character 22)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "EVSample::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (range (start (line 2) (character 19)) (end (line 2) (character 21))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "StateSpaceRepresentation::*") (range (start (line 3) (character 19)) (end (line 3) (character 43))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::Battery::BatteryInput"))) (kind featureTyping) (ordinal 0)) (authored-target "Input") (range (start (line 32) (character 38)) (end (line 32) (character 43))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::Battery::BatteryOutput"))) (kind featureTyping) (ordinal 0)) (authored-target "Output") (range (start (line 36) (character 39)) (end (line 36) (character 45))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::Battery::BatteryState"))) (kind featureTyping) (ordinal 0)) (authored-target "StateSpace") (range (start (line 40) (character 38)) (end (line 40) (character 48))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::Battery::baseVoltage"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::electricPotential") (range (start (line 27) (character 33)) (end (line 27) (character 55))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::Battery::capacity"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::electricCharge") (range (start (line 29) (character 30)) (end (line 29) (character 49))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::Battery::internalResistance"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::resistance") (range (start (line 30) (character 40)) (end (line 30) (character 55))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::Battery::socInit"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::Battery::socInit"))) (kind featureTyping) (ordinal 1)) (authored-target "ScalarValues::Real") (range (start (line 28) (character 27)) (end (line 28) (character 45))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::EfficiencyAnalysis"))) (kind specialization) (ordinal 0)) (authored-target "VehicleAnalysis") (range (start (line 123) (character 39)) (end (line 123) (character 54))) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::VehicleAnalysis")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::EfficiencyAnalysis::efficiencyRequirement"))) (kind featureTyping) (ordinal 0)) (authored-target "EfficiencyRequirement") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::EfficiencyRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::EfficiencyRequirement"))) (kind specialization) (ordinal 0)) (authored-target "VehicleRequirement") (range (start (line 115) (character 45)) (end (line 115) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::VehicleRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::MaxSpeedAnalysis"))) (kind specialization) (ordinal 0)) (authored-target "VehicleAnalysis") (range (start (line 141) (character 37)) (end (line 141) (character 52))) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::VehicleAnalysis")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::MaxSpeedAnalysis::maxSpeedRequirement"))) (kind featureTyping) (ordinal 0)) (authored-target "MaxSpeedRequirement") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::MaxSpeedRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::MaxSpeedRequirement"))) (kind specialization) (ordinal 0)) (authored-target "VehicleRequirement") (range (start (line 135) (character 43)) (end (line 135) (character 61))) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::VehicleRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::MaxSpeedRequirement::actualMaxSpeed"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::speed") (range (start (line 137) (character 36)) (end (line 137) (character 46))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::MaxSpeedRequirement::requiredMaxSpeed"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::speed") (range (start (line 138) (character 38)) (end (line 138) (character 48))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::Motor::MotorInput"))) (kind featureTyping) (ordinal 0)) (authored-target "Input") (range (start (line 52) (character 36)) (end (line 52) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::Motor::MotorOutput"))) (kind featureTyping) (ordinal 0)) (authored-target "Output") (range (start (line 57) (character 37)) (end (line 57) (character 43))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::Motor::MotorState"))) (kind featureTyping) (ordinal 0)) (authored-target "StateSpace") (range (start (line 62) (character 36)) (end (line 62) (character 46))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::Motor::motL"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::inductance") (range (start (line 50) (character 26)) (end (line 50) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::Motor::motR"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::resistance") (range (start (line 49) (character 26)) (end (line 49) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::RangeAnalysis"))) (kind specialization) (ordinal 0)) (authored-target "VehicleAnalysis") (range (start (line 100) (character 34)) (end (line 100) (character 49))) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::VehicleAnalysis")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::RangeAnalysis::rangeRequirement"))) (kind featureTyping) (ordinal 0)) (authored-target "RangeRequirement") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::RangeRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::RangeAnalysis::simulatedRange"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::RangeRequirement"))) (kind specialization) (ordinal 0)) (authored-target "VehicleRequirement") (range (start (line 92) (character 40)) (end (line 92) (character 58))) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::VehicleRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::RangeRequirement::actualRange"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::RangeRequirement::requiredRange"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::Tire::TireInput"))) (kind featureTyping) (ordinal 0)) (authored-target "Input") (range (start (line 71) (character 35)) (end (line 71) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::Tire::TireOutput"))) (kind featureTyping) (ordinal 0)) (authored-target "Output") (range (start (line 76) (character 36)) (end (line 76) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::Tire::moment"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::momentOfInertia") (range (start (line 69) (character 28)) (end (line 69) (character 48))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::Tire::radius"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::length") (range (start (line 68) (character 28)) (end (line 68) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::Vehicle::VehicleInput"))) (kind featureTyping) (ordinal 0)) (authored-target "Input") (range (start (line 10) (character 38)) (end (line 10) (character 43))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::Vehicle::VehicleOutput"))) (kind featureTyping) (ordinal 0)) (authored-target "Output") (range (start (line 14) (character 39)) (end (line 14) (character 45))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::Vehicle::VehicleState"))) (kind featureTyping) (ordinal 0)) (authored-target "StateSpace") (range (start (line 20) (character 38)) (end (line 20) (character 48))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::Vehicle::mass"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (range (start (line 8) (character 26)) (end (line 8) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::VehicleAnalysis::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::VehicleAnalysis::vehicleRequirement"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleRequirement") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::VehicleRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::VehicleRequirement"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "EVSample::VehicleRequirement::vehicle") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::VehicleRequirement::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::VehicleRequirement::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::ampere hour"))) (kind featureTyping) (ordinal 0)) (authored-target "ElectricChargeUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 154) (character 19)) (end (line 154) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle"))) (kind flowSource) (ordinal 0)) (authored-target "battery::batteryBehavior::output") (range (start (line 181) (character 13)) (end (line 181) (character 43))) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle::battery::batteryBehavior::output")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle"))) (kind flowSource) (ordinal 1)) (authored-target "motor::motorBehavior::output") (range (start (line 194) (character 13)) (end (line 194) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle::motor::motorBehavior::output")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle"))) (kind flowSource) (ordinal 2)) (authored-target "tire::tireBehavior::output") (range (start (line 205) (character 13)) (end (line 205) (character 37))) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle::tire::tireBehavior::output")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle"))) (kind flowSource) (ordinal 3)) (authored-target "tire::tireBehavior::output") (range (start (line 206) (character 13)) (end (line 206) (character 37))) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle::tire::tireBehavior::output")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle"))) (kind flowTarget) (ordinal 0)) (authored-target "motor::motorBehavior::input") (range (start (line 181) (character 47)) (end (line 181) (character 72))) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle::motor::motorBehavior::input")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle"))) (kind flowTarget) (ordinal 1)) (authored-target "tire::tireBehavior::input") (range (start (line 194) (character 43)) (end (line 194) (character 66))) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle::tire::tireBehavior::input")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle"))) (kind flowTarget) (ordinal 2)) (authored-target "motor::motorBehavior::input") (range (start (line 205) (character 41)) (end (line 205) (character 66))) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle::motor::motorBehavior::input")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle"))) (kind flowTarget) (ordinal 3)) (authored-target "vehicleBehavior::input") (range (start (line 206) (character 41)) (end (line 206) (character 62))) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle::vehicleBehavior::input")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::battery"))) (kind featureTyping) (ordinal 0)) (authored-target "Battery") (range (start (line 169) (character 22)) (end (line 169) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::Battery")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::battery::baseVoltage"))) (kind redefinition) (ordinal 0)) (authored-target "baseVoltage") (range (start (line 170) (character 12)) (end (line 170) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle::battery::baseVoltage")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::battery::batteryBehavior"))) (kind featureTyping) (ordinal 0)) (authored-target "ContinuousStateSpaceDynamics") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::battery::batteryBehavior::input"))) (kind featureTyping) (ordinal 0)) (authored-target "BatteryInput") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::battery::batteryBehavior::output"))) (kind featureTyping) (ordinal 0)) (authored-target "BatteryOutput") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::battery::capacity"))) (kind redefinition) (ordinal 0)) (authored-target "capacity") (range (start (line 171) (character 12)) (end (line 171) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle::battery::capacity")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::battery::internalResistance"))) (kind redefinition) (ordinal 0)) (authored-target "internalResistance") (range (start (line 173) (character 12)) (end (line 173) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle::battery::internalResistance")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::battery::socInit"))) (kind redefinition) (ordinal 0)) (authored-target "socInit") (range (start (line 172) (character 12)) (end (line 172) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle::battery::socInit")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::mass"))) (kind redefinition) (ordinal 0)) (authored-target "mass") (range (start (line 155) (character 22)) (end (line 155) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle::mass")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::motor"))) (kind featureTyping) (ordinal 0)) (authored-target "Motor") (range (start (line 183) (character 20)) (end (line 183) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::Motor")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::motor::motL"))) (kind redefinition) (ordinal 0)) (authored-target "motL") (range (start (line 185) (character 12)) (end (line 185) (character 20))) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle::motor::motL")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::motor::motR"))) (kind redefinition) (ordinal 0)) (authored-target "motR") (range (start (line 184) (character 12)) (end (line 184) (character 20))) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle::motor::motR")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::motor::motorBehavior"))) (kind featureTyping) (ordinal 0)) (authored-target "ContinuousStateSpaceDynamics") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::motor::motorBehavior::input"))) (kind featureTyping) (ordinal 0)) (authored-target "MotorInput") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::motor::motorBehavior::output"))) (kind featureTyping) (ordinal 0)) (authored-target "MotorOutput") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::tire"))) (kind featureTyping) (ordinal 0)) (authored-target "Tire") (range (start (line 196) (character 19)) (end (line 196) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::Tire")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::tire::moment"))) (kind redefinition) (ordinal 0)) (authored-target "moment") (range (start (line 197) (character 12)) (end (line 197) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle::tire::moment")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::tire::radius"))) (kind redefinition) (ordinal 0)) (authored-target "radius") (range (start (line 198) (character 12)) (end (line 198) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle::tire::radius")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::tire::tireBehavior"))) (kind featureTyping) (ordinal 0)) (authored-target "ContinuousStateSpaceDynamics") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::tire::tireBehavior::input"))) (kind featureTyping) (ordinal 0)) (authored-target "TireInput") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::tire::tireBehavior::output"))) (kind featureTyping) (ordinal 0)) (authored-target "TireOutput") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::vehicleBehavior"))) (kind featureTyping) (ordinal 0)) (authored-target "ContinuousStateSpaceDynamics") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::vehicleBehavior::input"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleInput") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::vehicleBehavior::output"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleOutput") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle_compact"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle") (range (start (line 209) (character 28)) (end (line 209) (character 35))) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle_compact::mass"))) (kind redefinition) (ordinal 0)) (authored-target "mass") (range (start (line 210) (character 22)) (end (line 210) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle_compact::mass")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle_compact::tire"))) (kind redefinition) (ordinal 0)) (authored-target "tire") (range (start (line 211) (character 17)) (end (line 211) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle_compact::tire")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle_compact::tire::moment"))) (kind redefinition) (ordinal 0)) (authored-target "moment") (range (start (line 212) (character 12)) (end (line 212) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle_compact::tire::moment")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle_compact::tire::radius"))) (kind redefinition) (ordinal 0)) (authored-target "radius") (range (start (line 213) (character 12)) (end (line 213) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle_compact::tire::radius")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle_large"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle") (range (start (line 264) (character 26)) (end (line 264) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle_large::mass"))) (kind redefinition) (ordinal 0)) (authored-target "mass") (range (start (line 265) (character 22)) (end (line 265) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle_large::mass")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle_large::tire"))) (kind redefinition) (ordinal 0)) (authored-target "tire") (range (start (line 266) (character 17)) (end (line 266) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle_large::tire")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle_large::tire::moment"))) (kind redefinition) (ordinal 0)) (authored-target "moment") (range (start (line 267) (character 12)) (end (line 267) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle_large::tire::moment")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle_large::tire::radius"))) (kind redefinition) (ordinal 0)) (authored-target "radius") (range (start (line 268) (character 12)) (end (line 268) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle_large::tire::radius")))))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "EVSample::EfficiencyAnalysis"))) (target (node (document "d0") (qualified-name "EVSample::VehicleAnalysis"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::EfficiencyAnalysis"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "EVSample::EfficiencyAnalysis::efficiencyRequirement"))) (target (node (document "d0") (qualified-name "EVSample::EfficiencyRequirement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::EfficiencyAnalysis::efficiencyRequirement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "EVSample::EfficiencyRequirement"))) (target (node (document "d0") (qualified-name "EVSample::VehicleRequirement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::EfficiencyRequirement"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "EVSample::MaxSpeedAnalysis"))) (target (node (document "d0") (qualified-name "EVSample::VehicleAnalysis"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::MaxSpeedAnalysis"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "EVSample::MaxSpeedAnalysis::maxSpeedRequirement"))) (target (node (document "d0") (qualified-name "EVSample::MaxSpeedRequirement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::MaxSpeedAnalysis::maxSpeedRequirement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "EVSample::MaxSpeedRequirement"))) (target (node (document "d0") (qualified-name "EVSample::VehicleRequirement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::MaxSpeedRequirement"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "EVSample::RangeAnalysis"))) (target (node (document "d0") (qualified-name "EVSample::VehicleAnalysis"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::RangeAnalysis"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "EVSample::RangeAnalysis::rangeRequirement"))) (target (node (document "d0") (qualified-name "EVSample::RangeRequirement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::RangeAnalysis::rangeRequirement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "EVSample::RangeRequirement"))) (target (node (document "d0") (qualified-name "EVSample::VehicleRequirement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::RangeRequirement"))) (kind specialization) (ordinal 0)))
    (relationship (kind subject) (source (node (document "d0") (qualified-name "EVSample::VehicleAnalysis"))) (target (node (document "d0") (qualified-name "EVSample::Vehicle"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "EVSample::VehicleAnalysis::vehicle"))) (target (node (document "d0") (qualified-name "EVSample::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::VehicleAnalysis::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "EVSample::VehicleAnalysis::vehicleRequirement"))) (target (node (document "d0") (qualified-name "EVSample::VehicleRequirement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::VehicleAnalysis::vehicleRequirement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "EVSample::VehicleRequirement"))) (target (node (document "d0") (qualified-name "EVSample::VehicleRequirement::vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::VehicleRequirement"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind subject) (source (node (document "d0") (qualified-name "EVSample::VehicleRequirement"))) (target (node (document "d0") (qualified-name "EVSample::Vehicle"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "EVSample::VehicleRequirement::vehicle"))) (target (node (document "d0") (qualified-name "EVSample::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::VehicleRequirement::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "EVSample::vehicle"))) (target (node (document "d0") (qualified-name "EVSample::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "EVSample::vehicle::battery"))) (target (node (document "d0") (qualified-name "EVSample::Battery"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::vehicle::battery"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "EVSample::vehicle::battery::baseVoltage"))) (target (node (document "d0") (qualified-name "EVSample::vehicle::battery::baseVoltage"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::vehicle::battery::baseVoltage"))) (kind redefinition) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "EVSample::vehicle::battery::batteryBehavior::output"))) (target (node (document "d0") (qualified-name "EVSample::vehicle::motor::motorBehavior::input"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::vehicle"))) (kind flowSource) (ordinal 0)) (expression (kind flow) (source "battery::batteryBehavior::output") (target "motor::motorBehavior::input") (source-range (start (line 181) (character 13)) (end (line 181) (character 43))) (target-range (start (line 181) (character 47)) (end (line 181) (character 72)))))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "EVSample::vehicle::battery::capacity"))) (target (node (document "d0") (qualified-name "EVSample::vehicle::battery::capacity"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::vehicle::battery::capacity"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "EVSample::vehicle::battery::internalResistance"))) (target (node (document "d0") (qualified-name "EVSample::vehicle::battery::internalResistance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::vehicle::battery::internalResistance"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "EVSample::vehicle::battery::socInit"))) (target (node (document "d0") (qualified-name "EVSample::vehicle::battery::socInit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::vehicle::battery::socInit"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "EVSample::vehicle::mass"))) (target (node (document "d0") (qualified-name "EVSample::vehicle::mass"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::vehicle::mass"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "EVSample::vehicle::motor"))) (target (node (document "d0") (qualified-name "EVSample::Motor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::vehicle::motor"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "EVSample::vehicle::motor::motL"))) (target (node (document "d0") (qualified-name "EVSample::vehicle::motor::motL"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::vehicle::motor::motL"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "EVSample::vehicle::motor::motR"))) (target (node (document "d0") (qualified-name "EVSample::vehicle::motor::motR"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::vehicle::motor::motR"))) (kind redefinition) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "EVSample::vehicle::motor::motorBehavior::output"))) (target (node (document "d0") (qualified-name "EVSample::vehicle::tire::tireBehavior::input"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::vehicle"))) (kind flowSource) (ordinal 1)) (expression (kind flow) (source "motor::motorBehavior::output") (target "tire::tireBehavior::input") (source-range (start (line 194) (character 13)) (end (line 194) (character 39))) (target-range (start (line 194) (character 43)) (end (line 194) (character 66)))))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "EVSample::vehicle::tire"))) (target (node (document "d0") (qualified-name "EVSample::Tire"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::vehicle::tire"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "EVSample::vehicle::tire::moment"))) (target (node (document "d0") (qualified-name "EVSample::vehicle::tire::moment"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::vehicle::tire::moment"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "EVSample::vehicle::tire::radius"))) (target (node (document "d0") (qualified-name "EVSample::vehicle::tire::radius"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::vehicle::tire::radius"))) (kind redefinition) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "EVSample::vehicle::tire::tireBehavior::output"))) (target (node (document "d0") (qualified-name "EVSample::vehicle::motor::motorBehavior::input"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::vehicle"))) (kind flowSource) (ordinal 2)) (expression (kind flow) (source "tire::tireBehavior::output") (target "motor::motorBehavior::input") (source-range (start (line 205) (character 13)) (end (line 205) (character 37))) (target-range (start (line 205) (character 41)) (end (line 205) (character 66)))))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "EVSample::vehicle::tire::tireBehavior::output"))) (target (node (document "d0") (qualified-name "EVSample::vehicle::vehicleBehavior::input"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::vehicle"))) (kind flowSource) (ordinal 3)) (expression (kind flow) (source "tire::tireBehavior::output") (target "vehicleBehavior::input") (source-range (start (line 206) (character 13)) (end (line 206) (character 37))) (target-range (start (line 206) (character 41)) (end (line 206) (character 62)))))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "EVSample::vehicle_compact"))) (target (node (document "d0") (qualified-name "EVSample::vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::vehicle_compact"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "EVSample::vehicle_compact::mass"))) (target (node (document "d0") (qualified-name "EVSample::vehicle_compact::mass"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::vehicle_compact::mass"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "EVSample::vehicle_compact::tire"))) (target (node (document "d0") (qualified-name "EVSample::vehicle_compact::tire"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::vehicle_compact::tire"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "EVSample::vehicle_compact::tire::moment"))) (target (node (document "d0") (qualified-name "EVSample::vehicle_compact::tire::moment"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::vehicle_compact::tire::moment"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "EVSample::vehicle_compact::tire::radius"))) (target (node (document "d0") (qualified-name "EVSample::vehicle_compact::tire::radius"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::vehicle_compact::tire::radius"))) (kind redefinition) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "EVSample::vehicle_large"))) (target (node (document "d0") (qualified-name "EVSample::vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::vehicle_large"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "EVSample::vehicle_large::mass"))) (target (node (document "d0") (qualified-name "EVSample::vehicle_large::mass"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::vehicle_large::mass"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "EVSample::vehicle_large::tire"))) (target (node (document "d0") (qualified-name "EVSample::vehicle_large::tire"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::vehicle_large::tire"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "EVSample::vehicle_large::tire::moment"))) (target (node (document "d0") (qualified-name "EVSample::vehicle_large::tire::moment"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::vehicle_large::tire::moment"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "EVSample::vehicle_large::tire::radius"))) (target (node (document "d0") (qualified-name "EVSample::vehicle_large::tire::radius"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::vehicle_large::tire::radius"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "EVSample::EfficiencyAnalysis::efficiencyRequirement")) (expression (status "incomplete") (error "expression is incomplete")) (analysis (status "incomplete")))
    (node (node (document "d0") (qualified-name "EVSample::EfficiencyRequirement")) (expression (status "incomplete") (error "expression is incomplete")) (analysis (status "incomplete")))
    (node (node (document "d0") (qualified-name "EVSample::EfficiencyRequirement::_requireConstraint_0")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "EVSample::RangeAnalysis::rangeRequirement")) (expression (status "incomplete") (error "expression is incomplete")) (analysis (status "incomplete")))
    (node (node (document "d0") (qualified-name "EVSample::RangeRequirement")) (expression (status "incomplete") (error "expression is incomplete")) (analysis (status "incomplete")))
    (node (node (document "d0") (qualified-name "EVSample::RangeRequirement::_requireConstraint_0")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "EVSample::ampere hour")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "EVSample::vehicle::airFrictionCoefficient")) (expression (status "ok") (value (real 0.2))))
    (node (node (document "d0") (qualified-name "EVSample::vehicle::battery::baseVoltage")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "EVSample::vehicle::battery::capacity")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "EVSample::vehicle::battery::internalResistance")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "EVSample::vehicle::battery::socInit")) (expression (status "ok") (value (real 0.8))))
    (node (node (document "d0") (qualified-name "EVSample::vehicle::mass")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "EVSample::vehicle::motor::motL")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "EVSample::vehicle::motor::motR")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "EVSample::vehicle::tire::moment")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "EVSample::vehicle::tire::radius")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "EVSample::vehicle_compact::mass")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "EVSample::vehicle_compact::tire::moment")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "EVSample::vehicle_compact::tire::radius")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "EVSample::vehicle_large::mass")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "EVSample::vehicle_large::tire::moment")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "EVSample::vehicle_large::tire::radius")) (expression (status "unsupported") (error "declared expression form is not supported")))
  )
)
~~~
