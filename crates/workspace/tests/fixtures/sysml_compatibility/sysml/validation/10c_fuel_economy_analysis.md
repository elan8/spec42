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

            require constraint {
                = actualFuelEconomy >= requiredFuelEconomy;
            }
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

                require constraint fuelEconomyRequirement {
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

                assume constraint {
                    = vehicle.cargoWeight == 1000[lb];
                }
            }

            requirement vehicleFuelEconomyRequirement_highway :> highwayFuelEconomyRequirement {
                doc /* The vehicle shall provide a fuel economy that is greater than or equal to
				     * 30 miles per gallon for the nominal highway driving scenarios.
				     */

                :>> actualFuelEconomy = vehicle.fuelEconomy_highway;

                assume constraint {
                    = vehicle.cargoWeight == 1000[lb];
                }
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
(model
  (namespace
    (package '10c-Fuel Economy Analysis'
      (namespace_import private -> 'ScalarValues'[unresolved])
      (namespace_import private -> 'Quantities'[unresolved])
      (namespace_import private -> 'MeasurementReferences'[unresolved])
      (namespace_import private -> 'ISQ'[unresolved])
      (namespace_import private -> 'USCustomaryUnits'[unresolved])
      (attribute_usage 'distancePerVolume' : 'ScalarQuantityValue'[unresolved]
        (feature_value (=)))
      (attribute_usage 'gallon' : 'MeasurementUnit'[unresolved]
        (feature_value (=)))
      (package 'FuelEconomyRequirementsModel'
        (requirement_def 'FuelEconomyRequirement'
          (attribute_usage composite 'actualFuelEconomy' :> '10c-Fuel Economy Analysis::distancePerVolume'[attribute_usage])
          (attribute_usage composite 'requiredFuelEconomy' :> '10c-Fuel Economy Analysis::distancePerVolume'[attribute_usage])
          (require_constraint_usage composite
            (result_expr_membership)))
        (requirement_usage 'cityFuelEconomyRequirement' : '10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement'[requirement_def]
          (reference_usage reference :>> '10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::requiredFuelEconomy'[attribute_usage]
            (feature_value (=))))
        (requirement_usage 'highwayFuelEconomyRequirement' : '10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement'[requirement_def]
          (reference_usage reference :>> '10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::requiredFuelEconomy'[attribute_usage]
            (feature_value (=)))))
      (package 'VehicleDesignModel'
        (part_def 'Vehicle'
          (attribute_usage composite 'fuelEconomy_city' :> '10c-Fuel Economy Analysis::distancePerVolume'[attribute_usage])
          (attribute_usage composite 'fuelEconomy_highway' :> '10c-Fuel Economy Analysis::distancePerVolume'[attribute_usage])
          (attribute_usage composite 'cargoWeight' : 'MassValue'[unresolved]))
        (part_def 'Engine')
        (part_def 'Transmission')
        (part_usage 'vehicle1_c1' : '10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle'[part_def]
          (part_usage composite 'engine' : '10c-Fuel Economy Analysis::VehicleDesignModel::Engine'[part_def])
          (part_usage composite 'transmission' : '10c-Fuel Economy Analysis::VehicleDesignModel::Transmission'[part_def]
            (state_usage composite 'transmissionState'
              (state_subaction_membership 'entry'
                (action_usage))
              (source_succession
                (reference_usage reference '1stGear'))
              (state_usage composite '1stGear')
              (source_succession
                (reference_usage reference '2ndGear'))
              (state_usage composite '2ndGear')
              (source_succession
                (reference_usage reference '3rdGear'))
              (state_usage composite '3rdGear')
              (source_succession
                (reference_usage reference '4thGear'))
              (state_usage composite '4thGear')))))
      (package 'FuelEconomyAnalysisModel'
        (namespace_import private -> '10c-Fuel Economy Analysis::VehicleDesignModel'[package])
        (namespace_import private -> '10c-Fuel Economy Analysis::FuelEconomyRequirementsModel'[package])
        (attribute_def 'ScenarioState'
          (reference_usage reference 'position' : 'LengthValue'[unresolved])
          (reference_usage reference 'velocity' : 'SpeedValue'[unresolved])
          (reference_usage reference 'acceleration' : 'AccelerationValue'[unresolved])
          (reference_usage reference 'inclineAngle' : 'AngularMeasureValue'[unresolved]))
        (calculation_def abstract 'NominalScenario'
          (reference_usage in reference 't' : 'TimeValue'[unresolved])
          (return_parameter_membership
            (feature_def out : '10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState'[attribute_def])))
        (calculation_usage 'cityScenario' : '10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario'[calculation_def])
        (calculation_usage 'highwayScenario' : '10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario'[calculation_def])
        (analysis_case_def 'FuelEconomyAnalysis'
          (subject_membership in 'vehicle' : '10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle'[part_def])
          (calculation_usage in 'scenario' : '10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario'[calculation_def])
          (requirement_usage in 'fuelEconomyRequirement' : '10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement'[requirement_def])
          (return_parameter_membership
            (feature_def out 'calculatedFuelEconomy' : 'ScalarQuantityValue'[unresolved]))
          (objective_membership composite 'fuelEconomyAnalysisObjective'
            (documentation)
            (assume_constraint_usage composite
              (documentation))
            (require_constraint_usage composite 'fuelEconomyRequirement'
              (reference_usage reference :>> 'actualFuelEconomy'[unresolved]
                (feature_value (=)))))
          (action_usage composite 'dynamicsAnalysis')
          (action_usage composite 'fuelConsumptionAnalysis'))
        (requirement_usage 'vehicleFuelEconomyRequirementsGroup'
          (subject_membership in 'vehicle' : '10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle'[part_def])
          (requirement_usage composite 'vehicleFuelEconomyRequirement_city' :> '10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::cityFuelEconomyRequirement'[requirement_usage]
            (documentation)
            (reference_usage reference :>> '10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::actualFuelEconomy'[attribute_usage]
              (feature_value (=)))
            (assume_constraint_usage composite
              (result_expr_membership)))
          (requirement_usage composite 'vehicleFuelEconomyRequirement_highway' :> '10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement'[requirement_usage]
            (documentation)
            (reference_usage reference :>> '10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::actualFuelEconomy'[attribute_usage]
              (feature_value (=)))
            (assume_constraint_usage composite
              (result_expr_membership))))
        (part_usage 'analysisContext'
          (analysis_case_usage composite 'cityFuelEconomyAnalysis' : '10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis'[analysis_case_def]
            (subject_membership in 'vehicle'
              (feature_value (=)))
            (calculation_usage in 'scenario'
              (feature_value (=)))
            (requirement_usage in 'fuelEconomyRequirement'
              (feature_value (=))))
          (analysis_case_usage composite 'highwayFuelEconomyAnalysis' : '10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis'[analysis_case_def]
            (subject_membership in 'vehicle'
              (feature_value (=)))
            (calculation_usage in 'scenario'
              (feature_value (=)))
            (requirement_usage in 'fuelEconomyRequirement'
              (feature_value (=))))
          (part_usage composite 'vehicle1_c1_analysized' :> '10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1'[part_usage]
            (reference_usage reference :>> '10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle::fuelEconomy_city'[attribute_usage]
              (feature_value (=)))
            (reference_usage reference :>> '10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle::fuelEconomy_highway'[attribute_usage]
              (feature_value (=))))
          (satisfy_requirement_usage 'vehicleFuelEconomyRequirementsGroup' by '10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized'[part_usage]))))))
~~~
