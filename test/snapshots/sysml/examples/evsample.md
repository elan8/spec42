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
  (document "memory://snapshot/evsample.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 19) (end 2 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 19) (end 3 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 41) (end 5 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 5 62) (end 5 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 5 64) (end 5 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 8 26) (end 8 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 10 38) (end 10 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 31) (end 11 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 14 39) (end 14 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 15 31) (end 15 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 16 34) (end 16 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 17 34) (end 17 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 20 38) (end 20 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 21 34) (end 21 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 22 34) (end 22 47))
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
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 32 38) (end 32 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 33 33) (end 33 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 36 39) (end 36 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 37 33) (end 37 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 40 38) (end 40 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 41 27) (end 41 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 47 28) (end 47 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 47 59) (end 47 70))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 47 73) (end 47 93))
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
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 52 36) (end 52 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 53 33) (end 53 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 54 34) (end 54 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 57 37) (end 57 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 58 33) (end 58 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 59 32) (end 59 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 62 36) (end 62 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 63 33) (end 63 53))
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
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 71 35) (end 71 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 72 32) (end 72 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 73 31) (end 73 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 76 36) (end 76 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 77 31) (end 77 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 78 35) (end 78 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 94 32) (end 94 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 95 34) (end 95 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 101 29) (end 101 40))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 109 12) (end 111 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 129 12) (end 131 13))
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
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 147 12) (end 149 13))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 163 33) (end 163 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 164 23) (end 164 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 165 25) (end 165 38))
      )
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "parser")
        (range (start 166 12) (end 167 8))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 166 12) (end 167 8))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 174 37) (end 174 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 175 27) (end 175 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 176 29) (end 176 42))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 181 8) (end 181 73))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 187 35) (end 187 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 188 27) (end 188 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 189 29) (end 189 40))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 194 8) (end 194 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 199 34) (end 199 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 200 27) (end 200 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 201 29) (end 201 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 205 8) (end 205 67))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 206 8) (end 206 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 210 22) (end 210 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 211 17) (end 211 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 212 16) (end 212 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 213 16) (end 213 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 220 12) (end 220 50))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 226 12) (end 226 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 237 36) (end 237 75))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 248 41) (end 248 59))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 259 12) (end 259 99))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 260 39) (end 260 78))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 265 22) (end 265 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 266 17) (end 266 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 267 16) (end 267 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 268 16) (end 268 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 275 12) (end 275 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 281 12) (end 281 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 292 36) (end 292 75))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 303 41) (end 303 59))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 314 12) (end 314 73))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 315 39) (end 315 78))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:421fcb90e6f0bfc15eca186560c63a0142814f1aee97d175a438f45c6371efe6") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "SI") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (anonymous (kind import) (ordinal 1)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StateSpaceRepresentation") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery::BatteryInput"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Input"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery::BatteryInput::current"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::electricCurrent"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery::BatteryOutput"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Output"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery::BatteryOutput::voltage"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::electricPotential"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery::BatteryState"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "StateSpace"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery::BatteryState::soc"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Real"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery::baseVoltage"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::electricPotential"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery::capacity"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::electricCharge"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery::internalResistance"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::resistance"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery::socInit"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Real"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyAnalysis"))) (kind analysis-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "VehicleAnalysis"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyAnalysis::efficiencyAnalysisObjective"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyAnalysis::efficiencyRequirement"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "EfficiencyRequirement"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyAnalysis::simulatedEfficiency"))) (kind parameter) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyRequirement"))) (kind requirement-def) (membership (kind owning) (visibility default)) (documentation (doc (text " The efficiency of EV must be better than the required spec. "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "VehicleRequirement"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind requirement-def) (name "EfficiencyRequirement")) (anonymous (kind constraint) (ordinal 0)))))) (kind constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "actualEfficiency")) (expressionOperand (reference "requiredEfficiency"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyRequirement::actualEfficiency"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyRequirement::requiredEfficiency"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedAnalysis"))) (kind analysis-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "VehicleAnalysis"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedAnalysis::maxSpeedAnalysisObjective"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedAnalysis::maxSpeedRequirement"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MaxSpeedRequirement"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedAnalysis::simulatedMaxSpeed"))) (kind parameter) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedRequirement"))) (kind requirement-def) (membership (kind owning) (visibility default)) (documentation (doc (text " The maximum speed of EV must be larger than the required spec. "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "VehicleRequirement"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedRequirement::actualMaxSpeed"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::speed"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedRequirement::requiredMaxSpeed"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::speed"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::MotorInput"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Input"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::MotorInput::friction"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::torque"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::MotorInput::voltage"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::electricPotential"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::MotorOutput"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Output"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::MotorOutput::current"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::electricCurrent"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::MotorOutput::torque"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::torque"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::MotorState"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "StateSpace"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::MotorState::current"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::electricCurrent"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::motL"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::inductance"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::motR"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::resistance"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::torquePerCurrent"))) (kind default-reference) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "Quantities::scalarQuantities")) (expressionOperand (reference "ISQ::torque")) (expressionOperand (reference "ISQ::electricCurrent"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeAnalysis"))) (kind analysis-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "VehicleAnalysis"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeAnalysis::rangeAnalysisObjective"))) (kind requirement) (membership (kind feature) (visibility default)) (documentation (doc (text " This analysis is to estimate the range of\n                 * the EV by simulating the vehicle driving under the compact vehicle regulation.\n                 "))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeAnalysis::rangeRequirement"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "RangeRequirement"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeAnalysis::simulatedRange"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeRequirement"))) (kind requirement-def) (membership (kind owning) (visibility default)) (documentation (doc (text " The range of EV must be longer than the required spec under the flat road. "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "VehicleRequirement"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind requirement-def) (name "RangeRequirement")) (anonymous (kind constraint) (ordinal 0)))))) (kind constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "actualRange")) (expressionOperand (reference "requiredRange"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeRequirement::actualRange"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeRequirement::requiredRange"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Tire"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Tire::TireInput"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Input"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Tire::TireInput::accel"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::acceleration"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Tire::TireInput::torque"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::torque"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Tire::TireOutput"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Output"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Tire::TireOutput::force"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::force"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Tire::TireOutput::outTorque"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::torque"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Tire::moment"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::momentOfInertia"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Tire::radius"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::length"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle::VehicleInput"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Input"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle::VehicleInput::force"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::force"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle::VehicleOutput"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Output"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle::VehicleOutput::accel"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::acceleration"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle::VehicleOutput::distance"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::distance"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle::VehicleOutput::velocity"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::speed"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle::VehicleState"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "StateSpace"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle::VehicleState::distance"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::distance"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle::VehicleState::velocity"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::speed"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleAnalysis"))) (kind analysis-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleAnalysis::vehicle"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleAnalysis::vehicleRequirement"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleRequirement"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleRequirement"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleRequirement::vehicle"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::ampere hour"))) (kind attribute-def) (membership (kind owning) (visibility default)) (facts (short-name "A⋅h")) (feature-value (kind bind)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "ElectricChargeUnit")) (expressionOperand (reference "A")) (expressionOperand (reference "h"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::efficiencyAnalysisLarge"))) (kind analysis) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "EfficiencyAnalysis")) (subsetting (reference "largeEVAnalysis"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "largeEVRangeContext")) (named (kind analysis) (name "efficiencyAnalysisLarge")) (anonymous (kind requirement) (ordinal 0)))))) (kind requirement) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::efficiencyAnalysisLarge::simulatedEfficiency"))) (kind parameter) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "vehicle::efficiency"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::efficiencyRequirementLarge"))) (kind requirement) (membership (kind feature) (visibility default)) (facts (short-name "L2")) (documentation (doc (text " The target efficiency of large EVs is 0.8. "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "EfficiencyRequirement")) (subsetting (reference "largeEVRequirement"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "largeEVRangeContext")) (named (kind requirement) (name "efficiencyRequirementLarge")) (anonymous (kind attribute) (ordinal 0)))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "requiredEfficiency"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::largeEVAnalysis"))) (kind analysis) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleAnalysis"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "largeEVRangeContext")) (named (kind analysis) (name "largeEVAnalysis")) (anonymous (kind requirement) (ordinal 0)))))) (kind requirement) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::largeEVRequirement"))) (kind requirement) (membership (kind feature) (visibility default)) (documentation (doc (text " The large EVs must be ligher than 900[kg] "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleRequirement"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "largeEVRangeContext")) (named (kind requirement) (name "largeEVRequirement")) (anonymous (kind constraint) (ordinal 0)))))) (kind constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "vehicle::mass"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::maxSpeedAnalysisLarge"))) (kind analysis) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MaxSpeedAnalysis")) (subsetting (reference "largeEVAnalysis"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "largeEVRangeContext")) (named (kind analysis) (name "maxSpeedAnalysisLarge")) (anonymous (kind requirement) (ordinal 0)))))) (kind requirement) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::maxSpeedAnalysisLarge::simulatedMaxSpeed"))) (kind parameter) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "vehicle::vehicleBehavior::output::velocity"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::maxSpeedRequirementLarge"))) (kind requirement) (membership (kind feature) (visibility default)) (facts (short-name "L3")) (documentation (doc (text " The target maximum speed of large EVs is 140 [km/h]. "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MaxSpeedRequirement")) (subsetting (reference "largeEVRequirement"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "largeEVRangeContext")) (named (kind requirement) (name "maxSpeedRequirementLarge")) (anonymous (kind attribute) (ordinal 0)))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "requiredMaxSpeed"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::rangeAnalysisLarge"))) (kind analysis) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "RangeAnalysis")) (subsetting (reference "largeEVAnalysis"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "largeEVRangeContext")) (named (kind analysis) (name "rangeAnalysisLarge")) (anonymous (kind requirement) (ordinal 0)))))) (kind requirement) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::rangeAnalysisLarge::simulatedRange"))) (kind parameter) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "vehicle::vehicleBehavior::output::distance"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::rangeRequirementLarge"))) (kind requirement) (membership (kind feature) (visibility default)) (facts (short-name "L1")) (documentation (doc (text " The large EVs must run longer than 200km "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "RangeRequirement")) (subsetting (reference "largeEVRequirement"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "largeEVRangeContext")) (named (kind requirement) (name "rangeRequirementLarge")) (anonymous (kind attribute) (ordinal 0)))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "requiredRange"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::efficiencyAnalysisSmall"))) (kind analysis) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "EfficiencyAnalysis")) (subsetting (reference "smallEVAnalysis"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "smallEVRangeContext")) (named (kind analysis) (name "efficiencyAnalysisSmall")) (anonymous (kind requirement) (ordinal 0)))))) (kind requirement) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::efficiencyAnalysisSmall::simulatedEfficiency"))) (kind parameter) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "vehicle::efficiency"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::efficiencyRequirementSmall"))) (kind requirement) (membership (kind feature) (visibility default)) (facts (short-name "C2")) (documentation (doc (text " The target efficiency of small EVs is 0.9. "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "EfficiencyRequirement")) (subsetting (reference "smallEVRequirement"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "smallEVRangeContext")) (named (kind requirement) (name "efficiencyRequirementSmall")) (anonymous (kind attribute) (ordinal 0)))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "requiredEfficiency"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::maxSpeedAnalysisSmall"))) (kind analysis) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MaxSpeedAnalysis")) (subsetting (reference "smallEVAnalysis"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "smallEVRangeContext")) (named (kind analysis) (name "maxSpeedAnalysisSmall")) (anonymous (kind requirement) (ordinal 0)))))) (kind requirement) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::maxSpeedAnalysisSmall::simulatedMaxSpeed"))) (kind parameter) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "vehicle::vehicleBehavior::output::velocity"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::maxSpeedRequirementSmall"))) (kind requirement) (membership (kind feature) (visibility default)) (facts (short-name "C3")) (documentation (doc (text " The target maximum speed of small EVs is 130 [km/h]. "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MaxSpeedRequirement")) (subsetting (reference "smallEVRequirement"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "smallEVRangeContext")) (named (kind requirement) (name "maxSpeedRequirementSmall")) (anonymous (kind attribute) (ordinal 0)))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "requiredMaxSpeed"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::rangeAnalysisSmall"))) (kind analysis) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "RangeAnalysis")) (subsetting (reference "smallEVAnalysis"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "smallEVRangeContext")) (named (kind analysis) (name "rangeAnalysisSmall")) (anonymous (kind requirement) (ordinal 0)))))) (kind requirement) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::rangeAnalysisSmall::simulatedRange"))) (kind parameter) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "vehicle::vehicleBehavior::output::distance"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::rangeRequirementSmall"))) (kind requirement) (membership (kind feature) (visibility default)) (facts (short-name "C1")) (documentation (doc (text " The small EVs must run longer than 130km "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "RangeRequirement")) (subsetting (reference "smallEVRequirement"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "smallEVRangeContext")) (named (kind requirement) (name "rangeRequirementSmall")) (anonymous (kind attribute) (ordinal 0)))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "requiredRange"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::smallEVAnalysis"))) (kind analysis) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleAnalysis"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "smallEVRangeContext")) (named (kind analysis) (name "smallEVAnalysis")) (anonymous (kind requirement) (ordinal 0)))))) (kind requirement) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::smallEVRequirement"))) (kind requirement) (membership (kind feature) (visibility default)) (documentation (doc (text " The small EVs must be ligher than 900[kg] "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleRequirement"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "smallEVRangeContext")) (named (kind requirement) (name "smallEVRequirement")) (anonymous (kind constraint) (ordinal 0)))))) (kind constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "vehicle::mass"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (anonymous (kind attribute) (ordinal 0)))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (default true) (operator false)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "mass"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::airFrictionCoefficient"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::battery"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Battery"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "battery")) (anonymous (kind attribute) (ordinal 0)))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseVoltage"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "battery")) (anonymous (kind attribute) (ordinal 1)))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "capacity"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "battery")) (anonymous (kind attribute) (ordinal 2)))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "socInit"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "battery")) (anonymous (kind attribute) (ordinal 3)))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "internalResistance"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::battery::batteryBehavior"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ContinuousStateSpaceDynamics"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::battery::batteryBehavior::input"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "BatteryInput") (direction in))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::battery::batteryBehavior::output"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "BatteryOutput") (direction out))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::efficiency"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::motor"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Motor"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "motor")) (anonymous (kind attribute) (ordinal 0)))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "motR"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "motor")) (anonymous (kind attribute) (ordinal 1)))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "motL"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::motor::motorBehavior"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ContinuousStateSpaceDynamics"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::motor::motorBehavior::input"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MotorInput") (direction in))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::motor::motorBehavior::output"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MotorOutput") (direction out))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::tire"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Tire"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "tire")) (anonymous (kind attribute) (ordinal 0)))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (default true) (operator false)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "moment"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "tire")) (anonymous (kind attribute) (ordinal 1)))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (default true) (operator false)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "radius"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::tire::tireBehavior"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ContinuousStateSpaceDynamics"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::tire::tireBehavior::input"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TireInput") (direction in))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::tire::tireBehavior::output"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TireOutput") (direction out))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::vehicleBehavior"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ContinuousStateSpaceDynamics"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::vehicleBehavior::input"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleInput") (direction in))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::vehicleBehavior::output"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleOutput") (direction out))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle_compact"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "vehicle"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle_compact")) (anonymous (kind attribute) (ordinal 0)))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "mass"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle_compact")) (anonymous (kind part) (ordinal 0)))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "tire"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle_compact")) (anonymous (kind part) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "moment"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle_compact")) (anonymous (kind part) (ordinal 0)) (anonymous (kind attribute) (ordinal 1)))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "radius"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle_large"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "vehicle"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle_large")) (anonymous (kind attribute) (ordinal 0)))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "mass"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle_large")) (anonymous (kind part) (ordinal 0)))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "tire"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle_large")) (anonymous (kind part) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "moment"))))
    (declaration (id (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle_large")) (anonymous (kind part) (ordinal 0)) (anonymous (kind attribute) (ordinal 1)))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "radius"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "SI")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (anonymous (kind import) (ordinal 1)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StateSpaceRepresentation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery::BatteryInput"))) (kind specialization) (ordinal 0))
      (authored-target "Input")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery::BatteryInput::current"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::electricCurrent")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery::BatteryOutput"))) (kind specialization) (ordinal 0))
      (authored-target "Output")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery::BatteryOutput::voltage"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::electricPotential")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery::BatteryState"))) (kind specialization) (ordinal 0))
      (authored-target "StateSpace")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery::BatteryState::soc"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery::baseVoltage"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::electricPotential")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery::capacity"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::electricCharge")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery::internalResistance"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::resistance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery::socInit"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyAnalysis"))) (kind specialization) (ordinal 0))
      (authored-target "VehicleAnalysis")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleAnalysis")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyAnalysis::efficiencyRequirement"))) (kind featureTyping) (ordinal 0))
      (authored-target "EfficiencyRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyRequirement")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyRequirement"))) (kind specialization) (ordinal 0))
      (authored-target "VehicleRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleRequirement")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind requirement-def) (name "EfficiencyRequirement")) (anonymous (kind constraint) (ordinal 0)))))) (kind expressionOperand) (ordinal 0))
      (authored-target "actualEfficiency")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyRequirement::actualEfficiency")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind requirement-def) (name "EfficiencyRequirement")) (anonymous (kind constraint) (ordinal 0)))))) (kind expressionOperand) (ordinal 1))
      (authored-target "requiredEfficiency")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyRequirement::requiredEfficiency")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedAnalysis"))) (kind specialization) (ordinal 0))
      (authored-target "VehicleAnalysis")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleAnalysis")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedAnalysis::maxSpeedRequirement"))) (kind featureTyping) (ordinal 0))
      (authored-target "MaxSpeedRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedRequirement")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedRequirement"))) (kind specialization) (ordinal 0))
      (authored-target "VehicleRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleRequirement")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedRequirement::actualMaxSpeed"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::speed")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedRequirement::requiredMaxSpeed"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::speed")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::MotorInput"))) (kind specialization) (ordinal 0))
      (authored-target "Input")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::MotorInput::friction"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::torque")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::MotorInput::voltage"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::electricPotential")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::MotorOutput"))) (kind specialization) (ordinal 0))
      (authored-target "Output")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::MotorOutput::current"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::electricCurrent")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::MotorOutput::torque"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::torque")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::MotorState"))) (kind specialization) (ordinal 0))
      (authored-target "StateSpace")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::MotorState::current"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::electricCurrent")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::motL"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::inductance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::motR"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::resistance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::torquePerCurrent"))) (kind subsetting) (ordinal 0))
      (authored-target "Quantities::scalarQuantities")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::torquePerCurrent"))) (kind expressionOperand) (ordinal 0))
      (authored-target "ISQ::torque")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::torquePerCurrent"))) (kind expressionOperand) (ordinal 1))
      (authored-target "ISQ::electricCurrent")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeAnalysis"))) (kind specialization) (ordinal 0))
      (authored-target "VehicleAnalysis")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleAnalysis")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeAnalysis::rangeRequirement"))) (kind featureTyping) (ordinal 0))
      (authored-target "RangeRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeRequirement")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeAnalysis::simulatedRange"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeRequirement"))) (kind specialization) (ordinal 0))
      (authored-target "VehicleRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleRequirement")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind requirement-def) (name "RangeRequirement")) (anonymous (kind constraint) (ordinal 0)))))) (kind expressionOperand) (ordinal 0))
      (authored-target "actualRange")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeRequirement::actualRange")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind requirement-def) (name "RangeRequirement")) (anonymous (kind constraint) (ordinal 0)))))) (kind expressionOperand) (ordinal 1))
      (authored-target "requiredRange")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeRequirement::requiredRange")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeRequirement::actualRange"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeRequirement::requiredRange"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Tire::TireInput"))) (kind specialization) (ordinal 0))
      (authored-target "Input")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Tire::TireInput::accel"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::acceleration")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Tire::TireInput::torque"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::torque")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Tire::TireOutput"))) (kind specialization) (ordinal 0))
      (authored-target "Output")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Tire::TireOutput::force"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::force")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Tire::TireOutput::outTorque"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::torque")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Tire::moment"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::momentOfInertia")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Tire::radius"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::length")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle::VehicleInput"))) (kind specialization) (ordinal 0))
      (authored-target "Input")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle::VehicleInput::force"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::force")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle::VehicleOutput"))) (kind specialization) (ordinal 0))
      (authored-target "Output")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle::VehicleOutput::accel"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::acceleration")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle::VehicleOutput::distance"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::distance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle::VehicleOutput::velocity"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::speed")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle::VehicleState"))) (kind specialization) (ordinal 0))
      (authored-target "StateSpace")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle::VehicleState::distance"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::distance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle::VehicleState::velocity"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::speed")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle::mass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleAnalysis::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleAnalysis::vehicleRequirement"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleRequirement")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleRequirement::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::ampere hour"))) (kind featureTyping) (ordinal 0))
      (authored-target "ElectricChargeUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::ampere hour"))) (kind expressionOperand) (ordinal 0))
      (authored-target "A")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::ampere hour"))) (kind expressionOperand) (ordinal 1))
      (authored-target "h")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::efficiencyAnalysisLarge"))) (kind featureTyping) (ordinal 0))
      (authored-target "EfficiencyAnalysis")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyAnalysis")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::efficiencyAnalysisLarge"))) (kind subsetting) (ordinal 0))
      (authored-target "largeEVAnalysis")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::largeEVAnalysis")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::efficiencyAnalysisLarge::simulatedEfficiency"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "vehicle::efficiency")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::efficiencyRequirementLarge"))) (kind featureTyping) (ordinal 0))
      (authored-target "EfficiencyRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyRequirement")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::efficiencyRequirementLarge"))) (kind subsetting) (ordinal 0))
      (authored-target "largeEVRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::largeEVRequirement")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "largeEVRangeContext")) (named (kind requirement) (name "efficiencyRequirementLarge")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "requiredEfficiency")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyRequirement::requiredEfficiency")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::largeEVAnalysis"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleAnalysis")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleAnalysis")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::largeEVRequirement"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleRequirement")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "largeEVRangeContext")) (named (kind requirement) (name "largeEVRequirement")) (anonymous (kind constraint) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "vehicle::mass")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle::mass")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::maxSpeedAnalysisLarge"))) (kind featureTyping) (ordinal 0))
      (authored-target "MaxSpeedAnalysis")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedAnalysis")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::maxSpeedAnalysisLarge"))) (kind subsetting) (ordinal 0))
      (authored-target "largeEVAnalysis")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::largeEVAnalysis")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::maxSpeedAnalysisLarge::simulatedMaxSpeed"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "vehicle::vehicleBehavior::output::velocity")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::maxSpeedRequirementLarge"))) (kind featureTyping) (ordinal 0))
      (authored-target "MaxSpeedRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedRequirement")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::maxSpeedRequirementLarge"))) (kind subsetting) (ordinal 0))
      (authored-target "largeEVRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::largeEVRequirement")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "largeEVRangeContext")) (named (kind requirement) (name "maxSpeedRequirementLarge")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "requiredMaxSpeed")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedRequirement::requiredMaxSpeed")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::rangeAnalysisLarge"))) (kind featureTyping) (ordinal 0))
      (authored-target "RangeAnalysis")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeAnalysis")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::rangeAnalysisLarge"))) (kind subsetting) (ordinal 0))
      (authored-target "largeEVAnalysis")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::largeEVAnalysis")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::rangeAnalysisLarge::simulatedRange"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "vehicle::vehicleBehavior::output::distance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::rangeRequirementLarge"))) (kind featureTyping) (ordinal 0))
      (authored-target "RangeRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeRequirement")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::rangeRequirementLarge"))) (kind subsetting) (ordinal 0))
      (authored-target "largeEVRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::largeEVRequirement")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "largeEVRangeContext")) (named (kind requirement) (name "rangeRequirementLarge")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "requiredRange")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeRequirement::requiredRange")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::efficiencyAnalysisSmall"))) (kind featureTyping) (ordinal 0))
      (authored-target "EfficiencyAnalysis")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyAnalysis")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::efficiencyAnalysisSmall"))) (kind subsetting) (ordinal 0))
      (authored-target "smallEVAnalysis")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::smallEVAnalysis")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::efficiencyAnalysisSmall::simulatedEfficiency"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "vehicle::efficiency")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::efficiencyRequirementSmall"))) (kind featureTyping) (ordinal 0))
      (authored-target "EfficiencyRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyRequirement")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::efficiencyRequirementSmall"))) (kind subsetting) (ordinal 0))
      (authored-target "smallEVRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::smallEVRequirement")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "smallEVRangeContext")) (named (kind requirement) (name "efficiencyRequirementSmall")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "requiredEfficiency")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyRequirement::requiredEfficiency")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::maxSpeedAnalysisSmall"))) (kind featureTyping) (ordinal 0))
      (authored-target "MaxSpeedAnalysis")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedAnalysis")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::maxSpeedAnalysisSmall"))) (kind subsetting) (ordinal 0))
      (authored-target "smallEVAnalysis")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::smallEVAnalysis")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::maxSpeedAnalysisSmall::simulatedMaxSpeed"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "vehicle::vehicleBehavior::output::velocity")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::maxSpeedRequirementSmall"))) (kind featureTyping) (ordinal 0))
      (authored-target "MaxSpeedRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedRequirement")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::maxSpeedRequirementSmall"))) (kind subsetting) (ordinal 0))
      (authored-target "smallEVRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::smallEVRequirement")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "smallEVRangeContext")) (named (kind requirement) (name "maxSpeedRequirementSmall")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "requiredMaxSpeed")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedRequirement::requiredMaxSpeed")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::rangeAnalysisSmall"))) (kind featureTyping) (ordinal 0))
      (authored-target "RangeAnalysis")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeAnalysis")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::rangeAnalysisSmall"))) (kind subsetting) (ordinal 0))
      (authored-target "smallEVAnalysis")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::smallEVAnalysis")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::rangeAnalysisSmall::simulatedRange"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "vehicle::vehicleBehavior::output::distance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::rangeRequirementSmall"))) (kind featureTyping) (ordinal 0))
      (authored-target "RangeRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeRequirement")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::rangeRequirementSmall"))) (kind subsetting) (ordinal 0))
      (authored-target "smallEVRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::smallEVRequirement")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "smallEVRangeContext")) (named (kind requirement) (name "rangeRequirementSmall")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "requiredRange")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeRequirement::requiredRange")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::smallEVAnalysis"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleAnalysis")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleAnalysis")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::smallEVRequirement"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleRequirement")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "smallEVRangeContext")) (named (kind requirement) (name "smallEVRequirement")) (anonymous (kind constraint) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "vehicle::mass")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle::mass")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "mass")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle::mass")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::battery"))) (kind featureTyping) (ordinal 0))
      (authored-target "Battery")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "battery")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "baseVoltage")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery::baseVoltage")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "battery")) (anonymous (kind attribute) (ordinal 1)))))) (kind redefinition) (ordinal 0))
      (authored-target "capacity")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery::capacity")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "battery")) (anonymous (kind attribute) (ordinal 2)))))) (kind redefinition) (ordinal 0))
      (authored-target "socInit")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery::socInit")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "battery")) (anonymous (kind attribute) (ordinal 3)))))) (kind redefinition) (ordinal 0))
      (authored-target "internalResistance")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery::internalResistance")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::battery::batteryBehavior"))) (kind featureTyping) (ordinal 0))
      (authored-target "ContinuousStateSpaceDynamics")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::battery::batteryBehavior::input"))) (kind featureTyping) (ordinal 0))
      (authored-target "BatteryInput")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::battery::batteryBehavior::output"))) (kind featureTyping) (ordinal 0))
      (authored-target "BatteryOutput")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::motor"))) (kind featureTyping) (ordinal 0))
      (authored-target "Motor")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "motor")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "motR")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::motR")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "motor")) (anonymous (kind attribute) (ordinal 1)))))) (kind redefinition) (ordinal 0))
      (authored-target "motL")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::motL")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::motor::motorBehavior"))) (kind featureTyping) (ordinal 0))
      (authored-target "ContinuousStateSpaceDynamics")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::motor::motorBehavior::input"))) (kind featureTyping) (ordinal 0))
      (authored-target "MotorInput")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::motor::motorBehavior::output"))) (kind featureTyping) (ordinal 0))
      (authored-target "MotorOutput")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::tire"))) (kind featureTyping) (ordinal 0))
      (authored-target "Tire")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Tire")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "tire")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "moment")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Tire::moment")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "tire")) (anonymous (kind attribute) (ordinal 1)))))) (kind redefinition) (ordinal 0))
      (authored-target "radius")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Tire::radius")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::tire::tireBehavior"))) (kind featureTyping) (ordinal 0))
      (authored-target "ContinuousStateSpaceDynamics")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::tire::tireBehavior::input"))) (kind featureTyping) (ordinal 0))
      (authored-target "TireInput")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::tire::tireBehavior::output"))) (kind featureTyping) (ordinal 0))
      (authored-target "TireOutput")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::vehicleBehavior"))) (kind featureTyping) (ordinal 0))
      (authored-target "ContinuousStateSpaceDynamics")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::vehicleBehavior::input"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleInput")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::vehicleBehavior::output"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleOutput")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle_compact"))) (kind subsetting) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle_compact")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle_compact")) (anonymous (kind part) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "tire")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle_compact")) (anonymous (kind part) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "moment")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle_compact")) (anonymous (kind part) (ordinal 0)) (anonymous (kind attribute) (ordinal 1)))))) (kind redefinition) (ordinal 0))
      (authored-target "radius")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle_large"))) (kind subsetting) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle")))))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle_large")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle_large")) (anonymous (kind part) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "tire")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle_large")) (anonymous (kind part) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "moment")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle_large")) (anonymous (kind part) (ordinal 0)) (anonymous (kind attribute) (ordinal 1)))))) (kind redefinition) (ordinal 0))
      (authored-target "radius")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyAnalysis"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleAnalysis"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyAnalysis"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyAnalysis::efficiencyRequirement"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyAnalysis::efficiencyRequirement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyRequirement"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyRequirement"))) (kind specialization) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind requirement-def) (name "EfficiencyRequirement")) (anonymous (kind constraint) (ordinal 0)))))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyRequirement::actualEfficiency"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind requirement-def) (name "EfficiencyRequirement")) (anonymous (kind constraint) (ordinal 0)))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind requirement-def) (name "EfficiencyRequirement")) (anonymous (kind constraint) (ordinal 0)))))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyRequirement::requiredEfficiency"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind requirement-def) (name "EfficiencyRequirement")) (anonymous (kind constraint) (ordinal 0)))))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedAnalysis"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleAnalysis"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedAnalysis"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedAnalysis::maxSpeedRequirement"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedAnalysis::maxSpeedRequirement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedRequirement"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedRequirement"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeAnalysis"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleAnalysis"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeAnalysis"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeAnalysis::rangeRequirement"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeAnalysis::rangeRequirement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeRequirement"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeRequirement"))) (kind specialization) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind requirement-def) (name "RangeRequirement")) (anonymous (kind constraint) (ordinal 0)))))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeRequirement::actualRange"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind requirement-def) (name "RangeRequirement")) (anonymous (kind constraint) (ordinal 0)))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind requirement-def) (name "RangeRequirement")) (anonymous (kind constraint) (ordinal 0)))))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeRequirement::requiredRange"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind requirement-def) (name "RangeRequirement")) (anonymous (kind constraint) (ordinal 0)))))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleAnalysis::vehicle"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleAnalysis::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleAnalysis::vehicleRequirement"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleAnalysis::vehicleRequirement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleRequirement::vehicle"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleRequirement::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::efficiencyAnalysisLarge"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyAnalysis"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::efficiencyAnalysisLarge"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::efficiencyAnalysisLarge"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::largeEVAnalysis"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::efficiencyAnalysisLarge"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::efficiencyRequirementLarge"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::efficiencyRequirementLarge"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::efficiencyRequirementLarge"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::largeEVRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::efficiencyRequirementLarge"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "largeEVRangeContext")) (named (kind requirement) (name "efficiencyRequirementLarge")) (anonymous (kind attribute) (ordinal 0)))))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyRequirement::requiredEfficiency"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "largeEVRangeContext")) (named (kind requirement) (name "efficiencyRequirementLarge")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::largeEVAnalysis"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleAnalysis"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::largeEVAnalysis"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::largeEVRequirement"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::largeEVRequirement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "largeEVRangeContext")) (named (kind requirement) (name "largeEVRequirement")) (anonymous (kind constraint) (ordinal 0)))))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle::mass"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "largeEVRangeContext")) (named (kind requirement) (name "largeEVRequirement")) (anonymous (kind constraint) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::maxSpeedAnalysisLarge"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedAnalysis"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::maxSpeedAnalysisLarge"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::maxSpeedAnalysisLarge"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::largeEVAnalysis"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::maxSpeedAnalysisLarge"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::maxSpeedRequirementLarge"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::maxSpeedRequirementLarge"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::maxSpeedRequirementLarge"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::largeEVRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::maxSpeedRequirementLarge"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "largeEVRangeContext")) (named (kind requirement) (name "maxSpeedRequirementLarge")) (anonymous (kind attribute) (ordinal 0)))))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedRequirement::requiredMaxSpeed"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "largeEVRangeContext")) (named (kind requirement) (name "maxSpeedRequirementLarge")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::rangeAnalysisLarge"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeAnalysis"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::rangeAnalysisLarge"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::rangeAnalysisLarge"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::largeEVAnalysis"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::rangeAnalysisLarge"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::rangeRequirementLarge"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::rangeRequirementLarge"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::rangeRequirementLarge"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::largeEVRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::rangeRequirementLarge"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "largeEVRangeContext")) (named (kind requirement) (name "rangeRequirementLarge")) (anonymous (kind attribute) (ordinal 0)))))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeRequirement::requiredRange"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "largeEVRangeContext")) (named (kind requirement) (name "rangeRequirementLarge")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::efficiencyAnalysisSmall"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyAnalysis"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::efficiencyAnalysisSmall"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::efficiencyAnalysisSmall"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::smallEVAnalysis"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::efficiencyAnalysisSmall"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::efficiencyRequirementSmall"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::efficiencyRequirementSmall"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::efficiencyRequirementSmall"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::smallEVRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::efficiencyRequirementSmall"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "smallEVRangeContext")) (named (kind requirement) (name "efficiencyRequirementSmall")) (anonymous (kind attribute) (ordinal 0)))))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyRequirement::requiredEfficiency"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "smallEVRangeContext")) (named (kind requirement) (name "efficiencyRequirementSmall")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::maxSpeedAnalysisSmall"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedAnalysis"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::maxSpeedAnalysisSmall"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::maxSpeedAnalysisSmall"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::smallEVAnalysis"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::maxSpeedAnalysisSmall"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::maxSpeedRequirementSmall"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::maxSpeedRequirementSmall"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::maxSpeedRequirementSmall"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::smallEVRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::maxSpeedRequirementSmall"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "smallEVRangeContext")) (named (kind requirement) (name "maxSpeedRequirementSmall")) (anonymous (kind attribute) (ordinal 0)))))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedRequirement::requiredMaxSpeed"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "smallEVRangeContext")) (named (kind requirement) (name "maxSpeedRequirementSmall")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::rangeAnalysisSmall"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeAnalysis"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::rangeAnalysisSmall"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::rangeAnalysisSmall"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::smallEVAnalysis"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::rangeAnalysisSmall"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::rangeRequirementSmall"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::rangeRequirementSmall"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::rangeRequirementSmall"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::smallEVRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::rangeRequirementSmall"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "smallEVRangeContext")) (named (kind requirement) (name "rangeRequirementSmall")) (anonymous (kind attribute) (ordinal 0)))))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeRequirement::requiredRange"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "smallEVRangeContext")) (named (kind requirement) (name "rangeRequirementSmall")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::smallEVAnalysis"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleAnalysis"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::smallEVAnalysis"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::smallEVRequirement"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::smallEVRequirement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "smallEVRangeContext")) (named (kind requirement) (name "smallEVRequirement")) (anonymous (kind constraint) (ordinal 0)))))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle::mass"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "smallEVRangeContext")) (named (kind requirement) (name "smallEVRequirement")) (anonymous (kind constraint) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (anonymous (kind attribute) (ordinal 0)))))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle::mass"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::battery"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::battery"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "battery")) (anonymous (kind attribute) (ordinal 0)))))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery::baseVoltage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "battery")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "battery")) (anonymous (kind attribute) (ordinal 1)))))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery::capacity"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "battery")) (anonymous (kind attribute) (ordinal 1)))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "battery")) (anonymous (kind attribute) (ordinal 2)))))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery::socInit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "battery")) (anonymous (kind attribute) (ordinal 2)))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "battery")) (anonymous (kind attribute) (ordinal 3)))))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery::internalResistance"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "battery")) (anonymous (kind attribute) (ordinal 3)))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::motor"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::motor"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "motor")) (anonymous (kind attribute) (ordinal 0)))))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::motR"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "motor")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "motor")) (anonymous (kind attribute) (ordinal 1)))))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::motL"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "motor")) (anonymous (kind attribute) (ordinal 1)))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::tire"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Tire"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::tire"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "tire")) (anonymous (kind attribute) (ordinal 0)))))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Tire::moment"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "tire")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "tire")) (anonymous (kind attribute) (ordinal 1)))))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Tire::radius"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "tire")) (anonymous (kind attribute) (ordinal 1)))))) (kind redefinition) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle_compact"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle_compact"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle_large"))) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle_large"))) (kind subsetting) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind requirement-def) (name "EfficiencyRequirement")) (anonymous (kind constraint) (ordinal 0)))))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::torquePerCurrent"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind requirement-def) (name "RangeRequirement")) (anonymous (kind constraint) (ordinal 0)))))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::ampere hour"))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "largeEVRangeContext")) (named (kind requirement) (name "efficiencyRequirementLarge")) (anonymous (kind attribute) (ordinal 0)))))) (value (kind real) (real 0.8)))
    (evaluated (declaration (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "largeEVRangeContext")) (named (kind requirement) (name "maxSpeedRequirementLarge")) (anonymous (kind attribute) (ordinal 0)))))) (value (kind quantity) (magnitude (value (kind integer) (integer 140))) (unit "km/h")))
    (evaluated (declaration (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "largeEVRangeContext")) (named (kind requirement) (name "rangeRequirementLarge")) (anonymous (kind attribute) (ordinal 0)))))) (value (kind quantity) (magnitude (value (kind integer) (integer 200))) (unit "km")))
    (evaluated (declaration (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "smallEVRangeContext")) (named (kind requirement) (name "efficiencyRequirementSmall")) (anonymous (kind attribute) (ordinal 0)))))) (value (kind real) (real 0.9)))
    (evaluated (declaration (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "smallEVRangeContext")) (named (kind requirement) (name "maxSpeedRequirementSmall")) (anonymous (kind attribute) (ordinal 0)))))) (value (kind quantity) (magnitude (value (kind integer) (integer 130))) (unit "km/h")))
    (evaluated (declaration (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "smallEVRangeContext")) (named (kind requirement) (name "rangeRequirementSmall")) (anonymous (kind attribute) (ordinal 0)))))) (value (kind quantity) (magnitude (value (kind integer) (integer 130))) (unit "km")))
    (evaluated (declaration (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (anonymous (kind attribute) (ordinal 0)))))) (value (kind quantity) (magnitude (value (kind integer) (integer 1000))) (unit "kg")))
    (evaluated (declaration (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::airFrictionCoefficient"))) (value (kind real) (real 0.2)))
    (evaluated (declaration (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "battery")) (anonymous (kind attribute) (ordinal 0)))))) (value (kind quantity) (magnitude (value (kind integer) (integer 300))) (unit "V")))
    (evaluated (declaration (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "battery")) (anonymous (kind attribute) (ordinal 1)))))) (value (kind quantity) (magnitude (value (kind integer) (integer 50))) (unit "A⋅h")))
    (evaluated (declaration (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "battery")) (anonymous (kind attribute) (ordinal 2)))))) (value (kind real) (real 0.8)))
    (evaluated (declaration (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "battery")) (anonymous (kind attribute) (ordinal 3)))))) (value (kind quantity) (magnitude (value (kind real) (real 1.8))) (unit "Ω")))
    (evaluated (declaration (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "motor")) (anonymous (kind attribute) (ordinal 0)))))) (value (kind quantity) (magnitude (value (kind integer) (integer 4))) (unit "Ω")))
    (evaluated (declaration (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "motor")) (anonymous (kind attribute) (ordinal 1)))))) (value (kind quantity) (magnitude (value (kind real) (real 0.2))) (unit "H")))
    (evaluated (declaration (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "tire")) (anonymous (kind attribute) (ordinal 0)))))) (value (kind quantity) (magnitude (value (kind integer) (integer 300))) (unit "kg⋅m²")))
    (evaluated (declaration (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "tire")) (anonymous (kind attribute) (ordinal 1)))))) (value (kind quantity) (magnitude (value (kind real) (real 0.7))) (unit "m")))
    (evaluated (declaration (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle_compact")) (anonymous (kind attribute) (ordinal 0)))))) (value (kind quantity) (magnitude (value (kind integer) (integer 800))) (unit "kg")))
    (evaluated (declaration (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle_compact")) (anonymous (kind part) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)))))) (value (kind quantity) (magnitude (value (kind integer) (integer 200))) (unit "kg⋅m²")))
    (evaluated (declaration (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle_compact")) (anonymous (kind part) (ordinal 0)) (anonymous (kind attribute) (ordinal 1)))))) (value (kind quantity) (magnitude (value (kind real) (real 0.5))) (unit "m")))
    (evaluated (declaration (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle_large")) (anonymous (kind attribute) (ordinal 0)))))) (value (kind quantity) (magnitude (value (kind integer) (integer 1100))) (unit "kg")))
    (evaluated (declaration (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle_large")) (anonymous (kind part) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)))))) (value (kind quantity) (magnitude (value (kind integer) (integer 300))) (unit "kg⋅m²")))
    (evaluated (declaration (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle_large")) (anonymous (kind part) (ordinal 0)) (anonymous (kind attribute) (ordinal 1)))))) (value (kind quantity) (magnitude (value (kind real) (real 0.7))) (unit "m")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/evsample.md") (range (start 2 19) (end 2 24)) (probe (position 2 19))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0) (authored-target "SI")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 3 19) (end 3 46)) (probe (position 3 19))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (anonymous (kind import) (ordinal 1)))))) (kind namespaceImport) (ordinal 0) (authored-target "StateSpaceRepresentation")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 32 38) (end 32 43)) (probe (position 32 38))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery::BatteryInput"))) (kind specialization) (ordinal 0) (authored-target "Input")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 33 33) (end 33 53)) (probe (position 33 33))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery::BatteryInput::current"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::electricCurrent")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 36 39) (end 36 45)) (probe (position 36 39))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery::BatteryOutput"))) (kind specialization) (ordinal 0) (authored-target "Output")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 37 33) (end 37 55)) (probe (position 37 33))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery::BatteryOutput::voltage"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::electricPotential")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 40 38) (end 40 48)) (probe (position 40 38))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery::BatteryState"))) (kind specialization) (ordinal 0) (authored-target "StateSpace")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 41 27) (end 41 45)) (probe (position 41 27))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery::BatteryState::soc"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 27 33) (end 27 55)) (probe (position 27 33))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery::baseVoltage"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::electricPotential")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 29 30) (end 29 49)) (probe (position 29 30))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery::capacity"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::electricCharge")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 30 40) (end 30 55)) (probe (position 30 40))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery::internalResistance"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::resistance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 28 27) (end 28 45)) (probe (position 28 27))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery::socInit"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 123 39) (end 123 54)) (probe (position 123 39))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyAnalysis"))) (kind specialization) (ordinal 0) (authored-target "VehicleAnalysis")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleAnalysis")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 126 67) (end 126 88)) (probe (position 126 67))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyAnalysis::efficiencyRequirement"))) (kind featureTyping) (ordinal 0) (authored-target "EfficiencyRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyRequirement")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 115 45) (end 115 63)) (probe (position 115 45))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyRequirement"))) (kind specialization) (ordinal 0) (authored-target "VehicleRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleRequirement")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 120 29) (end 120 45)) (probe (position 120 29))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind requirement-def) (name "EfficiencyRequirement")) (anonymous (kind constraint) (ordinal 0)))))) (kind expressionOperand) (ordinal 0) (authored-target "actualEfficiency")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyRequirement::actualEfficiency")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 120 49) (end 120 67)) (probe (position 120 49))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind requirement-def) (name "EfficiencyRequirement")) (anonymous (kind constraint) (ordinal 0)))))) (kind expressionOperand) (ordinal 1) (authored-target "requiredEfficiency")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyRequirement::requiredEfficiency")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 141 37) (end 141 52)) (probe (position 141 37))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedAnalysis"))) (kind specialization) (ordinal 0) (authored-target "VehicleAnalysis")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleAnalysis")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 144 65) (end 144 84)) (probe (position 144 65))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedAnalysis::maxSpeedRequirement"))) (kind featureTyping) (ordinal 0) (authored-target "MaxSpeedRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedRequirement")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 135 43) (end 135 61)) (probe (position 135 43))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedRequirement"))) (kind specialization) (ordinal 0) (authored-target "VehicleRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleRequirement")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 137 36) (end 137 46)) (probe (position 137 36))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedRequirement::actualMaxSpeed"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::speed")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 138 38) (end 138 48)) (probe (position 138 38))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedRequirement::requiredMaxSpeed"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::speed")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 52 36) (end 52 41)) (probe (position 52 36))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::MotorInput"))) (kind specialization) (ordinal 0) (authored-target "Input")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 54 34) (end 54 45)) (probe (position 54 34))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::MotorInput::friction"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::torque")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 53 33) (end 53 55)) (probe (position 53 33))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::MotorInput::voltage"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::electricPotential")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 57 37) (end 57 43)) (probe (position 57 37))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::MotorOutput"))) (kind specialization) (ordinal 0) (authored-target "Output")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 58 33) (end 58 53)) (probe (position 58 33))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::MotorOutput::current"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::electricCurrent")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 59 32) (end 59 43)) (probe (position 59 32))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::MotorOutput::torque"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::torque")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 62 36) (end 62 46)) (probe (position 62 36))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::MotorState"))) (kind specialization) (ordinal 0) (authored-target "StateSpace")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 63 33) (end 63 53)) (probe (position 63 33))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::MotorState::current"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::electricCurrent")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 50 26) (end 50 41)) (probe (position 50 26))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::motL"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::inductance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 49 26) (end 49 41)) (probe (position 49 26))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::motR"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::resistance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 47 28) (end 47 56)) (probe (position 47 28))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::torquePerCurrent"))) (kind subsetting) (ordinal 0) (authored-target "Quantities::scalarQuantities")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 47 59) (end 47 70)) (probe (position 47 59))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::torquePerCurrent"))) (kind expressionOperand) (ordinal 0) (authored-target "ISQ::torque")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 47 73) (end 47 93)) (probe (position 47 73))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::torquePerCurrent"))) (kind expressionOperand) (ordinal 1) (authored-target "ISQ::electricCurrent")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 100 34) (end 100 49)) (probe (position 100 34))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeAnalysis"))) (kind specialization) (ordinal 0) (authored-target "VehicleAnalysis")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleAnalysis")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 103 62) (end 103 78)) (probe (position 103 62))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeAnalysis::rangeRequirement"))) (kind featureTyping) (ordinal 0) (authored-target "RangeRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeRequirement")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 101 29) (end 101 40)) (probe (position 101 29))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeAnalysis::simulatedRange"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 92 40) (end 92 58)) (probe (position 92 40))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeRequirement"))) (kind specialization) (ordinal 0) (authored-target "VehicleRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleRequirement")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 97 29) (end 97 40)) (probe (position 97 29))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind requirement-def) (name "RangeRequirement")) (anonymous (kind constraint) (ordinal 0)))))) (kind expressionOperand) (ordinal 0) (authored-target "actualRange")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeRequirement::actualRange")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 97 44) (end 97 57)) (probe (position 97 44))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind requirement-def) (name "RangeRequirement")) (anonymous (kind constraint) (ordinal 0)))))) (kind expressionOperand) (ordinal 1) (authored-target "requiredRange")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeRequirement::requiredRange")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 94 32) (end 94 43)) (probe (position 94 32))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeRequirement::actualRange"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 95 34) (end 95 45)) (probe (position 95 34))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeRequirement::requiredRange"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 71 35) (end 71 40)) (probe (position 71 35))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Tire::TireInput"))) (kind specialization) (ordinal 0) (authored-target "Input")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 73 31) (end 73 48)) (probe (position 73 31))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Tire::TireInput::accel"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::acceleration")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 72 32) (end 72 43)) (probe (position 72 32))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Tire::TireInput::torque"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::torque")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 76 36) (end 76 42)) (probe (position 76 36))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Tire::TireOutput"))) (kind specialization) (ordinal 0) (authored-target "Output")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 77 31) (end 77 41)) (probe (position 77 31))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Tire::TireOutput::force"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::force")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 78 35) (end 78 46)) (probe (position 78 35))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Tire::TireOutput::outTorque"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::torque")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 69 28) (end 69 48)) (probe (position 69 28))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Tire::moment"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::momentOfInertia")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 68 28) (end 68 39)) (probe (position 68 28))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Tire::radius"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::length")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 10 38) (end 10 43)) (probe (position 10 38))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle::VehicleInput"))) (kind specialization) (ordinal 0) (authored-target "Input")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 11 31) (end 11 41)) (probe (position 11 31))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle::VehicleInput::force"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::force")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 14 39) (end 14 45)) (probe (position 14 39))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle::VehicleOutput"))) (kind specialization) (ordinal 0) (authored-target "Output")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 15 31) (end 15 48)) (probe (position 15 31))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle::VehicleOutput::accel"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::acceleration")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 17 34) (end 17 47)) (probe (position 17 34))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle::VehicleOutput::distance"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::distance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 16 34) (end 16 44)) (probe (position 16 34))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle::VehicleOutput::velocity"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::speed")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 20 38) (end 20 48)) (probe (position 20 38))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle::VehicleState"))) (kind specialization) (ordinal 0) (authored-target "StateSpace")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 22 34) (end 22 47)) (probe (position 22 34))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle::VehicleState::distance"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::distance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 21 34) (end 21 44)) (probe (position 21 34))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle::VehicleState::velocity"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::speed")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 8 26) (end 8 35)) (probe (position 8 26))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle::mass"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 87 26) (end 87 33)) (probe (position 87 26))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleAnalysis::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 88 41) (end 88 59)) (probe (position 88 41))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleAnalysis::vehicleRequirement"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleRequirement")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 83 26) (end 83 33)) (probe (position 83 26))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleRequirement::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 5 41) (end 5 59)) (probe (position 5 41))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::ampere hour"))) (kind featureTyping) (ordinal 0) (authored-target "ElectricChargeUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 5 62) (end 5 63)) (probe (position 5 62))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::ampere hour"))) (kind expressionOperand) (ordinal 0) (authored-target "A")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 5 64) (end 5 65)) (probe (position 5 64))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::ampere hour"))) (kind expressionOperand) (ordinal 1) (authored-target "h")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 300 62) (end 300 80)) (probe (position 300 62))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::efficiencyAnalysisLarge"))) (kind featureTyping) (ordinal 0) (authored-target "EfficiencyAnalysis")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyAnalysis")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 300 44) (end 300 59)) (probe (position 300 44))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::efficiencyAnalysisLarge"))) (kind subsetting) (ordinal 0) (authored-target "largeEVAnalysis")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::largeEVAnalysis")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 303 41) (end 303 59)) (probe (position 303 41))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::efficiencyAnalysisLarge::simulatedEfficiency"))) (kind memberAccessOperand) (ordinal 0) (authored-target "vehicle::efficiency")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 295 76) (end 295 97)) (probe (position 295 76))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::efficiencyRequirementLarge"))) (kind featureTyping) (ordinal 0) (authored-target "EfficiencyRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyRequirement")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 295 55) (end 295 73)) (probe (position 295 55))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::efficiencyRequirementLarge"))) (kind subsetting) (ordinal 0) (authored-target "largeEVRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::largeEVRequirement")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 297 26) (end 297 44)) (probe (position 297 26))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "largeEVRangeContext")) (named (kind requirement) (name "efficiencyRequirementLarge")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "requiredEfficiency")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyRequirement::requiredEfficiency")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 280 35) (end 280 50)) (probe (position 280 35))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::largeEVAnalysis"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleAnalysis")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleAnalysis")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 273 41) (end 273 59)) (probe (position 273 41))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::largeEVRequirement"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleRequirement")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 277 32) (end 277 44)) (probe (position 277 32))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "largeEVRangeContext")) (named (kind requirement) (name "largeEVRequirement")) (anonymous (kind constraint) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 0) (authored-target "vehicle::mass")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle::mass")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 311 60) (end 311 76)) (probe (position 311 60))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::maxSpeedAnalysisLarge"))) (kind featureTyping) (ordinal 0) (authored-target "MaxSpeedAnalysis")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedAnalysis")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 311 42) (end 311 57)) (probe (position 311 42))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::maxSpeedAnalysisLarge"))) (kind subsetting) (ordinal 0) (authored-target "largeEVAnalysis")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::largeEVAnalysis")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 315 39) (end 315 78)) (probe (position 315 39))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::maxSpeedAnalysisLarge::simulatedMaxSpeed"))) (kind memberAccessOperand) (ordinal 0) (authored-target "vehicle::vehicleBehavior::output::velocity")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 306 74) (end 306 93)) (probe (position 306 74))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::maxSpeedRequirementLarge"))) (kind featureTyping) (ordinal 0) (authored-target "MaxSpeedRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedRequirement")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 306 53) (end 306 71)) (probe (position 306 53))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::maxSpeedRequirementLarge"))) (kind subsetting) (ordinal 0) (authored-target "largeEVRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::largeEVRequirement")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 308 26) (end 308 42)) (probe (position 308 26))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "largeEVRangeContext")) (named (kind requirement) (name "maxSpeedRequirementLarge")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "requiredMaxSpeed")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedRequirement::requiredMaxSpeed")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 290 57) (end 290 70)) (probe (position 290 57))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::rangeAnalysisLarge"))) (kind featureTyping) (ordinal 0) (authored-target "RangeAnalysis")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeAnalysis")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 290 39) (end 290 54)) (probe (position 290 39))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::rangeAnalysisLarge"))) (kind subsetting) (ordinal 0) (authored-target "largeEVAnalysis")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::largeEVAnalysis")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 292 36) (end 292 75)) (probe (position 292 36))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::rangeAnalysisLarge::simulatedRange"))) (kind memberAccessOperand) (ordinal 0) (authored-target "vehicle::vehicleBehavior::output::distance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 285 71) (end 285 87)) (probe (position 285 71))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::rangeRequirementLarge"))) (kind featureTyping) (ordinal 0) (authored-target "RangeRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeRequirement")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 285 50) (end 285 68)) (probe (position 285 50))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::rangeRequirementLarge"))) (kind subsetting) (ordinal 0) (authored-target "largeEVRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::largeEVRangeContext::largeEVRequirement")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 287 26) (end 287 39)) (probe (position 287 26))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "largeEVRangeContext")) (named (kind requirement) (name "rangeRequirementLarge")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "requiredRange")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeRequirement::requiredRange")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 245 62) (end 245 80)) (probe (position 245 62))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::efficiencyAnalysisSmall"))) (kind featureTyping) (ordinal 0) (authored-target "EfficiencyAnalysis")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyAnalysis")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 245 44) (end 245 59)) (probe (position 245 44))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::efficiencyAnalysisSmall"))) (kind subsetting) (ordinal 0) (authored-target "smallEVAnalysis")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::smallEVAnalysis")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 248 41) (end 248 59)) (probe (position 248 41))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::efficiencyAnalysisSmall::simulatedEfficiency"))) (kind memberAccessOperand) (ordinal 0) (authored-target "vehicle::efficiency")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 240 76) (end 240 97)) (probe (position 240 76))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::efficiencyRequirementSmall"))) (kind featureTyping) (ordinal 0) (authored-target "EfficiencyRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyRequirement")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 240 55) (end 240 73)) (probe (position 240 55))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::efficiencyRequirementSmall"))) (kind subsetting) (ordinal 0) (authored-target "smallEVRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::smallEVRequirement")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 242 26) (end 242 44)) (probe (position 242 26))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "smallEVRangeContext")) (named (kind requirement) (name "efficiencyRequirementSmall")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "requiredEfficiency")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::EfficiencyRequirement::requiredEfficiency")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 256 60) (end 256 76)) (probe (position 256 60))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::maxSpeedAnalysisSmall"))) (kind featureTyping) (ordinal 0) (authored-target "MaxSpeedAnalysis")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedAnalysis")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 256 42) (end 256 57)) (probe (position 256 42))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::maxSpeedAnalysisSmall"))) (kind subsetting) (ordinal 0) (authored-target "smallEVAnalysis")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::smallEVAnalysis")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 260 39) (end 260 78)) (probe (position 260 39))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::maxSpeedAnalysisSmall::simulatedMaxSpeed"))) (kind memberAccessOperand) (ordinal 0) (authored-target "vehicle::vehicleBehavior::output::velocity")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 251 74) (end 251 93)) (probe (position 251 74))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::maxSpeedRequirementSmall"))) (kind featureTyping) (ordinal 0) (authored-target "MaxSpeedRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedRequirement")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 251 53) (end 251 71)) (probe (position 251 53))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::maxSpeedRequirementSmall"))) (kind subsetting) (ordinal 0) (authored-target "smallEVRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::smallEVRequirement")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 253 26) (end 253 42)) (probe (position 253 26))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "smallEVRangeContext")) (named (kind requirement) (name "maxSpeedRequirementSmall")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "requiredMaxSpeed")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::MaxSpeedRequirement::requiredMaxSpeed")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 235 57) (end 235 70)) (probe (position 235 57))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::rangeAnalysisSmall"))) (kind featureTyping) (ordinal 0) (authored-target "RangeAnalysis")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeAnalysis")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 235 39) (end 235 54)) (probe (position 235 39))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::rangeAnalysisSmall"))) (kind subsetting) (ordinal 0) (authored-target "smallEVAnalysis")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::smallEVAnalysis")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 237 36) (end 237 75)) (probe (position 237 36))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::rangeAnalysisSmall::simulatedRange"))) (kind memberAccessOperand) (ordinal 0) (authored-target "vehicle::vehicleBehavior::output::distance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 230 71) (end 230 87)) (probe (position 230 71))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::rangeRequirementSmall"))) (kind featureTyping) (ordinal 0) (authored-target "RangeRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeRequirement")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 230 50) (end 230 68)) (probe (position 230 50))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::rangeRequirementSmall"))) (kind subsetting) (ordinal 0) (authored-target "smallEVRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::smallEVRequirement")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 232 26) (end 232 39)) (probe (position 232 26))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "smallEVRangeContext")) (named (kind requirement) (name "rangeRequirementSmall")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "requiredRange")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::RangeRequirement::requiredRange")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 225 35) (end 225 50)) (probe (position 225 35))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::smallEVAnalysis"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleAnalysis")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleAnalysis")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 218 41) (end 218 59)) (probe (position 218 41))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::smallEVRangeContext::smallEVRequirement"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::VehicleRequirement")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 222 32) (end 222 44)) (probe (position 222 32))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "smallEVRangeContext")) (named (kind requirement) (name "smallEVRequirement")) (anonymous (kind constraint) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 0) (authored-target "vehicle::mass")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle::mass")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 154 19) (end 154 26)) (probe (position 154 19))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 155 22) (end 155 26)) (probe (position 155 22))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "mass")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Vehicle::mass")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 169 22) (end 169 29)) (probe (position 169 22))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::battery"))) (kind featureTyping) (ordinal 0) (authored-target "Battery")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 170 16) (end 170 27)) (probe (position 170 16))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "battery")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "baseVoltage")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery::baseVoltage")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 171 16) (end 171 24)) (probe (position 171 16))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "battery")) (anonymous (kind attribute) (ordinal 1)))))) (kind redefinition) (ordinal 0) (authored-target "capacity")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery::capacity")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 172 16) (end 172 23)) (probe (position 172 16))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "battery")) (anonymous (kind attribute) (ordinal 2)))))) (kind redefinition) (ordinal 0) (authored-target "socInit")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery::socInit")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 173 16) (end 173 34)) (probe (position 173 16))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "battery")) (anonymous (kind attribute) (ordinal 3)))))) (kind redefinition) (ordinal 0) (authored-target "internalResistance")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Battery::internalResistance")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 174 37) (end 174 65)) (probe (position 174 37))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::battery::batteryBehavior"))) (kind featureTyping) (ordinal 0) (authored-target "ContinuousStateSpaceDynamics")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 175 27) (end 175 39)) (probe (position 175 27))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::battery::batteryBehavior::input"))) (kind featureTyping) (ordinal 0) (authored-target "BatteryInput")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 176 29) (end 176 42)) (probe (position 176 29))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::battery::batteryBehavior::output"))) (kind featureTyping) (ordinal 0) (authored-target "BatteryOutput")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 183 20) (end 183 25)) (probe (position 183 20))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::motor"))) (kind featureTyping) (ordinal 0) (authored-target "Motor")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 184 16) (end 184 20)) (probe (position 184 16))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "motor")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "motR")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::motR")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 185 16) (end 185 20)) (probe (position 185 16))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "motor")) (anonymous (kind attribute) (ordinal 1)))))) (kind redefinition) (ordinal 0) (authored-target "motL")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Motor::motL")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 187 35) (end 187 63)) (probe (position 187 35))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::motor::motorBehavior"))) (kind featureTyping) (ordinal 0) (authored-target "ContinuousStateSpaceDynamics")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 188 27) (end 188 37)) (probe (position 188 27))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::motor::motorBehavior::input"))) (kind featureTyping) (ordinal 0) (authored-target "MotorInput")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 189 29) (end 189 40)) (probe (position 189 29))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::motor::motorBehavior::output"))) (kind featureTyping) (ordinal 0) (authored-target "MotorOutput")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 196 19) (end 196 23)) (probe (position 196 19))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::tire"))) (kind featureTyping) (ordinal 0) (authored-target "Tire")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Tire")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 197 16) (end 197 22)) (probe (position 197 16))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "tire")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "moment")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Tire::moment")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 198 16) (end 198 22)) (probe (position 198 16))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle")) (named (kind part) (name "tire")) (anonymous (kind attribute) (ordinal 1)))))) (kind redefinition) (ordinal 0) (authored-target "radius")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::Tire::radius")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 199 34) (end 199 62)) (probe (position 199 34))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::tire::tireBehavior"))) (kind featureTyping) (ordinal 0) (authored-target "ContinuousStateSpaceDynamics")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 200 27) (end 200 36)) (probe (position 200 27))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::tire::tireBehavior::input"))) (kind featureTyping) (ordinal 0) (authored-target "TireInput")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 201 29) (end 201 39)) (probe (position 201 29))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::tire::tireBehavior::output"))) (kind featureTyping) (ordinal 0) (authored-target "TireOutput")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 163 33) (end 163 61)) (probe (position 163 33))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::vehicleBehavior"))) (kind featureTyping) (ordinal 0) (authored-target "ContinuousStateSpaceDynamics")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 164 23) (end 164 35)) (probe (position 164 23))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::vehicleBehavior::input"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleInput")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 165 25) (end 165 38)) (probe (position 165 25))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle::vehicleBehavior::output"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleOutput")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 209 28) (end 209 35)) (probe (position 209 28))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle_compact"))) (kind subsetting) (ordinal 0) (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 210 22) (end 210 26)) (probe (position 210 22))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle_compact")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "mass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 211 17) (end 211 21)) (probe (position 211 17))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle_compact")) (anonymous (kind part) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "tire")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 212 16) (end 212 22)) (probe (position 212 16))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle_compact")) (anonymous (kind part) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "moment")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 213 16) (end 213 22)) (probe (position 213 16))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle_compact")) (anonymous (kind part) (ordinal 0)) (anonymous (kind attribute) (ordinal 1)))))) (kind redefinition) (ordinal 0) (authored-target "radius")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 264 26) (end 264 33)) (probe (position 264 26))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle_large"))) (kind subsetting) (ordinal 0) (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/evsample.md") (qualified-name "EVSample::vehicle")))))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 265 22) (end 265 26)) (probe (position 265 22))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle_large")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "mass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 266 17) (end 266 21)) (probe (position 266 17))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle_large")) (anonymous (kind part) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "tire")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 267 16) (end 267 22)) (probe (position 267 16))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle_large")) (anonymous (kind part) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "moment")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/evsample.md") (range (start 268 16) (end 268 22)) (probe (position 268 16))
    (reference (id (source (node (document "memory://snapshot/evsample.md") (path (named (kind package) (name "EVSample")) (named (kind part) (name "vehicle_large")) (anonymous (kind part) (ordinal 0)) (anonymous (kind attribute) (ordinal 1)))))) (kind redefinition) (ordinal 0) (authored-target "radius")
      (outcome (status unresolved)))
  )
)
~~~
