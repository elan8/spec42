# META
~~~ini
description=SysML Validation (15-Properties-Values-Expressions): 15_04-Logical Expressions
type=file
~~~
# SOURCE
~~~sysml
package '15_04-Logical Expressions' {
	private import ScalarValues::*;
	
	part def Engine;
	part def '4CylEngine' :> Engine;
	part def '6CylEngine' :> Engine;
	
	part def Transmission;
	part def ManualTransmission :> Transmission;
	part def AutomaticTransmission :> Transmission;
	
	part def Vehicle {
		attribute isHighPerformance: Boolean;
		
		part engine: Engine[1];
		part transmission: Transmission[1];
		
		assert constraint {
			if isHighPerformance? engine istype '6CylEngine'
			else engine istype '4CylEngine'
		}
		
		assert constraint {
			(engine istype '4CylEngine' and 
			 transmission istype ManualTransmission) xor
			(engine istype '6CylEngine' and
			 transmission istype AutomaticTransmission)
		}
	}
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,UnrestrictedName,ColonGt,Ident,Semicolon,
KwPart,KwDef,UnrestrictedName,ColonGt,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,ColonGt,Ident,Semicolon,
KwPart,KwDef,Ident,ColonGt,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAssert,KwConstraint,OpenCurly,
KwIf,Ident,Question,Ident,KwIstype,UnrestrictedName,
KwElse,Ident,KwIstype,UnrestrictedName,
CloseCurly,
KwAssert,KwConstraint,OpenCurly,
OpenParen,Ident,KwIstype,UnrestrictedName,KwAnd,
Ident,KwIstype,Ident,CloseParen,KwXor,
OpenParen,Ident,KwIstype,UnrestrictedName,KwAnd,
Ident,KwIstype,Ident,CloseParen,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''15_04-Logical Expressions''
    (import_decl private 'ScalarValues::*')
    (part_def 'Engine')
    (part_def ''4CylEngine'' :> 'Engine')
    (part_def ''6CylEngine'' :> 'Engine')
    (part_def 'Transmission')
    (part_def 'ManualTransmission' :> 'Transmission')
    (part_def 'AutomaticTransmission' :> 'Transmission')
    (part_def 'Vehicle'
      (attribute_usage 'isHighPerformance' : 'Boolean')
      (part_usage 'engine' : 'Engine' multiplicity)
      (part_usage 'transmission' : 'Transmission' multiplicity)
      (sysml_decl
        (result_expr_member))
      (sysml_decl
        (result_expr_member)))))
~~~
# FORMAT
~~~sysml
package '15_04-Logical Expressions' {
    private import ScalarValues::*;

    part def Engine;
    part def '4CylEngine' :> Engine;
    part def '6CylEngine' :> Engine;

    part def Transmission;
    part def ManualTransmission :> Transmission;
    part def AutomaticTransmission :> Transmission;

    part def Vehicle {
        attribute isHighPerformance: Boolean;

        part engine: Engine[1];
        part transmission: Transmission[1];

        assert constraint {
            if isHighPerformance? engine istype '6CylEngine'
            else engine istype '4CylEngine'
        }

        assert constraint {
            (engine istype '4CylEngine' and
            transmission istype ManualTransmission) xor
            (engine istype '6CylEngine' and
            transmission istype AutomaticTransmission)
        }
    }
}

~~~
# EXPECTED
~~~
semantic.unresolved_name 'Boolean'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Boolean'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "15_04-Logical Expressions"))) (name "15_04-Logical Expressions") (declared-name "15_04-Logical Expressions")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "15_04-Logical Expressions::*"))) (name "*") (declared-name "*"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "15_04-Logical Expressions::4CylEngine"))) (name "4CylEngine") (declared-name "4CylEngine") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "15_04-Logical Expressions::6CylEngine"))) (name "6CylEngine") (declared-name "6CylEngine") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "15_04-Logical Expressions::AutomaticTransmission"))) (name "AutomaticTransmission") (declared-name "AutomaticTransmission") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "15_04-Logical Expressions::Engine"))) (name "Engine") (declared-name "Engine") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "15_04-Logical Expressions::ManualTransmission"))) (name "ManualTransmission") (declared-name "ManualTransmission") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "15_04-Logical Expressions::Transmission"))) (name "Transmission") (declared-name "Transmission") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "15_04-Logical Expressions::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "15_04-Logical Expressions::Vehicle::engine"))) (name "engine") (declared-name "engine") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "15_04-Logical Expressions::Vehicle")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_04-Logical Expressions::Vehicle::isHighPerformance"))) (name "isHighPerformance") (declared-name "isHighPerformance") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_04-Logical Expressions::Vehicle")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "15_04-Logical Expressions::Vehicle::transmission"))) (name "transmission") (declared-name "transmission") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "15_04-Logical Expressions::Vehicle")))))
          )
        )
      )
    )
  )
  (relationships
    (specializes (status resolved) (from (node (document "d0") (qualified-name "15_04-Logical Expressions::4CylEngine"))) (to (node (document "d0") (qualified-name "15_04-Logical Expressions::Engine"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "15_04-Logical Expressions::6CylEngine"))) (to (node (document "d0") (qualified-name "15_04-Logical Expressions::Engine"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "15_04-Logical Expressions::AutomaticTransmission"))) (to (node (document "d0") (qualified-name "15_04-Logical Expressions::Transmission"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "15_04-Logical Expressions::ManualTransmission"))) (to (node (document "d0") (qualified-name "15_04-Logical Expressions::Transmission"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "15_04-Logical Expressions::Vehicle::engine"))) (to (node (document "d0") (qualified-name "15_04-Logical Expressions::Engine"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "15_04-Logical Expressions::Vehicle::transmission"))) (to (node (document "d0") (qualified-name "15_04-Logical Expressions::Transmission"))))
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
  (document "sysml/validation/15_04_logical_expressions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 1) (end 1 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 2) (end 12 39))
      )
    )
  )
)
~~~
