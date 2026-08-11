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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "dynamics.md"
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
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 74 7) (end 74 12))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 74 15) (end 74 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 81 7) (end 81 12))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 81 15) (end 81 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 88 7) (end 88 12))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 88 15) (end 88 20))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "8c477366c3d204950283106cbd2d1ce9e50f5dcd4a6d228fe0bb623977956afc") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Dynamics"))) (kind "package") (name "Dynamics") (declared-name "Dynamics") (range (start (line 0) (character 0)) (end (line 0) (character 2339))))
    (element (id (node (document "d0") (qualified-name "Dynamics::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 23))) (parent (node (document "d0") (qualified-name "Dynamics"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 19))))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Acceleration"))) (kind "calc def") (name "Acceleration") (declared-name "Acceleration") (range (start (line 11) (character 1)) (end (line 11) (character 132))) (parent (node (document "d0") (qualified-name "Dynamics"))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Acceleration::a"))) (kind "return parameter") (name "a") (declared-name "a") (range (start (line 12) (character 2)) (end (line 12) (character 46))) (parent (node (document "d0") (qualified-name "Dynamics::Acceleration"))) (authored (relationships (typing (reference "AccelerationValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Acceleration::dt"))) (kind "in out parameter") (name "dt") (declared-name "dt") (range (start (line 11) (character 25)) (end (line 11) (character 43))) (parent (node (document "d0") (qualified-name "Dynamics::Acceleration"))) (authored (relationships (typing (reference "TimeValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Acceleration::tm"))) (kind "in out parameter") (name "tm") (declared-name "tm") (range (start (line 11) (character 44)) (end (line 11) (character 62))) (parent (node (document "d0") (qualified-name "Dynamics::Acceleration"))) (authored (relationships (typing (reference "MassValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Acceleration::tp"))) (kind "in out parameter") (name "tp") (declared-name "tp") (range (start (line 11) (character 63)) (end (line 11) (character 81))) (parent (node (document "d0") (qualified-name "Dynamics::Acceleration"))) (authored (relationships (typing (reference "PowerValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Position"))) (kind "calc def") (name "Position") (declared-name "Position") (range (start (line 19) (character 1)) (end (line 19) (character 123))) (parent (node (document "d0") (qualified-name "Dynamics"))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Position::dt"))) (kind "in out parameter") (name "dt") (declared-name "dt") (range (start (line 19) (character 21)) (end (line 19) (character 39))) (parent (node (document "d0") (qualified-name "Dynamics::Position"))) (authored (relationships (typing (reference "TimeValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Position::v"))) (kind "in out parameter") (name "v") (declared-name "v") (range (start (line 19) (character 61)) (end (line 19) (character 79))) (parent (node (document "d0") (qualified-name "Dynamics::Position"))) (authored (relationships (typing (reference "SpeedValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Position::x"))) (kind "return parameter") (name "x") (declared-name "x") (range (start (line 20) (character 2)) (end (line 20) (character 39))) (parent (node (document "d0") (qualified-name "Dynamics::Position"))) (authored (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Position::x0"))) (kind "in out parameter") (name "x0") (declared-name "x0") (range (start (line 19) (character 40)) (end (line 19) (character 60))) (parent (node (document "d0") (qualified-name "Dynamics::Position"))) (authored (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Power"))) (kind "calc def") (name "Power") (declared-name "Power") (range (start (line 6) (character 1)) (end (line 6) (character 170))) (parent (node (document "d0") (qualified-name "Dynamics"))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Power::Cd"))) (kind "in out parameter") (name "Cd") (declared-name "Cd") (range (start (line 7) (character 26)) (end (line 7) (character 39))) (parent (node (document "d0") (qualified-name "Dynamics::Power"))) (authored (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Power::Cf"))) (kind "in out parameter") (name "Cf") (declared-name "Cf") (range (start (line 7) (character 40)) (end (line 7) (character 53))) (parent (node (document "d0") (qualified-name "Dynamics::Power"))) (authored (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Power::tm"))) (kind "in out parameter") (name "tm") (declared-name "tm") (range (start (line 7) (character 54)) (end (line 7) (character 72))) (parent (node (document "d0") (qualified-name "Dynamics::Power"))) (authored (relationships (typing (reference "MassValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Power::tp"))) (kind "return parameter") (name "tp") (declared-name "tp") (range (start (line 8) (character 2)) (end (line 8) (character 57))) (parent (node (document "d0") (qualified-name "Dynamics::Power"))) (authored (relationships (typing (reference "PowerValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Power::v"))) (kind "in out parameter") (name "v") (declared-name "v") (range (start (line 7) (character 73)) (end (line 7) (character 91))) (parent (node (document "d0") (qualified-name "Dynamics::Power"))) (authored (relationships (typing (reference "SpeedValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Power::whlpwr"))) (kind "in out parameter") (name "whlpwr") (declared-name "whlpwr") (range (start (line 7) (character 2)) (end (line 7) (character 25))) (parent (node (document "d0") (qualified-name "Dynamics::Power"))) (authored (relationships (typing (reference "PowerValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Real"))) (kind "import") (name "Real") (declared-name "Real") (range (start (line 1) (character 1)) (end (line 1) (character 35))) (parent (node (document "d0") (qualified-name "Dynamics"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 34))))))
    (element (id (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics"))) (kind "action def") (name "StraightLineVehicleDynamics") (declared-name "StraightLineVehicleDynamics") (range (start (line 25) (character 1)) (end (line 25) (character 607))) (parent (node (document "d0") (qualified-name "Dynamics"))))
    (element (id (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::Cd"))) (kind "in out parameter") (name "Cd") (declared-name "Cd") (range (start (line 29) (character 2)) (end (line 29) (character 25))) (parent (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics"))) (authored (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::Cf"))) (kind "in out parameter") (name "Cf") (declared-name "Cf") (range (start (line 30) (character 2)) (end (line 30) (character 24))) (parent (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics"))) (authored (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::a_out"))) (kind "in out parameter") (name "a_out") (declared-name "a_out") (range (start (line 35) (character 2)) (end (line 35) (character 42))) (parent (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics"))) (authored (relationships (typing (reference "AccelerationValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::dt"))) (kind "in out parameter") (name "dt") (declared-name "dt") (range (start (line 27) (character 2)) (end (line 27) (character 30))) (parent (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics"))) (authored (relationships (typing (reference "TimeValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::tm"))) (kind "in out parameter") (name "tm") (declared-name "tm") (range (start (line 31) (character 2)) (end (line 31) (character 30))) (parent (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics"))) (authored (relationships (typing (reference "MassValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::v_in"))) (kind "in out parameter") (name "v_in") (declared-name "v_in") (range (start (line 32) (character 2)) (end (line 32) (character 33))) (parent (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics"))) (authored (relationships (typing (reference "SpeedValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::v_out"))) (kind "in out parameter") (name "v_out") (declared-name "v_out") (range (start (line 36) (character 2)) (end (line 36) (character 35))) (parent (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics"))) (authored (relationships (typing (reference "SpeedValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::whlpwr"))) (kind "in out parameter") (name "whlpwr") (declared-name "whlpwr") (range (start (line 28) (character 2)) (end (line 28) (character 35))) (parent (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics"))) (authored (relationships (typing (reference "PowerValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::x_in"))) (kind "in out parameter") (name "x_in") (declared-name "x_in") (range (start (line 33) (character 2)) (end (line 33) (character 34))) (parent (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics"))) (authored (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::x_out"))) (kind "in out parameter") (name "x_out") (declared-name "x_out") (range (start (line 37) (character 2)) (end (line 37) (character 36))) (parent (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics"))) (authored (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Velocity"))) (kind "calc def") (name "Velocity") (declared-name "Velocity") (range (start (line 15) (character 1)) (end (line 15) (character 127))) (parent (node (document "d0") (qualified-name "Dynamics"))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Velocity::a"))) (kind "in out parameter") (name "a") (declared-name "a") (range (start (line 15) (character 60)) (end (line 15) (character 85))) (parent (node (document "d0") (qualified-name "Dynamics::Velocity"))) (authored (relationships (typing (reference "AccelerationValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Velocity::dt"))) (kind "in out parameter") (name "dt") (declared-name "dt") (range (start (line 15) (character 21)) (end (line 15) (character 39))) (parent (node (document "d0") (qualified-name "Dynamics::Velocity"))) (authored (relationships (typing (reference "TimeValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Velocity::v"))) (kind "return parameter") (name "v") (declared-name "v") (range (start (line 16) (character 2)) (end (line 16) (character 38))) (parent (node (document "d0") (qualified-name "Dynamics::Velocity"))) (authored (relationships (typing (reference "SpeedValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Dynamics::Velocity::v0"))) (kind "in out parameter") (name "v0") (declared-name "v0") (range (start (line 15) (character 40)) (end (line 15) (character 59))) (parent (node (document "d0") (qualified-name "Dynamics::Velocity"))) (authored (relationships (typing (reference "SpeedValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Dynamics::dyn1"))) (kind "action") (name "dyn1") (declared-name "dyn1") (range (start (line 52) (character 1)) (end (line 52) (character 584))) (parent (node (document "d0") (qualified-name "Dynamics"))) (authored (membership (kind Feature)) (relationships (typing (reference "StraightLineVehicleDynamics") (range none)))))
    (element (id (node (document "d0") (qualified-name "Dynamics::dyn1::Cd"))) (kind "in out parameter") (name "Cd") (declared-name "Cd") (range (start (line 55) (character 8)) (end (line 55) (character 31))) (parent (node (document "d0") (qualified-name "Dynamics::dyn1"))) (authored (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "Dynamics::dyn1::Cf"))) (kind "in out parameter") (name "Cf") (declared-name "Cf") (range (start (line 56) (character 8)) (end (line 56) (character 30))) (parent (node (document "d0") (qualified-name "Dynamics::dyn1"))) (authored (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "Dynamics::dyn1::dt"))) (kind "in out parameter") (name "dt") (declared-name "dt") (range (start (line 53) (character 8)) (end (line 53) (character 36))) (parent (node (document "d0") (qualified-name "Dynamics::dyn1"))) (authored (relationships (typing (reference "TimeValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Dynamics::dyn1::tm"))) (kind "in out parameter") (name "tm") (declared-name "tm") (range (start (line 57) (character 8)) (end (line 57) (character 36))) (parent (node (document "d0") (qualified-name "Dynamics::dyn1"))) (authored (relationships (typing (reference "MassValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Dynamics::dyn1::tp : PowerValue = Power(whlpwr, Cd, Cf, tm, v_in)"))) (kind "action body decl") (name "tp : PowerValue = Power(whlpwr, Cd, Cf, tm, v_in)") (declared-name "tp : PowerValue = Power(whlpwr, Cd, Cf, tm, v_in)") (range (start (line 61) (character 2)) (end (line 61) (character 62))) (parent (node (document "d0") (qualified-name "Dynamics::dyn1"))))
    (element (id (node (document "d0") (qualified-name "Dynamics::dyn1::v_in"))) (kind "in out parameter") (name "v_in") (declared-name "v_in") (range (start (line 58) (character 8)) (end (line 58) (character 39))) (parent (node (document "d0") (qualified-name "Dynamics::dyn1"))) (authored (relationships (typing (reference "SpeedValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Dynamics::dyn1::whlpwr"))) (kind "in out parameter") (name "whlpwr") (declared-name "whlpwr") (range (start (line 54) (character 8)) (end (line 54) (character 41))) (parent (node (document "d0") (qualified-name "Dynamics::dyn1"))) (authored (relationships (typing (reference "PowerValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Dynamics::dyn1::x_in"))) (kind "in out parameter") (name "x_in") (declared-name "x_in") (range (start (line 59) (character 8)) (end (line 59) (character 40))) (parent (node (document "d0") (qualified-name "Dynamics::dyn1"))) (authored (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Dynamics::dyn2"))) (kind "action") (name "dyn2") (declared-name "dyn2") (range (start (line 68) (character 1)) (end (line 68) (character 417))) (parent (node (document "d0") (qualified-name "Dynamics"))) (authored (membership (kind Feature)) (relationships (typing (reference "StraightLineVehicleDynamics") (range none)))))
    (element (id (node (document "d0") (qualified-name "Dynamics::dyn2::acc : Acceleration"))) (kind "action body decl") (name "acc : Acceleration") (declared-name "acc : Acceleration") (range (start (line 69) (character 2)) (end (line 69) (character 117))) (parent (node (document "d0") (qualified-name "Dynamics::dyn2"))))
    (element (id (node (document "d0") (qualified-name "Dynamics::dyn2::pos : Position"))) (kind "action body decl") (name "pos : Position") (declared-name "pos : Position") (range (start (line 83) (character 2)) (end (line 83) (character 89))) (parent (node (document "d0") (qualified-name "Dynamics::dyn2"))))
    (element (id (node (document "d0") (qualified-name "Dynamics::dyn2::vel : Velocity"))) (kind "action body decl") (name "vel : Velocity") (declared-name "vel : Velocity") (range (start (line 76) (character 2)) (end (line 76) (character 89))) (parent (node (document "d0") (qualified-name "Dynamics::dyn2"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (range (start (line 2) (character 16)) (end (line 2) (character 19))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::Acceleration::a"))) (kind featureTyping) (ordinal 0)) (authored-target "AccelerationValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::Acceleration::dt"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::Acceleration::tm"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::Acceleration::tp"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::Position::dt"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::Position::v"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::Position::x"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::Position::x0"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::Power::Cd"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Dynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::Power::Cf"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Dynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::Power::tm"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::Power::tp"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::Power::v"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::Power::whlpwr"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (range (start (line 1) (character 16)) (end (line 1) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::Cd"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Dynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::Cf"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Dynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::a_out"))) (kind featureTyping) (ordinal 0)) (authored-target "AccelerationValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::dt"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::tm"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::v_in"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::v_out"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::whlpwr"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::x_in"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::x_out"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::Velocity::a"))) (kind featureTyping) (ordinal 0)) (authored-target "AccelerationValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::Velocity::dt"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::Velocity::v"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::Velocity::v0"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::dyn1"))) (kind featureTyping) (ordinal 0)) (authored-target "StraightLineVehicleDynamics") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics")))))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::dyn1::Cd"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Dynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::dyn1::Cf"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Dynamics::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::dyn1::dt"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::dyn1::tm"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::dyn1::v_in"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::dyn1::whlpwr"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::dyn1::x_in"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::dyn2"))) (kind featureTyping) (ordinal 0)) (authored-target "StraightLineVehicleDynamics") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics")))))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::dyn2"))) (kind bindSource) (ordinal 0)) (authored-target "a_out") (range (start (line 74) (character 7)) (end (line 74) (character 12))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::dyn2"))) (kind bindSource) (ordinal 1)) (authored-target "v_out") (range (start (line 81) (character 7)) (end (line 81) (character 12))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::dyn2"))) (kind bindSource) (ordinal 2)) (authored-target "x_out") (range (start (line 88) (character 7)) (end (line 88) (character 12))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::dyn2"))) (kind bindTarget) (ordinal 0)) (authored-target "acc::a") (range (start (line 74) (character 15)) (end (line 74) (character 20))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::dyn2"))) (kind bindTarget) (ordinal 1)) (authored-target "vel::v") (range (start (line 81) (character 15)) (end (line 81) (character 20))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Dynamics::dyn2"))) (kind bindTarget) (ordinal 2)) (authored-target "pos::x") (range (start (line 88) (character 15)) (end (line 88) (character 20))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Dynamics::Power::Cd"))) (target (node (document "d0") (qualified-name "Dynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Dynamics::Power::Cd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Dynamics::Power::Cf"))) (target (node (document "d0") (qualified-name "Dynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Dynamics::Power::Cf"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::Cd"))) (target (node (document "d0") (qualified-name "Dynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::Cd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::Cf"))) (target (node (document "d0") (qualified-name "Dynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics::Cf"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Dynamics::dyn1"))) (target (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Dynamics::dyn1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Dynamics::dyn1::Cd"))) (target (node (document "d0") (qualified-name "Dynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Dynamics::dyn1::Cd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Dynamics::dyn1::Cf"))) (target (node (document "d0") (qualified-name "Dynamics::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Dynamics::dyn1::Cf"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Dynamics::dyn2"))) (target (node (document "d0") (qualified-name "Dynamics::StraightLineVehicleDynamics"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Dynamics::dyn2"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
