# META
~~~ini
description=SysML Training 36 (Variability): Variation Usages
type=file
~~~
# SOURCE
~~~sysml
package 'Variation Usages' {
	private import 'Variation Definitions'::*;
	
	part def Vehicle;
	part def Transmission;
	part manualTransmission;
	part automaticTransmission;
	
	abstract part vehicleFamily : Vehicle {
		part engine : EngineChoices[1];
		
		variation part transmission : Transmission[1] {
			variant manualTransmission;
			variant automaticTransmission;
		}
		
		assert constraint {
			(engine == engine::'4cylEngine' and
			 transmission == transmission::manualTransmission) xor
			(engine == engine::'6cylEngine' and 
			 transmission == transmission::automaticTransmission)
		}	
	}
	
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,Ident,Semicolon,
KwPart,Ident,Semicolon,
KwAbstract,KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwVariation,KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwVariant,Ident,Semicolon,
KwVariant,Ident,Semicolon,
CloseCurly,
KwAssert,KwConstraint,OpenCurly,
OpenParen,Ident,EqEq,Ident,ColonColon,UnrestrictedName,KwAnd,
Ident,EqEq,Ident,ColonColon,Ident,CloseParen,KwXor,
OpenParen,Ident,EqEq,Ident,ColonColon,UnrestrictedName,KwAnd,
Ident,EqEq,Ident,ColonColon,Ident,CloseParen,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Variation Usages''
    (import_decl private ''Variation Definitions'::*')
    (part_def 'Vehicle')
    (part_def 'Transmission')
    (part_usage 'manualTransmission')
    (part_usage 'automaticTransmission')
    (part_usage abstract 'vehicleFamily' : 'Vehicle'
      (part_usage 'engine' : 'EngineChoices' multiplicity)
      (part_usage variation 'transmission' : 'Transmission' multiplicity
        (variant_usage
          (default_ref_usage 'manualTransmission'))
        (variant_usage
          (default_ref_usage 'automaticTransmission')))
      (sysml_decl
        (result_expr_member)))))
~~~
# FORMAT
~~~sysml
package 'Variation Usages' {
    private import 'Variation Definitions'::*;

    part def Vehicle;
    part def Transmission;
    part manualTransmission;
    part automaticTransmission;

    abstract part vehicleFamily : Vehicle {
        part engine : EngineChoices[1];

        variation part transmission : Transmission[1] {
            variant manualTransmission;
            variant automaticTransmission;
        }

        assert constraint {
            (engine == engine::'4cylEngine' and
            transmission == transmission::manualTransmission) xor
            (engine == engine::'6cylEngine' and
            transmission == transmission::automaticTransmission)
        }
    }

}

~~~
# EXPECTED
~~~
semantic.unresolved_name 'EngineChoices'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'EngineChoices'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Variation Usages"))) (name "Variation Usages") (declared-name "Variation Usages")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Variation Usages::*"))) (name "*") (declared-name "*"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Variation Usages::Transmission"))) (name "Transmission") (declared-name "Transmission") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Variation Usages::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared))
        (element (kind "part") (id (node (document "d0") (qualified-name "Variation Usages::automaticTransmission"))) (name "automaticTransmission") (declared-name "automaticTransmission") (declared (properties (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "Variation Usages::manualTransmission"))) (name "manualTransmission") (declared-name "manualTransmission") (declared (properties (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "Variation Usages::vehicleFamily"))) (name "vehicleFamily") (declared-name "vehicleFamily") (declared (properties (abstract true) (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Variation Usages::vehicleFamily::engine"))) (name "engine") (declared-name "engine") (declared (properties (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Variation Usages::Vehicle")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "Variation Usages::vehicleFamily::transmission"))) (name "transmission") (declared-name "transmission") (declared (properties (variation true) (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Variation Usages::Vehicle"))))
              (contains
                (element (kind "variant") (id (node (document "d0") (qualified-name "Variation Usages::vehicleFamily::transmission::automaticTransmission"))) (name "automaticTransmission") (declared-name "automaticTransmission") (effective (featuring-type (node (document "d0") (qualified-name "Variation Usages::Transmission")))))
                (element (kind "variant") (id (node (document "d0") (qualified-name "Variation Usages::vehicleFamily::transmission::manualTransmission"))) (name "manualTransmission") (declared-name "manualTransmission") (effective (featuring-type (node (document "d0") (qualified-name "Variation Usages::Transmission")))))
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "Variation Usages::vehicleFamily"))) (to (node (document "d0") (qualified-name "Variation Usages::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Variation Usages::vehicleFamily::transmission"))) (to (node (document "d0") (qualified-name "Variation Usages::Transmission"))))
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
  (document "sysml/training/36_variation_usages.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 1) (end 1 43))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 5 1) (end 5 25))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 6 1) (end 6 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 16) (end 9 29))
      )
      (diagnostic
        (severity warning)
        (code "invalid_variation_member_kind")
        (source "semantic")
        (range (start 12 3) (end 12 30))
      )
      (diagnostic
        (severity warning)
        (code "invalid_variation_member_kind")
        (source "semantic")
        (range (start 13 3) (end 13 33))
      )
    )
  )
)
~~~
