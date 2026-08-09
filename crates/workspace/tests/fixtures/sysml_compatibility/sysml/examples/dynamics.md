# META
~~~ini
description=SysML Example (Analysis): Dynamics
type=file
~~~
# SOURCE
~~~sysml
package Dynamics {
	private import ScalarValues::Real;
	private import ISQ::*;
	
	// Function definitions
	
	calc def Power {
		in whlpwr : PowerValue; in Cd : Real; in Cf : Real; in tm : MassValue; in v : SpeedValue;
		return tp : PowerValue = whlpwr - Cd * v - Cf * tm * v;
	}
	
	calc def Acceleration { in dt : TimeValue; in tm : MassValue; in tp: PowerValue; 
		return a : AccelerationValue = tp * dt * tp;
	}
	
	calc def Velocity { in dt : TimeValue; in v0 : SpeedValue; in a : AccelerationValue;
		return v : SpeedValue = v0 + a * dt;
	}
 	
	calc def Position { in dt : TimeValue; in x0 : LengthValue; in v : SpeedValue; 
		return x : LengthValue = x0 + v * dt;
	}

	// Analysis action def
	
	action def StraightLineVehicleDynamics {
		
		in attribute dt : TimeValue;
		in attribute whlpwr : PowerValue;
		in attribute Cd : Real;
		in attribute Cf: Real;
		in attribute tm : MassValue;
		in attribute v_in : SpeedValue;
		in attribute x_in : LengthValue;
		
		out attribute a_out : AccelerationValue;
		out attribute v_out : SpeedValue;
		out attribute x_out : LengthValue;
			
		assert constraint {
			attribute tp : PowerValue;
			
			tp == Power(whlpwr, Cd, Cf, tm, v_in) &
			a_out == Acceleration(dt, tm, tp) &
			v_out == Velocity(dt, v_in, a_out) &
			x_out == Position(dt, x_in, v_in)
		}
	}
	

	// Analysis actions
	
	action dyn1 : StraightLineVehicleDynamics {
        in attribute dt : TimeValue;
        in attribute whlpwr : PowerValue;
        in attribute Cd : Real;
        in attribute Cf: Real;
        in attribute tm : MassValue;
        in attribute v_in : SpeedValue;
        in attribute x_in : LengthValue;

		attribute tp : PowerValue = Power(whlpwr, Cd, Cf, tm, v_in);
		
		out attribute :>> a_out : AccelerationValue = Acceleration(dt, tm, tp);
		out attribute :>> v_out : SpeedValue = Velocity(dt, v_in, a_out);
		out attribute :>> x_out : LengthValue = Position(dt, x_in, v_in);
	}	
	
	action dyn2 : StraightLineVehicleDynamics {
		calc acc : Acceleration {
			in dt = dyn2::dt;
			in tm = dyn2::tm;
			in tp = Power(whlpwr, Cd, Cf, tm, v_in);
		}
		bind a_out = acc.a;
		
		calc vel : Velocity {
			in dt = dyn2::dt;
			in v0 = dyn2::v_in; 
			in a = acc.a;
		}
		bind v_out = vel.v;
		
		calc pos : Position {
			in dt = dyn2::dt;
			in x0 = dyn2::x_in;
			in v0 = vel.v;
		}
		bind x_out = pos.x;
	}
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
LineComment,
KwCalc,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,KwIn,Ident,Colon,Ident,Semicolon,KwIn,Ident,Colon,Ident,Semicolon,KwIn,Ident,Colon,Ident,Semicolon,KwIn,Ident,Colon,Ident,Semicolon,
KwReturn,Ident,Colon,Ident,Eq,Ident,Minus,Ident,Star,Ident,Minus,Ident,Star,Ident,Star,Ident,Semicolon,
CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,KwIn,Ident,Colon,Ident,Semicolon,KwIn,Ident,Colon,Ident,Semicolon,
KwReturn,Ident,Colon,Ident,Eq,Ident,Star,Ident,Star,Ident,Semicolon,
CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,KwIn,Ident,Colon,Ident,Semicolon,KwIn,Ident,Colon,Ident,Semicolon,
KwReturn,Ident,Colon,Ident,Eq,Ident,Plus,Ident,Star,Ident,Semicolon,
CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,KwIn,Ident,Colon,Ident,Semicolon,KwIn,Ident,Colon,Ident,Semicolon,
KwReturn,Ident,Colon,Ident,Eq,Ident,Plus,Ident,Star,Ident,Semicolon,
CloseCurly,
LineComment,
KwAction,KwDef,Ident,OpenCurly,
KwIn,KwAttribute,Ident,Colon,Ident,Semicolon,
KwIn,KwAttribute,Ident,Colon,Ident,Semicolon,
KwIn,KwAttribute,Ident,Colon,Ident,Semicolon,
KwIn,KwAttribute,Ident,Colon,Ident,Semicolon,
KwIn,KwAttribute,Ident,Colon,Ident,Semicolon,
KwIn,KwAttribute,Ident,Colon,Ident,Semicolon,
KwIn,KwAttribute,Ident,Colon,Ident,Semicolon,
KwOut,KwAttribute,Ident,Colon,Ident,Semicolon,
KwOut,KwAttribute,Ident,Colon,Ident,Semicolon,
KwOut,KwAttribute,Ident,Colon,Ident,Semicolon,
KwAssert,KwConstraint,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
Ident,EqEq,Ident,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Ampersand,
Ident,EqEq,Ident,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Ampersand,
Ident,EqEq,Ident,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Ampersand,
Ident,EqEq,Ident,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,
CloseCurly,
CloseCurly,
LineComment,
KwAction,Ident,Colon,Ident,OpenCurly,
KwIn,KwAttribute,Ident,Colon,Ident,Semicolon,
KwIn,KwAttribute,Ident,Colon,Ident,Semicolon,
KwIn,KwAttribute,Ident,Colon,Ident,Semicolon,
KwIn,KwAttribute,Ident,Colon,Ident,Semicolon,
KwIn,KwAttribute,Ident,Colon,Ident,Semicolon,
KwIn,KwAttribute,Ident,Colon,Ident,Semicolon,
KwIn,KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Eq,Ident,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,
KwOut,KwAttribute,ColonGtGt,Ident,Colon,Ident,Eq,Ident,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,
KwOut,KwAttribute,ColonGtGt,Ident,Colon,Ident,Eq,Ident,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,
KwOut,KwAttribute,ColonGtGt,Ident,Colon,Ident,Eq,Ident,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,
CloseCurly,
KwAction,Ident,Colon,Ident,OpenCurly,
KwCalc,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwIn,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwIn,Ident,Eq,Ident,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,
CloseCurly,
KwBind,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwCalc,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwIn,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwBind,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwCalc,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwIn,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwBind,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'Dynamics'
    (import_decl private 'ScalarValues::Real')
    (import_decl private 'ISQ::*')
    (line_comment)
    (calc_def 'Power'
      (default_ref_usage in 'whlpwr' : 'PowerValue')
      (default_ref_usage in 'Cd' : 'Real')
      (default_ref_usage in 'Cf' : 'Real')
      (default_ref_usage in 'tm' : 'MassValue')
      (default_ref_usage in 'v' : 'SpeedValue')
      (return_member))
    (calc_def 'Acceleration'
      (default_ref_usage in 'dt' : 'TimeValue')
      (default_ref_usage in 'tm' : 'MassValue')
      (default_ref_usage in 'tp' : 'PowerValue')
      (return_member))
    (calc_def 'Velocity'
      (default_ref_usage in 'dt' : 'TimeValue')
      (default_ref_usage in 'v0' : 'SpeedValue')
      (default_ref_usage in 'a' : 'AccelerationValue')
      (return_member))
    (calc_def 'Position'
      (default_ref_usage in 'dt' : 'TimeValue')
      (default_ref_usage in 'x0' : 'LengthValue')
      (default_ref_usage in 'v' : 'SpeedValue')
      (return_member))
    (line_comment)
    (action_def 'StraightLineVehicleDynamics'
      (attribute_usage in 'dt' : 'TimeValue')
      (attribute_usage in 'whlpwr' : 'PowerValue')
      (attribute_usage in 'Cd' : 'Real')
      (attribute_usage in 'Cf' : 'Real')
      (attribute_usage in 'tm' : 'MassValue')
      (attribute_usage in 'v_in' : 'SpeedValue')
      (attribute_usage in 'x_in' : 'LengthValue')
      (attribute_usage out 'a_out' : 'AccelerationValue')
      (attribute_usage out 'v_out' : 'SpeedValue')
      (attribute_usage out 'x_out' : 'LengthValue')
      (sysml_decl
        (attribute_usage 'tp' : 'PowerValue')
        (result_expr_member)))
    (line_comment)
    (action_usage 'dyn1' : 'StraightLineVehicleDynamics'
      (attribute_usage in 'dt' : 'TimeValue')
      (attribute_usage in 'whlpwr' : 'PowerValue')
      (attribute_usage in 'Cd' : 'Real')
      (attribute_usage in 'Cf' : 'Real')
      (attribute_usage in 'tm' : 'MassValue')
      (attribute_usage in 'v_in' : 'SpeedValue')
      (attribute_usage in 'x_in' : 'LengthValue')
      (attribute_usage 'tp' : 'PowerValue' value)
      (attribute_usage out :>> 'a_out' : 'AccelerationValue' value)
      (attribute_usage out :>> 'v_out' : 'SpeedValue' value)
      (attribute_usage out :>> 'x_out' : 'LengthValue' value))
    (action_usage 'dyn2' : 'StraightLineVehicleDynamics'
      (calc_usage 'acc' : 'Acceleration'
        (default_ref_usage in 'dt' value)
        (default_ref_usage in 'tm' value)
        (default_ref_usage in 'tp' value))
      (binding_as_usage
        (connector_end)
        (connector_end))
      (calc_usage 'vel' : 'Velocity'
        (default_ref_usage in 'dt' value)
        (default_ref_usage in 'v0' value)
        (default_ref_usage in 'a' value))
      (binding_as_usage
        (connector_end)
        (connector_end))
      (calc_usage 'pos' : 'Position'
        (default_ref_usage in 'dt' value)
        (default_ref_usage in 'x0' value)
        (default_ref_usage in 'v0' value))
      (binding_as_usage
        (connector_end)
        (connector_end)))))
