# META
~~~ini
description=SysML Training 31 (Constraints): Analytical Constraints
type=file
~~~
# SOURCE
~~~sysml
package 'Analytical Constraints' {
	private import ISQ::*;
	private import 'Calculation Definitions'::*;
	
	constraint def StraightLineDynamicsEquations {
		in p : PowerValue;
		in m : MassValue;
		in dt : TimeValue;
		in x_i : LengthValue;
		in v_i : SpeedValue;
		in x_f : LengthValue;
		in v_f : SpeedValue;
		in a : AccelerationValue;
	
		attribute v_avg : SpeedValue = (v_i + v_f)/2;
		
		a == Acceleration(p, m, v_avg) and
		v_f == Velocity(dt, v_i, a) and
		x_f == Position(dt, x_i, v_avg)
	}
	
	action def StraightLineDynamics {
		in power : PowerValue;
		in mass : MassValue;
		in delta_t : TimeValue;
		in x_in : LengthValue;
		in v_in : SpeedValue;
		out x_out : LengthValue;
		out v_out : SpeedValue;
		out a_out : AccelerationValue;
	
	    assert constraint dynamics : StraightLineDynamicsEquations {
			in p = power;
			in m = mass;
			in dt = delta_t;
			in x_i = x_in;
			in v_i = v_in;
			in x_f = x_out;
			in v_f = v_out;
			in a = a_out;
	    }
	}
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwConstraint,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Eq,OpenParen,Ident,Plus,Ident,CloseParen,Slash,DecimalValue,Semicolon,
Ident,EqEq,Ident,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,KwAnd,
Ident,EqEq,Ident,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,KwAnd,
Ident,EqEq,Ident,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,
CloseCurly,
KwAction,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwOut,Ident,Colon,Ident,Semicolon,
KwOut,Ident,Colon,Ident,Semicolon,
KwOut,Ident,Colon,Ident,Semicolon,
KwAssert,KwConstraint,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Analytical Constraints''
    (import_decl private 'ISQ::*')
    (import_decl private ''Calculation Definitions'::*')
    (constraint_def 'StraightLineDynamicsEquations'
      (default_ref_usage in 'p' : 'PowerValue')
      (default_ref_usage in 'm' : 'MassValue')
      (default_ref_usage in 'dt' : 'TimeValue')
      (default_ref_usage in 'x_i' : 'LengthValue')
      (default_ref_usage in 'v_i' : 'SpeedValue')
      (default_ref_usage in 'x_f' : 'LengthValue')
      (default_ref_usage in 'v_f' : 'SpeedValue')
      (default_ref_usage in 'a' : 'AccelerationValue')
      (attribute_usage 'v_avg' : 'SpeedValue' value)
      (result_expr_member))
    (action_def 'StraightLineDynamics'
      (default_ref_usage in 'power' : 'PowerValue')
      (default_ref_usage in 'mass' : 'MassValue')
      (default_ref_usage in 'delta_t' : 'TimeValue')
      (default_ref_usage in 'x_in' : 'LengthValue')
      (default_ref_usage in 'v_in' : 'SpeedValue')
      (default_ref_usage out 'x_out' : 'LengthValue')
      (default_ref_usage out 'v_out' : 'SpeedValue')
      (default_ref_usage out 'a_out' : 'AccelerationValue')
      (sysml_decl 'dynamics' : 'StraightLineDynamicsEquations'
        (default_ref_usage in 'p' value)
        (default_ref_usage in 'm' value)
        (default_ref_usage in 'dt' value)
        (default_ref_usage in 'x_i' value)
        (default_ref_usage in 'v_i' value)
        (default_ref_usage in 'x_f' value)
        (default_ref_usage in 'v_f' value)
        (default_ref_usage in 'a' value)))))
~~~
# FORMAT
~~~sysml
package 'Analytical Constraints' {
    private import ISQ::*;
    private import 'Calculation Definitions'::*;

    constraint def StraightLineDynamicsEquations {
        in p : PowerValue;
        in m : MassValue;
        in dt : TimeValue;
        in x_i : LengthValue;
        in v_i : SpeedValue;
        in x_f : LengthValue;
        in v_f : SpeedValue;
        in a : AccelerationValue;

        attribute v_avg : SpeedValue = (v_i + v_f)/2;

        = a == Acceleration(p, m, v_avg) and v_f == Velocity(dt, v_i, a) and x_f == Position(dt, x_i, v_avg);
    }

    action def StraightLineDynamics {
        in power : PowerValue;
        in mass : MassValue;
        in delta_t : TimeValue;
        in x_in : LengthValue;
        in v_in : SpeedValue;
        out x_out : LengthValue;
        out v_out : SpeedValue;
        out a_out : AccelerationValue;

        assert constraint dynamics : StraightLineDynamicsEquations {
            in p = power;
            in m = mass;
            in dt = delta_t;
            in x_i = x_in;
            in v_i = v_in;
            in x_f = x_out;
            in v_f = v_out;
            in a = a_out;
        }
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'AccelerationValue'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'AccelerationValue'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Analytical Constraints"))) (name "Analytical Constraints") (declared-name "Analytical Constraints")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Analytical Constraints::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Analytical Constraints::*#import"))) (name "*") (declared-name "*"))
        (element (kind "action def") (id (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamics"))) (name "StraightLineDynamics") (declared-name "StraightLineDynamics")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamics::a_out"))) (name "a_out") (declared-name "a_out") (effective (featuring-type (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamics")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamics::delta_t"))) (name "delta_t") (declared-name "delta_t") (effective (featuring-type (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamics")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamics::mass"))) (name "mass") (declared-name "mass") (effective (featuring-type (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamics")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamics::power"))) (name "power") (declared-name "power") (effective (featuring-type (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamics")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamics::v_in"))) (name "v_in") (declared-name "v_in") (effective (featuring-type (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamics")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamics::v_out"))) (name "v_out") (declared-name "v_out") (effective (featuring-type (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamics")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamics::x_in"))) (name "x_in") (declared-name "x_in") (effective (featuring-type (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamics")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamics::x_out"))) (name "x_out") (declared-name "x_out") (effective (featuring-type (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamics")))))
          )
        )
        (element (kind "constraint def") (id (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamicsEquations"))) (name "StraightLineDynamicsEquations") (declared-name "StraightLineDynamicsEquations"))
      )
    )
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
