# META
~~~ini
description=SysML Training 20 (Assignment Actions): Assignment Example
type=file
~~~
# SOURCE
~~~sysml
package 'For Loop Example' {
	private import SequenceFunctions::*;
	
    action def StraightLineDynamics {
        in power : ISQ::PowerValue;
        in mass : ISQ::MassValue;
        in delta_t : ISQ::TimeValue;
        in x_in : ISQ::LengthValue;
        in v_in : ISQ::SpeedValue;
        out x_out : ISQ::LengthValue;
        out v_out : ISQ::SpeedValue;
    }
	    
	action def ComputeMotion {
		in attribute powerProfile :> ISQ::power[*];
		in attribute vehicleMass :> ISQ::mass;
		in attribute initialPosition :> ISQ::length;
		in attribute initialSpeed :> ISQ::speed;
		in attribute deltaT :> ISQ::time;
		out attribute positions :> ISQ::length[*] := ( );
		
		private attribute position := initialPosition;
		private attribute speed := initialSpeed;
		
		for vehiclePower in powerProfile {
			perform action dynamics : StraightLineDynamics {
				in power = vehiclePower;
				in mass = vehicleMass;
				in delta_t = deltaT;
				in x_in = position;
				in v_in = speed;
				out x_out;
				out v_out;
			}
			then assign position := dynamics.x_out;
			then assign speed := dynamics.v_out;
			then assign positions := positions->including(position);
		}
	}
	
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwAction,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
KwOut,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
KwOut,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwAction,KwDef,Ident,OpenCurly,
KwIn,KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,OpenSquare,Star,CloseSquare,Semicolon,
KwIn,KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwIn,KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwIn,KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwIn,KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwOut,KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,OpenSquare,Star,CloseSquare,ColonEq,OpenParen,CloseParen,Semicolon,
KwPrivate,KwAttribute,Ident,ColonEq,Ident,Semicolon,
KwPrivate,KwAttribute,Ident,ColonEq,Ident,Semicolon,
KwFor,Ident,KwIn,Ident,OpenCurly,
KwPerform,KwAction,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Semicolon,
KwOut,Ident,Semicolon,
KwOut,Ident,Semicolon,
CloseCurly,
KwThen,KwAssign,Ident,ColonEq,Ident,Dot,Ident,Semicolon,
KwThen,KwAssign,Ident,ColonEq,Ident,Dot,Ident,Semicolon,
KwThen,KwAssign,Ident,ColonEq,Ident,Arrow,Ident,OpenParen,Ident,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''For Loop Example''
    (import_decl private 'SequenceFunctions::*')
    (action_def 'StraightLineDynamics'
      (default_ref_usage in 'power' : 'ISQ::PowerValue')
      (default_ref_usage in 'mass' : 'ISQ::MassValue')
      (default_ref_usage in 'delta_t' : 'ISQ::TimeValue')
      (default_ref_usage in 'x_in' : 'ISQ::LengthValue')
      (default_ref_usage in 'v_in' : 'ISQ::SpeedValue')
      (default_ref_usage out 'x_out' : 'ISQ::LengthValue')
      (default_ref_usage out 'v_out' : 'ISQ::SpeedValue'))
    (action_def 'ComputeMotion'
      (attribute_usage in 'powerProfile' :> 'ISQ::power' multiplicity)
      (attribute_usage in 'vehicleMass' :> 'ISQ::mass')
      (attribute_usage in 'initialPosition' :> 'ISQ::length')
      (attribute_usage in 'initialSpeed' :> 'ISQ::speed')
      (attribute_usage in 'deltaT' :> 'ISQ::time')
      (attribute_usage out 'positions' :> 'ISQ::length' multiplicity value)
      (attribute_usage private 'position' value)
      (attribute_usage private 'speed' value)
      (for_loop_node))))
