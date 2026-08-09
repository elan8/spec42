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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Calculation Usages-2"))) (name "Calculation Usages-2") (declared-name "Calculation Usages-2")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Calculation Usages-2::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Calculation Usages-2::*#import"))) (name "*") (declared-name "*"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "Calculation Usages-2::DynamicState"))) (name "DynamicState") (declared-name "DynamicState") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Calculation Usages-2::DynamicState::v"))) (name "v") (declared-name "v") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Calculation Usages-2::DynamicState")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Calculation Usages-2::DynamicState::x"))) (name "x") (declared-name "x") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Calculation Usages-2::DynamicState")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Calculation Usages-2::Real"))) (name "Real") (declared-name "Real"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics"))) (name "VehicleDynamics") (declared-name "VehicleDynamics") (declared)
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::C_d"))) (name "C_d") (declared-name "C_d") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::C_f"))) (name "C_f") (declared-name "C_f") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::mass"))) (name "mass") (declared-name "mass") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics")))))
            (element (kind "calc") (id (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState"))) (name "updateState") (declared-name "updateState") (effective (featuring-type (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics"))))
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::currState"))) (name "currState") (declared-name "currState") (effective (featuring-type (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::delta_t"))) (name "delta_t") (declared-name "delta_t") (effective (featuring-type (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics")))))
              )
            )
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::wheelPower"))) (name "wheelPower") (declared-name "wheelPower") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics")))))
          )
        )
      )
    )
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::currState"))) (to (node (document "d0") (qualified-name "Calculation Usages-2::DynamicState"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/30_calculation_usages_2.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 1) (end 1 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 1) (end 2 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 1) (end 3 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 2) (end 6 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 2) (end 7 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 2) (end 11 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 2) (end 12 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 2) (end 13 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 2) (end 14 29))
      )
    )
  )
)
~~~