~~~
# FORMAT
~~~sysml
package Dynamics {
	private import ScalarValues::Real;
	private import ISQ::*;
	
	// Function definitions
	
	calc def Power {
		in whlpwr : PowerValue; in Cd : Real; in Cf : Real; in tm : MassValue; in v : SpeedValue;
		return tp : PowerValue = whlpwr - Cd * v - Cf * tm * v;
	}
	
	calc def Acceleration { in dt : TimeValue; in tm : MassValue; in tp: PowerValue; 
		return a : AccelerationValue = tp * dt * tp;
	}
	
	calc def Velocity { in dt : TimeValue; in v0 : SpeedValue; in a : AccelerationValue;
		return v : SpeedValue = v0 + a * dt;
	}
 	
	calc def Position { in dt : TimeValue; in x0 : LengthValue; in v : SpeedValue; 
		return x : LengthValue = x0 + v * dt;
	}

	// Analysis action def
	
	action def StraightLineVehicleDynamics {
		
		in attribute dt : TimeValue;
		in attribute whlpwr : PowerValue;
		in attribute Cd : Real;
		in attribute Cf: Real;
		in attribute tm : MassValue;
		in attribute v_in : SpeedValue;
		in attribute x_in : LengthValue;
		
		out attribute a_out : AccelerationValue;
		out attribute v_out : SpeedValue;
		out attribute x_out : LengthValue;
			
		assert constraint {
			attribute tp : PowerValue;
			
			tp == Power(whlpwr, Cd, Cf, tm, v_in) &
			a_out == Acceleration(dt, tm, tp) &
			v_out == Velocity(dt, v_in, a_out) &
			x_out == Position(dt, x_in, v_in)
		}
	}
	

	// Analysis actions
	
	action dyn1 : StraightLineVehicleDynamics {
        in attribute dt : TimeValue;
        in attribute whlpwr : PowerValue;
        in attribute Cd : Real;
        in attribute Cf: Real;
        in attribute tm : MassValue;
        in attribute v_in : SpeedValue;
        in attribute x_in : LengthValue;

		attribute tp : PowerValue = Power(whlpwr, Cd, Cf, tm, v_in);
		
		out attribute :>> a_out : AccelerationValue = Acceleration(dt, tm, tp);
		out attribute :>> v_out : SpeedValue = Velocity(dt, v_in, a_out);
		out attribute :>> x_out : LengthValue = Position(dt, x_in, v_in);
	}	
	
	action dyn2 : StraightLineVehicleDynamics {
		calc acc : Acceleration {
			in dt = dyn2::dt;
			in tm = dyn2::tm;
			in tp = Power(whlpwr, Cd, Cf, tm, v_in);
		}
		bind a_out = acc.a;
		
		calc vel : Velocity {
			in dt = dyn2::dt;
			in v0 = dyn2::v_in; 
			in a = acc.a;
		}
		bind v_out = vel.v;
		
		calc pos : Position {
			in dt = dyn2::dt;
			in x0 = dyn2::x_in;
			in v0 = vel.v;
		}
		bind x_out = pos.x;
	}
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'LengthValue'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'LengthValue'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Dynamics"))) (name "Dynamics") (declared-name "Dynamics")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Dynamics::*"))) (name "*") (declared-name "*"))
        (element (kind "calc def") (id (node (document "d0") (qualified-name "Dynamics::Acceleration"))) (name "Acceleration") (declared-name "Acceleration")
          (contains
            (element (kind "return parameter") (id (node (document "d0") (qualified-name "Dynamics::Acceleration::a"))) (name "a") (declared-name "a") (effective (featuring-type (node (document "d0") (qualified-name "Dynamics::Acceleration")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Dynamics::Acceleration::dt"))) (name "dt") (declared-name "dt") (effective (featuring-type (node (document "d0") (qualified-name "Dynamics::Acceleration")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Dynamics::Acceleration::tm"))) (name "tm") (declared-name "tm") (effective (featuring-type (node (document "d0") (qualified-name "Dynamics::Acceleration")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Dynamics::Acceleration::tp"))) (name "tp") (declared-name "tp") (effective (featuring-type (node (document "d0") (qualified-name "Dynamics::Acceleration")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "Dynamics::Position"))) (name "Position") (declared-name "Position")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Dynamics::Position::dt"))) (name "dt") (declared-name "dt") (effective (featuring-type (node (document "d0") (qualified-name "Dynamics::Position")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Dynamics::Position::v"))) (name "v") (declared-name "v") (effective (featuring-type (node (document "d0") (qualified-name "Dynamics::Position")))))
            (element (kind "return parameter") (id (node (document "d0") (qualified-name "Dynamics::Position::x"))) (name "x") (declared-name "x") (effective (featuring-type (node (document "d0") (qualified-name "Dynamics::Position")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Dynamics::Position::x0"))) (name "x0") (declared-name "x0") (effective (featuring-type (node (document "d0") (qualified-name "Dynamics::Position")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "Dynamics::Power"))) (name "Power") (declared-name "Power")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Dynamics::Power::Cd"))) (name "Cd") (declared-name "Cd") (effective (featuring-type (node (document "d0") (qualified-name "Dynamics::Power")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Dynamics::Power::Cf"))) (name "Cf") (declared-name "Cf") (effective (featuring-type (node (document "d0") (qualified-name "Dynamics::Power")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Dynamics::Power::tm"))) (name "tm") (declared-name "tm") (effective (featuring-type (node (document "d0") (qualified-name "Dynamics::Power")))))
            (element (kind "return parameter") (id (node (document "d0") (qualified-name "Dynamics::Power::tp"))) (name "tp") (declared-name "tp") (effective (featuring-type (node (document "d0") (qualified-name "Dynamics::Power")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Dynamics::Power::v"))) (name "v") (declared-name "v") (effective (featuring-type (node (document "d0") (qualified-name "Dynamics::Power")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Dynamics::Power::whlpwr"))) (name "whlpwr") (declared-name "whlpwr") (effective (featuring-type (node (document "d0") (qualified-name "Dynamics::Power")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Dynamics::Real"))) (name "Real") (declared-name "Real"))
        (element (kind "action def") (id (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics"))) (name "StraightLineVehicleDynamics") (declared-name "StraightLineVehicleDynamics")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::Cd"))) (name "Cd") (declared-name "Cd") (effective (featuring-type (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::Cf"))) (name "Cf") (declared-name "Cf") (effective (featuring-type (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::a_out"))) (name "a_out") (declared-name "a_out") (effective (featuring-type (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::dt"))) (name "dt") (declared-name "dt") (effective (featuring-type (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::tm"))) (name "tm") (declared-name "tm") (effective (featuring-type (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::v_in"))) (name "v_in") (declared-name "v_in") (effective (featuring-type (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::v_out"))) (name "v_out") (declared-name "v_out") (effective (featuring-type (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::whlpwr"))) (name "whlpwr") (declared-name "whlpwr") (effective (featuring-type (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::x_in"))) (name "x_in") (declared-name "x_in") (effective (featuring-type (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::x_out"))) (name "x_out") (declared-name "x_out") (effective (featuring-type (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "Dynamics::Velocity"))) (name "Velocity") (declared-name "Velocity")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Dynamics::Velocity::a"))) (name "a") (declared-name "a") (effective (featuring-type (node (document "d0") (qualified-name "Dynamics::Velocity")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Dynamics::Velocity::dt"))) (name "dt") (declared-name "dt") (effective (featuring-type (node (document "d0") (qualified-name "Dynamics::Velocity")))))
            (element (kind "return parameter") (id (node (document "d0") (qualified-name "Dynamics::Velocity::v"))) (name "v") (declared-name "v") (effective (featuring-type (node (document "d0") (qualified-name "Dynamics::Velocity")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Dynamics::Velocity::v0"))) (name "v0") (declared-name "v0") (effective (featuring-type (node (document "d0") (qualified-name "Dynamics::Velocity")))))
          )
        )
        (element (kind "action") (id (node (document "d0") (qualified-name "Dynamics::dyn1"))) (name "dyn1") (declared-name "dyn1") (declared)
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Dynamics::dyn1::Cd"))) (name "Cd") (declared-name "Cd") (effective (featuring-type (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Dynamics::dyn1::Cf"))) (name "Cf") (declared-name "Cf") (effective (featuring-type (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Dynamics::dyn1::dt"))) (name "dt") (declared-name "dt") (effective (featuring-type (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Dynamics::dyn1::tm"))) (name "tm") (declared-name "tm") (effective (featuring-type (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics")))))
            (element (kind "action body decl") (id (node (document "d0") (qualified-name "Dynamics::dyn1::tp : PowerValue = Power(whlpwr, Cd, Cf, tm, v_in)"))) (name "tp : PowerValue = Power(whlpwr, Cd, Cf, tm, v_in)") (declared-name "tp : PowerValue = Power(whlpwr, Cd, Cf, tm, v_in)") (effective (featuring-type (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Dynamics::dyn1::v_in"))) (name "v_in") (declared-name "v_in") (effective (featuring-type (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Dynamics::dyn1::whlpwr"))) (name "whlpwr") (declared-name "whlpwr") (effective (featuring-type (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Dynamics::dyn1::x_in"))) (name "x_in") (declared-name "x_in") (effective (featuring-type (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics")))))
          )
        )
        (element (kind "action") (id (node (document "d0") (qualified-name "Dynamics::dyn2"))) (name "dyn2") (declared-name "dyn2") (declared)
          (contains
            (element (kind "action body decl") (id (node (document "d0") (qualified-name "Dynamics::dyn2::acc : Acceleration"))) (name "acc : Acceleration") (declared-name "acc : Acceleration") (effective (featuring-type (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics")))))
            (element (kind "action body decl") (id (node (document "d0") (qualified-name "Dynamics::dyn2::pos : Position"))) (name "pos : Position") (declared-name "pos : Position") (effective (featuring-type (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics")))))
            (element (kind "action body decl") (id (node (document "d0") (qualified-name "Dynamics::dyn2::vel : Velocity"))) (name "vel : Velocity") (declared-name "vel : Velocity") (effective (featuring-type (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics")))))
          )
        )
      )
    )
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "Dynamics::dyn1"))) (to (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Dynamics::dyn2"))) (to (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
    (bind (status pending-expression) (document "d0") (source-expression "a_out") (target-expression "acc::a") (container-prefix "Dynamics::dyn2"))
    (bind (status pending-expression) (document "d0") (source-expression "v_out") (target-expression "vel::v") (container-prefix "Dynamics::dyn2"))
    (bind (status pending-expression) (document "d0") (source-expression "x_out") (target-expression "pos::x") (container-prefix "Dynamics::dyn2"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/dynamics.md"
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
        (range (start 2 16) (end 2 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 2) (end 7 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 26) (end 7 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 40) (end 7 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 54) (end 7 72))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 73) (end 7 91))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 2) (end 8 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 25) (end 11 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 44) (end 11 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 63) (end 11 81))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 2) (end 12 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 21) (end 15 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 40) (end 15 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 60) (end 15 85))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 2) (end 16 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 19 21) (end 19 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 19 40) (end 19 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 19 61) (end 19 79))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 20 2) (end 20 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 27 2) (end 27 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 28 2) (end 28 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 29 2) (end 29 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 30 2) (end 30 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 31 2) (end 31 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 32 2) (end 32 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 33 2) (end 33 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 35 2) (end 35 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 36 2) (end 36 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 37 2) (end 37 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 53 8) (end 53 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 54 8) (end 54 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 55 8) (end 55 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 56 8) (end 56 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 57 8) (end 57 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 58 8) (end 58 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 59 8) (end 59 40))
      )
      (diagnostic
        (severity error)
        (code "recovered_action_body_element")
        (source "sysml")
        (range (start 63 2) (end 63 76))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 63 2) (end 63 76))
      )
      (diagnostic
        (severity error)
        (code "unresolved_pending_expression_relationship")
        (source "semantic")
        (range (start 74 7) (end 74 12))
      )
      (diagnostic
        (severity error)
        (code "unresolved_pending_expression_relationship")
        (source "semantic")
        (range (start 81 7) (end 81 12))
      )
      (diagnostic
        (severity error)
        (code "unresolved_pending_expression_relationship")
        (source "semantic")
        (range (start 88 7) (end 88 12))
      )
    )
  )
)
~~~
