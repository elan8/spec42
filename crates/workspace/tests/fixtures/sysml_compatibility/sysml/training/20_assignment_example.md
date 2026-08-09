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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "For Loop Example"))) (name "For Loop Example") (declared-name "For Loop Example")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "For Loop Example::*"))) (name "*") (declared-name "*"))
        (element (kind "action def") (id (node (document "d0") (qualified-name "For Loop Example::ComputeMotion"))) (name "ComputeMotion") (declared-name "ComputeMotion")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::deltaT"))) (name "deltaT") (declared-name "deltaT") (effective (featuring-type (node (document "d0") (qualified-name "For Loop Example::ComputeMotion")))))
            (element (kind "for loop") (id (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::for_vehiclePower"))) (name "vehiclePower") (declared-name "vehiclePower") (effective (featuring-type (node (document "d0") (qualified-name "For Loop Example::ComputeMotion"))))
              (contains
                (element (kind "assign") (id (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::for_vehiclePower::_assign"))) (name "assign") (declared-name "assign") (effective (featuring-type (node (document "d0") (qualified-name "For Loop Example::ComputeMotion")))))
                (element (kind "assign") (id (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::for_vehiclePower::_assign#assign"))) (name "assign") (declared-name "assign") (effective (featuring-type (node (document "d0") (qualified-name "For Loop Example::ComputeMotion")))))
                (element (kind "assign") (id (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::for_vehiclePower::_assign#assign2"))) (name "assign") (declared-name "assign") (effective (featuring-type (node (document "d0") (qualified-name "For Loop Example::ComputeMotion")))))
                (element (kind "perform") (id (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::for_vehiclePower::dynamics"))) (name "dynamics") (declared-name "dynamics") (effective (featuring-type (node (document "d0") (qualified-name "For Loop Example::ComputeMotion")))))
              )
            )
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::initialPosition"))) (name "initialPosition") (declared-name "initialPosition") (effective (featuring-type (node (document "d0") (qualified-name "For Loop Example::ComputeMotion")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::initialSpeed"))) (name "initialSpeed") (declared-name "initialSpeed") (effective (featuring-type (node (document "d0") (qualified-name "For Loop Example::ComputeMotion")))))
            (element (kind "action body decl") (id (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::position := initialPosition"))) (name "position := initialPosition") (declared-name "position := initialPosition") (effective (featuring-type (node (document "d0") (qualified-name "For Loop Example::ComputeMotion")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::positions"))) (name "positions") (declared-name "positions") (effective (featuring-type (node (document "d0") (qualified-name "For Loop Example::ComputeMotion")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::powerProfile"))) (name "powerProfile") (declared-name "powerProfile") (effective (featuring-type (node (document "d0") (qualified-name "For Loop Example::ComputeMotion")))))
            (element (kind "action body decl") (id (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::speed := initialSpeed"))) (name "speed := initialSpeed") (declared-name "speed := initialSpeed") (effective (featuring-type (node (document "d0") (qualified-name "For Loop Example::ComputeMotion")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::vehicleMass"))) (name "vehicleMass") (declared-name "vehicleMass") (effective (featuring-type (node (document "d0") (qualified-name "For Loop Example::ComputeMotion")))))
          )
        )
        (element (kind "action def") (id (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics"))) (name "StraightLineDynamics") (declared-name "StraightLineDynamics")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics::delta_t"))) (name "delta_t") (declared-name "delta_t") (effective (featuring-type (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics::mass"))) (name "mass") (declared-name "mass") (effective (featuring-type (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics::power"))) (name "power") (declared-name "power") (effective (featuring-type (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics::v_in"))) (name "v_in") (declared-name "v_in") (effective (featuring-type (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics::v_out"))) (name "v_out") (declared-name "v_out") (effective (featuring-type (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics::x_in"))) (name "x_in") (declared-name "x_in") (effective (featuring-type (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics::x_out"))) (name "x_out") (declared-name "x_out") (effective (featuring-type (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics")))))
          )
        )
      )
    )
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "For Loop Example::ComputeMotion::for_vehiclePower::dynamics"))) (to (node (document "d0") (qualified-name "For Loop Example::StraightLineDynamics"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
