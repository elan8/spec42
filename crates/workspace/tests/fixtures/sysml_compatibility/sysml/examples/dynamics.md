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
        in whlpwr : PowerValue;
        in Cd : Real;
        in Cf : Real;
        in tm : MassValue;
        in v : SpeedValue;
        return tp : PowerValue = whlpwr - Cd * v - Cf * tm * v;
    }

    calc def Acceleration {
        in dt : TimeValue;
        in tm : MassValue;
        in tp : PowerValue;
        return a : AccelerationValue = tp * dt * tp;
    }

    calc def Velocity {
        in dt : TimeValue;
        in v0 : SpeedValue;
        in a : AccelerationValue;
        return v : SpeedValue = v0 + a * dt;
    }

    calc def Position {
        in dt : TimeValue;
        in x0 : LengthValue;
        in v : SpeedValue;
        return x : LengthValue = x0 + v * dt;
    }

    // Analysis action def

    action def StraightLineVehicleDynamics {
        in attribute dt : TimeValue;
        in attribute whlpwr : PowerValue;
        in attribute Cd : Real;
        in attribute Cf : Real;
        in attribute tm : MassValue;
        in attribute v_in : SpeedValue;
        in attribute x_in : LengthValue;

        out attribute a_out : AccelerationValue;
        out attribute v_out : SpeedValue;
        out attribute x_out : LengthValue;

        assert constraint {
            attribute tp : PowerValue;

            = tp == Power(whlpwr, Cd, Cf, tm, v_in) & a_out == Acceleration(dt, tm, tp) & v_out == Velocity(dt, v_in, a_out) & x_out == Position(dt, x_in, v_in);
        }
    }

    // Analysis actions

    action dyn1 : StraightLineVehicleDynamics {
        in attribute dt : TimeValue;
        in attribute whlpwr : PowerValue;
        in attribute Cd : Real;
        in attribute Cf : Real;
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
(model
  (namespace
    (package 'Dynamics'
      (membership_import private -> 'ScalarValues::Real'[unresolved])
      (namespace_import private -> 'ISQ'[unresolved])
      (calculation_def 'Power'
        (reference_usage in reference 'whlpwr' : 'PowerValue'[unresolved])
        (reference_usage in reference 'Cd' : 'Real'[unresolved])
        (reference_usage in reference 'Cf' : 'Real'[unresolved])
        (reference_usage in reference 'tm' : 'MassValue'[unresolved])
        (reference_usage in reference 'v' : 'SpeedValue'[unresolved])
        (return_parameter_membership
          (feature_def out 'tp' : 'PowerValue'[unresolved]
            (feature_value (=)))))
      (calculation_def 'Acceleration'
        (reference_usage in reference 'dt' : 'TimeValue'[unresolved])
        (reference_usage in reference 'tm' : 'MassValue'[unresolved])
        (reference_usage in reference 'tp' : 'PowerValue'[unresolved])
        (return_parameter_membership
          (feature_def out 'a' : 'AccelerationValue'[unresolved]
            (feature_value (=)))))
      (calculation_def 'Velocity'
        (reference_usage in reference 'dt' : 'TimeValue'[unresolved])
        (reference_usage in reference 'v0' : 'SpeedValue'[unresolved])
        (reference_usage in reference 'a' : 'AccelerationValue'[unresolved])
        (return_parameter_membership
          (feature_def out 'v' : 'SpeedValue'[unresolved]
            (feature_value (=)))))
      (calculation_def 'Position'
        (reference_usage in reference 'dt' : 'TimeValue'[unresolved])
        (reference_usage in reference 'x0' : 'LengthValue'[unresolved])
        (reference_usage in reference 'v' : 'SpeedValue'[unresolved])
        (return_parameter_membership
          (feature_def out 'x' : 'LengthValue'[unresolved]
            (feature_value (=)))))
      (action_def 'StraightLineVehicleDynamics'
        (attribute_usage in 'dt' : 'TimeValue'[unresolved])
        (attribute_usage in 'whlpwr' : 'PowerValue'[unresolved])
        (attribute_usage in 'Cd' : 'Real'[unresolved])
        (attribute_usage in 'Cf' : 'Real'[unresolved])
        (attribute_usage in 'tm' : 'MassValue'[unresolved])
        (attribute_usage in 'v_in' : 'SpeedValue'[unresolved])
        (attribute_usage in 'x_in' : 'LengthValue'[unresolved])
        (attribute_usage out 'a_out' : 'AccelerationValue'[unresolved])
        (attribute_usage out 'v_out' : 'SpeedValue'[unresolved])
        (attribute_usage out 'x_out' : 'LengthValue'[unresolved])
        (assert_constraint_usage
          (attribute_usage 'tp' : 'PowerValue'[unresolved])
          (result_expr_membership)))
      (action_usage 'dyn1' : 'Dynamics::StraightLineVehicleDynamics'[action_def]
        (attribute_usage in 'dt' : 'TimeValue'[unresolved])
        (attribute_usage in 'whlpwr' : 'PowerValue'[unresolved])
        (attribute_usage in 'Cd' : 'Real'[unresolved])
        (attribute_usage in 'Cf' : 'Real'[unresolved])
        (attribute_usage in 'tm' : 'MassValue'[unresolved])
        (attribute_usage in 'v_in' : 'SpeedValue'[unresolved])
        (attribute_usage in 'x_in' : 'LengthValue'[unresolved])
        (attribute_usage composite 'tp' : 'PowerValue'[unresolved]
          (feature_value (=)))
        (attribute_usage out :>> 'Dynamics::StraightLineVehicleDynamics::a_out'[attribute_usage] : 'AccelerationValue'[unresolved]
          (feature_value (=)))
        (attribute_usage out :>> 'Dynamics::StraightLineVehicleDynamics::v_out'[attribute_usage] : 'SpeedValue'[unresolved]
          (feature_value (=)))
        (attribute_usage out :>> 'Dynamics::StraightLineVehicleDynamics::x_out'[attribute_usage] : 'LengthValue'[unresolved]
          (feature_value (=))))
      (action_usage 'dyn2' : 'Dynamics::StraightLineVehicleDynamics'[action_def]
        (calculation_usage composite 'acc' : 'Dynamics::Acceleration'[calculation_def]
          (reference_usage in reference 'dt'
            (feature_value (=)))
          (reference_usage in reference 'tm'
            (feature_value (=)))
          (reference_usage in reference 'tp'
            (feature_value (=))))
        (binding_connector_def
          (connector_end 'a_out')
          (connector_end 'acc.a'))
        (calculation_usage composite 'vel' : 'Dynamics::Velocity'[calculation_def]
          (reference_usage in reference 'dt'
            (feature_value (=)))
          (reference_usage in reference 'v0'
            (feature_value (=)))
          (reference_usage in reference 'a'
            (feature_value (=))))
        (binding_connector_def
          (connector_end 'v_out')
          (connector_end 'vel.v'))
        (calculation_usage composite 'pos' : 'Dynamics::Position'[calculation_def]
          (reference_usage in reference 'dt'
            (feature_value (=)))
          (reference_usage in reference 'x0'
            (feature_value (=)))
          (reference_usage in reference 'v0'
            (feature_value (=))))
        (binding_connector_def
          (connector_end 'x_out')
          (connector_end 'pos.x'))))))
~~~
