# META
~~~ini
description=SysML Validation (10-Analysis and Trades): 10d-Dynamics Analysis
type=file
~~~
# SOURCE
~~~sysml
package '10d-Dynamics Analysis' {
	private import ISQ::*;
	
	package VehicleModel {
	
		part def Vehicle {
			attribute mass :> ISQ::mass;
		}
	
	}
	
	package DynamicsModel {
	    
	    calc def Acceleration {
	    	in p : PowerValue;
	    	in m : MassValue;
	    	in v : SpeedValue;
	    	return : AccelerationValue = p / (m * v);
	    }
	    
	    calc def Velocity {
	    	in v0 : SpeedValue; 
	    	in a : AccelerationValue; 
	    	in dt : TimeValue;
	    	return : SpeedValue = v0 + a * dt;
	    }
	    
	    calc def Position {
	    	in x0 : LengthValue;
	    	in v : SpeedValue; 
	    	in dt : TimeValue;
	    	return : LengthValue = x0 + v * dt;
	    }
	    
	    action def StraightLineDynamics {
	        in power : PowerValue;
	        in mass : MassValue;
	        in delta_t : TimeValue;
	        in x_in : LengthValue;
	        in v_in : SpeedValue;
	        out x_out : LengthValue = Position(x_in, v_in, delta_t);
	        out v_out : SpeedValue = Velocity(v_in, a_out, delta_t);
	        out a_out : AccelerationValue = Acceleration(power, mass, v_in);
	    }
	}
	
	package AnalysisModel {
		private import VehicleModel::*;
		private import DynamicsModel::*;
		private import SampledFunctions::*;
		private import ScalarValues::Natural;
		private import SequenceFunctions::*;
		
		analysis def DynamicsAnalysis {
			subject vehicle : Vehicle;
			in attribute powerProfile :> ISQ::power[*];
			in attribute initialPosition :> ISQ::length;
			in attribute initialSpeed :> ISQ::speed;
			in attribute deltaT :> ISQ::time;
			return attribute accelerationProfile :> ISQ::acceleration[*] := ();
			
			private attribute position := initialPosition;
			private attribute speed := initialSpeed;
			
			for i in 1..powerProfile->size()-1 {
				perform action dynamics : StraightLineDynamics {
					in power = powerProfile#(i);
					in mass = vehicle.mass;
					in delta_t = deltaT;
					in x_in = position;
					in v_in = speed;
				}
				then assign position := dynamics.x_out;
				then assign speed := dynamics.v_out;
				then assign accelerationProfile := accelerationProfile->including(dynamics.a_out);
			}
		}
	}
	
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwCalc,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwReturn,Colon,Ident,Eq,Ident,Slash,OpenParen,Ident,Star,Ident,CloseParen,Semicolon,
CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwReturn,Colon,Ident,Eq,Ident,Plus,Ident,Star,Ident,Semicolon,
CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwReturn,Colon,Ident,Eq,Ident,Plus,Ident,Star,Ident,Semicolon,
CloseCurly,
KwAction,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwOut,Ident,Colon,Ident,Eq,Ident,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,
KwOut,Ident,Colon,Ident,Eq,Ident,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,
KwOut,Ident,Colon,Ident,Eq,Ident,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwAnalysis,KwDef,Ident,OpenCurly,
KwSubject,Ident,Colon,Ident,Semicolon,
KwIn,KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,OpenSquare,Star,CloseSquare,Semicolon,
KwIn,KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwIn,KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwIn,KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwReturn,KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,OpenSquare,Star,CloseSquare,ColonEq,OpenParen,CloseParen,Semicolon,
KwPrivate,KwAttribute,Ident,ColonEq,Ident,Semicolon,
KwPrivate,KwAttribute,Ident,ColonEq,Ident,Semicolon,
KwFor,Ident,KwIn,DecimalValue,DotDot,Ident,Arrow,Ident,OpenParen,CloseParen,Minus,DecimalValue,OpenCurly,
KwPerform,KwAction,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Eq,Ident,Hash,OpenParen,Ident,CloseParen,Semicolon,
KwIn,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Semicolon,
CloseCurly,
KwThen,KwAssign,Ident,ColonEq,Ident,Dot,Ident,Semicolon,
KwThen,KwAssign,Ident,ColonEq,Ident,Dot,Ident,Semicolon,
KwThen,KwAssign,Ident,ColonEq,Ident,Arrow,Ident,OpenParen,Ident,Dot,Ident,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''10d-Dynamics Analysis''
    (import_decl private 'ISQ::*')
    (package_def 'VehicleModel'
      (part_def 'Vehicle'
        (attribute_usage 'mass' :> 'ISQ::mass')))
    (package_def 'DynamicsModel'
      (calc_def 'Acceleration'
        (default_ref_usage in 'p' : 'PowerValue')
        (default_ref_usage in 'm' : 'MassValue')
        (default_ref_usage in 'v' : 'SpeedValue')
        (return_member))
      (calc_def 'Velocity'
        (default_ref_usage in 'v0' : 'SpeedValue')
        (default_ref_usage in 'a' : 'AccelerationValue')
        (default_ref_usage in 'dt' : 'TimeValue')
        (return_member))
      (calc_def 'Position'
        (default_ref_usage in 'x0' : 'LengthValue')
        (default_ref_usage in 'v' : 'SpeedValue')
        (default_ref_usage in 'dt' : 'TimeValue')
        (return_member))
      (action_def 'StraightLineDynamics'
        (default_ref_usage in 'power' : 'PowerValue')
        (default_ref_usage in 'mass' : 'MassValue')
        (default_ref_usage in 'delta_t' : 'TimeValue')
        (default_ref_usage in 'x_in' : 'LengthValue')
        (default_ref_usage in 'v_in' : 'SpeedValue')
        (default_ref_usage out 'x_out' : 'LengthValue' value)
        (default_ref_usage out 'v_out' : 'SpeedValue' value)
        (default_ref_usage out 'a_out' : 'AccelerationValue' value)))
    (package_def 'AnalysisModel'
      (import_decl private 'VehicleModel::*')
      (import_decl private 'DynamicsModel::*')
      (import_decl private 'SampledFunctions::*')
      (import_decl private 'ScalarValues::Natural')
      (import_decl private 'SequenceFunctions::*')
      (analysis_case_def 'DynamicsAnalysis'
        (sysml_decl 'vehicle' : 'Vehicle')
        (attribute_usage in 'powerProfile' :> 'ISQ::power' multiplicity)
        (attribute_usage in 'initialPosition' :> 'ISQ::length')
        (attribute_usage in 'initialSpeed' :> 'ISQ::speed')
        (attribute_usage in 'deltaT' :> 'ISQ::time')
        (return_member)
        (attribute_usage private 'position' value)
        (attribute_usage private 'speed' value)
        (for_loop_node)))))
