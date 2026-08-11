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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "6f8c93a64f9b2389c3de07894f26c0f05f3b47ab6ab3538d43b9a5cc4f10badd") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis"))) (kind "package") (name "10c-Fuel Economy Analysis") (declared-name "10c-Fuel Economy Analysis"))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::*#import2"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::*#import3"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::*#import4"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis"))) (authored (membership (kind Import) (visibility "private") (import (reference "USCustomaryUnits::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel"))) (kind "package") (name "FuelEconomyAnalysisModel") (declared-name "FuelEconomyAnalysisModel") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "VehicleDesignModel::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "FuelEconomyRequirementsModel::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))) (kind "analysis def") (name "FuelEconomyAnalysis") (declared-name "FuelEconomyAnalysis") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::calculatedFuelEconomy"))) (kind "analysis result") (name "calculatedFuelEconomy") (declared-name "calculatedFuelEconomy") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))) (authored (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::dynamicsAnalysis"))) (kind "action") (name "dynamicsAnalysis") (declared-name "dynamicsAnalysis") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumptionAnalysis"))) (kind "action") (name "fuelConsumptionAnalysis") (declared-name "fuelConsumptionAnalysis") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelEconomyAnalysisObjective"))) (kind "objective") (name "fuelEconomyAnalysisObjective") (declared-name "fuelEconomyAnalysisObjective") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelEconomyRequirement"))) (kind "requirement") (name "fuelEconomyRequirement") (declared-name "fuelEconomyRequirement") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelEconomyRequirement")))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::scenario"))) (kind "calc") (name "scenario") (declared-name "scenario") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))) (authored (membership (kind Feature)) (relationships (typing (reference "NominalScenario")))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))) (authored (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario"))) (kind "calc def") (name "NominalScenario") (declared-name "NominalScenario") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario::"))) (kind "return parameter") (name "") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario"))) (authored (relationships (typing (reference "ScenarioState")))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario::t"))) (kind "in out parameter") (name "t") (declared-name "t") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario"))) (authored (relationships (typing (reference "TimeValue")))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState"))) (kind "attribute def") (name "ScenarioState") (declared-name "ScenarioState") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState::acceleration"))) (kind "attribute") (name "acceleration") (declared-name "acceleration") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState"))) (authored (membership (kind Feature)) (relationships (typing (reference "AccelerationValue")))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState::inclineAngle"))) (kind "attribute") (name "inclineAngle") (declared-name "inclineAngle") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState"))) (authored (membership (kind Feature)) (relationships (typing (reference "AngularMeasureValue")))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState::position"))) (kind "attribute") (name "position") (declared-name "position") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState::velocity"))) (kind "attribute") (name "velocity") (declared-name "velocity") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState"))) (authored (membership (kind Feature)) (relationships (typing (reference "SpeedValue")))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext"))) (kind "part") (name "analysisContext") (declared-name "analysisContext") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized"))) (kind "part") (name "vehicle1_c1_analysized") (declared-name "vehicle1_c1_analysized") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle1_c1")))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized::fuelEconomy_city"))) (kind "attribute") (name "fuelEconomy_city") (declared-name "fuelEconomy_city") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "fuelEconomy_city")))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized::fuelEconomy_highway"))) (kind "attribute") (name "fuelEconomy_highway") (declared-name "fuelEconomy_highway") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "fuelEconomy_highway")))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::cityScenario"))) (kind "calc def") (name "cityScenario") (declared-name "cityScenario") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::highwayScenario"))) (kind "calc def") (name "highwayScenario") (declared-name "highwayScenario") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup"))) (kind "requirement") (name "vehicleFuelEconomyRequirementsGroup") (declared-name "vehicleFuelEconomyRequirementsGroup") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel"))) (authored (membership (kind Feature)) (relationships (subject (reference "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicle")))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup"))) (authored (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city"))) (kind "requirement") (name "vehicleFuelEconomyRequirement_city") (declared-name "vehicleFuelEconomyRequirement_city") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "cityFuelEconomyRequirement")))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city::_requireConstraint_0"))) (kind "require constraint") (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city::actualFuelEconomy"))) (kind "attribute") (name "actualFuelEconomy") (declared-name "actualFuelEconomy") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city"))) (authored (relationships (redefinition (reference "actualFuelEconomy")))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway"))) (kind "requirement") (name "vehicleFuelEconomyRequirement_highway") (declared-name "vehicleFuelEconomyRequirement_highway") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "highwayFuelEconomyRequirement")))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway::_requireConstraint_0"))) (kind "require constraint") (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway::actualFuelEconomy"))) (kind "attribute") (name "actualFuelEconomy") (declared-name "actualFuelEconomy") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway"))) (authored (relationships (redefinition (reference "actualFuelEconomy")))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel"))) (kind "package") (name "FuelEconomyRequirementsModel") (declared-name "FuelEconomyRequirementsModel") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement"))) (kind "requirement def") (name "FuelEconomyRequirement") (declared-name "FuelEconomyRequirement") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::_requireConstraint_0"))) (kind "require constraint") (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::actualFuelEconomy"))) (kind "attribute") (name "actualFuelEconomy") (declared-name "actualFuelEconomy") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement"))) (authored (relationships (subsetting (reference "distancePerVolume")))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::requiredFuelEconomy"))) (kind "attribute") (name "requiredFuelEconomy") (declared-name "requiredFuelEconomy") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement"))) (authored (relationships (subsetting (reference "distancePerVolume")))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::cityFuelEconomyRequirement"))) (kind "requirement") (name "cityFuelEconomyRequirement") (declared-name "cityFuelEconomyRequirement") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelEconomyRequirement")))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::cityFuelEconomyRequirement::requiredFuelEconomy"))) (kind "attribute") (name "requiredFuelEconomy") (declared-name "requiredFuelEconomy") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::cityFuelEconomyRequirement"))) (authored (relationships (redefinition (reference "requiredFuelEconomy")))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement"))) (kind "requirement") (name "highwayFuelEconomyRequirement") (declared-name "highwayFuelEconomyRequirement") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelEconomyRequirement")))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement::requiredFuelEconomy"))) (kind "attribute") (name "requiredFuelEconomy") (declared-name "requiredFuelEconomy") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement"))) (authored (relationships (redefinition (reference "requiredFuelEconomy")))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel"))) (kind "package") (name "VehicleDesignModel") (declared-name "VehicleDesignModel") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Transmission"))) (kind "part def") (name "Transmission") (declared-name "Transmission") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle::cargoWeight"))) (kind "attribute") (name "cargoWeight") (declared-name "cargoWeight") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue")) (typing (reference "MassValue")))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle::fuelEconomy_city"))) (kind "attribute") (name "fuelEconomy_city") (declared-name "fuelEconomy_city") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "distancePerVolume")))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle::fuelEconomy_highway"))) (kind "attribute") (name "fuelEconomy_highway") (declared-name "fuelEconomy_highway") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "distancePerVolume")))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1"))) (kind "part") (name "vehicle1_c1") (declared-name "vehicle1_c1") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::engine"))) (kind "part") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission"))) (kind "part") (name "transmission") (declared-name "transmission") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Transmission")))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState"))) (kind "state") (name "transmissionState") (declared-name "transmissionState") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission"))) (authored (membership (kind Feature)) (relationships (initial-state (reference "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::1stGear")) (initial-state (reference "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::2ndGear")) (initial-state (reference "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::3rdGear")) (initial-state (reference "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::4thGear")))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::1stGear"))) (kind "state") (name "1stGear") (declared-name "1stGear") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::2ndGear"))) (kind "state") (name "2ndGear") (declared-name "2ndGear") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::3rdGear"))) (kind "state") (name "3rdGear") (declared-name "3rdGear") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::4thGear"))) (kind "state") (name "4thGear") (declared-name "4thGear") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::_entry"))) (kind "action") (name "entry") (declared-name "entry") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState"))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::distancePerVolume"))) (kind "attribute def") (name "distancePerVolume") (declared-name "distancePerVolume") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::gallon"))) (kind "attribute def") (name "gallon") (declared-name "gallon") (parent (node (document "d0") (qualified-name "10c-Fuel Economy Analysis"))) (authored (membership (kind Owning)) (relationships (typing (reference "MeasurementUnit")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Quantities::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "MeasurementReferences::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::*#import3"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::*#import4"))) (kind namespaceImport) (ordinal 0)) (authored-target "USCustomaryUnits::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "VehicleDesignModel::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "FuelEconomyRequirementsModel::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::calculatedFuelEconomy"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelEconomyRequirement"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelEconomyRequirement") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::scenario"))) (kind featureTyping) (ordinal 0)) (authored-target "NominalScenario") (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario::"))) (kind featureTyping) (ordinal 0)) (authored-target "ScenarioState") (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario::t"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState::acceleration"))) (kind featureTyping) (ordinal 0)) (authored-target "AccelerationValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState::inclineAngle"))) (kind featureTyping) (ordinal 0)) (authored-target "AngularMeasureValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState::position"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState::velocity"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext"))) (kind satisfySource) (ordinal 0)) (authored-target "vehicleFuelEconomyRequirementsGroup") (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext"))) (kind satisfyTarget) (ordinal 0)) (authored-target "vehicle1_c1_analysized") (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle1_c1") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized::fuelEconomy_city"))) (kind redefinition) (ordinal 0)) (authored-target "fuelEconomy_city") (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized::fuelEconomy_city")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized::fuelEconomy_highway"))) (kind redefinition) (ordinal 0)) (authored-target "fuelEconomy_highway") (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized::fuelEconomy_highway")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city"))) (kind subsetting) (ordinal 0)) (authored-target "cityFuelEconomyRequirement") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city::actualFuelEconomy"))) (kind redefinition) (ordinal 0)) (authored-target "actualFuelEconomy") (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city::actualFuelEconomy")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway"))) (kind subsetting) (ordinal 0)) (authored-target "highwayFuelEconomyRequirement") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway::actualFuelEconomy"))) (kind redefinition) (ordinal 0)) (authored-target "actualFuelEconomy") (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway::actualFuelEconomy")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::actualFuelEconomy"))) (kind subsetting) (ordinal 0)) (authored-target "distancePerVolume") (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::distancePerVolume")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::requiredFuelEconomy"))) (kind subsetting) (ordinal 0)) (authored-target "distancePerVolume") (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::distancePerVolume")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::cityFuelEconomyRequirement"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelEconomyRequirement") (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::cityFuelEconomyRequirement::requiredFuelEconomy"))) (kind redefinition) (ordinal 0)) (authored-target "requiredFuelEconomy") (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::cityFuelEconomyRequirement::requiredFuelEconomy")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelEconomyRequirement") (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement::requiredFuelEconomy"))) (kind redefinition) (ordinal 0)) (authored-target "requiredFuelEconomy") (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement::requiredFuelEconomy")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle::cargoWeight"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle::cargoWeight"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle::fuelEconomy_city"))) (kind subsetting) (ordinal 0)) (authored-target "distancePerVolume") (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::distancePerVolume")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle::fuelEconomy_highway"))) (kind subsetting) (ordinal 0)) (authored-target "distancePerVolume") (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::distancePerVolume")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission"))) (kind featureTyping) (ordinal 0)) (authored-target "Transmission") (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Transmission")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState"))) (kind initialStateSource) (ordinal 0)) (authored-target "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::1stGear") (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::1stGear")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState"))) (kind initialStateSource) (ordinal 1)) (authored-target "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::2ndGear") (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::2ndGear")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState"))) (kind initialStateSource) (ordinal 2)) (authored-target "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::3rdGear") (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::3rdGear")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState"))) (kind initialStateSource) (ordinal 3)) (authored-target "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::4thGear") (outcome (status resolved) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission::transmissionState::4thGear")))))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::distancePerVolume"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::gallon"))) (kind featureTyping) (ordinal 0)) (authored-target "MeasurementUnit") (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::scenario"))) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::FuelEconomyAnalysis::scenario"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario::"))) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::ScenarioState"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::NominalScenario::"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized::fuelEconomy_city"))) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized::fuelEconomy_city"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized::fuelEconomy_city"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized::fuelEconomy_highway"))) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized::fuelEconomy_highway"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized::fuelEconomy_highway"))) (kind redefinition) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup"))) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind satisfy) (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup"))) (target (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext"))) (kind satisfySource) (ordinal 0)) (expression (kind satisfy) (source "vehicleFuelEconomyRequirementsGroup") (target "vehicle1_c1_analysized")))
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
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 4 16) (end 4 19)) (probe (position 4 16))
      (reference
        (source (document "d0") (qualified-name "10c-Fuel Economy Analysis::*#import3"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQ::*")
        (range (start 4 16) (end 4 19))
        (outcome (status unresolved))
      )
    )
    (query (range (start 42 17) (end 42 23)) (probe (position 42 17))
      (reference
        (source (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::engine"))
        (kind featureTyping) (ordinal 0) (authored-target "Engine")
        (range (start 42 17) (end 42 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Engine") (range (start 38 2) (end 38 18)))
        )
      )
    )
    (query (range (start 41 21) (end 41 28)) (probe (position 41 21))
      (reference
        (source (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 41 21) (end 41 28))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle") (range (start 31 2) (end 31 173)))
        )
      )
    )
    (query (range (start 35 27) (end 35 36)) (probe (position 35 27))
      (reference
        (source (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle::cargoWeight"))
        (kind featureTyping) (ordinal 1) (authored-target "MassValue")
        (range (start 35 27) (end 35 36))
        (outcome (status unresolved))
      )
    )
    (query (range (start 2 16) (end 2 26)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "10c-Fuel Economy Analysis::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "Quantities::*")
        (range (start 2 16) (end 2 26))
        (outcome (status unresolved))
      )
    )
    (query (range (start 158 34) (end 158 45)) (probe (position 158 34))
      (reference
        (source (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized"))
        (kind subsetting) (ordinal 0) (authored-target "vehicle1_c1")
        (range (start 158 34) (end 158 45))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 28)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "10c-Fuel Economy Analysis::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 1 16) (end 1 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 43 23) (end 43 35)) (probe (position 43 23))
      (reference
        (source (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::vehicle1_c1::transmission"))
        (kind featureTyping) (ordinal 0) (authored-target "Transmission")
        (range (start 43 23) (end 43 35))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Transmission") (range (start 39 2) (end 39 24)))
        )
      )
    )
    (query (range (start 5 16) (end 5 32)) (probe (position 5 16))
      (reference
        (source (document "d0") (qualified-name "10c-Fuel Economy Analysis::*#import4"))
        (kind namespaceImport) (ordinal 0) (authored-target "USCustomaryUnits::*")
        (range (start 5 16) (end 5 32))
        (outcome (status unresolved))
      )
    )
    (query (range (start 13 34) (end 13 51)) (probe (position 13 34))
      (reference
        (source (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::actualFuelEconomy"))
        (kind subsetting) (ordinal 0) (authored-target "distancePerVolume")
        (range (start 13 34) (end 13 51))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "10c-Fuel Economy Analysis::distancePerVolume") (range (start 7 1) (end 7 69)))
        )
      )
    )
    (query (range (start 14 36) (end 14 53)) (probe (position 14 36))
      (reference
        (source (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::FuelEconomyRequirement::requiredFuelEconomy"))
        (kind subsetting) (ordinal 0) (authored-target "distancePerVolume")
        (range (start 14 36) (end 14 53))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "10c-Fuel Economy Analysis::distancePerVolume") (range (start 7 1) (end 7 69)))
        )
      )
    )
    (query (range (start 32 33) (end 32 50)) (probe (position 32 33))
      (reference
        (source (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle::fuelEconomy_city"))
        (kind subsetting) (ordinal 0) (authored-target "distancePerVolume")
        (range (start 32 33) (end 32 50))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "10c-Fuel Economy Analysis::distancePerVolume") (range (start 7 1) (end 7 69)))
        )
      )
    )
    (query (range (start 33 36) (end 33 53)) (probe (position 33 36))
      (reference
        (source (document "d0") (qualified-name "10c-Fuel Economy Analysis::VehicleDesignModel::Vehicle::fuelEconomy_highway"))
        (kind subsetting) (ordinal 0) (authored-target "distancePerVolume")
        (range (start 33 36) (end 33 53))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "10c-Fuel Economy Analysis::distancePerVolume") (range (start 7 1) (end 7 69)))
        )
      )
    )
    (query (range (start 60 17) (end 60 35)) (probe (position 60 17))
      (reference
        (source (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "VehicleDesignModel::*")
        (range (start 60 17) (end 60 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 159 4) (end 159 24)) (probe (position 159 4))
      (reference
        (source (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized::fuelEconomy_city"))
        (kind redefinition) (ordinal 0) (authored-target "fuelEconomy_city")
        (range (start 159 4) (end 159 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized::fuelEconomy_city") (range (start 159 4) (end 159 73)))
        )
      )
    )
    (query (range (start 3 16) (end 3 37)) (probe (position 3 16))
      (reference
        (source (document "d0") (qualified-name "10c-Fuel Economy Analysis::*#import2"))
        (kind namespaceImport) (ordinal 0) (authored-target "MeasurementReferences::*")
        (range (start 3 16) (end 3 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 127 4) (end 127 25)) (probe (position 127 4))
      (reference
        (source (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city::actualFuelEconomy"))
        (kind redefinition) (ordinal 0) (authored-target "actualFuelEconomy")
        (range (start 127 4) (end 127 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city::actualFuelEconomy") (range (start 127 4) (end 127 53)))
        )
      )
    )
    (query (range (start 137 4) (end 137 25)) (probe (position 137 4))
      (reference
        (source (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway::actualFuelEconomy"))
        (kind redefinition) (ordinal 0) (authored-target "actualFuelEconomy")
        (range (start 137 4) (end 137 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway::actualFuelEconomy") (range (start 137 4) (end 137 56)))
        )
      )
    )
    (query (range (start 163 50) (end 163 72)) (probe (position 163 50))
      (reference
        (source (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext"))
        (kind satisfyTarget) (ordinal 0) (authored-target "vehicle1_c1_analysized")
        (range (start 163 50) (end 163 72))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized") (range (start 158 3) (end 158 206)))
        )
      )
    )
    (query (range (start 20 3) (end 20 26)) (probe (position 20 3))
      (reference
        (source (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::cityFuelEconomyRequirement::requiredFuelEconomy"))
        (kind redefinition) (ordinal 0) (authored-target "requiredFuelEconomy")
        (range (start 20 3) (end 20 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::cityFuelEconomyRequirement::requiredFuelEconomy") (range (start 20 3) (end 20 44)))
        )
      )
    )
    (query (range (start 24 3) (end 24 26)) (probe (position 24 3))
      (reference
        (source (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement::requiredFuelEconomy"))
        (kind redefinition) (ordinal 0) (authored-target "requiredFuelEconomy")
        (range (start 24 3) (end 24 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyRequirementsModel::highwayFuelEconomyRequirement::requiredFuelEconomy") (range (start 24 3) (end 24 44)))
        )
      )
    )
    (query (range (start 160 4) (end 160 27)) (probe (position 160 4))
      (reference
        (source (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized::fuelEconomy_highway"))
        (kind redefinition) (ordinal 0) (authored-target "fuelEconomy_highway")
        (range (start 160 4) (end 160 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext::vehicle1_c1_analysized::fuelEconomy_highway") (range (start 160 4) (end 160 79)))
        )
      )
    )
    (query (range (start 122 53) (end 122 79)) (probe (position 122 53))
      (reference
        (source (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_city"))
        (kind subsetting) (ordinal 0) (authored-target "cityFuelEconomyRequirement")
        (range (start 122 53) (end 122 79))
        (outcome (status unresolved))
      )
    )
    (query (range (start 61 17) (end 61 45)) (probe (position 61 17))
      (reference
        (source (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "FuelEconomyRequirementsModel::*")
        (range (start 61 17) (end 61 45))
        (outcome (status unresolved))
      )
    )
    (query (range (start 132 56) (end 132 85)) (probe (position 132 56))
      (reference
        (source (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup::vehicleFuelEconomyRequirement_highway"))
        (kind subsetting) (ordinal 0) (authored-target "highwayFuelEconomyRequirement")
        (range (start 132 56) (end 132 85))
        (outcome (status unresolved))
      )
    )
    (query (range (start 163 11) (end 163 46)) (probe (position 163 11))
      (reference
        (source (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::analysisContext"))
        (kind satisfySource) (ordinal 0) (authored-target "vehicleFuelEconomyRequirementsGroup")
        (range (start 163 11) (end 163 46))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "10c-Fuel Economy Analysis::FuelEconomyAnalysisModel::vehicleFuelEconomyRequirementsGroup") (range (start 120 2) (end 120 856)))
        )
      )
    )
  )
)
~~~
