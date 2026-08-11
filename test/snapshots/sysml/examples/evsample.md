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
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "8e5d15046c8cde81ae6c2fc5ec3d5deb1e9ad15f6b582905667a5d8127419aa4") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "EVSample"))) (kind "package") (name "EVSample") (declared-name "EVSample"))
    (element (id (node (document "d0") (qualified-name "EVSample::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "EVSample"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "EVSample::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "EVSample"))) (authored (membership (kind Import) (visibility "private") (import (reference "StateSpaceRepresentation::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "EVSample::Battery"))) (kind "part def") (name "Battery") (declared-name "Battery") (parent (node (document "d0") (qualified-name "EVSample"))))
    (element (id (node (document "d0") (qualified-name "EVSample::Battery::BatteryInput"))) (kind "attribute def") (name "BatteryInput") (declared-name "BatteryInput") (parent (node (document "d0") (qualified-name "EVSample::Battery"))) (authored (membership (kind Owning)) (relationships (typing (reference "Input")))))
    (element (id (node (document "d0") (qualified-name "EVSample::Battery::BatteryOutput"))) (kind "attribute def") (name "BatteryOutput") (declared-name "BatteryOutput") (parent (node (document "d0") (qualified-name "EVSample::Battery"))) (authored (membership (kind Owning)) (relationships (typing (reference "Output")))))
    (element (id (node (document "d0") (qualified-name "EVSample::Battery::BatteryState"))) (kind "attribute def") (name "BatteryState") (declared-name "BatteryState") (parent (node (document "d0") (qualified-name "EVSample::Battery"))) (authored (membership (kind Owning)) (relationships (typing (reference "StateSpace")))))
    (element (id (node (document "d0") (qualified-name "EVSample::Battery::baseVoltage"))) (kind "attribute") (name "baseVoltage") (declared-name "baseVoltage") (parent (node (document "d0") (qualified-name "EVSample::Battery"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::electricPotential")))))
    (element (id (node (document "d0") (qualified-name "EVSample::Battery::capacity"))) (kind "attribute") (name "capacity") (declared-name "capacity") (parent (node (document "d0") (qualified-name "EVSample::Battery"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::electricCharge")))))
    (element (id (node (document "d0") (qualified-name "EVSample::Battery::internalResistance"))) (kind "attribute") (name "internalResistance") (declared-name "internalResistance") (parent (node (document "d0") (qualified-name "EVSample::Battery"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::resistance")))))
    (element (id (node (document "d0") (qualified-name "EVSample::Battery::socInit"))) (kind "attribute") (name "socInit") (declared-name "socInit") (parent (node (document "d0") (qualified-name "EVSample::Battery"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (typing (reference "ScalarValues::Real")))))
    (element (id (node (document "d0") (qualified-name "EVSample::EfficiencyAnalysis"))) (kind "analysis def") (name "EfficiencyAnalysis") (declared-name "EfficiencyAnalysis") (parent (node (document "d0") (qualified-name "EVSample"))) (authored (membership (kind Owning)) (relationships (specializes (reference "VehicleAnalysis")))))
    (element (id (node (document "d0") (qualified-name "EVSample::EfficiencyAnalysis::efficiencyAnalysisObjective"))) (kind "objective") (name "efficiencyAnalysisObjective") (declared-name "efficiencyAnalysisObjective") (parent (node (document "d0") (qualified-name "EVSample::EfficiencyAnalysis"))))
    (element (id (node (document "d0") (qualified-name "EVSample::EfficiencyAnalysis::efficiencyRequirement"))) (kind "requirement") (name "efficiencyRequirement") (declared-name "efficiencyRequirement") (parent (node (document "d0") (qualified-name "EVSample::EfficiencyAnalysis"))) (authored (membership (kind Feature)) (relationships (typing (reference "EfficiencyRequirement")))))
    (element (id (node (document "d0") (qualified-name "EVSample::EfficiencyAnalysis::simulatedEfficiency"))) (kind "analysis result") (name "simulatedEfficiency") (declared-name "simulatedEfficiency") (parent (node (document "d0") (qualified-name "EVSample::EfficiencyAnalysis"))))
    (element (id (node (document "d0") (qualified-name "EVSample::EfficiencyRequirement"))) (kind "requirement def") (name "EfficiencyRequirement") (declared-name "EfficiencyRequirement") (parent (node (document "d0") (qualified-name "EVSample"))) (authored (membership (kind Owning)) (relationships (specializes (reference "VehicleRequirement")))))
    (element (id (node (document "d0") (qualified-name "EVSample::EfficiencyRequirement::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "EVSample::EfficiencyRequirement"))))
    (element (id (node (document "d0") (qualified-name "EVSample::EfficiencyRequirement::_requireConstraint_0"))) (kind "require constraint") (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (parent (node (document "d0") (qualified-name "EVSample::EfficiencyRequirement"))))
    (element (id (node (document "d0") (qualified-name "EVSample::EfficiencyRequirement::actualEfficiency"))) (kind "attribute") (name "actualEfficiency") (declared-name "actualEfficiency") (parent (node (document "d0") (qualified-name "EVSample::EfficiencyRequirement"))))
    (element (id (node (document "d0") (qualified-name "EVSample::EfficiencyRequirement::requiredEfficiency"))) (kind "attribute") (name "requiredEfficiency") (declared-name "requiredEfficiency") (parent (node (document "d0") (qualified-name "EVSample::EfficiencyRequirement"))))
    (element (id (node (document "d0") (qualified-name "EVSample::MaxSpeedAnalysis"))) (kind "analysis def") (name "MaxSpeedAnalysis") (declared-name "MaxSpeedAnalysis") (parent (node (document "d0") (qualified-name "EVSample"))) (authored (membership (kind Owning)) (relationships (specializes (reference "VehicleAnalysis")))))
    (element (id (node (document "d0") (qualified-name "EVSample::MaxSpeedAnalysis::maxSpeedAnalysisObjective"))) (kind "objective") (name "maxSpeedAnalysisObjective") (declared-name "maxSpeedAnalysisObjective") (parent (node (document "d0") (qualified-name "EVSample::MaxSpeedAnalysis"))))
    (element (id (node (document "d0") (qualified-name "EVSample::MaxSpeedAnalysis::maxSpeedRequirement"))) (kind "requirement") (name "maxSpeedRequirement") (declared-name "maxSpeedRequirement") (parent (node (document "d0") (qualified-name "EVSample::MaxSpeedAnalysis"))) (authored (membership (kind Feature)) (relationships (typing (reference "MaxSpeedRequirement")))))
    (element (id (node (document "d0") (qualified-name "EVSample::MaxSpeedAnalysis::simulatedMaxSpeed"))) (kind "analysis result") (name "simulatedMaxSpeed") (declared-name "simulatedMaxSpeed") (parent (node (document "d0") (qualified-name "EVSample::MaxSpeedAnalysis"))))
    (element (id (node (document "d0") (qualified-name "EVSample::MaxSpeedRequirement"))) (kind "requirement def") (name "MaxSpeedRequirement") (declared-name "MaxSpeedRequirement") (parent (node (document "d0") (qualified-name "EVSample"))) (authored (membership (kind Owning)) (relationships (specializes (reference "VehicleRequirement")))))
    (element (id (node (document "d0") (qualified-name "EVSample::MaxSpeedRequirement::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "EVSample::MaxSpeedRequirement"))))
    (element (id (node (document "d0") (qualified-name "EVSample::MaxSpeedRequirement::actualMaxSpeed"))) (kind "attribute") (name "actualMaxSpeed") (declared-name "actualMaxSpeed") (parent (node (document "d0") (qualified-name "EVSample::MaxSpeedRequirement"))) (authored (relationships (subsetting (reference "ISQ::speed")))))
    (element (id (node (document "d0") (qualified-name "EVSample::MaxSpeedRequirement::requiredMaxSpeed"))) (kind "attribute") (name "requiredMaxSpeed") (declared-name "requiredMaxSpeed") (parent (node (document "d0") (qualified-name "EVSample::MaxSpeedRequirement"))) (authored (relationships (subsetting (reference "ISQ::speed")))))
    (element (id (node (document "d0") (qualified-name "EVSample::Motor"))) (kind "part def") (name "Motor") (declared-name "Motor") (parent (node (document "d0") (qualified-name "EVSample"))))
    (element (id (node (document "d0") (qualified-name "EVSample::Motor::MotorInput"))) (kind "attribute def") (name "MotorInput") (declared-name "MotorInput") (parent (node (document "d0") (qualified-name "EVSample::Motor"))) (authored (membership (kind Owning)) (relationships (typing (reference "Input")))))
    (element (id (node (document "d0") (qualified-name "EVSample::Motor::MotorOutput"))) (kind "attribute def") (name "MotorOutput") (declared-name "MotorOutput") (parent (node (document "d0") (qualified-name "EVSample::Motor"))) (authored (membership (kind Owning)) (relationships (typing (reference "Output")))))
    (element (id (node (document "d0") (qualified-name "EVSample::Motor::MotorState"))) (kind "attribute def") (name "MotorState") (declared-name "MotorState") (parent (node (document "d0") (qualified-name "EVSample::Motor"))) (authored (membership (kind Owning)) (relationships (typing (reference "StateSpace")))))
    (element (id (node (document "d0") (qualified-name "EVSample::Motor::motL"))) (kind "attribute") (name "motL") (declared-name "motL") (parent (node (document "d0") (qualified-name "EVSample::Motor"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::inductance")))))
    (element (id (node (document "d0") (qualified-name "EVSample::Motor::motR"))) (kind "attribute") (name "motR") (declared-name "motR") (parent (node (document "d0") (qualified-name "EVSample::Motor"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::resistance")))))
    (element (id (node (document "d0") (qualified-name "EVSample::RangeAnalysis"))) (kind "analysis def") (name "RangeAnalysis") (declared-name "RangeAnalysis") (parent (node (document "d0") (qualified-name "EVSample"))) (authored (membership (kind Owning)) (relationships (specializes (reference "VehicleAnalysis")))))
    (element (id (node (document "d0") (qualified-name "EVSample::RangeAnalysis::rangeAnalysisObjective"))) (kind "objective") (name "rangeAnalysisObjective") (declared-name "rangeAnalysisObjective") (parent (node (document "d0") (qualified-name "EVSample::RangeAnalysis"))))
    (element (id (node (document "d0") (qualified-name "EVSample::RangeAnalysis::rangeRequirement"))) (kind "requirement") (name "rangeRequirement") (declared-name "rangeRequirement") (parent (node (document "d0") (qualified-name "EVSample::RangeAnalysis"))) (authored (membership (kind Feature)) (relationships (typing (reference "RangeRequirement")))))
    (element (id (node (document "d0") (qualified-name "EVSample::RangeAnalysis::simulatedRange"))) (kind "analysis result") (name "simulatedRange") (declared-name "simulatedRange") (parent (node (document "d0") (qualified-name "EVSample::RangeAnalysis"))) (authored (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "EVSample::RangeRequirement"))) (kind "requirement def") (name "RangeRequirement") (declared-name "RangeRequirement") (parent (node (document "d0") (qualified-name "EVSample"))) (authored (membership (kind Owning)) (relationships (specializes (reference "VehicleRequirement")))))
    (element (id (node (document "d0") (qualified-name "EVSample::RangeRequirement::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "EVSample::RangeRequirement"))))
    (element (id (node (document "d0") (qualified-name "EVSample::RangeRequirement::_requireConstraint_0"))) (kind "require constraint") (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (parent (node (document "d0") (qualified-name "EVSample::RangeRequirement"))))
    (element (id (node (document "d0") (qualified-name "EVSample::RangeRequirement::actualRange"))) (kind "attribute") (name "actualRange") (declared-name "actualRange") (parent (node (document "d0") (qualified-name "EVSample::RangeRequirement"))) (authored (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "EVSample::RangeRequirement::requiredRange"))) (kind "attribute") (name "requiredRange") (declared-name "requiredRange") (parent (node (document "d0") (qualified-name "EVSample::RangeRequirement"))) (authored (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "EVSample::Tire"))) (kind "part def") (name "Tire") (declared-name "Tire") (parent (node (document "d0") (qualified-name "EVSample"))))
    (element (id (node (document "d0") (qualified-name "EVSample::Tire::TireInput"))) (kind "attribute def") (name "TireInput") (declared-name "TireInput") (parent (node (document "d0") (qualified-name "EVSample::Tire"))) (authored (membership (kind Owning)) (relationships (typing (reference "Input")))))
    (element (id (node (document "d0") (qualified-name "EVSample::Tire::TireOutput"))) (kind "attribute def") (name "TireOutput") (declared-name "TireOutput") (parent (node (document "d0") (qualified-name "EVSample::Tire"))) (authored (membership (kind Owning)) (relationships (typing (reference "Output")))))
    (element (id (node (document "d0") (qualified-name "EVSample::Tire::moment"))) (kind "attribute") (name "moment") (declared-name "moment") (parent (node (document "d0") (qualified-name "EVSample::Tire"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::momentOfInertia")))))
    (element (id (node (document "d0") (qualified-name "EVSample::Tire::radius"))) (kind "attribute") (name "radius") (declared-name "radius") (parent (node (document "d0") (qualified-name "EVSample::Tire"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::length")))))
    (element (id (node (document "d0") (qualified-name "EVSample::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "EVSample"))))
    (element (id (node (document "d0") (qualified-name "EVSample::Vehicle::VehicleInput"))) (kind "attribute def") (name "VehicleInput") (declared-name "VehicleInput") (parent (node (document "d0") (qualified-name "EVSample::Vehicle"))) (authored (membership (kind Owning)) (relationships (typing (reference "Input")))))
    (element (id (node (document "d0") (qualified-name "EVSample::Vehicle::VehicleOutput"))) (kind "attribute def") (name "VehicleOutput") (declared-name "VehicleOutput") (parent (node (document "d0") (qualified-name "EVSample::Vehicle"))) (authored (membership (kind Owning)) (relationships (typing (reference "Output")))))
    (element (id (node (document "d0") (qualified-name "EVSample::Vehicle::VehicleState"))) (kind "attribute def") (name "VehicleState") (declared-name "VehicleState") (parent (node (document "d0") (qualified-name "EVSample::Vehicle"))) (authored (membership (kind Owning)) (relationships (typing (reference "StateSpace")))))
    (element (id (node (document "d0") (qualified-name "EVSample::Vehicle::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "EVSample::Vehicle"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::mass")))))
    (element (id (node (document "d0") (qualified-name "EVSample::VehicleAnalysis"))) (kind "analysis def") (name "VehicleAnalysis") (declared-name "VehicleAnalysis") (parent (node (document "d0") (qualified-name "EVSample"))))
    (element (id (node (document "d0") (qualified-name "EVSample::VehicleAnalysis::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "EVSample::VehicleAnalysis"))) (authored (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "EVSample::VehicleAnalysis::vehicleRequirement"))) (kind "requirement") (name "vehicleRequirement") (declared-name "vehicleRequirement") (parent (node (document "d0") (qualified-name "EVSample::VehicleAnalysis"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleRequirement")))))
    (element (id (node (document "d0") (qualified-name "EVSample::VehicleRequirement"))) (kind "requirement def") (name "VehicleRequirement") (declared-name "VehicleRequirement") (parent (node (document "d0") (qualified-name "EVSample"))) (authored (membership (kind Owning)) (relationships (subject (reference "EVSample::VehicleRequirement::vehicle")))))
    (element (id (node (document "d0") (qualified-name "EVSample::VehicleRequirement::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "EVSample::VehicleRequirement"))) (authored (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "EVSample::ampere hour"))) (kind "attribute def") (name "ampere hour") (declared-name "ampere hour") (parent (node (document "d0") (qualified-name "EVSample"))) (authored (membership (kind Owning)) (relationships (typing (reference "ElectricChargeUnit")))))
    (element (id (node (document "d0") (qualified-name "EVSample::largeEVRangeContext"))) (kind "part") (name "largeEVRangeContext") (declared-name "largeEVRangeContext") (parent (node (document "d0") (qualified-name "EVSample"))))
    (element (id (node (document "d0") (qualified-name "EVSample::smallEVRangeContext"))) (kind "part") (name "smallEVRangeContext") (declared-name "smallEVRangeContext") (parent (node (document "d0") (qualified-name "EVSample"))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "EVSample"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::airFrictionCoefficient"))) (kind "attribute") (name "airFrictionCoefficient") (declared-name "airFrictionCoefficient") (parent (node (document "d0") (qualified-name "EVSample::vehicle"))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::battery"))) (kind "part") (name "battery") (declared-name "battery") (parent (node (document "d0") (qualified-name "EVSample::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Battery")))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::battery::baseVoltage"))) (kind "attribute") (name "baseVoltage") (declared-name "baseVoltage") (parent (node (document "d0") (qualified-name "EVSample::vehicle::battery"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseVoltage")))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::battery::batteryBehavior"))) (kind "action") (name "batteryBehavior") (declared-name "batteryBehavior") (parent (node (document "d0") (qualified-name "EVSample::vehicle::battery"))) (authored (membership (kind Feature)) (relationships (typing (reference "ContinuousStateSpaceDynamics")))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::battery::batteryBehavior::input"))) (kind "in out parameter") (name "input") (declared-name "input") (parent (node (document "d0") (qualified-name "EVSample::vehicle::battery::batteryBehavior"))) (authored (relationships (typing (reference "BatteryInput")))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::battery::batteryBehavior::output"))) (kind "in out parameter") (name "output") (declared-name "output") (parent (node (document "d0") (qualified-name "EVSample::vehicle::battery::batteryBehavior"))) (authored (relationships (typing (reference "BatteryOutput")))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::battery::capacity"))) (kind "attribute") (name "capacity") (declared-name "capacity") (parent (node (document "d0") (qualified-name "EVSample::vehicle::battery"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "capacity")))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::battery::internalResistance"))) (kind "attribute") (name "internalResistance") (declared-name "internalResistance") (parent (node (document "d0") (qualified-name "EVSample::vehicle::battery"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "internalResistance")))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::battery::socInit"))) (kind "attribute") (name "socInit") (declared-name "socInit") (parent (node (document "d0") (qualified-name "EVSample::vehicle::battery"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "socInit")))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::efficiency"))) (kind "attribute") (name "efficiency") (declared-name "efficiency") (parent (node (document "d0") (qualified-name "EVSample::vehicle"))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "EVSample::vehicle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "mass")))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::motor"))) (kind "part") (name "motor") (declared-name "motor") (parent (node (document "d0") (qualified-name "EVSample::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Motor")))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::motor::motL"))) (kind "attribute") (name "motL") (declared-name "motL") (parent (node (document "d0") (qualified-name "EVSample::vehicle::motor"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "motL")))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::motor::motR"))) (kind "attribute") (name "motR") (declared-name "motR") (parent (node (document "d0") (qualified-name "EVSample::vehicle::motor"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "motR")))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::motor::motorBehavior"))) (kind "action") (name "motorBehavior") (declared-name "motorBehavior") (parent (node (document "d0") (qualified-name "EVSample::vehicle::motor"))) (authored (membership (kind Feature)) (relationships (typing (reference "ContinuousStateSpaceDynamics")))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::motor::motorBehavior::input"))) (kind "in out parameter") (name "input") (declared-name "input") (parent (node (document "d0") (qualified-name "EVSample::vehicle::motor::motorBehavior"))) (authored (relationships (typing (reference "MotorInput")))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::motor::motorBehavior::output"))) (kind "in out parameter") (name "output") (declared-name "output") (parent (node (document "d0") (qualified-name "EVSample::vehicle::motor::motorBehavior"))) (authored (relationships (typing (reference "MotorOutput")))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::tire"))) (kind "part") (name "tire") (declared-name "tire") (parent (node (document "d0") (qualified-name "EVSample::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Tire")))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::tire::moment"))) (kind "attribute") (name "moment") (declared-name "moment") (parent (node (document "d0") (qualified-name "EVSample::vehicle::tire"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "moment")))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::tire::radius"))) (kind "attribute") (name "radius") (declared-name "radius") (parent (node (document "d0") (qualified-name "EVSample::vehicle::tire"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "radius")))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::tire::tireBehavior"))) (kind "action") (name "tireBehavior") (declared-name "tireBehavior") (parent (node (document "d0") (qualified-name "EVSample::vehicle::tire"))) (authored (membership (kind Feature)) (relationships (typing (reference "ContinuousStateSpaceDynamics")))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::tire::tireBehavior::input"))) (kind "in out parameter") (name "input") (declared-name "input") (parent (node (document "d0") (qualified-name "EVSample::vehicle::tire::tireBehavior"))) (authored (relationships (typing (reference "TireInput")))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::tire::tireBehavior::output"))) (kind "in out parameter") (name "output") (declared-name "output") (parent (node (document "d0") (qualified-name "EVSample::vehicle::tire::tireBehavior"))) (authored (relationships (typing (reference "TireOutput")))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::vehicleBehavior"))) (kind "action") (name "vehicleBehavior") (declared-name "vehicleBehavior") (parent (node (document "d0") (qualified-name "EVSample::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "ContinuousStateSpaceDynamics")))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::vehicleBehavior::input"))) (kind "in out parameter") (name "input") (declared-name "input") (parent (node (document "d0") (qualified-name "EVSample::vehicle::vehicleBehavior"))) (authored (relationships (typing (reference "VehicleInput")))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle::vehicleBehavior::output"))) (kind "in out parameter") (name "output") (declared-name "output") (parent (node (document "d0") (qualified-name "EVSample::vehicle::vehicleBehavior"))) (authored (relationships (typing (reference "VehicleOutput")))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle_compact"))) (kind "part") (name "vehicle_compact") (declared-name "vehicle_compact") (parent (node (document "d0") (qualified-name "EVSample"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle")))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle_compact::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "EVSample::vehicle_compact"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "mass")))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle_compact::tire"))) (kind "part") (name "tire") (parent (node (document "d0") (qualified-name "EVSample::vehicle_compact"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "tire")))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle_compact::tire::moment"))) (kind "attribute") (name "moment") (declared-name "moment") (parent (node (document "d0") (qualified-name "EVSample::vehicle_compact::tire"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "moment")))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle_compact::tire::radius"))) (kind "attribute") (name "radius") (declared-name "radius") (parent (node (document "d0") (qualified-name "EVSample::vehicle_compact::tire"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "radius")))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle_large"))) (kind "part") (name "vehicle_large") (declared-name "vehicle_large") (parent (node (document "d0") (qualified-name "EVSample"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle")))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle_large::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "EVSample::vehicle_large"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "mass")))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle_large::tire"))) (kind "part") (name "tire") (parent (node (document "d0") (qualified-name "EVSample::vehicle_large"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "tire")))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle_large::tire::moment"))) (kind "attribute") (name "moment") (declared-name "moment") (parent (node (document "d0") (qualified-name "EVSample::vehicle_large::tire"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "moment")))))
    (element (id (node (document "d0") (qualified-name "EVSample::vehicle_large::tire::radius"))) (kind "attribute") (name "radius") (declared-name "radius") (parent (node (document "d0") (qualified-name "EVSample::vehicle_large::tire"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "radius")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "EVSample::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "StateSpaceRepresentation::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::Battery::BatteryInput"))) (kind featureTyping) (ordinal 0)) (authored-target "Input") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::Battery::BatteryOutput"))) (kind featureTyping) (ordinal 0)) (authored-target "Output") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::Battery::BatteryState"))) (kind featureTyping) (ordinal 0)) (authored-target "StateSpace") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::Battery::baseVoltage"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::electricPotential") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::Battery::capacity"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::electricCharge") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::Battery::internalResistance"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::resistance") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::Battery::socInit"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::Battery::socInit"))) (kind featureTyping) (ordinal 1)) (authored-target "ScalarValues::Real") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::EfficiencyAnalysis"))) (kind specialization) (ordinal 0)) (authored-target "VehicleAnalysis") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::VehicleAnalysis")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::EfficiencyAnalysis::efficiencyRequirement"))) (kind featureTyping) (ordinal 0)) (authored-target "EfficiencyRequirement") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::EfficiencyRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::EfficiencyRequirement"))) (kind specialization) (ordinal 0)) (authored-target "VehicleRequirement") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::VehicleRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::MaxSpeedAnalysis"))) (kind specialization) (ordinal 0)) (authored-target "VehicleAnalysis") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::VehicleAnalysis")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::MaxSpeedAnalysis::maxSpeedRequirement"))) (kind featureTyping) (ordinal 0)) (authored-target "MaxSpeedRequirement") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::MaxSpeedRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::MaxSpeedRequirement"))) (kind specialization) (ordinal 0)) (authored-target "VehicleRequirement") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::VehicleRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::MaxSpeedRequirement::actualMaxSpeed"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::speed") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::MaxSpeedRequirement::requiredMaxSpeed"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::speed") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::Motor::MotorInput"))) (kind featureTyping) (ordinal 0)) (authored-target "Input") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::Motor::MotorOutput"))) (kind featureTyping) (ordinal 0)) (authored-target "Output") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::Motor::MotorState"))) (kind featureTyping) (ordinal 0)) (authored-target "StateSpace") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::Motor::motL"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::inductance") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::Motor::motR"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::resistance") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::RangeAnalysis"))) (kind specialization) (ordinal 0)) (authored-target "VehicleAnalysis") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::VehicleAnalysis")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::RangeAnalysis::rangeRequirement"))) (kind featureTyping) (ordinal 0)) (authored-target "RangeRequirement") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::RangeRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::RangeAnalysis::simulatedRange"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::RangeRequirement"))) (kind specialization) (ordinal 0)) (authored-target "VehicleRequirement") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::VehicleRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::RangeRequirement::actualRange"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::RangeRequirement::requiredRange"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::Tire::TireInput"))) (kind featureTyping) (ordinal 0)) (authored-target "Input") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::Tire::TireOutput"))) (kind featureTyping) (ordinal 0)) (authored-target "Output") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::Tire::moment"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::momentOfInertia") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::Tire::radius"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::length") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::Vehicle::VehicleInput"))) (kind featureTyping) (ordinal 0)) (authored-target "Input") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::Vehicle::VehicleOutput"))) (kind featureTyping) (ordinal 0)) (authored-target "Output") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::Vehicle::VehicleState"))) (kind featureTyping) (ordinal 0)) (authored-target "StateSpace") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::Vehicle::mass"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::VehicleAnalysis::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::VehicleAnalysis::vehicleRequirement"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleRequirement") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::VehicleRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::VehicleRequirement"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "EVSample::VehicleRequirement::vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::VehicleRequirement::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::VehicleRequirement::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::ampere hour"))) (kind featureTyping) (ordinal 0)) (authored-target "ElectricChargeUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle"))) (kind flowSource) (ordinal 0)) (authored-target "battery::batteryBehavior::output") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle::battery::batteryBehavior::output")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle"))) (kind flowSource) (ordinal 1)) (authored-target "motor::motorBehavior::output") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle::motor::motorBehavior::output")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle"))) (kind flowSource) (ordinal 2)) (authored-target "tire::tireBehavior::output") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle::tire::tireBehavior::output")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle"))) (kind flowSource) (ordinal 3)) (authored-target "tire::tireBehavior::output") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle::tire::tireBehavior::output")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle"))) (kind flowTarget) (ordinal 0)) (authored-target "motor::motorBehavior::input") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle::motor::motorBehavior::input")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle"))) (kind flowTarget) (ordinal 1)) (authored-target "tire::tireBehavior::input") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle::tire::tireBehavior::input")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle"))) (kind flowTarget) (ordinal 2)) (authored-target "motor::motorBehavior::input") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle::motor::motorBehavior::input")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle"))) (kind flowTarget) (ordinal 3)) (authored-target "vehicleBehavior::input") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle::vehicleBehavior::input")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::battery"))) (kind featureTyping) (ordinal 0)) (authored-target "Battery") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::Battery")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::battery::baseVoltage"))) (kind redefinition) (ordinal 0)) (authored-target "baseVoltage") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle::battery::baseVoltage")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::battery::batteryBehavior"))) (kind featureTyping) (ordinal 0)) (authored-target "ContinuousStateSpaceDynamics") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::battery::batteryBehavior::input"))) (kind featureTyping) (ordinal 0)) (authored-target "BatteryInput") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::battery::batteryBehavior::output"))) (kind featureTyping) (ordinal 0)) (authored-target "BatteryOutput") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::battery::capacity"))) (kind redefinition) (ordinal 0)) (authored-target "capacity") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle::battery::capacity")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::battery::internalResistance"))) (kind redefinition) (ordinal 0)) (authored-target "internalResistance") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle::battery::internalResistance")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::battery::socInit"))) (kind redefinition) (ordinal 0)) (authored-target "socInit") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle::battery::socInit")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::mass"))) (kind redefinition) (ordinal 0)) (authored-target "mass") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle::mass")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::motor"))) (kind featureTyping) (ordinal 0)) (authored-target "Motor") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::Motor")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::motor::motL"))) (kind redefinition) (ordinal 0)) (authored-target "motL") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle::motor::motL")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::motor::motR"))) (kind redefinition) (ordinal 0)) (authored-target "motR") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle::motor::motR")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::motor::motorBehavior"))) (kind featureTyping) (ordinal 0)) (authored-target "ContinuousStateSpaceDynamics") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::motor::motorBehavior::input"))) (kind featureTyping) (ordinal 0)) (authored-target "MotorInput") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::motor::motorBehavior::output"))) (kind featureTyping) (ordinal 0)) (authored-target "MotorOutput") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::tire"))) (kind featureTyping) (ordinal 0)) (authored-target "Tire") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::Tire")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::tire::moment"))) (kind redefinition) (ordinal 0)) (authored-target "moment") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle::tire::moment")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::tire::radius"))) (kind redefinition) (ordinal 0)) (authored-target "radius") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle::tire::radius")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::tire::tireBehavior"))) (kind featureTyping) (ordinal 0)) (authored-target "ContinuousStateSpaceDynamics") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::tire::tireBehavior::input"))) (kind featureTyping) (ordinal 0)) (authored-target "TireInput") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::tire::tireBehavior::output"))) (kind featureTyping) (ordinal 0)) (authored-target "TireOutput") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::vehicleBehavior"))) (kind featureTyping) (ordinal 0)) (authored-target "ContinuousStateSpaceDynamics") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::vehicleBehavior::input"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleInput") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle::vehicleBehavior::output"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleOutput") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle_compact"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle_compact::mass"))) (kind redefinition) (ordinal 0)) (authored-target "mass") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle_compact::mass")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle_compact::tire"))) (kind redefinition) (ordinal 0)) (authored-target "tire") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle_compact::tire")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle_compact::tire::moment"))) (kind redefinition) (ordinal 0)) (authored-target "moment") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle_compact::tire::moment")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle_compact::tire::radius"))) (kind redefinition) (ordinal 0)) (authored-target "radius") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle_compact::tire::radius")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle_large"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle_large::mass"))) (kind redefinition) (ordinal 0)) (authored-target "mass") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle_large::mass")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle_large::tire"))) (kind redefinition) (ordinal 0)) (authored-target "tire") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle_large::tire")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle_large::tire::moment"))) (kind redefinition) (ordinal 0)) (authored-target "moment") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle_large::tire::moment")))))
    (reference (id (source (node (document "d0") (qualified-name "EVSample::vehicle_large::tire::radius"))) (kind redefinition) (ordinal 0)) (authored-target "radius") (outcome (status resolved) (target (node (document "d0") (qualified-name "EVSample::vehicle_large::tire::radius")))))
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
    (relationship (kind flow) (source (node (document "d0") (qualified-name "EVSample::vehicle::battery::batteryBehavior::output"))) (target (node (document "d0") (qualified-name "EVSample::vehicle::motor::motorBehavior::input"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::vehicle"))) (kind flowSource) (ordinal 0)) (expression (kind flow) (source "battery::batteryBehavior::output") (target "motor::motorBehavior::input")))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "EVSample::vehicle::battery::capacity"))) (target (node (document "d0") (qualified-name "EVSample::vehicle::battery::capacity"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::vehicle::battery::capacity"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "EVSample::vehicle::battery::internalResistance"))) (target (node (document "d0") (qualified-name "EVSample::vehicle::battery::internalResistance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::vehicle::battery::internalResistance"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "EVSample::vehicle::battery::socInit"))) (target (node (document "d0") (qualified-name "EVSample::vehicle::battery::socInit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::vehicle::battery::socInit"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "EVSample::vehicle::mass"))) (target (node (document "d0") (qualified-name "EVSample::vehicle::mass"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::vehicle::mass"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "EVSample::vehicle::motor"))) (target (node (document "d0") (qualified-name "EVSample::Motor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::vehicle::motor"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "EVSample::vehicle::motor::motL"))) (target (node (document "d0") (qualified-name "EVSample::vehicle::motor::motL"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::vehicle::motor::motL"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "EVSample::vehicle::motor::motR"))) (target (node (document "d0") (qualified-name "EVSample::vehicle::motor::motR"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::vehicle::motor::motR"))) (kind redefinition) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "EVSample::vehicle::motor::motorBehavior::output"))) (target (node (document "d0") (qualified-name "EVSample::vehicle::tire::tireBehavior::input"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::vehicle"))) (kind flowSource) (ordinal 1)) (expression (kind flow) (source "motor::motorBehavior::output") (target "tire::tireBehavior::input")))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "EVSample::vehicle::tire"))) (target (node (document "d0") (qualified-name "EVSample::Tire"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::vehicle::tire"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "EVSample::vehicle::tire::moment"))) (target (node (document "d0") (qualified-name "EVSample::vehicle::tire::moment"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::vehicle::tire::moment"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "EVSample::vehicle::tire::radius"))) (target (node (document "d0") (qualified-name "EVSample::vehicle::tire::radius"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::vehicle::tire::radius"))) (kind redefinition) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "EVSample::vehicle::tire::tireBehavior::output"))) (target (node (document "d0") (qualified-name "EVSample::vehicle::motor::motorBehavior::input"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::vehicle"))) (kind flowSource) (ordinal 2)) (expression (kind flow) (source "tire::tireBehavior::output") (target "motor::motorBehavior::input")))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "EVSample::vehicle::tire::tireBehavior::output"))) (target (node (document "d0") (qualified-name "EVSample::vehicle::vehicleBehavior::input"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EVSample::vehicle"))) (kind flowSource) (ordinal 3)) (expression (kind flow) (source "tire::tireBehavior::output") (target "vehicleBehavior::input")))
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
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 2 19) (end 2 21)) (probe (position 2 19))
      (reference
        (source (document "d0") (qualified-name "EVSample::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "SI::*")
        (range (start 2 19) (end 2 21))
        (outcome (status unresolved))
      )
    )
    (query (range (start 155 22) (end 155 26)) (probe (position 155 22))
      (reference
        (source (document "d0") (qualified-name "EVSample::vehicle::mass"))
        (kind redefinition) (ordinal 0) (authored-target "mass")
        (range (start 155 22) (end 155 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "EVSample::vehicle::mass") (range (start 155 8) (end 155 44)))
        )
      )
    )
    (query (range (start 196 19) (end 196 23)) (probe (position 196 19))
      (reference
        (source (document "d0") (qualified-name "EVSample::vehicle::tire"))
        (kind featureTyping) (ordinal 0) (authored-target "Tire")
        (range (start 196 19) (end 196 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "EVSample::Tire") (range (start 67 4) (end 67 412)))
        )
      )
    )
    (query (range (start 210 22) (end 210 26)) (probe (position 210 22))
      (reference
        (source (document "d0") (qualified-name "EVSample::vehicle_compact::mass"))
        (kind redefinition) (ordinal 0) (authored-target "mass")
        (range (start 210 22) (end 210 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "EVSample::vehicle_compact::mass") (range (start 210 8) (end 210 37)))
        )
      )
    )
    (query (range (start 211 17) (end 211 21)) (probe (position 211 17))
      (reference
        (source (document "d0") (qualified-name "EVSample::vehicle_compact::tire"))
        (kind redefinition) (ordinal 0) (authored-target "tire")
        (range (start 211 17) (end 211 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "EVSample::vehicle_compact::tire") (range (start 211 8) (end 211 108)))
        )
      )
    )
    (query (range (start 265 22) (end 265 26)) (probe (position 265 22))
      (reference
        (source (document "d0") (qualified-name "EVSample::vehicle_large::mass"))
        (kind redefinition) (ordinal 0) (authored-target "mass")
        (range (start 265 22) (end 265 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "EVSample::vehicle_large::mass") (range (start 265 8) (end 265 38)))
        )
      )
    )
    (query (range (start 266 17) (end 266 21)) (probe (position 266 17))
      (reference
        (source (document "d0") (qualified-name "EVSample::vehicle_large::tire"))
        (kind redefinition) (ordinal 0) (authored-target "tire")
        (range (start 266 17) (end 266 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "EVSample::vehicle_large::tire") (range (start 266 8) (end 266 108)))
        )
      )
    )
    (query (range (start 10 38) (end 10 43)) (probe (position 10 38))
      (reference
        (source (document "d0") (qualified-name "EVSample::Vehicle::VehicleInput"))
        (kind featureTyping) (ordinal 0) (authored-target "Input")
        (range (start 10 38) (end 10 43))
        (outcome (status unresolved))
      )
    )
    (query (range (start 32 38) (end 32 43)) (probe (position 32 38))
      (reference
        (source (document "d0") (qualified-name "EVSample::Battery::BatteryInput"))
        (kind featureTyping) (ordinal 0) (authored-target "Input")
        (range (start 32 38) (end 32 43))
        (outcome (status unresolved))
      )
    )
    (query (range (start 52 36) (end 52 41)) (probe (position 52 36))
      (reference
        (source (document "d0") (qualified-name "EVSample::Motor::MotorInput"))
        (kind featureTyping) (ordinal 0) (authored-target "Input")
        (range (start 52 36) (end 52 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 71 35) (end 71 40)) (probe (position 71 35))
      (reference
        (source (document "d0") (qualified-name "EVSample::Tire::TireInput"))
        (kind featureTyping) (ordinal 0) (authored-target "Input")
        (range (start 71 35) (end 71 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 183 20) (end 183 25)) (probe (position 183 20))
      (reference
        (source (document "d0") (qualified-name "EVSample::vehicle::motor"))
        (kind featureTyping) (ordinal 0) (authored-target "Motor")
        (range (start 183 20) (end 183 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "EVSample::Motor") (range (start 46 4) (end 46 639)))
        )
      )
    )
    (query (range (start 14 39) (end 14 45)) (probe (position 14 39))
      (reference
        (source (document "d0") (qualified-name "EVSample::Vehicle::VehicleOutput"))
        (kind featureTyping) (ordinal 0) (authored-target "Output")
        (range (start 14 39) (end 14 45))
        (outcome (status unresolved))
      )
    )
    (query (range (start 36 39) (end 36 45)) (probe (position 36 39))
      (reference
        (source (document "d0") (qualified-name "EVSample::Battery::BatteryOutput"))
        (kind featureTyping) (ordinal 0) (authored-target "Output")
        (range (start 36 39) (end 36 45))
        (outcome (status unresolved))
      )
    )
    (query (range (start 57 37) (end 57 43)) (probe (position 57 37))
      (reference
        (source (document "d0") (qualified-name "EVSample::Motor::MotorOutput"))
        (kind featureTyping) (ordinal 0) (authored-target "Output")
        (range (start 57 37) (end 57 43))
        (outcome (status unresolved))
      )
    )
    (query (range (start 76 36) (end 76 42)) (probe (position 76 36))
      (reference
        (source (document "d0") (qualified-name "EVSample::Tire::TireOutput"))
        (kind featureTyping) (ordinal 0) (authored-target "Output")
        (range (start 76 36) (end 76 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 154 19) (end 154 26)) (probe (position 154 19))
      (reference
        (source (document "d0") (qualified-name "EVSample::vehicle"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 154 19) (end 154 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "EVSample::Vehicle") (range (start 7 4) (end 7 526)))
        )
      )
    )
    (query (range (start 169 22) (end 169 29)) (probe (position 169 22))
      (reference
        (source (document "d0") (qualified-name "EVSample::vehicle::battery"))
        (kind featureTyping) (ordinal 0) (authored-target "Battery")
        (range (start 169 22) (end 169 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "EVSample::Battery") (range (start 26 4) (end 26 578)))
        )
      )
    )
    (query (range (start 209 28) (end 209 35)) (probe (position 209 28))
      (reference
        (source (document "d0") (qualified-name "EVSample::vehicle_compact"))
        (kind subsetting) (ordinal 0) (authored-target "vehicle")
        (range (start 209 28) (end 209 35))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "EVSample::vehicle") (range (start 154 4) (end 154 1789)))
        )
      )
    )
    (query (range (start 264 26) (end 264 33)) (probe (position 264 26))
      (reference
        (source (document "d0") (qualified-name "EVSample::vehicle_large"))
        (kind subsetting) (ordinal 0) (authored-target "vehicle")
        (range (start 264 26) (end 264 33))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "EVSample::vehicle") (range (start 154 4) (end 154 1789)))
        )
      )
    )
    (query (range (start 184 12) (end 184 20)) (probe (position 184 12))
      (reference
        (source (document "d0") (qualified-name "EVSample::vehicle::motor::motR"))
        (kind redefinition) (ordinal 0) (authored-target "motR")
        (range (start 184 12) (end 184 20))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "EVSample::vehicle::motor::motR") (range (start 184 12) (end 184 31)))
        )
      )
    )
    (query (range (start 185 12) (end 185 20)) (probe (position 185 12))
      (reference
        (source (document "d0") (qualified-name "EVSample::vehicle::motor::motL"))
        (kind redefinition) (ordinal 0) (authored-target "motL")
        (range (start 185 12) (end 185 20))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "EVSample::vehicle::motor::motL") (range (start 185 12) (end 185 30)))
        )
      )
    )
    (query (range (start 8 26) (end 8 35)) (probe (position 8 26))
      (reference
        (source (document "d0") (qualified-name "EVSample::Vehicle::mass"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
        (range (start 8 26) (end 8 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 20 38) (end 20 48)) (probe (position 20 38))
      (reference
        (source (document "d0") (qualified-name "EVSample::Vehicle::VehicleState"))
        (kind featureTyping) (ordinal 0) (authored-target "StateSpace")
        (range (start 20 38) (end 20 48))
        (outcome (status unresolved))
      )
    )
    (query (range (start 40 38) (end 40 48)) (probe (position 40 38))
      (reference
        (source (document "d0") (qualified-name "EVSample::Battery::BatteryState"))
        (kind featureTyping) (ordinal 0) (authored-target "StateSpace")
        (range (start 40 38) (end 40 48))
        (outcome (status unresolved))
      )
    )
    (query (range (start 62 36) (end 62 46)) (probe (position 62 36))
      (reference
        (source (document "d0") (qualified-name "EVSample::Motor::MotorState"))
        (kind featureTyping) (ordinal 0) (authored-target "StateSpace")
        (range (start 62 36) (end 62 46))
        (outcome (status unresolved))
      )
    )
    (query (range (start 137 36) (end 137 46)) (probe (position 137 36))
      (reference
        (source (document "d0") (qualified-name "EVSample::MaxSpeedRequirement::actualMaxSpeed"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::speed")
        (range (start 137 36) (end 137 46))
        (outcome (status unresolved))
      )
    )
    (query (range (start 138 38) (end 138 48)) (probe (position 138 38))
      (reference
        (source (document "d0") (qualified-name "EVSample::MaxSpeedRequirement::requiredMaxSpeed"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::speed")
        (range (start 138 38) (end 138 48))
        (outcome (status unresolved))
      )
    )
    (query (range (start 197 12) (end 197 22)) (probe (position 197 12))
      (reference
        (source (document "d0") (qualified-name "EVSample::vehicle::tire::moment"))
        (kind redefinition) (ordinal 0) (authored-target "moment")
        (range (start 197 12) (end 197 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "EVSample::vehicle::tire::moment") (range (start 197 12) (end 197 47)))
        )
      )
    )
    (query (range (start 198 12) (end 198 22)) (probe (position 198 12))
      (reference
        (source (document "d0") (qualified-name "EVSample::vehicle::tire::radius"))
        (kind redefinition) (ordinal 0) (authored-target "radius")
        (range (start 198 12) (end 198 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "EVSample::vehicle::tire::radius") (range (start 198 12) (end 198 38)))
        )
      )
    )
    (query (range (start 212 12) (end 212 22)) (probe (position 212 12))
      (reference
        (source (document "d0") (qualified-name "EVSample::vehicle_compact::tire::moment"))
        (kind redefinition) (ordinal 0) (authored-target "moment")
        (range (start 212 12) (end 212 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "EVSample::vehicle_compact::tire::moment") (range (start 212 12) (end 212 41)))
        )
      )
    )
    (query (range (start 213 12) (end 213 22)) (probe (position 213 12))
      (reference
        (source (document "d0") (qualified-name "EVSample::vehicle_compact::tire::radius"))
        (kind redefinition) (ordinal 0) (authored-target "radius")
        (range (start 213 12) (end 213 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "EVSample::vehicle_compact::tire::radius") (range (start 213 12) (end 213 32)))
        )
      )
    )
    (query (range (start 267 12) (end 267 22)) (probe (position 267 12))
      (reference
        (source (document "d0") (qualified-name "EVSample::vehicle_large::tire::moment"))
        (kind redefinition) (ordinal 0) (authored-target "moment")
        (range (start 267 12) (end 267 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "EVSample::vehicle_large::tire::moment") (range (start 267 12) (end 267 41)))
        )
      )
    )
    (query (range (start 268 12) (end 268 22)) (probe (position 268 12))
      (reference
        (source (document "d0") (qualified-name "EVSample::vehicle_large::tire::radius"))
        (kind redefinition) (ordinal 0) (authored-target "radius")
        (range (start 268 12) (end 268 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "EVSample::vehicle_large::tire::radius") (range (start 268 12) (end 268 32)))
        )
      )
    )
    (query (range (start 68 28) (end 68 39)) (probe (position 68 28))
      (reference
        (source (document "d0") (qualified-name "EVSample::Tire::radius"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::length")
        (range (start 68 28) (end 68 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 172 12) (end 172 23)) (probe (position 172 12))
      (reference
        (source (document "d0") (qualified-name "EVSample::vehicle::battery::socInit"))
        (kind redefinition) (ordinal 0) (authored-target "socInit")
        (range (start 172 12) (end 172 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "EVSample::vehicle::battery::socInit") (range (start 172 12) (end 172 30)))
        )
      )
    )
    (query (range (start 171 12) (end 171 24)) (probe (position 171 12))
      (reference
        (source (document "d0") (qualified-name "EVSample::vehicle::battery::capacity"))
        (kind redefinition) (ordinal 0) (authored-target "capacity")
        (range (start 171 12) (end 171 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "EVSample::vehicle::battery::capacity") (range (start 171 12) (end 171 39)))
        )
      )
    )
    (query (range (start 30 40) (end 30 55)) (probe (position 30 40))
      (reference
        (source (document "d0") (qualified-name "EVSample::Battery::internalResistance"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::resistance")
        (range (start 30 40) (end 30 55))
        (outcome (status unresolved))
      )
    )
    (query (range (start 49 26) (end 49 41)) (probe (position 49 26))
      (reference
        (source (document "d0") (qualified-name "EVSample::Motor::motR"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::resistance")
        (range (start 49 26) (end 49 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 50 26) (end 50 41)) (probe (position 50 26))
      (reference
        (source (document "d0") (qualified-name "EVSample::Motor::motL"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::inductance")
        (range (start 50 26) (end 50 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 100 34) (end 100 49)) (probe (position 100 34))
      (reference
        (source (document "d0") (qualified-name "EVSample::RangeAnalysis"))
        (kind specialization) (ordinal 0) (authored-target "VehicleAnalysis")
        (range (start 100 34) (end 100 49))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "EVSample::VehicleAnalysis") (range (start 86 4) (end 86 136)))
        )
      )
    )
    (query (range (start 123 39) (end 123 54)) (probe (position 123 39))
      (reference
        (source (document "d0") (qualified-name "EVSample::EfficiencyAnalysis"))
        (kind specialization) (ordinal 0) (authored-target "VehicleAnalysis")
        (range (start 123 39) (end 123 54))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "EVSample::VehicleAnalysis") (range (start 86 4) (end 86 136)))
        )
      )
    )
    (query (range (start 141 37) (end 141 52)) (probe (position 141 37))
      (reference
        (source (document "d0") (qualified-name "EVSample::MaxSpeedAnalysis"))
        (kind specialization) (ordinal 0) (authored-target "VehicleAnalysis")
        (range (start 141 37) (end 141 52))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "EVSample::VehicleAnalysis") (range (start 86 4) (end 86 136)))
        )
      )
    )
    (query (range (start 170 12) (end 170 27)) (probe (position 170 12))
      (reference
        (source (document "d0") (qualified-name "EVSample::vehicle::battery::baseVoltage"))
        (kind redefinition) (ordinal 0) (authored-target "baseVoltage")
        (range (start 170 12) (end 170 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "EVSample::vehicle::battery::baseVoltage") (range (start 170 12) (end 170 37)))
        )
      )
    )
    (query (range (start 28 27) (end 28 45)) (probe (position 28 27))
      (reference
        (source (document "d0") (qualified-name "EVSample::Battery::socInit"))
        (kind featureTyping) (ordinal 1) (authored-target "ScalarValues::Real")
        (range (start 28 27) (end 28 45))
        (outcome (status unresolved))
      )
    )
    (query (range (start 92 40) (end 92 58)) (probe (position 92 40))
      (reference
        (source (document "d0") (qualified-name "EVSample::RangeRequirement"))
        (kind specialization) (ordinal 0) (authored-target "VehicleRequirement")
        (range (start 92 40) (end 92 58))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "EVSample::VehicleRequirement") (range (start 82 4) (end 82 81)))
        )
      )
    )
    (query (range (start 115 45) (end 115 63)) (probe (position 115 45))
      (reference
        (source (document "d0") (qualified-name "EVSample::EfficiencyRequirement"))
        (kind specialization) (ordinal 0) (authored-target "VehicleRequirement")
        (range (start 115 45) (end 115 63))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "EVSample::VehicleRequirement") (range (start 82 4) (end 82 81)))
        )
      )
    )
    (query (range (start 135 43) (end 135 61)) (probe (position 135 43))
      (reference
        (source (document "d0") (qualified-name "EVSample::MaxSpeedRequirement"))
        (kind specialization) (ordinal 0) (authored-target "VehicleRequirement")
        (range (start 135 43) (end 135 61))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "EVSample::VehicleRequirement") (range (start 82 4) (end 82 81)))
        )
      )
    )
    (query (range (start 29 30) (end 29 49)) (probe (position 29 30))
      (reference
        (source (document "d0") (qualified-name "EVSample::Battery::capacity"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::electricCharge")
        (range (start 29 30) (end 29 49))
        (outcome (status unresolved))
      )
    )
    (query (range (start 69 28) (end 69 48)) (probe (position 69 28))
      (reference
        (source (document "d0") (qualified-name "EVSample::Tire::moment"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::momentOfInertia")
        (range (start 69 28) (end 69 48))
        (outcome (status unresolved))
      )
    )
    (query (range (start 206 41) (end 206 62)) (probe (position 206 41))
      (reference
        (source (document "d0") (qualified-name "EVSample::vehicle"))
        (kind flowTarget) (ordinal 3) (authored-target "vehicleBehavior::input")
        (range (start 206 41) (end 206 62))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "EVSample::vehicle::vehicleBehavior::input") (range (start 164 12) (end 164 36)))
        )
      )
    )
    (query (range (start 27 33) (end 27 55)) (probe (position 27 33))
      (reference
        (source (document "d0") (qualified-name "EVSample::Battery::baseVoltage"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::electricPotential")
        (range (start 27 33) (end 27 55))
        (outcome (status unresolved))
      )
    )
    (query (range (start 173 12) (end 173 34)) (probe (position 173 12))
      (reference
        (source (document "d0") (qualified-name "EVSample::vehicle::battery::internalResistance"))
        (kind redefinition) (ordinal 0) (authored-target "internalResistance")
        (range (start 173 12) (end 173 34))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "EVSample::vehicle::battery::internalResistance") (range (start 173 12) (end 173 47)))
        )
      )
    )
    (query (range (start 194 43) (end 194 66)) (probe (position 194 43))
      (reference
        (source (document "d0") (qualified-name "EVSample::vehicle"))
        (kind flowTarget) (ordinal 1) (authored-target "tire::tireBehavior::input")
        (range (start 194 43) (end 194 66))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "EVSample::vehicle::tire::tireBehavior::input") (range (start 200 16) (end 200 37)))
        )
      )
    )
    (query (range (start 3 19) (end 3 43)) (probe (position 3 19))
      (reference
        (source (document "d0") (qualified-name "EVSample::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "StateSpaceRepresentation::*")
        (range (start 3 19) (end 3 43))
        (outcome (status unresolved))
      )
    )
    (query (range (start 205 13) (end 205 37)) (probe (position 205 13))
      (reference
        (source (document "d0") (qualified-name "EVSample::vehicle"))
        (kind flowSource) (ordinal 2) (authored-target "tire::tireBehavior::output")
        (range (start 205 13) (end 205 37))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "EVSample::vehicle::tire::tireBehavior::output") (range (start 201 16) (end 201 40)))
        )
      )
    )
    (query (range (start 206 13) (end 206 37)) (probe (position 206 13))
      (reference
        (source (document "d0") (qualified-name "EVSample::vehicle"))
        (kind flowSource) (ordinal 3) (authored-target "tire::tireBehavior::output")
        (range (start 206 13) (end 206 37))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "EVSample::vehicle::tire::tireBehavior::output") (range (start 201 16) (end 201 40)))
        )
      )
    )
    (query (range (start 181 47) (end 181 72)) (probe (position 181 47))
      (reference
        (source (document "d0") (qualified-name "EVSample::vehicle"))
        (kind flowTarget) (ordinal 0) (authored-target "motor::motorBehavior::input")
        (range (start 181 47) (end 181 72))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "EVSample::vehicle::motor::motorBehavior::input") (range (start 188 16) (end 188 38)))
        )
      )
    )
    (query (range (start 205 41) (end 205 66)) (probe (position 205 41))
      (reference
        (source (document "d0") (qualified-name "EVSample::vehicle"))
        (kind flowTarget) (ordinal 2) (authored-target "motor::motorBehavior::input")
        (range (start 205 41) (end 205 66))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "EVSample::vehicle::motor::motorBehavior::input") (range (start 188 16) (end 188 38)))
        )
      )
    )
    (query (range (start 194 13) (end 194 39)) (probe (position 194 13))
      (reference
        (source (document "d0") (qualified-name "EVSample::vehicle"))
        (kind flowSource) (ordinal 1) (authored-target "motor::motorBehavior::output")
        (range (start 194 13) (end 194 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "EVSample::vehicle::motor::motorBehavior::output") (range (start 189 16) (end 189 41)))
        )
      )
    )
    (query (range (start 181 13) (end 181 43)) (probe (position 181 13))
      (reference
        (source (document "d0") (qualified-name "EVSample::vehicle"))
        (kind flowSource) (ordinal 0) (authored-target "battery::batteryBehavior::output")
        (range (start 181 13) (end 181 43))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "EVSample::vehicle::battery::batteryBehavior::output") (range (start 176 16) (end 176 43)))
        )
      )
    )
  )
)
~~~
