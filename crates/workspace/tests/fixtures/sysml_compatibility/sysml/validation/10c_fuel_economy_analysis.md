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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis"))) (name "10c-Fuel Economy Analysis") (declared-name "10c-Fuel Economy Analysis")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::*#import2"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::*#import3"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::*#import4"))) (name "*") (declared-name "*"))
        (element (kind "package") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel"))) (name "FuelEconomyAnalysisModel") (declared-name "FuelEconomyAnalysisModel")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::*"))) (name "*") (declared-name "*"))
            (element (kind "import") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::*#import"))) (name "*") (declared-name "*"))
            (element (kind "analysis def") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))) (name "FuelEconomyAnalysis") (declared-name "FuelEconomyAnalysis")
              (contains
                (element (kind "analysis result") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::calculatedFuelEconomy"))) (name "calculatedFuelEconomy") (declared-name "calculatedFuelEconomy") (effective (featuring-type (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis")))))
                (element (kind "action") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::dynamicsAnalysis"))) (name "dynamicsAnalysis") (declared-name "dynamicsAnalysis") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis")))))
                (element (kind "action") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumptionAnalysis"))) (name "fuelConsumptionAnalysis") (declared-name "fuelConsumptionAnalysis") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis")))))
                (element (kind "objective") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelEconomyAnalysisObjective"))) (name "fuelEconomyAnalysisObjective") (declared-name "fuelEconomyAnalysisObjective") (effective (featuring-type (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis")))))
                (element (kind "requirement") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelEconomyRequirement"))) (name "fuelEconomyRequirement") (declared-name "fuelEconomyRequirement") (effective (featuring-type (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis")))))
                (element (kind "calc") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::scenario"))) (name "scenario") (declared-name "scenario") (effective (featuring-type (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis")))))
                (element (kind "subject") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::vehicle"))) (name "vehicle") (declared-name "vehicle") (effective (featuring-type (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis")))))
              )
            )
            (element (kind "calc def") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario"))) (name "NominalScenario") (declared-name "NominalScenario")
              (contains
                (element (kind "return parameter") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario::"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario::t"))) (name "t") (declared-name "t") (effective (featuring-type (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario")))))
              )
            )
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState"))) (name "ScenarioState") (declared-name "ScenarioState") (declared (properties (ordered false) (unique true)))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState::acceleration"))) (name "acceleration") (declared-name "acceleration") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState::inclineAngle"))) (name "inclineAngle") (declared-name "inclineAngle") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState::position"))) (name "position") (declared-name "position") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState::velocity"))) (name "velocity") (declared-name "velocity") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState")))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext"))) (name "analysisContext") (declared-name "analysisContext") (declared (properties (ordered false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized"))) (name "vehicle1_c1_analysized") (declared-name "vehicle1_c1_analysized") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized::fuelEconomy_city"))) (name "fuelEconomy_city") (declared-name "fuelEconomy_city") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "memberAccess") (reference "calculatedFuelEconomy") (children (expression (kind "featureReference") (reference "cityFuelEconomyAnalysis")))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized::fuelEconomy_city"))) (role feature-value))))
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized::fuelEconomy_highway"))) (name "fuelEconomy_highway") (declared-name "fuelEconomy_highway") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "memberAccess") (reference "calculatedFuelEconomy") (children (expression (kind "featureReference") (reference "highwayFuelEconomyAnalysis")))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized::fuelEconomy_highway"))) (role feature-value))))
                  )
                )
              )
            )
            (element (kind "calc def") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::cityScenario"))) (name "cityScenario") (declared-name "cityScenario"))
            (element (kind "calc def") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::highwayScenario"))) (name "highwayScenario") (declared-name "highwayScenario"))
            (element (kind "requirement") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup"))) (name "vehicleFuelEconomyRequirementsGroup") (declared-name "vehicleFuelEconomyRequirementsGroup")
              (contains
                (element (kind "subject") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicle"))) (name "vehicle") (declared-name "vehicle"))
                (element (kind "requirement") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city"))) (name "vehicleFuelEconomyRequirement_city") (declared-name "vehicleFuelEconomyRequirement_city")
                  (contains
                    (element (kind "documentation") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city::_documentation"))) (name ""))
                    (element (kind "require constraint") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city::_requireConstraint_0"))) (name "_requireConstraint_0") (declared-name "_requireConstraint_0"))
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city::actualFuelEconomy"))) (name "actualFuelEconomy") (declared-name "actualFuelEconomy") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                  )
                )
                (element (kind "requirement") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway"))) (name "vehicleFuelEconomyRequirement_highway") (declared-name "vehicleFuelEconomyRequirement_highway")
                  (contains
                    (element (kind "documentation") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway::_documentation"))) (name ""))
                    (element (kind "require constraint") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway::_requireConstraint_0"))) (name "_requireConstraint_0") (declared-name "_requireConstraint_0"))
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway::actualFuelEconomy"))) (name "actualFuelEconomy") (declared-name "actualFuelEconomy") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                  )
                )
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel"))) (name "FuelEconomyRequirementsModel") (declared-name "FuelEconomyRequirementsModel")
          (contains
            (element (kind "requirement def") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement"))) (name "FuelEconomyRequirement") (declared-name "FuelEconomyRequirement")
              (contains
                (element (kind "require constraint") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::_requireConstraint_0"))) (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (effective (featuring-type (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::actualFuelEconomy"))) (name "actualFuelEconomy") (declared-name "actualFuelEconomy") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::requiredFuelEconomy"))) (name "requiredFuelEconomy") (declared-name "requiredFuelEconomy") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement")))))
              )
            )
            (element (kind "requirement") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::cityFuelEconomyRequirement"))) (name "cityFuelEconomyRequirement") (declared-name "cityFuelEconomyRequirement")
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::cityFuelEconomyRequirement::requiredFuelEconomy"))) (name "requiredFuelEconomy") (declared-name "requiredFuelEconomy") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement")))))
              )
            )
            (element (kind "requirement") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement"))) (name "highwayFuelEconomyRequirement") (declared-name "highwayFuelEconomyRequirement")
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement::requiredFuelEconomy"))) (name "requiredFuelEconomy") (declared-name "requiredFuelEconomy") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement")))))
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel"))) (name "VehicleDesignModel") (declared-name "VehicleDesignModel")
          (contains
            (element (kind "part def") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Engine"))) (name "Engine") (declared-name "Engine") (declared))
            (element (kind "part def") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Transmission"))) (name "Transmission") (declared-name "Transmission") (declared))
            (element (kind "part def") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared)
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle::cargoWeight"))) (name "cargoWeight") (declared-name "cargoWeight") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle::fuelEconomy_city"))) (name "fuelEconomy_city") (declared-name "fuelEconomy_city") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle::fuelEconomy_highway"))) (name "fuelEconomy_highway") (declared-name "fuelEconomy_highway") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle")))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1"))) (name "vehicle1_c1") (declared-name "vehicle1_c1") (declared (properties (ordered false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::engine"))) (name "engine") (declared-name "engine") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle")))))
                (element (kind "part") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission"))) (name "transmission") (declared-name "transmission") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle"))))
                  (contains
                    (element (kind "state") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState"))) (name "transmissionState") (declared-name "transmissionState") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Transmission"))))
                      (contains
                        (element (kind "state") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::1stGear"))) (name "1stGear") (declared-name "1stGear") (effective (featuring-type (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Transmission")))))
                        (element (kind "state") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::2ndGear"))) (name "2ndGear") (declared-name "2ndGear") (effective (featuring-type (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Transmission")))))
                        (element (kind "state") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::3rdGear"))) (name "3rdGear") (declared-name "3rdGear") (effective (featuring-type (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Transmission")))))
                        (element (kind "state") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::4thGear"))) (name "4thGear") (declared-name "4thGear") (effective (featuring-type (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Transmission")))))
                        (element (kind "action") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::_entry"))) (name "entry") (declared-name "entry") (effective (featuring-type (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Transmission")))))
                      )
                    )
                  )
                )
              )
            )
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::distancePerVolume"))) (name "distancePerVolume") (declared-name "distancePerVolume") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "length")) (expression (kind "featureReference") (reference "volume")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::distancePerVolume"))) (role feature-value))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::gallon"))) (name "gallon") (declared-name "gallon") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "*") (children (expression (kind "realLiteral") (literal "231.0")) (expression (kind "featureReference") (reference "in")))) (expression (kind "integerLiteral") (literal 3)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::gallon"))) (role feature-value))))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city::_documentation"))) (to (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway::_documentation"))) (to (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway"))) (provenance authored))
    (initialState (status resolved) (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState"))) (to (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::1stGear"))) (provenance authored))
    (initialState (status resolved) (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState"))) (to (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::2ndGear"))) (provenance authored))
    (initialState (status resolved) (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState"))) (to (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::3rdGear"))) (provenance authored))
    (initialState (status resolved) (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState"))) (to (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::4thGear"))) (provenance authored))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::cityFuelEconomyRequirement::requiredFuelEconomy"))) (to (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::requiredFuelEconomy"))) (provenance authored))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement::requiredFuelEconomy"))) (to (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::requiredFuelEconomy"))) (provenance authored))
    (satisfy (status resolved) (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup"))) (to (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized"))) (provenance authored))
    (subject (status resolved) (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))) (to (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle"))) (provenance authored))
    (subject (status resolved) (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup"))) (to (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicle"))) (provenance authored))
    (subject (status resolved) (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup"))) (to (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle"))) (provenance authored))
    (subject (status resolved) (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city"))) (to (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicle"))) (provenance authored))
    (subject (status resolved) (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway"))) (to (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicle"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelEconomyRequirement"))) (to (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::scenario"))) (to (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::vehicle"))) (to (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario::"))) (to (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicle"))) (to (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::cityFuelEconomyRequirement"))) (to (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement"))) (to (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1"))) (to (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::engine"))) (to (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Engine"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission"))) (to (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Transmission"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))) (status missing-prerequisite) (target "AnalysisCases::AnalysisCase"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::dynamicsAnalysis"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumptionAnalysis"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelEconomyAnalysisObjective"))) (status missing-prerequisite) (target "Requirements::requirementChecks"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelEconomyRequirement"))) (status missing-prerequisite) (target "Requirements::requirementChecks"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::scenario"))) (status missing-prerequisite) (target "Calculations::calculations"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState::acceleration"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState::inclineAngle"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState::position"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState::velocity"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized::fuelEconomy_city"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized::fuelEconomy_highway"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::cityScenario"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::highwayScenario"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup"))) (status missing-prerequisite) (target "Requirements::requirementChecks"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city"))) (status missing-prerequisite) (target "Requirements::requirementChecks"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city::_requireConstraint_0"))) (status missing-prerequisite) (target "Constraints::constraintChecks"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city::actualFuelEconomy"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway"))) (status missing-prerequisite) (target "Requirements::requirementChecks"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway::_requireConstraint_0"))) (status missing-prerequisite) (target "Constraints::constraintChecks"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway::actualFuelEconomy"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement"))) (status missing-prerequisite) (target "Requirements::RequirementCheck"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::_requireConstraint_0"))) (status missing-prerequisite) (target "Constraints::constraintChecks"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::actualFuelEconomy"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::requiredFuelEconomy"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::cityFuelEconomyRequirement"))) (status missing-prerequisite) (target "Requirements::requirementChecks"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::cityFuelEconomyRequirement::requiredFuelEconomy"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement"))) (status missing-prerequisite) (target "Requirements::requirementChecks"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement::requiredFuelEconomy"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Engine"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Transmission"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle::cargoWeight"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle::fuelEconomy_city"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle::fuelEconomy_highway"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::engine"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState"))) (status missing-prerequisite) (target "States::stateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::1stGear"))) (status missing-prerequisite) (target "States::stateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::2ndGear"))) (status missing-prerequisite) (target "States::stateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::3rdGear"))) (status missing-prerequisite) (target "States::stateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::4thGear"))) (status missing-prerequisite) (target "States::stateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::_entry"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::distancePerVolume"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::gallon"))) (status missing-prerequisite) (target "Base::DataValue"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/validation/10c_fuel_economy_analysis.md"
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
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 20 3) (end 20 44))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 24 3) (end 24 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 35 3) (end 35 37))
      )
      (diagnostic
        (severity warning)
        (code "multiple_initial_states")
        (source "semantic")
        (range (start 44 4) (end 44 222))
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
        (range (start 81 3) (end 81 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 127 4) (end 127 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 137 4) (end 137 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 159 4) (end 159 73))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 160 4) (end 160 79))
      )
    )
  )
)
~~~
