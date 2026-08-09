# META
~~~ini
description=SysML Training 31 (Constraints): Constraint Assertions-1
type=file
~~~
# SOURCE
~~~sysml
package 'Constraint Assertions-1' {
	private import ISQ::*;
	private import SI::*;
	private import NumericalFunctions::*;
	
	part def Engine;
	part def Transmission;
	
	constraint def MassConstraint {
		in partMasses : MassValue[0..*];
		in massLimit : MassValue;
			
		sum(partMasses) <= massLimit
	}
	
	part def Vehicle {
		assert constraint massConstraint : MassConstraint {
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
Ident,OpenParen,Ident,CloseParen,LtEq,Ident,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAssert,KwConstraint,Ident,Colon,Ident,OpenCurly,
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
  (package_def ''Constraint Assertions-1''
    (import_decl private 'ISQ::*')
    (import_decl private 'SI::*')
    (import_decl private 'NumericalFunctions::*')
    (part_def 'Engine')
    (part_def 'Transmission')
    (constraint_def 'MassConstraint'
      (default_ref_usage in 'partMasses' : 'MassValue' multiplicity)
      (default_ref_usage in 'massLimit' : 'MassValue')
      (result_expr_member))
    (part_def 'Vehicle'
      (sysml_decl 'massConstraint' : 'MassConstraint'
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
package 'Constraint Assertions-1' {
    private import ISQ::*;
    private import SI::*;
    private import NumericalFunctions::*;

    part def Engine;
    part def Transmission;

    constraint def MassConstraint {
        in partMasses : MassValue[0..*];
        in massLimit : MassValue;

        sum(partMasses) <= massLimit
    }

    part def Vehicle {
        assert constraint massConstraint : MassConstraint {
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
~~~
# PROBLEMS
~~~
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
    (element (kind "package") (id (node (document "d0") (qualified-name "Constraint Assertions-1"))) (name "Constraint Assertions-1") (declared-name "Constraint Assertions-1")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Constraint Assertions-1::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Constraint Assertions-1::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Constraint Assertions-1::*#import2"))) (name "*") (declared-name "*"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Constraint Assertions-1::Engine"))) (name "Engine") (declared-name "Engine") (declared))
        (element (kind "constraint def") (id (node (document "d0") (qualified-name "Constraint Assertions-1::MassConstraint"))) (name "MassConstraint") (declared-name "MassConstraint"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Constraint Assertions-1::Transmission"))) (name "Transmission") (declared-name "Transmission") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Constraint Assertions-1::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared)
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Constraint Assertions-1::Vehicle::chassisMass"))) (name "chassisMass") (declared-name "chassisMass") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Constraint Assertions-1::Vehicle")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "Constraint Assertions-1::Vehicle::engine"))) (name "engine") (declared-name "engine") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Constraint Assertions-1::Vehicle"))))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "Constraint Assertions-1::Vehicle::engine::mass"))) (name "mass") (declared-name "mass") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Constraint Assertions-1::Engine")))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "Constraint Assertions-1::Vehicle::transmission"))) (name "transmission") (declared-name "transmission") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Constraint Assertions-1::Vehicle"))))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "Constraint Assertions-1::Vehicle::transmission::mass"))) (name "mass") (declared-name "mass") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Constraint Assertions-1::Engine")))))
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "Constraint Assertions-1::Vehicle::engine"))) (to (node (document "d0") (qualified-name "Constraint Assertions-1::Engine"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Constraint Assertions-1::Vehicle::transmission"))) (to (node (document "d0") (qualified-name "Constraint Assertions-1::Engine"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Constraint Assertions-1::Engine"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Constraint Assertions-1::MassConstraint"))) (status missing-prerequisite) (target "Constraints::ConstraintCheck"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Constraint Assertions-1::Transmission"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Constraint Assertions-1::Vehicle"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Constraint Assertions-1::Vehicle::chassisMass"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Constraint Assertions-1::Vehicle::engine"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Constraint Assertions-1::Vehicle::engine::mass"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Constraint Assertions-1::Vehicle::transmission"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Constraint Assertions-1::Vehicle::transmission::mass"))) (status missing-prerequisite) (target "Base::dataValues"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/31_constraint_assertions_1.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 2) (end 21 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 24 3) (end 24 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 28 3) (end 28 30))
      )
    )
  )
)
~~~
