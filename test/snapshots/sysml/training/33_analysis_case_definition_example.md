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
  (document "33_analysis_case_definition_example.md"
    (diagnostics
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
        (range (start 2 16) (end 2 41))
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
        (range (start 4 16) (end 4 32))
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
        (range (start 7 16) (end 7 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 8) (end 13 35))
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
        (range (start 14 8) (end 14 40))
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
        (range (start 16 8) (end 16 46))
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
        (range (start 24 2) (end 24 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 25 2) (end 25 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 26 2) (end 26 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 50 3) (end 50 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 51 3) (end 51 43))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 83 8) (end 83 96))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "be48b1643ec6a715e71193bdd72d751c9ae36bd4e8fe6083885a0264d1954bc4") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example"))) (kind "package") (name "Analysis Case Definition Example") (declared-name "Analysis Case Definition Example"))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Analysis Case Definition Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "Calculation Definitions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Analysis Case Definition Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "Analytical Constraints::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::*#import2"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Analysis Case Definition Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "USCustomaryUnits::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::*#import3"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Analysis Case Definition Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue"))) (kind "attribute def") (name "DistancePerVolumeValue") (declared-name "DistancePerVolumeValue") (parent (node (document "d0") (qualified-name "Analysis Case Definition Example"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis"))) (kind "analysis def") (name "FuelEconomyAnalysis") (declared-name "FuelEconomyAnalysis") (parent (node (document "d0") (qualified-name "Analysis Case Definition Example"))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::fuelEconomyAnalysisObjective"))) (kind "objective") (name "fuelEconomyAnalysisObjective") (declared-name "fuelEconomyAnalysisObjective") (parent (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis"))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::fuelEconomyResult"))) (kind "analysis result") (name "fuelEconomyResult") (declared-name "fuelEconomyResult") (parent (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis"))) (authored (relationships (typing (reference "DistancePerVolumeValue")))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::scenario"))) (kind "attribute") (name "scenario") (declared-name "scenario") (parent (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis"))) (authored (membership (kind Feature)) (relationships (typing (reference "WayPoint")) (typing (reference "WayPoint")))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForPower"))) (kind "action") (name "solveForPower") (declared-name "solveForPower") (parent (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis"))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForPower::acceleration"))) (kind "in out parameter") (name "acceleration") (declared-name "acceleration") (parent (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForPower"))) (authored (relationships (typing (reference "acceleration : AccelerationValue[*]")))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForPower::power"))) (kind "in out parameter") (name "power") (declared-name "power") (parent (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForPower"))) (authored (relationships (typing (reference "power : PowerValue[*]")))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis"))) (authored (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::Positive"))) (kind "import") (name "Positive") (declared-name "Positive") (parent (node (document "d0") (qualified-name "Analysis Case Definition Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Positive") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::Real"))) (kind "import") (name "Real") (declared-name "Real") (parent (node (document "d0") (qualified-name "Analysis Case Definition Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::ScalarQuantityValue"))) (kind "import") (name "ScalarQuantityValue") (declared-name "ScalarQuantityValue") (parent (node (document "d0") (qualified-name "Analysis Case Definition Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::ScalarQuantityValue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "Analysis Case Definition Example"))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::cargoMass"))) (kind "attribute") (name "cargoMass") (declared-name "cargoMass") (parent (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue")) (typing (reference "MassValue")))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::driveTrainEfficiency"))) (kind "attribute") (name "driveTrainEfficiency") (declared-name "driveTrainEfficiency") (parent (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::fuelEconomy_city"))) (kind "attribute") (name "fuelEconomy_city") (declared-name "fuelEconomy_city") (parent (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "DistancePerVolumeValue")) (typing (reference "DistancePerVolumeValue")))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::fuelEconomy_highway"))) (kind "attribute") (name "fuelEconomy_highway") (declared-name "fuelEconomy_highway") (parent (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "DistancePerVolumeValue")) (typing (reference "DistancePerVolumeValue")))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue")) (typing (reference "MassValue")))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::wheelDiameter"))) (kind "attribute") (name "wheelDiameter") (declared-name "wheelDiameter") (parent (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")) (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::WayPoint"))) (kind "attribute def") (name "WayPoint") (declared-name "WayPoint") (parent (node (document "d0") (qualified-name "Analysis Case Definition Example"))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::WayPoint::position"))) (kind "attribute") (name "position") (declared-name "position") (parent (node (document "d0") (qualified-name "Analysis Case Definition Example::WayPoint"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::WayPoint::speed"))) (kind "attribute") (name "speed") (declared-name "speed") (parent (node (document "d0") (qualified-name "Analysis Case Definition Example::WayPoint"))) (authored (membership (kind Feature)) (relationships (typing (reference "SpeedValue")))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::WayPoint::time"))) (kind "attribute") (name "time") (declared-name "time") (parent (node (document "d0") (qualified-name "Analysis Case Definition Example::WayPoint"))) (authored (membership (kind Feature)) (relationships (typing (reference "TimeValue")))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::size"))) (kind "import") (name "size") (declared-name "size") (parent (node (document "d0") (qualified-name "Analysis Case Definition Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::size") (origin Import) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Calculation Definitions::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Analytical Constraints::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "USCustomaryUnits::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::*#import3"))) (kind namespaceImport) (ordinal 0)) (authored-target "ControlFunctions::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Analysis Case Definition Example::ScalarQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::fuelEconomyResult"))) (kind featureTyping) (ordinal 0)) (authored-target "DistancePerVolumeValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::scenario"))) (kind featureTyping) (ordinal 0)) (authored-target "WayPoint") (outcome (status resolved) (target (node (document "d0") (qualified-name "Analysis Case Definition Example::WayPoint")))))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::scenario"))) (kind featureTyping) (ordinal 1)) (authored-target "WayPoint") (outcome (status resolved) (target (node (document "d0") (qualified-name "Analysis Case Definition Example::WayPoint")))))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForPower::acceleration"))) (kind featureTyping) (ordinal 0)) (authored-target "acceleration : AccelerationValue[*]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForPower::power"))) (kind featureTyping) (ordinal 0)) (authored-target "power : PowerValue[*]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::Positive"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Positive") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::ScalarQuantityValue"))) (kind membershipImport) (ordinal 0)) (authored-target "Quantities::ScalarQuantityValue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::cargoMass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::cargoMass"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::driveTrainEfficiency"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "Analysis Case Definition Example::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::driveTrainEfficiency"))) (kind featureTyping) (ordinal 1)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "Analysis Case Definition Example::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::fuelEconomy_city"))) (kind featureTyping) (ordinal 0)) (authored-target "DistancePerVolumeValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::fuelEconomy_city"))) (kind featureTyping) (ordinal 1)) (authored-target "DistancePerVolumeValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::fuelEconomy_highway"))) (kind featureTyping) (ordinal 0)) (authored-target "DistancePerVolumeValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::fuelEconomy_highway"))) (kind featureTyping) (ordinal 1)) (authored-target "DistancePerVolumeValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::mass"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::wheelDiameter"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::wheelDiameter"))) (kind featureTyping) (ordinal 1)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::WayPoint::position"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::WayPoint::speed"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::WayPoint::time"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::size"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::size") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue"))) (target (node (document "d0") (qualified-name "Analysis Case Definition Example::ScalarQuantityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subject) (source (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis"))) (target (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::fuelEconomyResult"))) (target (node (document "d0") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::fuelEconomyResult"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::scenario"))) (target (node (document "d0") (qualified-name "Analysis Case Definition Example::WayPoint"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::scenario"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::scenario"))) (target (node (document "d0") (qualified-name "Analysis Case Definition Example::WayPoint"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::scenario"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::vehicle"))) (target (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::driveTrainEfficiency"))) (target (node (document "d0") (qualified-name "Analysis Case Definition Example::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::driveTrainEfficiency"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::driveTrainEfficiency"))) (target (node (document "d0") (qualified-name "Analysis Case Definition Example::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::driveTrainEfficiency"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::fuelEconomy_city"))) (target (node (document "d0") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::fuelEconomy_city"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::fuelEconomy_city"))) (target (node (document "d0") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::fuelEconomy_city"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::fuelEconomy_highway"))) (target (node (document "d0") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::fuelEconomy_highway"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::fuelEconomy_highway"))) (target (node (document "d0") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::fuelEconomy_highway"))) (kind featureTyping) (ordinal 1)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::fuelEconomyResult")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 17 41) (end 17 45)) (probe (position 17 41))
      (reference
        (source (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::driveTrainEfficiency"))
        (kind featureTyping) (ordinal 1) (authored-target "Real")
        (range (start 17 41) (end 17 45))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Analysis Case Definition Example::Real") (range (start 1 1) (end 1 35)))
        )
      )
    )
    (query (range (start 47 26) (end 47 34)) (probe (position 47 26))
      (reference
        (source (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::scenario"))
        (kind featureTyping) (ordinal 1) (authored-target "WayPoint")
        (range (start 47 26) (end 47 34))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Analysis Case Definition Example::WayPoint") (range (start 23 4) (end 23 104)))
        )
      )
    )
    (query (range (start 13 25) (end 13 34)) (probe (position 13 25))
      (reference
        (source (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::mass"))
        (kind featureTyping) (ordinal 1) (authored-target "MassValue")
        (range (start 13 25) (end 13 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 14 30) (end 14 39)) (probe (position 14 30))
      (reference
        (source (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::cargoMass"))
        (kind featureTyping) (ordinal 1) (authored-target "MassValue")
        (range (start 14 30) (end 14 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 16 34) (end 16 45)) (probe (position 16 34))
      (reference
        (source (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::wheelDiameter"))
        (kind featureTyping) (ordinal 1) (authored-target "LengthValue")
        (range (start 16 34) (end 16 45))
        (outcome (status unresolved))
      )
    )
    (query (range (start 4 16) (end 4 32)) (probe (position 4 16))
      (reference
        (source (document "d0") (qualified-name "Analysis Case Definition Example::*#import2"))
        (kind namespaceImport) (ordinal 0) (authored-target "USCustomaryUnits::*")
        (range (start 4 16) (end 4 32))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 16) (end 7 32)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "Analysis Case Definition Example::*#import3"))
        (kind namespaceImport) (ordinal 0) (authored-target "ControlFunctions::*")
        (range (start 7 16) (end 7 32))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 34)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Analysis Case Definition Example::Real"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
        (range (start 1 16) (end 1 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 16) (end 8 38)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "Analysis Case Definition Example::Positive"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Positive")
        (range (start 8 16) (end 8 38))
        (outcome (status unresolved))
      )
    )
    (query (range (start 19 37) (end 19 59)) (probe (position 19 37))
      (reference
        (source (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::fuelEconomy_city"))
        (kind featureTyping) (ordinal 1) (authored-target "DistancePerVolumeValue")
        (range (start 19 37) (end 19 59))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue") (range (start 10 1) (end 10 61)))
        )
      )
    )
    (query (range (start 20 40) (end 20 62)) (probe (position 20 40))
      (reference
        (source (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::fuelEconomy_highway"))
        (kind featureTyping) (ordinal 1) (authored-target "DistancePerVolumeValue")
        (range (start 20 40) (end 20 62))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue") (range (start 10 1) (end 10 61)))
        )
      )
    )
    (query (range (start 5 16) (end 5 39)) (probe (position 5 16))
      (reference
        (source (document "d0") (qualified-name "Analysis Case Definition Example::size"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::size")
        (range (start 5 16) (end 5 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 3 16) (end 3 40)) (probe (position 3 16))
      (reference
        (source (document "d0") (qualified-name "Analysis Case Definition Example::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "Analytical Constraints::*")
        (range (start 3 16) (end 3 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 2 16) (end 2 41)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "Analysis Case Definition Example::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Calculation Definitions::*")
        (range (start 2 16) (end 2 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 6 16) (end 6 47)) (probe (position 6 16))
      (reference
        (source (document "d0") (qualified-name "Analysis Case Definition Example::ScalarQuantityValue"))
        (kind membershipImport) (ordinal 0) (authored-target "Quantities::ScalarQuantityValue")
        (range (start 6 16) (end 6 47))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
