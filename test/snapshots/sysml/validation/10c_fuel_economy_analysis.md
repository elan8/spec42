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
  (document "memory://snapshot/10c_fuel_economy_analysis.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 4 16) (end 4 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 5 16) (end 5 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 31) (end 7 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 20) (end 8 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 16 3) (end 16 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 35 27) (end 35 36))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 45 5) (end 45 11))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 64 14) (end 64 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 65 14) (end 65 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 66 18) (end 66 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 67 18) (end 67 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 71 10) (end 71 19))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 81 3) (end 81 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 83 3) (end 99 4))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 127 8) (end 127 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 129 4) (end 129 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 137 8) (end 137 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 139 4) (end 139 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 159 8) (end 159 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 160 8) (end 160 27))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:c0bc4252eea02414014d5bd977826bf23d4a3b61993d5bf6d36a6477fffc864e") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Quantities") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "MeasurementReferences") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQ") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "USCustomaryUnits") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "VehicleDesignModel") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "FuelEconomyRequirementsModel") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))) (kind analysis-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::dynamicsAnalysis"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumptionAnalysis"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelEconomyRequirement"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelEconomyRequirement"))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::scenario"))) (kind calc) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "NominalScenario"))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::vehicle"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScenarioState"))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario::t"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TimeValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState::acceleration"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AccelerationValue"))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState::inclineAngle"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AngularMeasureValue"))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState::position"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue"))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState::velocity"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpeedValue"))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind satisfy) (ordinal 0))))) (kind satisfy) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (satisfySource (reference "vehicleFuelEconomyRequirementsGroup")) (satisfyTarget (reference "vehicle1_c1_analysized"))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::cityFuelEconomyAnalysis"))) (kind analysis) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelEconomyAnalysis"))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::cityFuelEconomyAnalysis::fuelEconomyRequirement"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::cityFuelEconomyAnalysis::scenario"))) (kind calc) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::cityFuelEconomyAnalysis::vehicle"))) (kind subject) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::highwayFuelEconomyAnalysis"))) (kind analysis) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelEconomyAnalysis"))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::highwayFuelEconomyAnalysis::fuelEconomyRequirement"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::highwayFuelEconomyAnalysis::scenario"))) (kind calc) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::highwayFuelEconomyAnalysis::vehicle"))) (kind subject) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "vehicle1_c1"))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "fuelEconomy_city"))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "fuelEconomy_highway"))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::cityScenario"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "NominalScenario"))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::highwayScenario"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "NominalScenario"))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicle"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "cityFuelEconomyRequirement"))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "actualFuelEconomy"))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "highwayFuelEconomyRequirement"))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "actualFuelEconomy"))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::actualFuelEconomy"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "distancePerVolume"))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::requiredFuelEconomy"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "distancePerVolume"))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::cityFuelEconomyRequirement"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelEconomyRequirement"))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "requiredFuelEconomy"))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelEconomyRequirement"))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "requiredFuelEconomy"))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Transmission"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle::cargoWeight"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue"))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle::fuelEconomy_city"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "distancePerVolume"))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle::fuelEconomy_highway"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "distancePerVolume"))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::engine"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Transmission"))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind initial-state) (ordinal 0))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "1stGear"))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind initial-state) (ordinal 1))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "2ndGear"))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind initial-state) (ordinal 2))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "3rdGear"))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind initial-state) (ordinal 3))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "4thGear"))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::1stGear"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::2ndGear"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::3rdGear"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::4thGear"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::distancePerVolume"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::gallon"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "MeasurementUnit"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Quantities")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "MeasurementReferences")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQ")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind import) (ordinal 4))))) (kind namespaceImport) (ordinal 0))
      (authored-target "USCustomaryUnits")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "VehicleDesignModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel")))))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "FuelEconomyRequirementsModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel")))))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelEconomyRequirement"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelEconomyRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement")))))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::scenario"))) (kind featureTyping) (ordinal 0))
      (authored-target "NominalScenario")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario")))))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "ScenarioState")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState")))))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario::t"))) (kind featureTyping) (ordinal 0))
      (authored-target "TimeValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState::acceleration"))) (kind featureTyping) (ordinal 0))
      (authored-target "AccelerationValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState::inclineAngle"))) (kind featureTyping) (ordinal 0))
      (authored-target "AngularMeasureValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState::position"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState::velocity"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpeedValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0))
      (authored-target "vehicleFuelEconomyRequirementsGroup")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup")))))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind satisfy) (ordinal 0))))) (kind satisfyTarget) (ordinal 0))
      (authored-target "vehicle1_c1_analysized")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized")))))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::cityFuelEconomyAnalysis"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelEconomyAnalysis")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis")))))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::highwayFuelEconomyAnalysis"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelEconomyAnalysis")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis")))))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized"))) (kind subsetting) (ordinal 0))
      (authored-target "vehicle1_c1")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1")))))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "fuelEconomy_city")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "fuelEconomy_highway")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::cityScenario"))) (kind featureTyping) (ordinal 0))
      (authored-target "NominalScenario")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario")))))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::highwayScenario"))) (kind featureTyping) (ordinal 0))
      (authored-target "NominalScenario")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario")))))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city"))) (kind subsetting) (ordinal 0))
      (authored-target "cityFuelEconomyRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::cityFuelEconomyRequirement")))))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "actualFuelEconomy")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway"))) (kind subsetting) (ordinal 0))
      (authored-target "highwayFuelEconomyRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement")))))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "actualFuelEconomy")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::actualFuelEconomy"))) (kind subsetting) (ordinal 0))
      (authored-target "distancePerVolume")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::distancePerVolume")))))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::requiredFuelEconomy"))) (kind subsetting) (ordinal 0))
      (authored-target "distancePerVolume")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::distancePerVolume")))))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::cityFuelEconomyRequirement"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelEconomyRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement")))))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "requiredFuelEconomy")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::requiredFuelEconomy")))))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelEconomyRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement")))))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "requiredFuelEconomy")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::requiredFuelEconomy")))))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle::cargoWeight"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle::fuelEconomy_city"))) (kind subsetting) (ordinal 0))
      (authored-target "distancePerVolume")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::distancePerVolume")))))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle::fuelEconomy_highway"))) (kind subsetting) (ordinal 0))
      (authored-target "distancePerVolume")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::distancePerVolume")))))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::engine"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Engine")))))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission"))) (kind featureTyping) (ordinal 0))
      (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Transmission")))))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0))
      (authored-target "1stGear")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::1stGear")))))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind initial-state) (ordinal 1))))) (kind initialState) (ordinal 0))
      (authored-target "2ndGear")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::2ndGear")))))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind initial-state) (ordinal 2))))) (kind initialState) (ordinal 0))
      (authored-target "3rdGear")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::3rdGear")))))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind initial-state) (ordinal 3))))) (kind initialState) (ordinal 0))
      (authored-target "4thGear")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::4thGear")))))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::distancePerVolume"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::gallon"))) (kind featureTyping) (ordinal 0))
      (authored-target "MeasurementUnit")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelEconomyRequirement"))) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelEconomyRequirement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::scenario"))) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::scenario"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::vehicle"))) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind satisfySource) (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind satisfy) (ordinal 0))))) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0)))
    (relationship (kind satisfyTarget) (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind satisfy) (ordinal 0))))) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind satisfy) (ordinal 0))))) (kind satisfyTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::cityFuelEconomyAnalysis"))) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::cityFuelEconomyAnalysis"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::highwayFuelEconomyAnalysis"))) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::highwayFuelEconomyAnalysis"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized"))) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::cityScenario"))) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::cityScenario"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::highwayScenario"))) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::highwayScenario"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicle"))) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city"))) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::cityFuelEconomyRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway"))) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::actualFuelEconomy"))) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::distancePerVolume"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::actualFuelEconomy"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::requiredFuelEconomy"))) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::distancePerVolume"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::requiredFuelEconomy"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::cityFuelEconomyRequirement"))) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::cityFuelEconomyRequirement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::requiredFuelEconomy"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement"))) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::requiredFuelEconomy"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle::fuelEconomy_city"))) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::distancePerVolume"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle::fuelEconomy_city"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle::fuelEconomy_highway"))) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::distancePerVolume"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle::fuelEconomy_highway"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1"))) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::engine"))) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission"))) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Transmission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind initialState) (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::1stGear"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0)))
    (relationship (kind initialState) (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind initial-state) (ordinal 1))))) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::2ndGear"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind initial-state) (ordinal 1))))) (kind initialState) (ordinal 0)))
    (relationship (kind initialState) (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind initial-state) (ordinal 2))))) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::3rdGear"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind initial-state) (ordinal 2))))) (kind initialState) (ordinal 0)))
    (relationship (kind initialState) (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind initial-state) (ordinal 3))))) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::4thGear"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind initial-state) (ordinal 3))))) (kind initialState) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 1 16) (end 1 31)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 2 16) (end 2 29)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "Quantities")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 3 16) (end 3 40)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "MeasurementReferences")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 4 16) (end 4 22)) (probe (position 4 16))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQ")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 5 16) (end 5 35)) (probe (position 5 16))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind import) (ordinal 4))))) (kind namespaceImport) (ordinal 0) (authored-target "USCustomaryUnits")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 60 17) (end 60 38)) (probe (position 60 17))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "VehicleDesignModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel")))))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 61 17) (end 61 48)) (probe (position 61 17))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "FuelEconomyRequirementsModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel")))))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 80 43) (end 80 65)) (probe (position 80 43))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelEconomyRequirement"))) (kind featureTyping) (ordinal 0) (authored-target "FuelEconomyRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement")))))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 79 22) (end 79 37)) (probe (position 79 22))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::scenario"))) (kind featureTyping) (ordinal 0) (authored-target "NominalScenario")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario")))))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 78 21) (end 78 28)) (probe (position 78 21))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle")))))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 72 12) (end 72 25)) (probe (position 72 12))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "ScenarioState")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState")))))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 71 10) (end 71 19)) (probe (position 71 10))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario::t"))) (kind featureTyping) (ordinal 0) (authored-target "TimeValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 66 18) (end 66 35)) (probe (position 66 18))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState::acceleration"))) (kind featureTyping) (ordinal 0) (authored-target "AccelerationValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 67 18) (end 67 37)) (probe (position 67 18))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState::inclineAngle"))) (kind featureTyping) (ordinal 0) (authored-target "AngularMeasureValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 64 14) (end 64 25)) (probe (position 64 14))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState::position"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 65 14) (end 65 24)) (probe (position 65 14))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState::velocity"))) (kind featureTyping) (ordinal 0) (authored-target "SpeedValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 163 11) (end 163 46)) (probe (position 163 11))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0) (authored-target "vehicleFuelEconomyRequirementsGroup")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup")))))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 163 50) (end 163 72)) (probe (position 163 50))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind satisfy) (ordinal 0))))) (kind satisfyTarget) (ordinal 0) (authored-target "vehicle1_c1_analysized")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized")))))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 146 38) (end 146 57)) (probe (position 146 38))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::cityFuelEconomyAnalysis"))) (kind featureTyping) (ordinal 0) (authored-target "FuelEconomyAnalysis")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis")))))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 152 41) (end 152 60)) (probe (position 152 41))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::highwayFuelEconomyAnalysis"))) (kind featureTyping) (ordinal 0) (authored-target "FuelEconomyAnalysis")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis")))))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 158 34) (end 158 45)) (probe (position 158 34))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized"))) (kind subsetting) (ordinal 0) (authored-target "vehicle1_c1")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1")))))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 159 8) (end 159 24)) (probe (position 159 8))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "fuelEconomy_city")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 160 8) (end 160 27)) (probe (position 160 8))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "fuelEconomy_highway")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 74 22) (end 74 37)) (probe (position 74 22))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::cityScenario"))) (kind featureTyping) (ordinal 0) (authored-target "NominalScenario")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario")))))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 75 25) (end 75 40)) (probe (position 75 25))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::highwayScenario"))) (kind featureTyping) (ordinal 0) (authored-target "NominalScenario")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario")))))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 121 21) (end 121 28)) (probe (position 121 21))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle")))))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 122 53) (end 122 79)) (probe (position 122 53))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city"))) (kind subsetting) (ordinal 0) (authored-target "cityFuelEconomyRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::cityFuelEconomyRequirement")))))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 127 8) (end 127 25)) (probe (position 127 8))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "actualFuelEconomy")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 132 56) (end 132 85)) (probe (position 132 56))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway"))) (kind subsetting) (ordinal 0) (authored-target "highwayFuelEconomyRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement")))))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 137 8) (end 137 25)) (probe (position 137 8))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "actualFuelEconomy")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 13 34) (end 13 51)) (probe (position 13 34))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::actualFuelEconomy"))) (kind subsetting) (ordinal 0) (authored-target "distancePerVolume")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::distancePerVolume")))))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 14 36) (end 14 53)) (probe (position 14 36))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::requiredFuelEconomy"))) (kind subsetting) (ordinal 0) (authored-target "distancePerVolume")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::distancePerVolume")))))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 19 43) (end 19 65)) (probe (position 19 43))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::cityFuelEconomyRequirement"))) (kind featureTyping) (ordinal 0) (authored-target "FuelEconomyRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement")))))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 20 7) (end 20 26)) (probe (position 20 7))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "requiredFuelEconomy")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::requiredFuelEconomy")))))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 23 46) (end 23 68)) (probe (position 23 46))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement"))) (kind featureTyping) (ordinal 0) (authored-target "FuelEconomyRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement")))))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 24 7) (end 24 26)) (probe (position 24 7))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "requiredFuelEconomy")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::requiredFuelEconomy")))))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 35 27) (end 35 36)) (probe (position 35 27))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle::cargoWeight"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 32 33) (end 32 50)) (probe (position 32 33))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle::fuelEconomy_city"))) (kind subsetting) (ordinal 0) (authored-target "distancePerVolume")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::distancePerVolume")))))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 33 36) (end 33 53)) (probe (position 33 36))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle::fuelEconomy_highway"))) (kind subsetting) (ordinal 0) (authored-target "distancePerVolume")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::distancePerVolume")))))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 41 21) (end 41 28)) (probe (position 41 21))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle")))))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 42 17) (end 42 23)) (probe (position 42 17))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::engine"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Engine")))))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 43 23) (end 43 35)) (probe (position 43 23))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission"))) (kind featureTyping) (ordinal 0) (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Transmission")))))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 45 17) (end 45 26)) (probe (position 45 17))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0) (authored-target "1stGear")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::1stGear")))))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 47 10) (end 47 19)) (probe (position 47 10))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind initial-state) (ordinal 1))))) (kind initialState) (ordinal 0) (authored-target "2ndGear")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::2ndGear")))))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 49 10) (end 49 19)) (probe (position 49 10))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind initial-state) (ordinal 2))))) (kind initialState) (ordinal 0) (authored-target "3rdGear")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::3rdGear")))))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 51 10) (end 51 19)) (probe (position 51 10))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (anonymous (kind initial-state) (ordinal 3))))) (kind initialState) (ordinal 0) (authored-target "4thGear")
      (outcome (status resolved) (target (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::4thGear")))))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 7 31) (end 7 50)) (probe (position 7 31))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::distancePerVolume"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10c_fuel_economy_analysis.md") (range (start 8 20) (end 8 35)) (probe (position 8 20))
    (reference (id (source (node (document "memory://snapshot/10c_fuel_economy_analysis.md") (qualified-name "10c-Fuel Economy Analysis::gallon"))) (kind featureTyping) (ordinal 0) (authored-target "MeasurementUnit")
      (outcome (status unresolved)))
  )
)
~~~
