# META
~~~ini
description=SysML Training 30 (Calculations): Calculation Usages-2
type=file
~~~
# SOURCE
~~~sysml
package 'Calculation Usages-2' {
	private import ScalarValues::Real;
	private import ISQ::*;
	private import 'Calculation Definitions'::*;
	
	attribute def DynamicState {
		attribute v: SpeedValue;
		attribute x: LengthValue;
	}
	
	part def VehicleDynamics {
		attribute C_d : Real;
		attribute C_f : Real;
		attribute wheelPower : PowerValue;
		attribute mass : MassValue;
		
		calc updateState { 
			in delta_t : TimeValue; 
			in currState : DynamicState;
			attribute totalPower : PowerValue = Power(wheelPower, C_d, C_f, mass, currState.v);
			
			return attribute newState : DynamicState {
				:>> v = Velocity(delta_t, currState.v, Acceleration(totalPower, mass, currState.v));
				:>> x = Position(delta_t, currState.x, currState.v);
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
KwAttribute,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwCalc,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Eq,Ident,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,Comma,Ident,Dot,Ident,CloseParen,Semicolon,
KwReturn,KwAttribute,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,OpenParen,Ident,Comma,Ident,Dot,Ident,Comma,Ident,OpenParen,Ident,Comma,Ident,Comma,Ident,Dot,Ident,CloseParen,CloseParen,Semicolon,
ColonGtGt,Ident,Eq,Ident,OpenParen,Ident,Comma,Ident,Dot,Ident,Comma,Ident,Dot,Ident,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Calculation Usages-2''
    (import_decl private 'ScalarValues::Real')
    (import_decl private 'ISQ::*')
    (import_decl private ''Calculation Definitions'::*')
    (attribute_def 'DynamicState'
      (attribute_usage 'v' : 'SpeedValue')
      (attribute_usage 'x' : 'LengthValue'))
    (part_def 'VehicleDynamics'
      (attribute_usage 'C_d' : 'Real')
      (attribute_usage 'C_f' : 'Real')
      (attribute_usage 'wheelPower' : 'PowerValue')
      (attribute_usage 'mass' : 'MassValue')
      (calc_usage 'updateState'
        (default_ref_usage in 'delta_t' : 'TimeValue')
        (default_ref_usage in 'currState' : 'DynamicState')
        (attribute_usage 'totalPower' : 'PowerValue' value)
        (return_member)))))
~~~
# FORMAT
~~~sysml
package 'Calculation Usages-2' {
    private import ScalarValues::Real;
    private import ISQ::*;
    private import 'Calculation Definitions'::*;

    attribute def DynamicState {
        attribute v : SpeedValue;
        attribute x : LengthValue;
    }

    part def VehicleDynamics {
        attribute C_d : Real;
        attribute C_f : Real;
        attribute wheelPower : PowerValue;
        attribute mass : MassValue;

        calc updateState {
            in delta_t : TimeValue;
            in currState : DynamicState;
            attribute totalPower : PowerValue = Power(wheelPower, C_d, C_f, mass, currState.v);

            return attribute newState : DynamicState {
				:>> v = Velocity(delta_t, currState.v, Acceleration(totalPower, mass, currState.v));
				:>> x = Position(delta_t, currState.x, currState.v);
			}
        }
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'PowerValue'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'PowerValue'
~~~
# SMG
~~~
(model
  (namespace
    (package 'Calculation Usages-2'
      (membership_import private -> 'ScalarValues::Real'[unresolved])
      (namespace_import private -> 'ISQ'[unresolved])
      (namespace_import private -> 'Calculation Definitions'[unresolved])
      (attribute_def 'DynamicState'
        (attribute_usage composite 'v' : 'SpeedValue'[unresolved])
        (attribute_usage composite 'x' : 'LengthValue'[unresolved]))
      (part_def 'VehicleDynamics'
        (attribute_usage composite 'C_d' : 'Real'[unresolved])
        (attribute_usage composite 'C_f' : 'Real'[unresolved])
        (attribute_usage composite 'wheelPower' : 'PowerValue'[unresolved])
        (attribute_usage composite 'mass' : 'MassValue'[unresolved])
        (calculation_usage composite 'updateState'
          (reference_usage in reference 'delta_t' : 'TimeValue'[unresolved])
          (reference_usage in reference 'currState' : 'Calculation Usages-2::DynamicState'[attribute_def])
          (attribute_usage composite 'totalPower' : 'PowerValue'[unresolved]
            (feature_value (=)))
          (return_parameter_membership
            (attribute_usage out 'newState' : 'Calculation Usages-2::DynamicState'[attribute_def]
              (reference_usage reference :>> 'Calculation Usages-2::DynamicState::v'[attribute_usage]
                (feature_value (=)))
              (reference_usage reference :>> 'Calculation Usages-2::DynamicState::x'[attribute_usage]
                (feature_value (=))))))))))
~~~
