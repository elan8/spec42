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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Derivation Constraints"))) (name "Derivation Constraints") (declared-name "Derivation Constraints")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Derivation Constraints::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Derivation Constraints::*#import"))) (name "*") (declared-name "*"))
        (element (kind "constraint def") (id (node (document "d0") (qualified-name "Derivation Constraints::Dynamics"))) (name "Dynamics") (declared-name "Dynamics") (declared (own-expression (expression (kind "binary") (operator "&&") (children (expression (kind "binary") (operator "==") (children (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "force")) (expression (kind "featureReference") (reference "deltaT")))) (expression (kind "binary") (operator "*") (children (expression (kind "featureReference") (reference "mass")) (expression (kind "parenthesized") (children (expression (kind "binary") (operator "-") (children (expression (kind "featureReference") (reference "finalSpeed")) (expression (kind "featureReference") (reference "initialSpeed")))))))))) (expression (kind "binary") (operator ">") (children (expression (kind "featureReference") (reference "mass")) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal (integer 0))) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "kg")))))))))))) (evaluation (expression (status "unresolved") (error "expression has an unresolved reference"))))
        (element (kind "part") (id (node (document "d0") (qualified-name "Derivation Constraints::vehicle1"))) (name "vehicle1") (declared-name "vehicle1") (declared (properties (ordered false)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Derivation Constraints::vehicle1::totalMass"))) (name "totalMass") (declared-name "totalMass") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "Derivation Constraints::vehicle2"))) (name "vehicle2") (declared-name "vehicle2") (declared (properties (ordered false)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Derivation Constraints::vehicle2::totalMass"))) (name "totalMass") (declared-name "totalMass") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "+") (children (expression (kind "binary") (operator "+") (children (expression (kind "featureReference") (reference "chassisMass")) (expression (kind "memberAccess") (reference "mass") (children (expression (kind "featureReference") (reference "engine")))))) (expression (kind "memberAccess") (reference "mass") (children (expression (kind "featureReference") (reference "transmission")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "Derivation Constraints::vehicle2::totalMass"))) (role feature-value))) (evaluation (expression (status "unresolved") (error "expression has an unresolved reference"))))
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
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Derivation Constraints::Dynamics"))) (status missing-prerequisite) (target "Constraints::ConstraintCheck"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Derivation Constraints::vehicle1"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Derivation Constraints::vehicle1::totalMass"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Derivation Constraints::vehicle2"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Derivation Constraints::vehicle2::totalMass"))) (status missing-prerequisite) (target "Base::dataValues"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/31_derivation_constraints.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 4 17) (end 4 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 2) (end 5 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 17) (end 9 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 2) (end 10 82))
      )
    )
  )
)
~~~