~~~
# FORMAT
~~~sysml
package 'For Loop Example' {
    private import SequenceFunctions::*;

    action def StraightLineDynamics {
        in power : ISQ::PowerValue;
        in mass : ISQ::MassValue;
        in delta_t : ISQ::TimeValue;
        in x_in : ISQ::LengthValue;
        in v_in : ISQ::SpeedValue;
        out x_out : ISQ::LengthValue;
        out v_out : ISQ::SpeedValue;
    }

    action def ComputeMotion {
        in attribute powerProfile :> ISQ::power [*];
        in attribute vehicleMass :> ISQ::mass;
        in attribute initialPosition :> ISQ::length;
        in attribute initialSpeed :> ISQ::speed;
        in attribute deltaT :> ISQ::time;
        out attribute positions :> ISQ::length [*] := ( );

        private attribute position := initialPosition;
        private attribute speed := initialSpeed;

        for vehiclePower in powerProfile {
            perform action dynamics : StraightLineDynamics {
                in power = vehiclePower;
                in mass = vehicleMass;
                in delta_t = deltaT;
                in x_in = position;
                in v_in = speed;
                out x_out;
                out v_out;
            }
            then assign position := dynamics.x_out;
            then assign speed := dynamics.v_out;
            then assign positions := positions->including(position);
        }
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'ISQ::PowerValue'
semantic.unresolved_name 'ISQ::MassValue'
semantic.unresolved_name 'ISQ::TimeValue'
semantic.unresolved_name 'ISQ::LengthValue'
semantic.unresolved_name 'ISQ::SpeedValue'
semantic.unresolved_name 'ISQ::LengthValue'
semantic.unresolved_name 'ISQ::SpeedValue'
semantic.unresolved_name 'ISQ::power'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::length'
semantic.unresolved_name 'ISQ::speed'
semantic.unresolved_name 'ISQ::time'
semantic.unresolved_name 'ISQ::length'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'ISQ::PowerValue'
semantic.unresolved_name 'ISQ::MassValue'
semantic.unresolved_name 'ISQ::TimeValue'
semantic.unresolved_name 'ISQ::LengthValue'
semantic.unresolved_name 'ISQ::SpeedValue'
semantic.unresolved_name 'ISQ::LengthValue'
semantic.unresolved_name 'ISQ::SpeedValue'
semantic.unresolved_name 'ISQ::power'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::length'
semantic.unresolved_name 'ISQ::speed'
semantic.unresolved_name 'ISQ::time'
semantic.unresolved_name 'ISQ::length'
~~~
# SMG
~~~
(model
  (namespace
    (package 'For Loop Example'
      (namespace_import private -> 'SequenceFunctions'[unresolved])
      (action_def 'StraightLineDynamics'
        (reference_usage in reference 'power' : 'ISQ::PowerValue'[unresolved])
        (reference_usage in reference 'mass' : 'ISQ::MassValue'[unresolved])
        (reference_usage in reference 'delta_t' : 'ISQ::TimeValue'[unresolved])
        (reference_usage in reference 'x_in' : 'ISQ::LengthValue'[unresolved])
        (reference_usage in reference 'v_in' : 'ISQ::SpeedValue'[unresolved])
        (reference_usage out reference 'x_out' : 'ISQ::LengthValue'[unresolved])
        (reference_usage out reference 'v_out' : 'ISQ::SpeedValue'[unresolved]))
      (action_def 'ComputeMotion'
        (attribute_usage in 'powerProfile' :> 'ISQ::power'[unresolved]
          (multiplicity_range [*]))
        (attribute_usage in 'vehicleMass' :> 'ISQ::mass'[unresolved])
        (attribute_usage in 'initialPosition' :> 'ISQ::length'[unresolved])
        (attribute_usage in 'initialSpeed' :> 'ISQ::speed'[unresolved])
        (attribute_usage in 'deltaT' :> 'ISQ::time'[unresolved])
        (attribute_usage out 'positions' :> 'ISQ::length'[unresolved]
          (multiplicity_range [*])
          (feature_value (:=)))
        (attribute_usage composite 'position'
          (feature_value (:=)))
        (attribute_usage composite 'speed'
          (feature_value (:=)))
        (for_loop_action_usage
          (perform_action_usage 'dynamics' : 'For Loop Example::StraightLineDynamics'[action_def]
            (reference_usage in reference 'power'
              (feature_value (=)))
            (reference_usage in reference 'mass'
              (feature_value (=)))
            (reference_usage in reference 'delta_t'
              (feature_value (=)))
            (reference_usage in reference 'x_in'
              (feature_value (=)))
            (reference_usage in reference 'v_in'
              (feature_value (=)))
            (reference_usage out reference 'x_out')
            (reference_usage out reference 'v_out'))
          (source_succession
            (assignment_action_usage))
          (source_succession
            (assignment_action_usage))
          (source_succession
            (assignment_action_usage)))))))
~~~