~~~
# FORMAT
~~~sysml
package '10d-Dynamics Analysis' {
    private import ISQ::*;

    package VehicleModel {
        part def Vehicle {
            attribute mass :> ISQ::mass;
        }
    }

    package DynamicsModel {
        calc def Acceleration {
            in p : PowerValue;
            in m : MassValue;
            in v : SpeedValue;
            return : AccelerationValue = p / (m * v);
        }

        calc def Velocity {
            in v0 : SpeedValue;
            in a : AccelerationValue;
            in dt : TimeValue;
            return : SpeedValue = v0 + a * dt;
        }

        calc def Position {
            in x0 : LengthValue;
            in v : SpeedValue;
            in dt : TimeValue;
            return : LengthValue = x0 + v * dt;
        }

        action def StraightLineDynamics {
            in power : PowerValue;
            in mass : MassValue;
            in delta_t : TimeValue;
            in x_in : LengthValue;
            in v_in : SpeedValue;
            out x_out : LengthValue = Position(x_in, v_in, delta_t);
            out v_out : SpeedValue = Velocity(v_in, a_out, delta_t);
            out a_out : AccelerationValue = Acceleration(power, mass, v_in);
        }
    }

    package AnalysisModel {
        private import VehicleModel::*;
        private import DynamicsModel::*;
        private import SampledFunctions::*;
        private import ScalarValues::Natural;
        private import SequenceFunctions::*;

        analysis def DynamicsAnalysis {
            subject vehicle : Vehicle;
            in attribute powerProfile :> ISQ::power [*];
            in attribute initialPosition :> ISQ::length;
            in attribute initialSpeed :> ISQ::speed;
            in attribute deltaT :> ISQ::time;
            return attribute accelerationProfile :> ISQ::acceleration[*] := ();

            private attribute position := initialPosition;
            private attribute speed := initialSpeed;

            for i in 1..powerProfile->size()-1 {
                perform action dynamics : StraightLineDynamics {
                    in power = powerProfile#(i);
                    in mass = vehicle.mass;
                    in delta_t = deltaT;
                    in x_in = position;
                    in v_in = speed;
                }
                then assign position := dynamics.x_out;
                then assign speed := dynamics.v_out;
                then assign accelerationProfile := accelerationProfile->including(dynamics.a_out);
            }
        }
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'ISQ::power'
semantic.unresolved_name 'ISQ::length'
semantic.unresolved_name 'ISQ::speed'
semantic.unresolved_name 'ISQ::time'
semantic.unresolved_name 'ISQ::acceleration'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'ISQ::power'
semantic.unresolved_name 'ISQ::length'
semantic.unresolved_name 'ISQ::speed'
semantic.unresolved_name 'ISQ::time'
semantic.unresolved_name 'ISQ::acceleration'
~~~
# SMG
~~~
(model
  (namespace
    (package '10d-Dynamics Analysis'
      (namespace_import private -> 'ISQ'[unresolved])
      (package 'VehicleModel'
        (part_def 'Vehicle'
          (attribute_usage composite 'mass' :> 'ISQ::mass'[unresolved])))
      (package 'DynamicsModel'
        (calculation_def 'Acceleration'
          (reference_usage in reference 'p' : 'PowerValue'[unresolved])
          (reference_usage in reference 'm' : 'MassValue'[unresolved])
          (reference_usage in reference 'v' : 'SpeedValue'[unresolved])
          (return_parameter_membership
            (feature_def out : 'AccelerationValue'[unresolved]
              (feature_value (=)))))
        (calculation_def 'Velocity'
          (reference_usage in reference 'v0' : 'SpeedValue'[unresolved])
          (reference_usage in reference 'a' : 'AccelerationValue'[unresolved])
          (reference_usage in reference 'dt' : 'TimeValue'[unresolved])
          (return_parameter_membership
            (feature_def out : 'SpeedValue'[unresolved]
              (feature_value (=)))))
        (calculation_def 'Position'
          (reference_usage in reference 'x0' : 'LengthValue'[unresolved])
          (reference_usage in reference 'v' : 'SpeedValue'[unresolved])
          (reference_usage in reference 'dt' : 'TimeValue'[unresolved])
          (return_parameter_membership
            (feature_def out : 'LengthValue'[unresolved]
              (feature_value (=)))))
        (action_def 'StraightLineDynamics'
          (reference_usage in reference 'power' : 'PowerValue'[unresolved])
          (reference_usage in reference 'mass' : 'MassValue'[unresolved])
          (reference_usage in reference 'delta_t' : 'TimeValue'[unresolved])
          (reference_usage in reference 'x_in' : 'LengthValue'[unresolved])
          (reference_usage in reference 'v_in' : 'SpeedValue'[unresolved])
          (reference_usage out reference 'x_out' : 'LengthValue'[unresolved]
            (feature_value (=)))
          (reference_usage out reference 'v_out' : 'SpeedValue'[unresolved]
            (feature_value (=)))
          (reference_usage out reference 'a_out' : 'AccelerationValue'[unresolved]
            (feature_value (=)))))
      (package 'AnalysisModel'
        (namespace_import private -> '10d-Dynamics Analysis::VehicleModel'[package])
        (namespace_import private -> '10d-Dynamics Analysis::DynamicsModel'[package])
        (namespace_import private -> 'SampledFunctions'[unresolved])
        (membership_import private -> 'ScalarValues::Natural'[unresolved])
        (namespace_import private -> 'SequenceFunctions'[unresolved])
        (analysis_case_def 'DynamicsAnalysis'
          (subject_membership in 'vehicle' : '10d-Dynamics Analysis::VehicleModel::Vehicle'[part_def])
          (attribute_usage in 'powerProfile' :> 'ISQ::power'[unresolved]
            (multiplicity_range [*]))
          (attribute_usage in 'initialPosition' :> 'ISQ::length'[unresolved])
          (attribute_usage in 'initialSpeed' :> 'ISQ::speed'[unresolved])
          (attribute_usage in 'deltaT' :> 'ISQ::time'[unresolved])
          (return_parameter_membership
            (attribute_usage out 'accelerationProfile' :> 'ISQ::acceleration'[unresolved]
              (multiplicity_range [*])
              (feature_value (:=))))
          (attribute_usage composite 'position'
            (feature_value (:=)))
          (attribute_usage composite 'speed'
            (feature_value (:=)))
          (for_loop_action_usage
            (perform_action_usage 'dynamics' : '10d-Dynamics Analysis::DynamicsModel::StraightLineDynamics'[action_def]
              (reference_usage in reference 'power'
                (feature_value (=)))
              (reference_usage in reference 'mass'
                (feature_value (=)))
              (reference_usage in reference 'delta_t'
                (feature_value (=)))
              (reference_usage in reference 'x_in'
                (feature_value (=)))
              (reference_usage in reference 'v_in'
                (feature_value (=))))
            (source_succession
              (assignment_action_usage))
            (source_succession
              (assignment_action_usage))
            (source_succession
              (assignment_action_usage))))))))
~~~
