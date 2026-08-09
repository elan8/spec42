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
(model
  (namespace
    (package 'Calculation Usages-1'
      (membership_import private -> 'ScalarValues::Real'[unresolved])
      (namespace_import private -> 'ISQ'[unresolved])
      (namespace_import private -> 'Calculation Definitions'[unresolved])
      (part_def 'VehicleDynamics'
        (attribute_usage composite 'C_d' : 'Real'[unresolved])
        (attribute_usage composite 'C_f' : 'Real'[unresolved])
        (attribute_usage composite 'wheelPower' : 'PowerValue'[unresolved])
        (attribute_usage composite 'mass' : 'MassValue'[unresolved])
        (action_usage composite 'straightLineDynamics'
          (reference_usage in reference 'delta_t' : 'TimeValue'[unresolved])
          (reference_usage in reference 'v_in' : 'SpeedValue'[unresolved])
          (reference_usage in reference 'x_in' : 'LengthValue'[unresolved])
          (reference_usage out reference 'v_out' : 'SpeedValue'[unresolved]
            (feature_value (=)))
          (reference_usage out reference 'x_out' : 'LengthValue'[unresolved]
            (feature_value (=)))
          (calculation_usage composite 'acc' : 'Acceleration'[unresolved]
            (reference_usage in reference 'tp'
              (feature_value (=)))
            (reference_usage in reference 'tm'
              (feature_value (=)))
            (reference_usage in reference 'v'
              (feature_value (=)))
            (return_parameter_membership
              (feature_def out 'a')))
          (calculation_usage composite 'vel' : 'Velocity'[unresolved]
            (reference_usage in reference 'dt'
              (feature_value (=)))
            (reference_usage in reference 'v0'
              (feature_value (=)))
            (reference_usage in reference 'a'
              (feature_value (=)))
            (return_parameter_membership
              (feature_def out 'v')))
          (calculation_usage composite 'pos' : 'Position'[unresolved]
            (reference_usage in reference 'dt'
              (feature_value (=)))
            (reference_usage in reference 'x0'
              (feature_value (=)))
            (reference_usage in reference 'v0'
              (feature_value (=)))
            (return_parameter_membership
              (feature_def out 'x'))))))))
~~~
