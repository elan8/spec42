# META
~~~ini
description=SysML Training 31 (Constraints): Constraint Assertions-2
type=file
~~~
# SOURCE
~~~sysml
package 'Constraint Assertions-2' {
	private import ISQ::*;
	private import SI::*;
	private import NumericalFunctions::*;
	
	part def Engine;
	part def Transmission;
	
	constraint def MassConstraint {
		in partMasses : MassValue[0..*];
		in massLimit : MassValue;
	}
	
	constraint massConstraint : MassConstraint {
		in partMasses : MassValue[0..*];
		in massLimit : MassValue;
			
		sum(partMasses) <= massLimit
	}
	
	part def Vehicle {
		assert massConstraint {
			in partMasses = (chassisMass, engine.mass, transmission.mass);
			in massLimit = 2500[kg];
		}
		
		attribute chassisMass : MassValue;
		
		part engine : Engine {
			attribute mass : MassValue;
		}
		
		part transmission : Engine {
			attribute mass : MassValue;
		}
	}	
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwConstraint,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwConstraint,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
Ident,OpenParen,Ident,CloseParen,LtEq,Ident,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAssert,Ident,OpenCurly,
KwIn,Ident,Eq,OpenParen,Ident,Comma,Ident,Dot,Ident,Comma,Ident,Dot,Ident,CloseParen,Semicolon,
KwIn,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Constraint Assertions-2''
    (import_decl private 'ISQ::*')
    (import_decl private 'SI::*')
    (import_decl private 'NumericalFunctions::*')
    (part_def 'Engine')
    (part_def 'Transmission')
    (constraint_def 'MassConstraint'
      (default_ref_usage in 'partMasses' : 'MassValue' multiplicity)
      (default_ref_usage in 'massLimit' : 'MassValue'))
    (constraint_usage 'massConstraint' : 'MassConstraint'
      (default_ref_usage in 'partMasses' : 'MassValue' multiplicity)
      (default_ref_usage in 'massLimit' : 'MassValue')
      (result_expr_member))
    (part_def 'Vehicle'
      (sysml_decl 'massConstraint'
        (default_ref_usage in 'partMasses' value)
        (default_ref_usage in 'massLimit' value))
      (attribute_usage 'chassisMass' : 'MassValue')
      (part_usage 'engine' : 'Engine'
        (attribute_usage 'mass' : 'MassValue'))
      (part_usage 'transmission' : 'Engine'
        (attribute_usage 'mass' : 'MassValue')))))
~~~
# FORMAT
~~~sysml
package 'Constraint Assertions-2' {
    private import ISQ::*;
    private import SI::*;
    private import NumericalFunctions::*;

    part def Engine;
    part def Transmission;

    constraint def MassConstraint {
        in partMasses : MassValue[0..*];
        in massLimit : MassValue;
    }

    constraint massConstraint : MassConstraint {
        in partMasses : MassValue[0..*];
        in massLimit : MassValue;

        sum(partMasses) <= massLimit
    }

    part def Vehicle {
        assert massConstraint {
            in partMasses = (chassisMass, engine.mass, transmission.mass);
            in massLimit = 2500[kg];
        }

        attribute chassisMass : MassValue;

        part engine : Engine {
            attribute mass : MassValue;
        }

        part transmission : Engine {
            attribute mass : MassValue;
        }
    }
}

~~~
# EXPECTED
~~~
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Constraint Assertions-2"))) (name "Constraint Assertions-2") (declared-name "Constraint Assertions-2")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Constraint Assertions-2::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Constraint Assertions-2::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Constraint Assertions-2::*#import2"))) (name "*") (declared-name "*"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Constraint Assertions-2::Engine"))) (name "Engine") (declared-name "Engine") (declared))
        (element (kind "constraint def") (id (node (document "d0") (qualified-name "Constraint Assertions-2::MassConstraint"))) (name "MassConstraint") (declared-name "MassConstraint"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Constraint Assertions-2::Transmission"))) (name "Transmission") (declared-name "Transmission") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared)
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::chassisMass"))) (name "chassisMass") (declared-name "chassisMass") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::engine"))) (name "engine") (declared-name "engine") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle"))))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::engine::mass"))) (name "mass") (declared-name "mass") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Constraint Assertions-2::Engine")))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::transmission"))) (name "transmission") (declared-name "transmission") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle"))))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::transmission::mass"))) (name "mass") (declared-name "mass") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Constraint Assertions-2::Engine")))))
              )
            )
          )
        )
        (element (kind "constraint") (id (node (document "d0") (qualified-name "Constraint Assertions-2::massConstraint"))) (name "massConstraint") (declared-name "massConstraint"))
      )
    )
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::engine"))) (to (node (document "d0") (qualified-name "Constraint Assertions-2::Engine"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Constraint Assertions-2::Vehicle::transmission"))) (to (node (document "d0") (qualified-name "Constraint Assertions-2::Engine"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Constraint Assertions-2::massConstraint"))) (to (node (document "d0") (qualified-name "Constraint Assertions-2::MassConstraint"))))
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
  (document "sysml/training/31_constraint_assertions_2.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 1) (end 1 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 1) (end 2 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 1) (end 3 38))
      )
      (diagnostic
        (severity warning)
        (code "analysis_evaluation_unresolved")
        (source "semantic")
        (range (start 13 1) (end 13 146))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_def_body_element")
        (source "sysml")
        (range (start 21 2) (end 21 129))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 26 2) (end 26 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 29 3) (end 29 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 33 3) (end 33 30))
      )
    )
  )
)
~~~
