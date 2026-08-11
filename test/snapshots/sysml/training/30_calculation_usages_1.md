# META
~~~ini
description=SysML Training 30 (Calculations): Calculation Usages-1
type=file
~~~
# SOURCE
~~~sysml
package 'Calculation Usages-1' {
	private import ScalarValues::Real;
	private import ISQ::*;
	private import 'Calculation Definitions'::*;
	
	part def VehicleDynamics {
		attribute C_d : Real;
		attribute C_f : Real;
		attribute wheelPower : PowerValue;
		attribute mass : MassValue;
		
		action straightLineDynamics {
			in delta_t : TimeValue;
			in v_in : SpeedValue;
			in x_in : LengthValue;
			out v_out : SpeedValue = vel.v;
			out x_out : LengthValue = pos.x;
		
			calc acc : Acceleration {
				in tp = Power(wheelPower, C_d, C_f, mass, v_in);
				in tm = mass;
				in v = v_in;
				return a;
			}
			
			calc vel : Velocity {
				in dt = delta_t;
				in v0 = v_in;
				in a = acc.a;
				return v;
			}
			
			calc pos : Position {
				in dt = delta_t;
				in x0 = x_in;
				in v0 = vel.v;
				return x;	
			}
		}
	} 
	
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAction,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwOut,Ident,Colon,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwOut,Ident,Colon,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwCalc,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Eq,Ident,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,
KwIn,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Semicolon,
KwReturn,Ident,Semicolon,
CloseCurly,
KwCalc,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwReturn,Ident,Semicolon,
CloseCurly,
KwCalc,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwReturn,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Calculation Usages-1''
    (import_decl private 'ScalarValues::Real')
    (import_decl private 'ISQ::*')
    (import_decl private ''Calculation Definitions'::*')
    (part_def 'VehicleDynamics'
      (attribute_usage 'C_d' : 'Real')
      (attribute_usage 'C_f' : 'Real')
      (attribute_usage 'wheelPower' : 'PowerValue')
      (attribute_usage 'mass' : 'MassValue')
      (action_usage 'straightLineDynamics'
        (default_ref_usage in 'delta_t' : 'TimeValue')
        (default_ref_usage in 'v_in' : 'SpeedValue')
        (default_ref_usage in 'x_in' : 'LengthValue')
        (default_ref_usage out 'v_out' : 'SpeedValue' value)
        (default_ref_usage out 'x_out' : 'LengthValue' value)
        (calc_usage 'acc' : 'Acceleration'
          (default_ref_usage in 'tp' value)
          (default_ref_usage in 'tm' value)
          (default_ref_usage in 'v' value)
          (return_member))
        (calc_usage 'vel' : 'Velocity'
          (default_ref_usage in 'dt' value)
          (default_ref_usage in 'v0' value)
          (default_ref_usage in 'a' value)
          (return_member))
        (calc_usage 'pos' : 'Position'
          (default_ref_usage in 'dt' value)
          (default_ref_usage in 'x0' value)
          (default_ref_usage in 'v0' value)
          (return_member))))))
~~~
# FORMAT
~~~sysml
package 'Calculation Usages-1' {
    private import ScalarValues::Real;
    private import ISQ::*;
    private import 'Calculation Definitions'::*;

    part def VehicleDynamics {
        attribute C_d : Real;
        attribute C_f : Real;
        attribute wheelPower : PowerValue;
        attribute mass : MassValue;

        action straightLineDynamics {
            in delta_t : TimeValue;
            in v_in : SpeedValue;
            in x_in : LengthValue;
            out v_out : SpeedValue = vel.v;
            out x_out : LengthValue = pos.x;

            calc acc : Acceleration {
                in tp = Power(wheelPower, C_d, C_f, mass, v_in);
                in tm = mass;
                in v = v_in;
                return a;
            }

            calc vel : Velocity {
                in dt = delta_t;
                in v0 = v_in;
                in a = acc.a;
                return v;
            }

            calc pos : Position {
                in dt = delta_t;
                in x0 = x_in;
                in v0 = vel.v;
                return x;
            }
        }
    }

}

~~~
# EXPECTED
~~~
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'Acceleration'
semantic.unresolved_name 'Velocity'
semantic.unresolved_name 'Position'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'Acceleration'
semantic.unresolved_name 'Velocity'
semantic.unresolved_name 'Position'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Calculation Usages-1"))) (name "Calculation Usages-1") (declared-name "Calculation Usages-1")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Calculation Usages-1::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Calculation Usages-1::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Calculation Usages-1::Real"))) (name "Real") (declared-name "Real"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics"))) (name "VehicleDynamics") (declared-name "VehicleDynamics") (declared)
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::C_d"))) (name "C_d") (declared-name "C_d") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::C_f"))) (name "C_f") (declared-name "C_f") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::mass"))) (name "mass") (declared-name "mass") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics")))))
            (element (kind "action") (id (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics"))) (name "straightLineDynamics") (declared-name "straightLineDynamics") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics"))))
              (contains
                (element (kind "action body decl") (id (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics::acc : Acceleration"))) (name "acc : Acceleration") (declared-name "acc : Acceleration") (effective (featuring-type (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics::delta_t"))) (name "delta_t") (declared-name "delta_t") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics")))))
                (element (kind "action body decl") (id (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics::pos : Position"))) (name "pos : Position") (declared-name "pos : Position") (effective (featuring-type (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics::v_in"))) (name "v_in") (declared-name "v_in") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics::v_out"))) (name "v_out") (declared-name "v_out") (declared (properties (direction "out")) (own-expression (expression (kind "memberAccess") (reference "v") (children (expression (kind "featureReference") (reference "vel")))))) (effective (featuring-type (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics")))) (evaluation (expression (status "unresolved") (error "expression has an unresolved reference"))))
                (element (kind "action body decl") (id (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics::vel : Velocity"))) (name "vel : Velocity") (declared-name "vel : Velocity") (effective (featuring-type (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics::x_in"))) (name "x_in") (declared-name "x_in") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics::x_out"))) (name "x_out") (declared-name "x_out") (declared (properties (direction "out")) (own-expression (expression (kind "memberAccess") (reference "x") (children (expression (kind "featureReference") (reference "pos")))))) (effective (featuring-type (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics")))) (evaluation (expression (status "unresolved") (error "expression has an unresolved reference"))))
              )
            )
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::wheelPower"))) (name "wheelPower") (declared-name "wheelPower") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics")))))
          )
        )
      )
    )
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::C_d"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::C_f"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::mass"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::wheelPower"))) (status missing-prerequisite) (target "Base::dataValues"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/30_calculation_usages_1.md"
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
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 2) (end 6 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 2) (end 7 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 2) (end 8 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 2) (end 9 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 3) (end 12 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 3) (end 13 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 3) (end 14 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 3) (end 15 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 3) (end 16 35))
      )
    )
  )
)
~~~
