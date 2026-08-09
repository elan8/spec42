# META
~~~ini
description=SysML Training 31 (Constraints): Derivation Constraints
type=file
~~~
# SOURCE
~~~sysml
package 'Derivation Constraints' {
	private import SI::*;
	private import 'Constraints Example-1'::*;
	
	part vehicle1 : Vehicle {
		attribute totalMass : MassValue;			
		assert constraint {totalMass == chassisMass + engine.mass + transmission.mass}	
	}
	
	part vehicle2 : Vehicle {
		attribute totalMass : MassValue = chassisMass + engine.mass + transmission.mass;
	}
	
	constraint def Dynamics {
		in mass: MassValue;
		in initialSpeed : SpeedValue;
		in finalSpeed : SpeedValue;
		in deltaT : TimeValue;
		in force : ForceValue;

		force * deltaT == mass * (finalSpeed - initialSpeed) and
		mass > 0[kg]
	}
	
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAssert,KwConstraint,OpenCurly,Ident,EqEq,Ident,Plus,Ident,Dot,Ident,Plus,Ident,Dot,Ident,CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Eq,Ident,Plus,Ident,Dot,Ident,Plus,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwConstraint,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
Ident,Star,Ident,EqEq,Ident,Star,OpenParen,Ident,Minus,Ident,CloseParen,KwAnd,
Ident,CloseAngle,DecimalValue,OpenSquare,Ident,CloseSquare,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Derivation Constraints''
    (import_decl private 'SI::*')
    (import_decl private ''Constraints Example-1'::*')
    (part_usage 'vehicle1' : 'Vehicle'
      (attribute_usage 'totalMass' : 'MassValue')
      (sysml_decl
        (result_expr_member)))
    (part_usage 'vehicle2' : 'Vehicle'
      (attribute_usage 'totalMass' : 'MassValue' value))
    (constraint_def 'Dynamics'
      (default_ref_usage in 'mass' : 'MassValue')
      (default_ref_usage in 'initialSpeed' : 'SpeedValue')
      (default_ref_usage in 'finalSpeed' : 'SpeedValue')
      (default_ref_usage in 'deltaT' : 'TimeValue')
      (default_ref_usage in 'force' : 'ForceValue')
      (result_expr_member))))
~~~
# FORMAT
~~~sysml
package 'Derivation Constraints' {
    private import SI::*;
    private import 'Constraints Example-1'::*;

    part vehicle1 : Vehicle {
        attribute totalMass : MassValue;
        assert constraint {
            = totalMass == chassisMass + engine.mass + transmission.mass;
        }
    }

    part vehicle2 : Vehicle {
        attribute totalMass : MassValue = chassisMass + engine.mass + transmission.mass;
    }

    constraint def Dynamics {
        in mass : MassValue;
        in initialSpeed : SpeedValue;
        in finalSpeed : SpeedValue;
        in deltaT : TimeValue;
        in force : ForceValue;

        = force * deltaT == mass * (finalSpeed - initialSpeed) and mass > 0[kg];
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'ForceValue'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'ForceValue'
~~~
# SMG
~~~
(model
  (namespace
    (package 'Derivation Constraints'
      (namespace_import private -> 'SI'[unresolved])
      (namespace_import private -> 'Constraints Example-1'[unresolved])
      (part_usage 'vehicle1' : 'Vehicle'[unresolved]
        (attribute_usage composite 'totalMass' : 'MassValue'[unresolved])
        (assert_constraint_usage
          (result_expr_membership)))
      (part_usage 'vehicle2' : 'Vehicle'[unresolved]
        (attribute_usage composite 'totalMass' : 'MassValue'[unresolved]
          (feature_value (=))))
      (constraint_def 'Dynamics'
        (reference_usage in reference 'mass' : 'MassValue'[unresolved])
        (reference_usage in reference 'initialSpeed' : 'SpeedValue'[unresolved])
        (reference_usage in reference 'finalSpeed' : 'SpeedValue'[unresolved])
        (reference_usage in reference 'deltaT' : 'TimeValue'[unresolved])
        (reference_usage in reference 'force' : 'ForceValue'[unresolved])
        (result_expr_membership)))))
~~~
