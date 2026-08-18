# META
~~~ini
description=SysML Training 33 (Analysis): Analysis Case Definition Example
type=file
~~~
# SOURCE
~~~sysml
package 'Analysis Case Definition Example' {
	private import ScalarValues::Real;
	private import 'Calculation Definitions'::*;
	private import 'Analytical Constraints'::*;
	private import USCustomaryUnits::*;
	private import SequenceFunctions::size;
	private import Quantities::ScalarQuantityValue;
	private import ControlFunctions::*;
	private import ScalarValues::Positive;
	
	attribute def DistancePerVolumeValue :> ScalarQuantityValue;

	part def Vehicle {
        attribute mass : MassValue;
        attribute cargoMass : MassValue;
        
        attribute wheelDiameter : LengthValue;
        attribute driveTrainEfficiency : Real;
        
        attribute fuelEconomy_city : DistancePerVolumeValue;
        attribute fuelEconomy_highway : DistancePerVolumeValue;
    }
    
    attribute def WayPoint {
		time : TimeValue;
		position : LengthValue;
		speed : SpeedValue;    	
	}
    
	analysis def FuelEconomyAnalysis {
		subject vehicle : Vehicle;
		objective fuelEconomyAnalysisObjective {
			/*
			 * The objective of this analysis is to determine whether the
			 * subject vehicle can satisfy the fuel economy requirement.
			 */
			
			assume constraint {
				vehicle.wheelDiameter == 33 ['in'] &
				vehicle.driveTrainEfficiency == 0.4
			}
			
			require constraint {
				fuelEconomyResult > 30 [mi / gal]
			}
		}
	    
		in attribute scenario : WayPoint[*];
	
		action solveForPower {
			out power : PowerValue[*];
			out acceleration : AccelerationValue[*];
		
			/*
			 * Solve for the required engine power as a function of time
			 * to support the scenario.
			 */
			assert constraint {
				(1..size(scenario)-1)->forAll {in i: Positive;
					StraightLineDynamicsEquations (
						power#(i),
						vehicle.mass,
						scenario.time#(i+1) - scenario.time#(i),
						scenario.position#(i),
						scenario.speed#(i),
						scenario.position#(i+1),
						scenario.speed#(i+1),
						acceleration#(i+1)                    
					)
				}
			}
		}
		
		then action solveForFuelConsumption {
			in power : PowerValue[*] = solveForPower.power;
			out fuelEconomy : DistancePerVolumeValue;
		
			/*
			 * Solve the engine equations to determine how much fuel is
			 * consumed.
			 */
		}
		
        return fuelEconomyResult : DistancePerVolumeValue = solveForFuelConsumption.fuelEconomy;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/33_analysis_case_definition_example.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 4 16) (end 4 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 5 16) (end 5 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 16) (end 6 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 10 41) (end 10 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 25) (end 13 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 30) (end 14 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 34) (end 16 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 17 41) (end 17 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 24 9) (end 24 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 25 13) (end 25 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 26 10) (end 26 20))
      )
      (diagnostic
        (severity error)
        (code "recovered_constraint_body_element")
        (source "parser")
        (range (start 43 27) (end 44 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 50 15) (end 50 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 51 22) (end 51 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 58 8) (end 58 12))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 74 14) (end 74 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 74 30) (end 74 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 83 60) (end 83 95))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:52bd8f23719161240156c3beabf807679efad4d5b81a722269ac9d48a9f68c31") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Real") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Calculation Definitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Analytical Constraints") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "USCustomaryUnits") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::size") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (anonymous (kind import) (ordinal 5))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Quantities::ScalarQuantityValue") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (anonymous (kind import) (ordinal 6))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ControlFunctions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (anonymous (kind import) (ordinal 7))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Positive") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue")))))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis"))) (kind analysis-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::fuelEconomyAnalysisObjective"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (named (kind analysis-def) (name "FuelEconomyAnalysis")) (named (kind requirement) (name "fuelEconomyAnalysisObjective")) (anonymous (kind assume-constraint) (ordinal 0))))) (kind assume-constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "vehicle::wheelDiameter")) (memberAccessOperand (reference "vehicle::driveTrainEfficiency")))))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (named (kind analysis-def) (name "FuelEconomyAnalysis")) (named (kind requirement) (name "fuelEconomyAnalysisObjective")) (anonymous (kind require-constraint) (ordinal 0))))) (kind require-constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "fuelEconomyResult")))))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::fuelEconomyResult"))) (kind parameter) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DistancePerVolumeValue")) (memberAccessOperand (reference "solveForFuelConsumption::fuelEconomy")))))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::scenario"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (direction in) (multiplicity (lower unbounded) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "WayPoint")))))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForFuelConsumption"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForFuelConsumption::fuelEconomy"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DistancePerVolumeValue") (direction out)))))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForFuelConsumption::power"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in) (multiplicity (lower unbounded) (upper unbounded))) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PowerValue") (direction in)) (memberAccessOperand (reference "solveForPower::power")))))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForPower"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (named (kind analysis-def) (name "FuelEconomyAnalysis")) (named (kind action) (name "solveForPower")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind assert-constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "scenario")) (invocationCallee (reference "size")))))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForPower::acceleration"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out) (multiplicity (lower unbounded) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AccelerationValue") (direction out)))))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForPower::power"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out) (multiplicity (lower unbounded) (upper unbounded))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PowerValue") (direction out)))))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::vehicle"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle::cargoMass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue")))))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle::driveTrainEfficiency"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")))))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle::fuelEconomy_city"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DistancePerVolumeValue")))))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle::fuelEconomy_highway"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DistancePerVolumeValue")))))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue")))))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle::wheelDiameter"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue")))))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::WayPoint"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::WayPoint::position"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue")))))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::WayPoint::speed"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpeedValue")))))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::WayPoint::time"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TimeValue")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Calculation Definitions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Analytical Constraints")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0))
      (authored-target "USCustomaryUnits")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (anonymous (kind import) (ordinal 6))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ControlFunctions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::size")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0))
      (authored-target "Quantities::ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Positive")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (named (kind analysis-def) (name "FuelEconomyAnalysis")) (named (kind requirement) (name "fuelEconomyAnalysisObjective")) (anonymous (kind require-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "fuelEconomyResult")
      (outcome (status resolved) (target (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::fuelEconomyResult")))))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (named (kind analysis-def) (name "FuelEconomyAnalysis")) (named (kind requirement) (name "fuelEconomyAnalysisObjective")) (anonymous (kind assume-constraint) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "vehicle::wheelDiameter")
      (outcome (status resolved) (target (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle::wheelDiameter")))))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (named (kind analysis-def) (name "FuelEconomyAnalysis")) (named (kind requirement) (name "fuelEconomyAnalysisObjective")) (anonymous (kind assume-constraint) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "vehicle::driveTrainEfficiency")
      (outcome (status resolved) (target (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle::driveTrainEfficiency")))))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::fuelEconomyResult"))) (kind featureTyping) (ordinal 0))
      (authored-target "DistancePerVolumeValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue")))))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::fuelEconomyResult"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "solveForFuelConsumption::fuelEconomy")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::scenario"))) (kind featureTyping) (ordinal 0))
      (authored-target "WayPoint")
      (outcome (status resolved) (target (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::WayPoint")))))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForFuelConsumption::fuelEconomy"))) (kind featureTyping) (ordinal 0))
      (authored-target "DistancePerVolumeValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue")))))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForFuelConsumption::power"))) (kind featureTyping) (ordinal 0))
      (authored-target "PowerValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForFuelConsumption::power"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "solveForPower::power")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (named (kind analysis-def) (name "FuelEconomyAnalysis")) (named (kind action) (name "solveForPower")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "scenario")
      (outcome (status resolved) (target (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::scenario")))))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (named (kind analysis-def) (name "FuelEconomyAnalysis")) (named (kind action) (name "solveForPower")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind invocationCallee) (ordinal 0))
      (authored-target "size")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForPower::acceleration"))) (kind featureTyping) (ordinal 0))
      (authored-target "AccelerationValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForPower::power"))) (kind featureTyping) (ordinal 0))
      (authored-target "PowerValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle::cargoMass"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle::driveTrainEfficiency"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle::fuelEconomy_city"))) (kind featureTyping) (ordinal 0))
      (authored-target "DistancePerVolumeValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue")))))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle::fuelEconomy_highway"))) (kind featureTyping) (ordinal 0))
      (authored-target "DistancePerVolumeValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue")))))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle::wheelDiameter"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::WayPoint::position"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::WayPoint::speed"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpeedValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::WayPoint::time"))) (kind featureTyping) (ordinal 0))
      (authored-target "TimeValue")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (named (kind analysis-def) (name "FuelEconomyAnalysis")) (named (kind requirement) (name "fuelEconomyAnalysisObjective")) (anonymous (kind require-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::fuelEconomyResult"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (named (kind analysis-def) (name "FuelEconomyAnalysis")) (named (kind requirement) (name "fuelEconomyAnalysisObjective")) (anonymous (kind require-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (named (kind analysis-def) (name "FuelEconomyAnalysis")) (named (kind requirement) (name "fuelEconomyAnalysisObjective")) (anonymous (kind assume-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle::wheelDiameter"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (named (kind analysis-def) (name "FuelEconomyAnalysis")) (named (kind requirement) (name "fuelEconomyAnalysisObjective")) (anonymous (kind assume-constraint) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (named (kind analysis-def) (name "FuelEconomyAnalysis")) (named (kind requirement) (name "fuelEconomyAnalysisObjective")) (anonymous (kind assume-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle::driveTrainEfficiency"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (named (kind analysis-def) (name "FuelEconomyAnalysis")) (named (kind requirement) (name "fuelEconomyAnalysisObjective")) (anonymous (kind assume-constraint) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::fuelEconomyResult"))) (target (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::fuelEconomyResult"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::scenario"))) (target (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::WayPoint"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::scenario"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForFuelConsumption::fuelEconomy"))) (target (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForFuelConsumption::fuelEconomy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (named (kind analysis-def) (name "FuelEconomyAnalysis")) (named (kind action) (name "solveForPower")) (anonymous (kind assert-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::scenario"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (named (kind analysis-def) (name "FuelEconomyAnalysis")) (named (kind action) (name "solveForPower")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::vehicle"))) (target (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle::fuelEconomy_city"))) (target (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle::fuelEconomy_city"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle::fuelEconomy_highway"))) (target (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle::fuelEconomy_highway"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (named (kind analysis-def) (name "FuelEconomyAnalysis")) (named (kind requirement) (name "fuelEconomyAnalysisObjective")) (anonymous (kind assume-constraint) (ordinal 0))))) (state unsupported))
    (evaluated (declaration (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (named (kind analysis-def) (name "FuelEconomyAnalysis")) (named (kind requirement) (name "fuelEconomyAnalysisObjective")) (anonymous (kind require-constraint) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::fuelEconomyResult"))) (state unsupported))
    (evaluated (declaration (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForFuelConsumption::power"))) (state unsupported))
    (evaluated (declaration (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (named (kind analysis-def) (name "FuelEconomyAnalysis")) (named (kind action) (name "solveForPower")) (anonymous (kind assert-constraint) (ordinal 0))))) (state unsupported))
    (unit (declaration (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (named (kind analysis-def) (name "FuelEconomyAnalysis")) (named (kind requirement) (name "fuelEconomyAnalysisObjective")) (anonymous (kind assume-constraint) (ordinal 0))))) (ordinal 0) (authored "in") (start 38 33) (end 38 37) (outcome (status catalog-unavailable)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue")))
      (subtype (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::fuelEconomyResult")) (scopes any))
      (subtype (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForFuelConsumption::fuelEconomy")) (scopes any))
      (subtype (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle::fuelEconomy_city")) (scopes any))
      (subtype (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle::fuelEconomy_highway")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::fuelEconomyAnalysisObjective")))
      (featured-by (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis")))
    )
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (named (kind analysis-def) (name "FuelEconomyAnalysis")) (named (kind requirement) (name "fuelEconomyAnalysisObjective")) (anonymous (kind assume-constraint) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::fuelEconomyAnalysisObjective")))
    )
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (named (kind analysis-def) (name "FuelEconomyAnalysis")) (named (kind requirement) (name "fuelEconomyAnalysisObjective")) (anonymous (kind require-constraint) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::fuelEconomyAnalysisObjective")))
    )
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::fuelEconomyResult")))
      (featured-by (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis")))
      (type (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue")) (provenance authored))
      (effective-type (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue")) (source direct))
      (supertype (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::scenario")))
      (featured-by (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis")))
      (type (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::WayPoint")) (provenance authored))
      (effective-type (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::WayPoint")) (source direct))
      (supertype (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::WayPoint")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForFuelConsumption")))
      (featured-by (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis")))
    )
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForFuelConsumption::fuelEconomy")))
      (featured-by (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForFuelConsumption")))
      (type (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue")) (provenance authored))
      (effective-type (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue")) (source direct))
      (supertype (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForFuelConsumption::power")))
      (featured-by (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForFuelConsumption")))
    )
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForPower")))
      (featured-by (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis")))
    )
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (named (kind analysis-def) (name "FuelEconomyAnalysis")) (named (kind action) (name "solveForPower")) (anonymous (kind assert-constraint) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForPower")))
    )
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForPower::acceleration")))
      (featured-by (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForPower")))
    )
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForPower::power")))
      (featured-by (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForPower")))
    )
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::vehicle")))
      (featured-by (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis")))
      (type (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle")))
      (subtype (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle::cargoMass")))
      (featured-by (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle::driveTrainEfficiency")))
      (featured-by (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle::fuelEconomy_city")))
      (featured-by (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle")))
      (type (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue")) (provenance authored))
      (effective-type (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue")) (source direct))
      (supertype (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle::fuelEconomy_highway")))
      (featured-by (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle")))
      (type (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue")) (provenance authored))
      (effective-type (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue")) (source direct))
      (supertype (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle::mass")))
      (featured-by (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle::wheelDiameter")))
      (featured-by (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::WayPoint")))
      (subtype (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::scenario")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::WayPoint::position")))
      (featured-by (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::WayPoint")))
    )
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::WayPoint::speed")))
      (featured-by (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::WayPoint")))
    )
    (declaration (id (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::WayPoint::time")))
      (featured-by (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::WayPoint")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/33_analysis_case_definition_example.md") (range (start 2 16) (end 2 44)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "Calculation Definitions")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/33_analysis_case_definition_example.md") (range (start 3 16) (end 3 43)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "Analytical Constraints")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/33_analysis_case_definition_example.md") (range (start 4 16) (end 4 35)) (probe (position 4 16))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0) (authored-target "USCustomaryUnits")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/33_analysis_case_definition_example.md") (range (start 7 16) (end 7 35)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (anonymous (kind import) (ordinal 6))))) (kind namespaceImport) (ordinal 0) (authored-target "ControlFunctions")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/33_analysis_case_definition_example.md") (range (start 1 16) (end 1 34)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/33_analysis_case_definition_example.md") (range (start 5 16) (end 5 39)) (probe (position 5 16))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::size")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/33_analysis_case_definition_example.md") (range (start 6 16) (end 6 47)) (probe (position 6 16))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0) (authored-target "Quantities::ScalarQuantityValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/33_analysis_case_definition_example.md") (range (start 8 16) (end 8 38)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Positive")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/33_analysis_case_definition_example.md") (range (start 10 41) (end 10 60)) (probe (position 10 41))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/33_analysis_case_definition_example.md") (range (start 43 4) (end 43 21)) (probe (position 43 4))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (named (kind analysis-def) (name "FuelEconomyAnalysis")) (named (kind requirement) (name "fuelEconomyAnalysisObjective")) (anonymous (kind require-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "fuelEconomyResult")
      (outcome (status resolved) (target (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::fuelEconomyResult")))))
    )
  )
  (query (document "memory://snapshot/33_analysis_case_definition_example.md") (range (start 38 4) (end 38 25)) (probe (position 38 4))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (named (kind analysis-def) (name "FuelEconomyAnalysis")) (named (kind requirement) (name "fuelEconomyAnalysisObjective")) (anonymous (kind assume-constraint) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "vehicle::wheelDiameter")
      (outcome (status resolved) (target (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle::wheelDiameter")))))
    )
  )
  (query (document "memory://snapshot/33_analysis_case_definition_example.md") (range (start 39 4) (end 39 32)) (probe (position 39 4))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (named (kind analysis-def) (name "FuelEconomyAnalysis")) (named (kind requirement) (name "fuelEconomyAnalysisObjective")) (anonymous (kind assume-constraint) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1) (authored-target "vehicle::driveTrainEfficiency")
      (outcome (status resolved) (target (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle::driveTrainEfficiency")))))
    )
  )
  (query (document "memory://snapshot/33_analysis_case_definition_example.md") (range (start 83 35) (end 83 57)) (probe (position 83 35))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::fuelEconomyResult"))) (kind featureTyping) (ordinal 0) (authored-target "DistancePerVolumeValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue")))))
    )
  )
  (query (document "memory://snapshot/33_analysis_case_definition_example.md") (range (start 83 60) (end 83 95)) (probe (position 83 60))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::fuelEconomyResult"))) (kind memberAccessOperand) (ordinal 0) (authored-target "solveForFuelConsumption::fuelEconomy")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/33_analysis_case_definition_example.md") (range (start 47 26) (end 47 34)) (probe (position 47 26))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::scenario"))) (kind featureTyping) (ordinal 0) (authored-target "WayPoint")
      (outcome (status resolved) (target (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::WayPoint")))))
    )
  )
  (query (document "memory://snapshot/33_analysis_case_definition_example.md") (range (start 75 21) (end 75 43)) (probe (position 75 21))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForFuelConsumption::fuelEconomy"))) (kind featureTyping) (ordinal 0) (authored-target "DistancePerVolumeValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue")))))
    )
  )
  (query (document "memory://snapshot/33_analysis_case_definition_example.md") (range (start 74 14) (end 74 24)) (probe (position 74 14))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForFuelConsumption::power"))) (kind featureTyping) (ordinal 0) (authored-target "PowerValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/33_analysis_case_definition_example.md") (range (start 74 30) (end 74 49)) (probe (position 74 30))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForFuelConsumption::power"))) (kind memberAccessOperand) (ordinal 0) (authored-target "solveForPower::power")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/33_analysis_case_definition_example.md") (range (start 58 13) (end 58 21)) (probe (position 58 13))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (named (kind analysis-def) (name "FuelEconomyAnalysis")) (named (kind action) (name "solveForPower")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "scenario")
      (outcome (status resolved) (target (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::scenario")))))
    )
  )
  (query (document "memory://snapshot/33_analysis_case_definition_example.md") (range (start 58 8) (end 58 12)) (probe (position 58 8))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (path (named (kind package) (name "Analysis Case Definition Example")) (named (kind analysis-def) (name "FuelEconomyAnalysis")) (named (kind action) (name "solveForPower")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind invocationCallee) (ordinal 0) (authored-target "size")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/33_analysis_case_definition_example.md") (range (start 51 22) (end 51 39)) (probe (position 51 22))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForPower::acceleration"))) (kind featureTyping) (ordinal 0) (authored-target "AccelerationValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/33_analysis_case_definition_example.md") (range (start 50 15) (end 50 25)) (probe (position 50 15))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForPower::power"))) (kind featureTyping) (ordinal 0) (authored-target "PowerValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/33_analysis_case_definition_example.md") (range (start 30 20) (end 30 27)) (probe (position 30 20))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/33_analysis_case_definition_example.md") (range (start 14 30) (end 14 39)) (probe (position 14 30))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle::cargoMass"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/33_analysis_case_definition_example.md") (range (start 17 41) (end 17 45)) (probe (position 17 41))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle::driveTrainEfficiency"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/33_analysis_case_definition_example.md") (range (start 19 37) (end 19 59)) (probe (position 19 37))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle::fuelEconomy_city"))) (kind featureTyping) (ordinal 0) (authored-target "DistancePerVolumeValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue")))))
    )
  )
  (query (document "memory://snapshot/33_analysis_case_definition_example.md") (range (start 20 40) (end 20 62)) (probe (position 20 40))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle::fuelEconomy_highway"))) (kind featureTyping) (ordinal 0) (authored-target "DistancePerVolumeValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue")))))
    )
  )
  (query (document "memory://snapshot/33_analysis_case_definition_example.md") (range (start 13 25) (end 13 34)) (probe (position 13 25))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle::mass"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/33_analysis_case_definition_example.md") (range (start 16 34) (end 16 45)) (probe (position 16 34))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::Vehicle::wheelDiameter"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/33_analysis_case_definition_example.md") (range (start 25 13) (end 25 24)) (probe (position 25 13))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::WayPoint::position"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/33_analysis_case_definition_example.md") (range (start 26 10) (end 26 20)) (probe (position 26 10))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::WayPoint::speed"))) (kind featureTyping) (ordinal 0) (authored-target "SpeedValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/33_analysis_case_definition_example.md") (range (start 24 9) (end 24 18)) (probe (position 24 9))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_definition_example.md") (qualified-name "Analysis Case Definition Example::WayPoint::time"))) (kind featureTyping) (ordinal 0) (authored-target "TimeValue")
      (outcome (status unresolved)))
    )
  )
)
~~~
