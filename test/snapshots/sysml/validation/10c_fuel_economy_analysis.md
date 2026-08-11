# META
~~~ini
description=SysML Validation (10-Analysis and Trades): 10c-Fuel Economy Analysis
type=file
~~~
# SOURCE
~~~sysml
package '10c-Fuel Economy Analysis' {
	private import ScalarValues::*;
	private import Quantities::*;
	private import MeasurementReferences::*;
	private import ISQ::*;
	private import USCustomaryUnits::*;
	
	attribute distancePerVolume : ScalarQuantityValue = length / volume;	
	attribute gallon : MeasurementUnit = 231.0 * 'in'^3;
	
	package FuelEconomyRequirementsModel {
		
		requirement def FuelEconomyRequirement {
			attribute actualFuelEconomy :> distancePerVolume;
			attribute requiredFuelEconomy :> distancePerVolume;
			
			require constraint { actualFuelEconomy >= requiredFuelEconomy }
		}
		
		requirement cityFuelEconomyRequirement : FuelEconomyRequirement {
			:>> requiredFuelEconomy = 25 [mi/gallon];
		}
		
		requirement highwayFuelEconomyRequirement : FuelEconomyRequirement {
			:>> requiredFuelEconomy = 30 [mi/gallon];
		}
		
	}
		
	package VehicleDesignModel {
		
		part def Vehicle {
			attribute fuelEconomy_city :> distancePerVolume;
			attribute fuelEconomy_highway :> distancePerVolume;
			
			attribute cargoWeight : MassValue;
		}
		
		part def Engine;
		part def Transmission;
		
		part vehicle1_c1 : Vehicle {
			part engine : Engine;
			part transmission : Transmission {
				exhibit state transmissionState {
					entry; then '1stGear';
					state '1stGear';
					then '2ndGear';
					state '2ndGear';
					then '3rdGear';
					state '3rdGear';
					then '4thGear';
					state '4thGear';
				}
			}
		}
		
	}
	
	package FuelEconomyAnalysisModel {
		private import VehicleDesignModel::*;
		private import FuelEconomyRequirementsModel::*;
		
		attribute def ScenarioState {
			position : LengthValue;
			velocity : SpeedValue;
			acceleration : AccelerationValue;
			inclineAngle : AngularMeasureValue;
		}
		
		abstract calc def NominalScenario { 
			in t : TimeValue; 
			return : ScenarioState;
		}
		calc cityScenario : NominalScenario;
		calc highwayScenario : NominalScenario;
		
		analysis def FuelEconomyAnalysis {
			subject vehicle : Vehicle;
			in calc scenario : NominalScenario;
			in requirement fuelEconomyRequirement : FuelEconomyRequirement;
			return calculatedFuelEconomy : ScalarQuantityValue;
			
			objective fuelEconomyAnalysisObjective {
				doc /*
				     * The objective of this analysis is to determine whether the
				     * current vehicle design configuration can satisfy the fuel
				     * economy requirement.
				     */
				 
				 assume constraint {
				 	doc /* wheelDiameter == 33 inches
				 	     * drive train efficiency == 0.4
				 	     */
				 }
				 
				 require fuelEconomyRequirement {
				 	:>> actualFuelEconomy = calculatedFuelEconomy;
				 }
			}
			
			action dynamicsAnalysis {
				/*
				 * Solve for the required engine power as a function of time
				 * to support the nominal scenarios.
				 * 
				 * Note: Vehicle force = power/speed
				 * Note: EngineRPM * EngineGearRatio/WheelRPM = constant
				 */
			}
			
			action fuelConsumptionAnalysis {
				/*
				 * Solve the engine equations to determine how much fuel is
				 * consumed. The engine RPM is a function of the speed of the
				 * vehicle and the gear state.
				 */
			}
		}
		
		requirement vehicleFuelEconomyRequirementsGroup {
			subject vehicle : Vehicle;
			requirement vehicleFuelEconomyRequirement_city :> cityFuelEconomyRequirement {
				doc /* The vehicle shall provide a fuel economy that is greater than or equal to
				     * 25 miles per gallon for the nominal city driving scenarios.
				     */
				 
				:>> actualFuelEconomy = vehicle.fuelEconomy_city;
				
				assume constraint { vehicle.cargoWeight == 1000 [lb] }
			}

			requirement vehicleFuelEconomyRequirement_highway :> highwayFuelEconomyRequirement {
				doc /* The vehicle shall provide a fuel economy that is greater than or equal to
				     * 30 miles per gallon for the nominal highway driving scenarios.
				     */
				
				:>> actualFuelEconomy = vehicle.fuelEconomy_highway;
				
				assume constraint { vehicle.cargoWeight == 1000 [lb] }
			}

		}

		part analysisContext {
			
			analysis cityFuelEconomyAnalysis : FuelEconomyAnalysis {
				subject vehicle = vehicle1_c1;
				in calc scenario = cityScenario;
				in requirement fuelEconomyRequirement = cityFuelEconomyRequirement;
			} 
			
			analysis highwayFuelEconomyAnalysis : FuelEconomyAnalysis {
				subject vehicle = vehicle1_c1;
				in calc scenario = highwayScenario;
				in requirement fuelEconomyRequirement = highwayFuelEconomyRequirement;
			}
			
			part vehicle1_c1_analysized :> vehicle1_c1 {
				:>> fuelEconomy_city = cityFuelEconomyAnalysis.calculatedFuelEconomy;
				:>> fuelEconomy_highway = highwayFuelEconomyAnalysis.calculatedFuelEconomy;
			}		
			
			satisfy vehicleFuelEconomyRequirementsGroup by vehicle1_c1_analysized;
		}
		
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "10c_fuel_economy_analysis.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 4 16) (end 4 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 5 16) (end 5 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 1) (end 7 69))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 1) (end 8 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 35 3) (end 35 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 35 27) (end 35 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 60 17) (end 60 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 61 17) (end 61 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 64 3) (end 64 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 65 3) (end 65 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 66 3) (end 66 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 67 3) (end 67 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 71 3) (end 71 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 78 3) (end 78 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 80 6) (end 80 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 81 3) (end 81 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 121 3) (end 121 29))
      )
      (diagnostic
        (severity warning)
        (code "analysis_evaluation_unresolved")
        (source "semantic")
        (range (start 122 3) (end 122 378))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 122 53) (end 122 79))
      )
      (diagnostic
        (severity warning)
        (code "analysis_evaluation_unresolved")
        (source "semantic")
        (range (start 132 3) (end 132 389))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 132 56) (end 132 85))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 158 34) (end 158 45))
      )
      (diagnostic
        (severity warning)
        (code "satisfy_target_invalid_kind")
        (source "semantic")
        (range (start 163 11) (end 163 46))
      )
    )
  )
)
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwAttribute,Ident,Colon,Ident,Eq,Ident,Slash,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Eq,DecimalValue,Dot,DecimalValue,Star,UnrestrictedName,Caret,DecimalValue,Semicolon,
KwPackage,Ident,OpenCurly,
KwRequirement,KwDef,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,Semicolon,
KwAttribute,Ident,ColonGt,Ident,Semicolon,
KwRequire,KwConstraint,OpenCurly,Ident,GtEq,Ident,CloseCurly,
CloseCurly,
KwRequirement,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,Slash,Ident,CloseSquare,Semicolon,
CloseCurly,
KwRequirement,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,Slash,Ident,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,Semicolon,
KwAttribute,Ident,ColonGt,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwExhibit,KwState,Ident,OpenCurly,
KwEntry,Semicolon,KwThen,UnrestrictedName,Semicolon,
KwState,UnrestrictedName,Semicolon,
KwThen,UnrestrictedName,Semicolon,
KwState,UnrestrictedName,Semicolon,
KwThen,UnrestrictedName,Semicolon,
KwState,UnrestrictedName,Semicolon,
KwThen,UnrestrictedName,Semicolon,
KwState,UnrestrictedName,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwAttribute,KwDef,Ident,OpenCurly,
Ident,Colon,Ident,Semicolon,
Ident,Colon,Ident,Semicolon,
Ident,Colon,Ident,Semicolon,
Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAbstract,KwCalc,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwReturn,Colon,Ident,Semicolon,
CloseCurly,
KwCalc,Ident,Colon,Ident,Semicolon,
KwCalc,Ident,Colon,Ident,Semicolon,
KwAnalysis,KwDef,Ident,OpenCurly,
KwSubject,Ident,Colon,Ident,Semicolon,
KwIn,KwCalc,Ident,Colon,Ident,Semicolon,
KwIn,KwRequirement,Ident,Colon,Ident,Semicolon,
KwReturn,Ident,Colon,Ident,Semicolon,
KwObjective,Ident,OpenCurly,
KwDoc,RegularComment,
KwAssume,KwConstraint,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
KwRequire,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwAction,Ident,OpenCurly,
RegularComment,
CloseCurly,
KwAction,Ident,OpenCurly,
RegularComment,
CloseCurly,
CloseCurly,
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
KwPart,Ident,OpenCurly,
KwAnalysis,Ident,Colon,Ident,OpenCurly,
KwSubject,Ident,Eq,Ident,Semicolon,
KwIn,KwCalc,Ident,Eq,Ident,Semicolon,
KwIn,KwRequirement,Ident,Eq,Ident,Semicolon,
CloseCurly,
KwAnalysis,Ident,Colon,Ident,OpenCurly,
KwSubject,Ident,Eq,Ident,Semicolon,
KwIn,KwCalc,Ident,Eq,Ident,Semicolon,
KwIn,KwRequirement,Ident,Eq,Ident,Semicolon,
CloseCurly,
KwPart,Ident,ColonGt,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,
ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwSatisfy,Ident,KwBy,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''10c-Fuel Economy Analysis''
    (import_decl private 'ScalarValues::*')
    (import_decl private 'Quantities::*')
    (import_decl private 'MeasurementReferences::*')
    (import_decl private 'ISQ::*')
    (import_decl private 'USCustomaryUnits::*')
    (attribute_usage 'distancePerVolume' : 'ScalarQuantityValue' value)
    (attribute_usage 'gallon' : 'MeasurementUnit' value)
    (package_def 'FuelEconomyRequirementsModel'
      (requirement_def 'FuelEconomyRequirement'
        (attribute_usage 'actualFuelEconomy' :> 'distancePerVolume')
        (attribute_usage 'requiredFuelEconomy' :> 'distancePerVolume')
        (sysml_decl
          (result_expr_member)))
      (requirement_usage 'cityFuelEconomyRequirement' : 'FuelEconomyRequirement'
        (default_ref_usage :>> 'requiredFuelEconomy' value))
      (requirement_usage 'highwayFuelEconomyRequirement' : 'FuelEconomyRequirement'
        (default_ref_usage :>> 'requiredFuelEconomy' value)))
    (package_def 'VehicleDesignModel'
      (part_def 'Vehicle'
        (attribute_usage 'fuelEconomy_city' :> 'distancePerVolume')
        (attribute_usage 'fuelEconomy_highway' :> 'distancePerVolume')
        (attribute_usage 'cargoWeight' : 'MassValue'))
      (part_def 'Engine')
      (part_def 'Transmission')
      (part_usage 'vehicle1_c1' : 'Vehicle'
        (part_usage 'engine' : 'Engine')
        (part_usage 'transmission' : 'Transmission'
          (exhibit_state 'transmissionState'
            (entry_action)
            (source_succession
              (default_ref_usage ''1stGear''))
            (state_usage ''1stGear'')
            (source_succession
              (default_ref_usage ''2ndGear''))
            (state_usage ''2ndGear'')
            (source_succession
              (default_ref_usage ''3rdGear''))
            (state_usage ''3rdGear'')
            (source_succession
              (default_ref_usage ''4thGear''))
            (state_usage ''4thGear'')))))
    (package_def 'FuelEconomyAnalysisModel'
      (import_decl private 'VehicleDesignModel::*')
      (import_decl private 'FuelEconomyRequirementsModel::*')
      (attribute_def 'ScenarioState'
        (default_ref_usage 'position' : 'LengthValue')
        (default_ref_usage 'velocity' : 'SpeedValue')
        (default_ref_usage 'acceleration' : 'AccelerationValue')
        (default_ref_usage 'inclineAngle' : 'AngularMeasureValue'))
      (calc_def abstract 'NominalScenario'
        (default_ref_usage in 't' : 'TimeValue')
        (return_member))
      (calc_usage 'cityScenario' : 'NominalScenario')
      (calc_usage 'highwayScenario' : 'NominalScenario')
      (analysis_case_def 'FuelEconomyAnalysis'
        (sysml_decl 'vehicle' : 'Vehicle')
        (calc_usage in 'scenario' : 'NominalScenario')
        (requirement_usage in 'fuelEconomyRequirement' : 'FuelEconomyRequirement')
        (return_member)
        (objective_member)
        (action_usage 'dynamicsAnalysis'
          (comment))
        (action_usage 'fuelConsumptionAnalysis'
          (comment)))
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
      (part_usage 'analysisContext'
        (sysml_decl 'cityFuelEconomyAnalysis' : 'FuelEconomyAnalysis'
          (sysml_decl 'vehicle' value)
          (calc_usage in 'scenario' value)
          (requirement_usage in 'fuelEconomyRequirement' value))
        (sysml_decl 'highwayFuelEconomyAnalysis' : 'FuelEconomyAnalysis'
          (sysml_decl 'vehicle' value)
          (calc_usage in 'scenario' value)
          (requirement_usage in 'fuelEconomyRequirement' value))
        (part_usage 'vehicle1_c1_analysized' :> 'vehicle1_c1'
          (default_ref_usage :>> 'fuelEconomy_city' value)
          (default_ref_usage :>> 'fuelEconomy_highway' value))
        (sysml_decl 'vehicleFuelEconomyRequirementsGroup')))))
