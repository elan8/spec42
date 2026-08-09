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
                = vehicle.wheelDiameter == 33['in'] & vehicle.driveTrainEfficiency == 0.4;
            }

            require constraint {
                = fuelEconomyResult > 30[mi / gal];
            }
        }

        in attribute scenario : WayPoint [*];

        action solveForPower {
            out power : PowerValue [*];
            out acceleration : AccelerationValue [*];

            /*
			 * Solve for the required engine power as a function of time
			 * to support the scenario.
			 */
            assert constraint {
                = (1 .. size(scenario) - 1)->forAll {in i: Positive;
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
				};
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
(model
  (namespace
    (package 'Analysis Case Definition Example'
      (membership_import private -> 'ScalarValues::Real'[unresolved])
      (namespace_import private -> 'Calculation Definitions'[unresolved])
      (namespace_import private -> 'Analytical Constraints'[unresolved])
      (namespace_import private -> 'USCustomaryUnits'[unresolved])
      (membership_import private -> 'SequenceFunctions::size'[unresolved])
      (membership_import private -> 'Quantities::ScalarQuantityValue'[unresolved])
      (namespace_import private -> 'ControlFunctions'[unresolved])
      (membership_import private -> 'ScalarValues::Positive'[unresolved])
      (attribute_def 'DistancePerVolumeValue' :> 'ScalarQuantityValue'[unresolved])
      (part_def 'Vehicle'
        (attribute_usage composite 'mass' : 'MassValue'[unresolved])
        (attribute_usage composite 'cargoMass' : 'MassValue'[unresolved])
        (attribute_usage composite 'wheelDiameter' : 'LengthValue'[unresolved])
        (attribute_usage composite 'driveTrainEfficiency' : 'Real'[unresolved])
        (attribute_usage composite 'fuelEconomy_city' : 'Analysis Case Definition Example::DistancePerVolumeValue'[attribute_def])
        (attribute_usage composite 'fuelEconomy_highway' : 'Analysis Case Definition Example::DistancePerVolumeValue'[attribute_def]))
      (attribute_def 'WayPoint'
        (reference_usage reference 'time' : 'TimeValue'[unresolved])
        (reference_usage reference 'position' : 'LengthValue'[unresolved])
        (reference_usage reference 'speed' : 'SpeedValue'[unresolved]))
      (analysis_case_def 'FuelEconomyAnalysis'
        (subject_membership in 'vehicle' : 'Analysis Case Definition Example::Vehicle'[part_def])
        (objective_membership composite 'fuelEconomyAnalysisObjective'
          (assume_constraint_usage composite
            (result_expr_membership))
          (require_constraint_usage composite
            (result_expr_membership)))
        (attribute_usage in 'scenario' : 'Analysis Case Definition Example::WayPoint'[attribute_def]
          (multiplicity_range [*]))
        (action_usage composite 'solveForPower'
          (reference_usage out reference 'power' : 'PowerValue'[unresolved]
            (multiplicity_range [*]))
          (reference_usage out reference 'acceleration' : 'AccelerationValue'[unresolved]
            (multiplicity_range [*]))
          (assert_constraint_usage
            (result_expr_membership)))
        (source_succession
          (action_usage 'solveForFuelConsumption'
            (reference_usage in reference 'power' : 'PowerValue'[unresolved]
              (multiplicity_range [*])
              (feature_value (=)))
            (reference_usage out reference 'fuelEconomy' : 'Analysis Case Definition Example::DistancePerVolumeValue'[attribute_def])))
        (return_parameter_membership
          (feature_def out 'fuelEconomyResult' : 'Analysis Case Definition Example::DistancePerVolumeValue'[attribute_def]
            (feature_value (=))))))))
~~~
