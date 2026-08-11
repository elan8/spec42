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
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "be48b1643ec6a715e71193bdd72d751c9ae36bd4e8fe6083885a0264d1954bc4") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example"))) (kind "package") (name "Analysis Case Definition Example") (declared-name "Analysis Case Definition Example") (range (start (line 0) (character 0)) (end (line 0) (character 2309))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 45))) (parent (node (document "d0") (qualified-name "Analysis Case Definition Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "Calculation Definitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 41))))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 1)) (end (line 3) (character 44))) (parent (node (document "d0") (qualified-name "Analysis Case Definition Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "Analytical Constraints::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 16)) (end (line 3) (character 40))))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 4) (character 1)) (end (line 4) (character 36))) (parent (node (document "d0") (qualified-name "Analysis Case Definition Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "USCustomaryUnits::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 4) (character 16)) (end (line 4) (character 32))))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::*#import3"))) (kind "import") (name "*") (declared-name "*") (range (start (line 7) (character 1)) (end (line 7) (character 36))) (parent (node (document "d0") (qualified-name "Analysis Case Definition Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 32))))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue"))) (kind "attribute def") (name "DistancePerVolumeValue") (declared-name "DistancePerVolumeValue") (range (start (line 10) (character 1)) (end (line 10) (character 61))) (parent (node (document "d0") (qualified-name "Analysis Case Definition Example"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis"))) (kind "analysis def") (name "FuelEconomyAnalysis") (declared-name "FuelEconomyAnalysis") (range (start (line 29) (character 1)) (end (line 29) (character 1411))) (parent (node (document "d0") (qualified-name "Analysis Case Definition Example"))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::fuelEconomyAnalysisObjective"))) (kind "objective") (name "fuelEconomyAnalysisObjective") (declared-name "fuelEconomyAnalysisObjective") (range (start (line 31) (character 2)) (end (line 31) (character 372))) (parent (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis"))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::fuelEconomyResult"))) (kind "analysis result") (name "fuelEconomyResult") (declared-name "fuelEconomyResult") (range (start (line 83) (character 8)) (end (line 83) (character 96))) (parent (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis"))) (authored (relationships (typing (reference "DistancePerVolumeValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::scenario"))) (kind "attribute") (name "scenario") (declared-name "scenario") (range (start (line 47) (character 2)) (end (line 47) (character 38))) (parent (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis"))) (authored (membership (kind Feature)) (relationships (typing (reference "WayPoint") (range none)) (typing (reference "WayPoint") (range (start (line 47) (character 26)) (end (line 47) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForPower"))) (kind "action") (name "solveForPower") (declared-name "solveForPower") (range (start (line 49) (character 2)) (end (line 49) (character 585))) (parent (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis"))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForPower::acceleration"))) (kind "in out parameter") (name "acceleration") (declared-name "acceleration") (range (start (line 51) (character 3)) (end (line 51) (character 43))) (parent (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForPower"))) (authored (relationships (typing (reference "acceleration : AccelerationValue[*]") (range none)))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForPower::power"))) (kind "in out parameter") (name "power") (declared-name "power") (range (start (line 50) (character 3)) (end (line 50) (character 29))) (parent (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForPower"))) (authored (relationships (typing (reference "power : PowerValue[*]") (range none)))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (range (start (line 30) (character 2)) (end (line 30) (character 28))) (parent (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis"))) (authored (relationships (typing (reference "Vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::Positive"))) (kind "import") (name "Positive") (declared-name "Positive") (range (start (line 8) (character 1)) (end (line 8) (character 39))) (parent (node (document "d0") (qualified-name "Analysis Case Definition Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Positive") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 16)) (end (line 8) (character 38))))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::Real"))) (kind "import") (name "Real") (declared-name "Real") (range (start (line 1) (character 1)) (end (line 1) (character 35))) (parent (node (document "d0") (qualified-name "Analysis Case Definition Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 34))))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::ScalarQuantityValue"))) (kind "import") (name "ScalarQuantityValue") (declared-name "ScalarQuantityValue") (range (start (line 6) (character 1)) (end (line 6) (character 48))) (parent (node (document "d0") (qualified-name "Analysis Case Definition Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::ScalarQuantityValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 6) (character 16)) (end (line 6) (character 47))))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 12) (character 1)) (end (line 12) (character 339))) (parent (node (document "d0") (qualified-name "Analysis Case Definition Example"))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::cargoMass"))) (kind "attribute") (name "cargoMass") (declared-name "cargoMass") (range (start (line 14) (character 8)) (end (line 14) (character 40))) (parent (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue") (range none)) (typing (reference "MassValue") (range (start (line 14) (character 30)) (end (line 14) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::driveTrainEfficiency"))) (kind "attribute") (name "driveTrainEfficiency") (declared-name "driveTrainEfficiency") (range (start (line 17) (character 8)) (end (line 17) (character 46))) (parent (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (typing (reference "Real") (range (start (line 17) (character 41)) (end (line 17) (character 45)))))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::fuelEconomy_city"))) (kind "attribute") (name "fuelEconomy_city") (declared-name "fuelEconomy_city") (range (start (line 19) (character 8)) (end (line 19) (character 60))) (parent (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "DistancePerVolumeValue") (range none)) (typing (reference "DistancePerVolumeValue") (range (start (line 19) (character 37)) (end (line 19) (character 59)))))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::fuelEconomy_highway"))) (kind "attribute") (name "fuelEconomy_highway") (declared-name "fuelEconomy_highway") (range (start (line 20) (character 8)) (end (line 20) (character 63))) (parent (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "DistancePerVolumeValue") (range none)) (typing (reference "DistancePerVolumeValue") (range (start (line 20) (character 40)) (end (line 20) (character 62)))))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 13) (character 8)) (end (line 13) (character 35))) (parent (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue") (range none)) (typing (reference "MassValue") (range (start (line 13) (character 25)) (end (line 13) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::wheelDiameter"))) (kind "attribute") (name "wheelDiameter") (declared-name "wheelDiameter") (range (start (line 16) (character 8)) (end (line 16) (character 46))) (parent (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)) (typing (reference "LengthValue") (range (start (line 16) (character 34)) (end (line 16) (character 45)))))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::WayPoint"))) (kind "attribute def") (name "WayPoint") (declared-name "WayPoint") (range (start (line 23) (character 4)) (end (line 23) (character 104))) (parent (node (document "d0") (qualified-name "Analysis Case Definition Example"))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::WayPoint::position"))) (kind "attribute") (name "position") (declared-name "position") (range (start (line 25) (character 2)) (end (line 25) (character 25))) (parent (node (document "d0") (qualified-name "Analysis Case Definition Example::WayPoint"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::WayPoint::speed"))) (kind "attribute") (name "speed") (declared-name "speed") (range (start (line 26) (character 2)) (end (line 26) (character 21))) (parent (node (document "d0") (qualified-name "Analysis Case Definition Example::WayPoint"))) (authored (membership (kind Feature)) (relationships (typing (reference "SpeedValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::WayPoint::time"))) (kind "attribute") (name "time") (declared-name "time") (range (start (line 24) (character 2)) (end (line 24) (character 19))) (parent (node (document "d0") (qualified-name "Analysis Case Definition Example::WayPoint"))) (authored (membership (kind Feature)) (relationships (typing (reference "TimeValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Definition Example::size"))) (kind "import") (name "size") (declared-name "size") (range (start (line 5) (character 1)) (end (line 5) (character 40))) (parent (node (document "d0") (qualified-name "Analysis Case Definition Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::size") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 5) (character 16)) (end (line 5) (character 39))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Calculation Definitions::*") (range (start (line 2) (character 16)) (end (line 2) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Analytical Constraints::*") (range (start (line 3) (character 16)) (end (line 3) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "USCustomaryUnits::*") (range (start (line 4) (character 16)) (end (line 4) (character 32))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::*#import3"))) (kind namespaceImport) (ordinal 0)) (authored-target "ControlFunctions::*") (range (start (line 7) (character 16)) (end (line 7) (character 32))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Analysis Case Definition Example::ScalarQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::fuelEconomyResult"))) (kind featureTyping) (ordinal 0)) (authored-target "DistancePerVolumeValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::scenario"))) (kind featureTyping) (ordinal 0)) (authored-target "WayPoint") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Analysis Case Definition Example::WayPoint")))))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::scenario"))) (kind featureTyping) (ordinal 1)) (authored-target "WayPoint") (range (start (line 47) (character 26)) (end (line 47) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Analysis Case Definition Example::WayPoint")))))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForPower::acceleration"))) (kind featureTyping) (ordinal 0)) (authored-target "acceleration : AccelerationValue[*]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForPower::power"))) (kind featureTyping) (ordinal 0)) (authored-target "power : PowerValue[*]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::Positive"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Positive") (range (start (line 8) (character 16)) (end (line 8) (character 38))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (range (start (line 1) (character 16)) (end (line 1) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::ScalarQuantityValue"))) (kind membershipImport) (ordinal 0)) (authored-target "Quantities::ScalarQuantityValue") (range (start (line 6) (character 16)) (end (line 6) (character 47))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::cargoMass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::cargoMass"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (range (start (line 14) (character 30)) (end (line 14) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::driveTrainEfficiency"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Analysis Case Definition Example::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::driveTrainEfficiency"))) (kind featureTyping) (ordinal 1)) (authored-target "Real") (range (start (line 17) (character 41)) (end (line 17) (character 45))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Analysis Case Definition Example::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::fuelEconomy_city"))) (kind featureTyping) (ordinal 0)) (authored-target "DistancePerVolumeValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::fuelEconomy_city"))) (kind featureTyping) (ordinal 1)) (authored-target "DistancePerVolumeValue") (range (start (line 19) (character 37)) (end (line 19) (character 59))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::fuelEconomy_highway"))) (kind featureTyping) (ordinal 0)) (authored-target "DistancePerVolumeValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::fuelEconomy_highway"))) (kind featureTyping) (ordinal 1)) (authored-target "DistancePerVolumeValue") (range (start (line 20) (character 40)) (end (line 20) (character 62))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::mass"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (range (start (line 13) (character 25)) (end (line 13) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::wheelDiameter"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::wheelDiameter"))) (kind featureTyping) (ordinal 1)) (authored-target "LengthValue") (range (start (line 16) (character 34)) (end (line 16) (character 45))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::WayPoint::position"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::WayPoint::speed"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::WayPoint::time"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Definition Example::size"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::size") (range (start (line 5) (character 16)) (end (line 5) (character 39))) (outcome (status unresolved)))
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
