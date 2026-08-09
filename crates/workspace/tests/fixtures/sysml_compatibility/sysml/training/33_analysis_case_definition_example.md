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
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,OpenCurly,
Ident,Colon,Ident,Semicolon,
Ident,Colon,Ident,Semicolon,
Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAnalysis,KwDef,Ident,OpenCurly,
KwSubject,Ident,Colon,Ident,Semicolon,
KwObjective,Ident,OpenCurly,
RegularComment,
KwAssume,KwConstraint,OpenCurly,
Ident,Dot,Ident,EqEq,DecimalValue,OpenSquare,UnrestrictedName,CloseSquare,Ampersand,
Ident,Dot,Ident,EqEq,DecimalValue,Dot,DecimalValue,
CloseCurly,
KwRequire,KwConstraint,OpenCurly,
Ident,CloseAngle,DecimalValue,OpenSquare,Ident,Slash,Ident,CloseSquare,
CloseCurly,
CloseCurly,
KwIn,KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Semicolon,
KwAction,Ident,OpenCurly,
KwOut,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Semicolon,
KwOut,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Semicolon,
RegularComment,
KwAssert,KwConstraint,OpenCurly,
OpenParen,DecimalValue,DotDot,Ident,OpenParen,Ident,CloseParen,Minus,DecimalValue,CloseParen,Arrow,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,
Ident,OpenParen,
Ident,Hash,OpenParen,Ident,CloseParen,Comma,
Ident,Dot,Ident,Comma,
Ident,Dot,Ident,Hash,OpenParen,Ident,Plus,DecimalValue,CloseParen,Minus,Ident,Dot,Ident,Hash,OpenParen,Ident,CloseParen,Comma,
Ident,Dot,Ident,Hash,OpenParen,Ident,CloseParen,Comma,
Ident,Dot,Ident,Hash,OpenParen,Ident,CloseParen,Comma,
Ident,Dot,Ident,Hash,OpenParen,Ident,Plus,DecimalValue,CloseParen,Comma,
Ident,Dot,Ident,Hash,OpenParen,Ident,Plus,DecimalValue,CloseParen,Comma,
Ident,Hash,OpenParen,Ident,Plus,DecimalValue,CloseParen,
CloseParen,
CloseCurly,
CloseCurly,
CloseCurly,
KwThen,KwAction,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Eq,Ident,Dot,Ident,Semicolon,
KwOut,Ident,Colon,Ident,Semicolon,
RegularComment,
CloseCurly,
KwReturn,Ident,Colon,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Analysis Case Definition Example''
    (import_decl private 'ScalarValues::Real')
    (import_decl private ''Calculation Definitions'::*')
    (import_decl private ''Analytical Constraints'::*')
    (import_decl private 'USCustomaryUnits::*')
    (import_decl private 'SequenceFunctions::size')
    (import_decl private 'Quantities::ScalarQuantityValue')
    (import_decl private 'ControlFunctions::*')
    (import_decl private 'ScalarValues::Positive')
    (attribute_def 'DistancePerVolumeValue' :> 'ScalarQuantityValue')
    (part_def 'Vehicle'
      (attribute_usage 'mass' : 'MassValue')
      (attribute_usage 'cargoMass' : 'MassValue')
      (attribute_usage 'wheelDiameter' : 'LengthValue')
      (attribute_usage 'driveTrainEfficiency' : 'Real')
      (attribute_usage 'fuelEconomy_city' : 'DistancePerVolumeValue')
      (attribute_usage 'fuelEconomy_highway' : 'DistancePerVolumeValue'))
    (attribute_def 'WayPoint'
      (default_ref_usage 'time' : 'TimeValue')
      (default_ref_usage 'position' : 'LengthValue')
      (default_ref_usage 'speed' : 'SpeedValue'))
    (analysis_case_def 'FuelEconomyAnalysis'
      (sysml_decl 'vehicle' : 'Vehicle')
      (objective_member)
      (attribute_usage in 'scenario' : 'WayPoint' multiplicity)
      (action_usage 'solveForPower'
        (default_ref_usage out 'power' : 'PowerValue' multiplicity)
        (default_ref_usage out 'acceleration' : 'AccelerationValue' multiplicity)
        (comment)
        (sysml_decl
          (result_expr_member)))
      (source_succession
        (action_usage 'solveForFuelConsumption'
          (default_ref_usage in 'power' : 'PowerValue' multiplicity value)
          (default_ref_usage out 'fuelEconomy' : 'DistancePerVolumeValue')
          (comment)))
      (return_member))))
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
# EXPECTED
~~~
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'PowerValue'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'PowerValue'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Analysis Case Definition Example"))) (name "Analysis Case Definition Example") (declared-name "Analysis Case Definition Example")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Analysis Case Definition Example::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Analysis Case Definition Example::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Analysis Case Definition Example::*#import2"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Analysis Case Definition Example::*#import3"))) (name "*") (declared-name "*"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue"))) (name "DistancePerVolumeValue") (declared-name "DistancePerVolumeValue") (declared (properties (ordered false) (unique true))))
        (element (kind "analysis def") (id (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis"))) (name "FuelEconomyAnalysis") (declared-name "FuelEconomyAnalysis")
          (contains
            (element (kind "objective") (id (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::fuelEconomyAnalysisObjective"))) (name "fuelEconomyAnalysisObjective") (declared-name "fuelEconomyAnalysisObjective") (effective (featuring-type (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis")))))
            (element (kind "analysis result") (id (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::fuelEconomyResult"))) (name "fuelEconomyResult") (declared-name "fuelEconomyResult") (effective (featuring-type (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::scenario"))) (name "scenario") (declared-name "scenario") (declared (properties (direction "in") (ordered false) (unique true)) (multiplicity (lower unbounded) (upper unbounded) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis")))))
            (element (kind "action") (id (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForPower"))) (name "solveForPower") (declared-name "solveForPower") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis"))))
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForPower::acceleration"))) (name "acceleration") (declared-name "acceleration") (effective (featuring-type (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::solveForPower::power"))) (name "power") (declared-name "power") (effective (featuring-type (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis")))))
              )
            )
            (element (kind "subject") (id (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::vehicle"))) (name "vehicle") (declared-name "vehicle") (effective (featuring-type (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Analysis Case Definition Example::Positive"))) (name "Positive") (declared-name "Positive"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Analysis Case Definition Example::Real"))) (name "Real") (declared-name "Real"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Analysis Case Definition Example::ScalarQuantityValue"))) (name "ScalarQuantityValue") (declared-name "ScalarQuantityValue"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared)
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::cargoMass"))) (name "cargoMass") (declared-name "cargoMass") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::driveTrainEfficiency"))) (name "driveTrainEfficiency") (declared-name "driveTrainEfficiency") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::fuelEconomy_city"))) (name "fuelEconomy_city") (declared-name "fuelEconomy_city") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::fuelEconomy_highway"))) (name "fuelEconomy_highway") (declared-name "fuelEconomy_highway") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::mass"))) (name "mass") (declared-name "mass") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::wheelDiameter"))) (name "wheelDiameter") (declared-name "wheelDiameter") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "Analysis Case Definition Example::WayPoint"))) (name "WayPoint") (declared-name "WayPoint") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Analysis Case Definition Example::WayPoint::position"))) (name "position") (declared-name "position") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Analysis Case Definition Example::WayPoint")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Analysis Case Definition Example::WayPoint::speed"))) (name "speed") (declared-name "speed") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Analysis Case Definition Example::WayPoint")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Analysis Case Definition Example::WayPoint::time"))) (name "time") (declared-name "time") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Analysis Case Definition Example::WayPoint")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Analysis Case Definition Example::size"))) (name "size") (declared-name "size"))
      )
    )
  )
  (relationships
    (subject (status resolved) (from (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis"))) (to (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::fuelEconomyResult"))) (to (node (document "d0") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::scenario"))) (to (node (document "d0") (qualified-name "Analysis Case Definition Example::WayPoint"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Analysis Case Definition Example::FuelEconomyAnalysis::vehicle"))) (to (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::fuelEconomy_city"))) (to (node (document "d0") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Analysis Case Definition Example::Vehicle::fuelEconomy_highway"))) (to (node (document "d0") (qualified-name "Analysis Case Definition Example::DistancePerVolumeValue"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/33_analysis_case_definition_example.md"
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
        (range (start 10 1) (end 10 61))
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
        (range (start 14 8) (end 14 40))
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
        (range (start 17 8) (end 17 46))
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
        (code "analysis_evaluation_unresolved")
        (source "semantic")
        (range (start 29 1) (end 29 1411))
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
    )
  )
)
~~~