~~~
# EXPECTED
~~~
semantic.duplicate_name '1stGear'
semantic.duplicate_name '2ndGear'
semantic.duplicate_name '3rdGear'
semantic.duplicate_name '4thGear'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'MeasurementUnit'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'AngularMeasureValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'actualFuelEconomy'
~~~
# PROBLEMS
~~~
semantic.duplicate_name '1stGear'
semantic.duplicate_name '2ndGear'
semantic.duplicate_name '3rdGear'
semantic.duplicate_name '4thGear'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'MeasurementUnit'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'AngularMeasureValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'actualFuelEconomy'
~~~
# FORMAT
~~~sysml
package '10c-Fuel Economy Analysis' {
    private import ScalarValues::*;
    private import Quantities::*;
    private import MeasurementReferences::*;
    private import ISQ::*;
    private import USCustomaryUnits::*;

    attribute distancePerVolume : ScalarQuantityValue = length / volume;
    attribute gallon : MeasurementUnit = 231.0 * 'in'^3;

    package FuelEconomyRequirementsModel {

        requirement def FuelEconomyRequirement {
            attribute actualFuelEconomy :> distancePerVolume;
            attribute requiredFuelEconomy :> distancePerVolume;

            require constraint { actualFuelEconomy >= requiredFuelEconomy }
        }

        requirement cityFuelEconomyRequirement : FuelEconomyRequirement {
            :>> requiredFuelEconomy = 25 [mi/gallon];
        }

        requirement highwayFuelEconomyRequirement : FuelEconomyRequirement {
            :>> requiredFuelEconomy = 30 [mi/gallon];
        }

    }

    package VehicleDesignModel {

        part def Vehicle {
            attribute fuelEconomy_city :> distancePerVolume;
            attribute fuelEconomy_highway :> distancePerVolume;

            attribute cargoWeight : MassValue;
        }

        part def Engine;
        part def Transmission;

        part vehicle1_c1 : Vehicle {
            part engine : Engine;
            part transmission : Transmission {
                exhibit state transmissionState {
                    entry; then '1stGear';
                    state '1stGear';
                    then '2ndGear';
                    state '2ndGear';
                    then '3rdGear';
                    state '3rdGear';
                    then '4thGear';
                    state '4thGear';
                }
            }
        }

    }

    package FuelEconomyAnalysisModel {
        private import VehicleDesignModel::*;
        private import FuelEconomyRequirementsModel::*;

        attribute def ScenarioState {
            position : LengthValue;
            velocity : SpeedValue;
            acceleration : AccelerationValue;
            inclineAngle : AngularMeasureValue;
        }

        abstract calc def NominalScenario {
            in t : TimeValue;
            return : ScenarioState;
        }
        calc cityScenario : NominalScenario;
        calc highwayScenario : NominalScenario;

        analysis def FuelEconomyAnalysis {
            subject vehicle : Vehicle;
            in calc scenario : NominalScenario;
            in requirement fuelEconomyRequirement : FuelEconomyRequirement;
            return calculatedFuelEconomy : ScalarQuantityValue;

            objective fuelEconomyAnalysisObjective {
                doc /*
				     * The objective of this analysis is to determine whether the
				     * current vehicle design configuration can satisfy the fuel
				     * economy requirement.
				     */

                assume constraint {
                    doc /* wheelDiameter == 33 inches
				 	     * drive train efficiency == 0.4
				 	     */
                }

                require fuelEconomyRequirement {
                    :>> actualFuelEconomy = calculatedFuelEconomy;
                }
            }

            action dynamicsAnalysis {
                /*
				 * Solve for the required engine power as a function of time
				 * to support the nominal scenarios.
				 * 
				 * Note: Vehicle force = power/speed
				 * Note: EngineRPM * EngineGearRatio/WheelRPM = constant
				 */
            }

            action fuelConsumptionAnalysis {
                /*
				 * Solve the engine equations to determine how much fuel is
				 * consumed. The engine RPM is a function of the speed of the
				 * vehicle and the gear state.
				 */
            }
        }

        requirement vehicleFuelEconomyRequirementsGroup {
            subject vehicle : Vehicle;
            requirement vehicleFuelEconomyRequirement_city :> cityFuelEconomyRequirement {
                doc /* The vehicle shall provide a fuel economy that is greater than or equal to
				     * 25 miles per gallon for the nominal city driving scenarios.
				     */

                :>> actualFuelEconomy = vehicle.fuelEconomy_city;

                assume constraint { vehicle.cargoWeight == 1000 [lb] }
            }

            requirement vehicleFuelEconomyRequirement_highway :> highwayFuelEconomyRequirement {
                doc /* The vehicle shall provide a fuel economy that is greater than or equal to
				     * 30 miles per gallon for the nominal highway driving scenarios.
				     */

                :>> actualFuelEconomy = vehicle.fuelEconomy_highway;

                assume constraint { vehicle.cargoWeight == 1000 [lb] }
            }

        }

        part analysisContext {

            analysis cityFuelEconomyAnalysis : FuelEconomyAnalysis {
                subject vehicle = vehicle1_c1;
                in calc scenario = cityScenario;
                in requirement fuelEconomyRequirement = cityFuelEconomyRequirement;
            }

            analysis highwayFuelEconomyAnalysis : FuelEconomyAnalysis {
                subject vehicle = vehicle1_c1;
                in calc scenario = highwayScenario;
                in requirement fuelEconomyRequirement = highwayFuelEconomyRequirement;
            }

            part vehicle1_c1_analysized :> vehicle1_c1 {
                :>> fuelEconomy_city = cityFuelEconomyAnalysis.calculatedFuelEconomy;
                :>> fuelEconomy_highway = highwayFuelEconomyAnalysis.calculatedFuelEconomy;
            }

            satisfy vehicleFuelEconomyRequirementsGroup by vehicle1_c1_analysized;
        }

    }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "6f8c93a64f9b2389c3de07894f26c0f05f3b47ab6ab3538d43b9a5cc4f10badd") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis"))) (kind "package") (name "10c-Fuel Economy Analysis") (declared-name "10c-Fuel Economy Analysis") (range (start (line 0) (character 0)) (end (line 0) (character 4755))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 32))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 28))))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 30))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 26))))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 1)) (end (line 3) (character 41))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 16)) (end (line 3) (character 37))))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::*#import3"))) (kind "import") (name "*") (declared-name "*") (range (start (line 4) (character 1)) (end (line 4) (character 23))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 4) (character 16)) (end (line 4) (character 19))))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::*#import4"))) (kind "import") (name "*") (declared-name "*") (range (start (line 5) (character 1)) (end (line 5) (character 36))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis"))) (authored (membership (kind Import) (visibility "private") (import (reference "USCustomaryUnits::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 5) (character 16)) (end (line 5) (character 32))))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel"))) (kind "package") (name "FuelEconomyAnalysisModel") (declared-name "FuelEconomyAnalysisModel") (range (start (line 59) (character 1)) (end (line 59) (character 3307))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 60) (character 2)) (end (line 60) (character 39))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "VehicleDesignModel::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 60) (character 17)) (end (line 60) (character 35))))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 61) (character 2)) (end (line 61) (character 49))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "FuelEconomyRequirementsModel::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 61) (character 17)) (end (line 61) (character 45))))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))) (kind "analysis def") (name "FuelEconomyAnalysis") (declared-name "FuelEconomyAnalysis") (range (start (line 77) (character 2)) (end (line 77) (character 1211))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::calculatedFuelEconomy"))) (kind "analysis result") (name "calculatedFuelEconomy") (declared-name "calculatedFuelEconomy") (range (start (line 81) (character 3)) (end (line 81) (character 54))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))) (authored (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::dynamicsAnalysis"))) (kind "action") (name "dynamicsAnalysis") (declared-name "dynamicsAnalysis") (range (start (line 101) (character 3)) (end (line 101) (character 264))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumptionAnalysis"))) (kind "action") (name "fuelConsumptionAnalysis") (declared-name "fuelConsumptionAnalysis") (range (start (line 111) (character 3)) (end (line 111) (character 220))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelEconomyAnalysisObjective"))) (kind "objective") (name "fuelEconomyAnalysisObjective") (declared-name "fuelEconomyAnalysisObjective") (range (start (line 83) (character 3)) (end (line 83) (character 481))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelEconomyRequirement"))) (kind "requirement") (name "fuelEconomyRequirement") (declared-name "fuelEconomyRequirement") (range (start (line 80) (character 6)) (end (line 80) (character 66))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelEconomyRequirement") (range none)))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::scenario"))) (kind "calc") (name "scenario") (declared-name "scenario") (range (start (line 79) (character 6)) (end (line 79) (character 38))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))) (authored (membership (kind Feature)) (relationships (typing (reference "NominalScenario") (range none)))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (range (start (line 78) (character 3)) (end (line 78) (character 29))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))) (authored (relationships (typing (reference "Vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario"))) (kind "calc def") (name "NominalScenario") (declared-name "NominalScenario") (range (start (line 70) (character 2)) (end (line 70) (character 91))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario::"))) (kind "return parameter") (name "") (range (start (line 72) (character 3)) (end (line 72) (character 26))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario"))) (authored (relationships (typing (reference "ScenarioState") (range none)))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario::t"))) (kind "in out parameter") (name "t") (declared-name "t") (range (start (line 71) (character 3)) (end (line 71) (character 20))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario"))) (authored (relationships (typing (reference "TimeValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState"))) (kind "attribute def") (name "ScenarioState") (declared-name "ScenarioState") (range (start (line 63) (character 2)) (end (line 63) (character 164))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState::acceleration"))) (kind "attribute") (name "acceleration") (declared-name "acceleration") (range (start (line 66) (character 3)) (end (line 66) (character 36))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState"))) (authored (membership (kind Feature)) (relationships (typing (reference "AccelerationValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState::inclineAngle"))) (kind "attribute") (name "inclineAngle") (declared-name "inclineAngle") (range (start (line 67) (character 3)) (end (line 67) (character 38))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularMeasureValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState::position"))) (kind "attribute") (name "position") (declared-name "position") (range (start (line 64) (character 3)) (end (line 64) (character 26))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState::velocity"))) (kind "attribute") (name "velocity") (declared-name "velocity") (range (start (line 65) (character 3)) (end (line 65) (character 25))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState"))) (authored (membership (kind Feature)) (relationships (typing (reference "SpeedValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext"))) (kind "part") (name "analysisContext") (declared-name "analysisContext") (range (start (line 144) (character 2)) (end (line 144) (character 755))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized"))) (kind "part") (name "vehicle1_c1_analysized") (declared-name "vehicle1_c1_analysized") (range (start (line 158) (character 3)) (end (line 158) (character 206))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle1_c1") (range (start (line 158) (character 34)) (end (line 158) (character 45)))))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized::fuelEconomy_city"))) (kind "attribute") (name "fuelEconomy_city") (declared-name "fuelEconomy_city") (range (start (line 159) (character 4)) (end (line 159) (character 73))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "fuelEconomy_city") (range (start (line 159) (character 4)) (end (line 159) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized::fuelEconomy_highway"))) (kind "attribute") (name "fuelEconomy_highway") (declared-name "fuelEconomy_highway") (range (start (line 160) (character 4)) (end (line 160) (character 79))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "fuelEconomy_highway") (range (start (line 160) (character 4)) (end (line 160) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::cityScenario"))) (kind "calc def") (name "cityScenario") (declared-name "cityScenario") (range (start (line 74) (character 2)) (end (line 74) (character 38))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::highwayScenario"))) (kind "calc def") (name "highwayScenario") (declared-name "highwayScenario") (range (start (line 75) (character 2)) (end (line 75) (character 41))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup"))) (kind "requirement") (name "vehicleFuelEconomyRequirementsGroup") (declared-name "vehicleFuelEconomyRequirementsGroup") (range (start (line 120) (character 2)) (end (line 120) (character 856))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel"))) (authored (membership (kind Feature)) (relationships (subject (reference "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (range (start (line 121) (character 3)) (end (line 121) (character 29))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup"))) (authored (relationships (typing (reference "Vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city"))) (kind "requirement") (name "vehicleFuelEconomyRequirement_city") (declared-name "vehicleFuelEconomyRequirement_city") (range (start (line 122) (character 3)) (end (line 122) (character 378))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "cityFuelEconomyRequirement") (range (start (line 122) (character 53)) (end (line 122) (character 79)))))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city::_documentation"))) (kind "documentation") (name "") (range (start (line 122) (character 3)) (end (line 122) (character 378))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city::_requireConstraint_0"))) (kind "require constraint") (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (range (start (line 129) (character 4)) (end (line 129) (character 58))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city::actualFuelEconomy"))) (kind "attribute") (name "actualFuelEconomy") (declared-name "actualFuelEconomy") (range (start (line 127) (character 4)) (end (line 127) (character 53))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city"))) (authored (relationships (redefinition (reference "actualFuelEconomy") (range (start (line 127) (character 4)) (end (line 127) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway"))) (kind "requirement") (name "vehicleFuelEconomyRequirement_highway") (declared-name "vehicleFuelEconomyRequirement_highway") (range (start (line 132) (character 3)) (end (line 132) (character 389))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "highwayFuelEconomyRequirement") (range (start (line 132) (character 56)) (end (line 132) (character 85)))))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway::_documentation"))) (kind "documentation") (name "") (range (start (line 132) (character 3)) (end (line 132) (character 389))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway::_requireConstraint_0"))) (kind "require constraint") (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (range (start (line 139) (character 4)) (end (line 139) (character 58))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway::actualFuelEconomy"))) (kind "attribute") (name "actualFuelEconomy") (declared-name "actualFuelEconomy") (range (start (line 137) (character 4)) (end (line 137) (character 56))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway"))) (authored (relationships (redefinition (reference "actualFuelEconomy") (range (start (line 137) (character 4)) (end (line 137) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel"))) (kind "package") (name "FuelEconomyRequirementsModel") (declared-name "FuelEconomyRequirementsModel") (range (start (line 10) (character 1)) (end (line 10) (character 517))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement"))) (kind "requirement def") (name "FuelEconomyRequirement") (declared-name "FuelEconomyRequirement") (range (start (line 12) (character 2)) (end (line 12) (character 225))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::_requireConstraint_0"))) (kind "require constraint") (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (range (start (line 16) (character 3)) (end (line 16) (character 66))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::actualFuelEconomy"))) (kind "attribute") (name "actualFuelEconomy") (declared-name "actualFuelEconomy") (range (start (line 13) (character 3)) (end (line 13) (character 52))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement"))) (authored (relationships (subsetting (reference "distancePerVolume") (range (start (line 13) (character 34)) (end (line 13) (character 51)))))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::requiredFuelEconomy"))) (kind "attribute") (name "requiredFuelEconomy") (declared-name "requiredFuelEconomy") (range (start (line 14) (character 3)) (end (line 14) (character 54))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement"))) (authored (relationships (subsetting (reference "distancePerVolume") (range (start (line 14) (character 36)) (end (line 14) (character 53)))))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::cityFuelEconomyRequirement"))) (kind "requirement") (name "cityFuelEconomyRequirement") (declared-name "cityFuelEconomyRequirement") (range (start (line 19) (character 2)) (end (line 19) (character 116))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelEconomyRequirement") (range none)))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::cityFuelEconomyRequirement::requiredFuelEconomy"))) (kind "attribute") (name "requiredFuelEconomy") (declared-name "requiredFuelEconomy") (range (start (line 20) (character 3)) (end (line 20) (character 44))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::cityFuelEconomyRequirement"))) (authored (relationships (redefinition (reference "requiredFuelEconomy") (range (start (line 20) (character 3)) (end (line 20) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement"))) (kind "requirement") (name "highwayFuelEconomyRequirement") (declared-name "highwayFuelEconomyRequirement") (range (start (line 23) (character 2)) (end (line 23) (character 119))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelEconomyRequirement") (range none)))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement::requiredFuelEconomy"))) (kind "attribute") (name "requiredFuelEconomy") (declared-name "requiredFuelEconomy") (range (start (line 24) (character 3)) (end (line 24) (character 44))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement"))) (authored (relationships (redefinition (reference "requiredFuelEconomy") (range (start (line 24) (character 3)) (end (line 24) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel"))) (kind "package") (name "VehicleDesignModel") (declared-name "VehicleDesignModel") (range (start (line 29) (character 1)) (end (line 29) (character 588))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (range (start (line 38) (character 2)) (end (line 38) (character 18))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Transmission"))) (kind "part def") (name "Transmission") (declared-name "Transmission") (range (start (line 39) (character 2)) (end (line 39) (character 24))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 31) (character 2)) (end (line 31) (character 173))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle::cargoWeight"))) (kind "attribute") (name "cargoWeight") (declared-name "cargoWeight") (range (start (line 35) (character 3)) (end (line 35) (character 37))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue") (range none)) (typing (reference "MassValue") (range (start (line 35) (character 27)) (end (line 35) (character 36)))))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle::fuelEconomy_city"))) (kind "attribute") (name "fuelEconomy_city") (declared-name "fuelEconomy_city") (range (start (line 32) (character 3)) (end (line 32) (character 51))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "distancePerVolume") (range (start (line 32) (character 33)) (end (line 32) (character 50)))))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle::fuelEconomy_highway"))) (kind "attribute") (name "fuelEconomy_highway") (declared-name "fuelEconomy_highway") (range (start (line 33) (character 3)) (end (line 33) (character 54))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "distancePerVolume") (range (start (line 33) (character 36)) (end (line 33) (character 53)))))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1"))) (kind "part") (name "vehicle1_c1") (declared-name "vehicle1_c1") (range (start (line 41) (character 2)) (end (line 41) (character 325))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 41) (character 21)) (end (line 41) (character 28)))))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::engine"))) (kind "part") (name "engine") (declared-name "engine") (range (start (line 42) (character 3)) (end (line 42) (character 24))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 42) (character 17)) (end (line 42) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission"))) (kind "part") (name "transmission") (declared-name "transmission") (range (start (line 43) (character 3)) (end (line 43) (character 265))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Transmission") (range (start (line 43) (character 23)) (end (line 43) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState"))) (kind "state") (name "transmissionState") (declared-name "transmissionState") (range (start (line 44) (character 4)) (end (line 44) (character 222))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission"))) (authored (membership (kind Feature)) (relationships (initial-state (reference "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::1stGear") (range none)) (initial-state (reference "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::2ndGear") (range none)) (initial-state (reference "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::3rdGear") (range none)) (initial-state (reference "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::4thGear") (range none)))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::1stGear"))) (kind "state") (name "1stGear") (declared-name "1stGear") (range (start (line 46) (character 5)) (end (line 46) (character 21))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::2ndGear"))) (kind "state") (name "2ndGear") (declared-name "2ndGear") (range (start (line 48) (character 5)) (end (line 48) (character 21))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::3rdGear"))) (kind "state") (name "3rdGear") (declared-name "3rdGear") (range (start (line 50) (character 5)) (end (line 50) (character 21))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::4thGear"))) (kind "state") (name "4thGear") (declared-name "4thGear") (range (start (line 52) (character 5)) (end (line 52) (character 21))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::_entry"))) (kind "action") (name "entry") (declared-name "entry") (range (start (line 45) (character 5)) (end (line 45) (character 11))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::distancePerVolume"))) (kind "attribute def") (name "distancePerVolume") (declared-name "distancePerVolume") (range (start (line 7) (character 1)) (end (line 7) (character 69))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::gallon"))) (kind "attribute def") (name "gallon") (declared-name "gallon") (range (start (line 8) (character 1)) (end (line 8) (character 53))) (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis"))) (authored (membership (kind Owning)) (relationships (typing (reference "MeasurementUnit") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 1) (character 16)) (end (line 1) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Quantities::*") (range (start (line 2) (character 16)) (end (line 2) (character 26))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "MeasurementReferences::*") (range (start (line 3) (character 16)) (end (line 3) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::*#import3"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (range (start (line 4) (character 16)) (end (line 4) (character 19))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::*#import4"))) (kind namespaceImport) (ordinal 0)) (authored-target "USCustomaryUnits::*") (range (start (line 5) (character 16)) (end (line 5) (character 32))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "VehicleDesignModel::*") (range (start (line 60) (character 17)) (end (line 60) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "FuelEconomyRequirementsModel::*") (range (start (line 61) (character 17)) (end (line 61) (character 45))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::calculatedFuelEconomy"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelEconomyRequirement"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelEconomyRequirement") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::scenario"))) (kind featureTyping) (ordinal 0)) (authored-target "NominalScenario") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario::"))) (kind featureTyping) (ordinal 0)) (authored-target "ScenarioState") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario::t"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState::acceleration"))) (kind featureTyping) (ordinal 0)) (authored-target "AccelerationValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState::inclineAngle"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState::position"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState::velocity"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext"))) (kind satisfySource) (ordinal 0)) (authored-target "vehicleFuelEconomyRequirementsGroup") (range (start (line 163) (character 11)) (end (line 163) (character 46))) (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext"))) (kind satisfyTarget) (ordinal 0)) (authored-target "vehicle1_c1_analysized") (range (start (line 163) (character 50)) (end (line 163) (character 72))) (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle1_c1") (range (start (line 158) (character 34)) (end (line 158) (character 45))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized::fuelEconomy_city"))) (kind redefinition) (ordinal 0)) (authored-target "fuelEconomy_city") (range (start (line 159) (character 4)) (end (line 159) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized::fuelEconomy_city")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized::fuelEconomy_highway"))) (kind redefinition) (ordinal 0)) (authored-target "fuelEconomy_highway") (range (start (line 160) (character 4)) (end (line 160) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized::fuelEconomy_highway")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicle") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city"))) (kind subsetting) (ordinal 0)) (authored-target "cityFuelEconomyRequirement") (range (start (line 122) (character 53)) (end (line 122) (character 79))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city::actualFuelEconomy"))) (kind redefinition) (ordinal 0)) (authored-target "actualFuelEconomy") (range (start (line 127) (character 4)) (end (line 127) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city::actualFuelEconomy")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway"))) (kind subsetting) (ordinal 0)) (authored-target "highwayFuelEconomyRequirement") (range (start (line 132) (character 56)) (end (line 132) (character 85))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway::actualFuelEconomy"))) (kind redefinition) (ordinal 0)) (authored-target "actualFuelEconomy") (range (start (line 137) (character 4)) (end (line 137) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway::actualFuelEconomy")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::actualFuelEconomy"))) (kind subsetting) (ordinal 0)) (authored-target "distancePerVolume") (range (start (line 13) (character 34)) (end (line 13) (character 51))) (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::distancePerVolume")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::requiredFuelEconomy"))) (kind subsetting) (ordinal 0)) (authored-target "distancePerVolume") (range (start (line 14) (character 36)) (end (line 14) (character 53))) (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::distancePerVolume")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::cityFuelEconomyRequirement"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelEconomyRequirement") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::cityFuelEconomyRequirement::requiredFuelEconomy"))) (kind redefinition) (ordinal 0)) (authored-target "requiredFuelEconomy") (range (start (line 20) (character 3)) (end (line 20) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::cityFuelEconomyRequirement::requiredFuelEconomy")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelEconomyRequirement") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement::requiredFuelEconomy"))) (kind redefinition) (ordinal 0)) (authored-target "requiredFuelEconomy") (range (start (line 24) (character 3)) (end (line 24) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement::requiredFuelEconomy")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle::cargoWeight"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle::cargoWeight"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (range (start (line 35) (character 27)) (end (line 35) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle::fuelEconomy_city"))) (kind subsetting) (ordinal 0)) (authored-target "distancePerVolume") (range (start (line 32) (character 33)) (end (line 32) (character 50))) (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::distancePerVolume")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle::fuelEconomy_highway"))) (kind subsetting) (ordinal 0)) (authored-target "distancePerVolume") (range (start (line 33) (character 36)) (end (line 33) (character 53))) (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::distancePerVolume")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 41) (character 21)) (end (line 41) (character 28))) (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 42) (character 17)) (end (line 42) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission"))) (kind featureTyping) (ordinal 0)) (authored-target "Transmission") (range (start (line 43) (character 23)) (end (line 43) (character 35))) (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Transmission")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState"))) (kind initialStateSource) (ordinal 0)) (authored-target "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::1stGear") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::1stGear")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState"))) (kind initialStateSource) (ordinal 1)) (authored-target "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::2ndGear") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::2ndGear")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState"))) (kind initialStateSource) (ordinal 2)) (authored-target "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::3rdGear") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::3rdGear")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState"))) (kind initialStateSource) (ordinal 3)) (authored-target "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::4thGear") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::4thGear")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::distancePerVolume"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::gallon"))) (kind featureTyping) (ordinal 0)) (authored-target "MeasurementUnit") (range none) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::scenario"))) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::scenario"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario::"))) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario::"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized::fuelEconomy_city"))) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized::fuelEconomy_city"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized::fuelEconomy_city"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized::fuelEconomy_highway"))) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized::fuelEconomy_highway"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized::fuelEconomy_highway"))) (kind redefinition) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup"))) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind satisfy) (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup"))) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext"))) (kind satisfySource) (ordinal 0)) (expression (kind satisfy) (source "vehicleFuelEconomyRequirementsGroup") (target "vehicle1_c1_analysized") (source-range (start (line 163) (character 11)) (end (line 163) (character 46))) (target-range (start (line 163) (character 50)) (end (line 163) (character 72)))))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city::actualFuelEconomy"))) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city::actualFuelEconomy"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city::actualFuelEconomy"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway::actualFuelEconomy"))) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway::actualFuelEconomy"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway::actualFuelEconomy"))) (kind redefinition) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::actualFuelEconomy"))) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::distancePerVolume"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::actualFuelEconomy"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::requiredFuelEconomy"))) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::distancePerVolume"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::requiredFuelEconomy"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::cityFuelEconomyRequirement"))) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::cityFuelEconomyRequirement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::cityFuelEconomyRequirement::requiredFuelEconomy"))) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::cityFuelEconomyRequirement::requiredFuelEconomy"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::cityFuelEconomyRequirement::requiredFuelEconomy"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement"))) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement::requiredFuelEconomy"))) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement::requiredFuelEconomy"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement::requiredFuelEconomy"))) (kind redefinition) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle::fuelEconomy_city"))) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::distancePerVolume"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle::fuelEconomy_city"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle::fuelEconomy_highway"))) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::distancePerVolume"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle::fuelEconomy_highway"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1"))) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::engine"))) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission"))) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Transmission"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind initialState) (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState"))) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::1stGear"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState"))) (kind initialStateSource) (ordinal 0)))
    (relationship (kind initialState) (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState"))) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::2ndGear"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState"))) (kind initialStateSource) (ordinal 1)))
    (relationship (kind initialState) (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState"))) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::3rdGear"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState"))) (kind initialStateSource) (ordinal 2)))
    (relationship (kind initialState) (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState"))) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::4thGear"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState"))) (kind initialStateSource) (ordinal 3)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized::fuelEconomy_city")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized::fuelEconomy_highway")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city")) (expression (status "unresolved") (error "expression has an unresolved reference")) (analysis (status "unresolved")))
    (node (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city::_requireConstraint_0")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city::actualFuelEconomy")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway")) (expression (status "unresolved") (error "expression has an unresolved reference")) (analysis (status "unresolved")))
    (node (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway::_requireConstraint_0")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway::actualFuelEconomy")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement")) (expression (status "incomplete") (error "expression is incomplete")) (analysis (status "incomplete")))
    (node (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::_requireConstraint_0")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::cityFuelEconomyRequirement")) (expression (status "incomplete") (error "expression is incomplete")) (analysis (status "incomplete")))
    (node (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::cityFuelEconomyRequirement::requiredFuelEconomy")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement")) (expression (status "incomplete") (error "expression is incomplete")) (analysis (status "incomplete")))
    (node (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement::requiredFuelEconomy")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::distancePerVolume")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::gallon")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
