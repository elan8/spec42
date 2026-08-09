# META
~~~ini
description=SysML Training 29 (Expressions): Car Mass Rollup Example 2
type=file
~~~
# SOURCE
~~~sysml
package 'Car Mass Rollup 1' {
	private import ScalarValues::*;
	private import MassRollup2::*;
	
	part def CarPart :> MassedThing {			
		attribute serialNumber: String;
	}
	
	part car: CarPart :> compositeThing {	
		attribute vin :>> serialNumber;
		
		part carParts: CarPart[*] :>> subcomponents;
		
		part engine :> carParts {
			//...
		}
		
		part transmission :> carParts {
			//...
		}
	}

	// Example usage
	
	private import SI::kg;
	part c :> car {
		attribute :>> simpleMass = 1000[kg];
		part :>> engine {
			attribute :>> simpleMass = 100[kg];
		}
		
		part redefines transmission {
			attribute :>> simpleMass = 50[kg];
		}	
	}
	
	// c::totalMass --> 1150.0[kg]
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,ColonGtGt,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,ColonGtGt,Ident,Semicolon,
KwPart,Ident,ColonGt,Ident,OpenCurly,
LineComment,
CloseCurly,
KwPart,Ident,ColonGt,Ident,OpenCurly,
LineComment,
CloseCurly,
CloseCurly,
LineComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwPart,ColonGtGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
KwPart,KwRedefines,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
LineComment,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Car Mass Rollup 1''
    (import_decl private 'ScalarValues::*')
    (import_decl private 'MassRollup2::*')
    (part_def 'CarPart' :> 'MassedThing'
      (attribute_usage 'serialNumber' : 'String'))
    (part_usage 'car' : 'CarPart' :> 'compositeThing'
      (attribute_usage 'vin' :>> 'serialNumber')
      (part_usage 'carParts' : 'CarPart' :>> 'subcomponents' multiplicity)
      (part_usage 'engine' :> 'carParts'
        (line_comment))
      (part_usage 'transmission' :> 'carParts'
        (line_comment)))
    (line_comment)
    (import_decl private 'SI::kg')
    (part_usage 'c' :> 'car'
      (attribute_usage :>> 'simpleMass' value)
      (part_usage :>> 'engine'
        (attribute_usage :>> 'simpleMass' value))
      (part_usage :>> 'transmission'
        (attribute_usage :>> 'simpleMass' value)))
    (line_comment)))
~~~
# FORMAT
~~~sysml
package 'Car Mass Rollup 1' {
    private import ScalarValues::*;
    private import MassRollup2::*;

    part def CarPart :> MassedThing {
        attribute serialNumber: String;
    }

    part car: CarPart :> compositeThing {
        attribute vin :>> serialNumber;

        part carParts: CarPart[*] :>> subcomponents;

        part engine :> carParts {
            //...
        }

        part transmission :> carParts {
            //...
        }
    }

    // Example usage

    private import SI::kg;
    part c :> car {
        attribute :>> simpleMass = 1000[kg];
        part :>> engine {
            attribute :>> simpleMass = 100[kg];
        }

        part redefines transmission {
            attribute :>> simpleMass = 50[kg];
        }
    }

    // c::totalMass --> 1150.0[kg]
}

~~~
# EXPECTED
~~~
semantic.unresolved_name 'MassedThing'
semantic.unresolved_name 'String'
semantic.unresolved_name 'compositeThing'
semantic.unresolved_name 'subcomponents'
semantic.unresolved_name 'simpleMass'
semantic.unresolved_name 'simpleMass'
semantic.unresolved_name 'simpleMass'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'MassedThing'
semantic.unresolved_name 'String'
semantic.unresolved_name 'compositeThing'
semantic.unresolved_name 'subcomponents'
semantic.unresolved_name 'simpleMass'
semantic.unresolved_name 'simpleMass'
semantic.unresolved_name 'simpleMass'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Car Mass Rollup 1"))) (name "Car Mass Rollup 1") (declared-name "Car Mass Rollup 1")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Car Mass Rollup 1::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Car Mass Rollup 1::*#import"))) (name "*") (declared-name "*"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Car Mass Rollup 1::CarPart"))) (name "CarPart") (declared-name "CarPart") (declared)
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Car Mass Rollup 1::CarPart::serialNumber"))) (name "serialNumber") (declared-name "serialNumber") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Car Mass Rollup 1::CarPart")))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "Car Mass Rollup 1::c"))) (name "c") (declared-name "c") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Car Mass Rollup 1::c::engine"))) (name "engine") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "Car Mass Rollup 1::c::engine::simpleMass"))) (name "simpleMass") (declared-name "simpleMass") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 100)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "kg")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "Car Mass Rollup 1::c::engine::simpleMass"))) (role feature-value))))
              )
            )
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Car Mass Rollup 1::c::simpleMass"))) (name "simpleMass") (declared-name "simpleMass") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 1000)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "kg")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "Car Mass Rollup 1::c::simpleMass"))) (role feature-value))))
            (element (kind "part") (id (node (document "d0") (qualified-name "Car Mass Rollup 1::c::transmission"))) (name "transmission") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "Car Mass Rollup 1::c::transmission::simpleMass"))) (name "simpleMass") (declared-name "simpleMass") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal 50)) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "kg")))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "Car Mass Rollup 1::c::transmission::simpleMass"))) (role feature-value))))
              )
            )
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "Car Mass Rollup 1::car"))) (name "car") (declared-name "car") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Car Mass Rollup 1::car::carParts"))) (name "carParts") (declared-name "carParts") (declared (properties (ordered false)) (multiplicity (lower unbounded) (upper unbounded) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Car Mass Rollup 1::CarPart")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "Car Mass Rollup 1::car::engine"))) (name "engine") (declared-name "engine") (declared (properties (ordered false))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Car Mass Rollup 1::CarPart")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "Car Mass Rollup 1::car::transmission"))) (name "transmission") (declared-name "transmission") (declared (properties (ordered false))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Car Mass Rollup 1::CarPart")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Car Mass Rollup 1::car::vin"))) (name "vin") (declared-name "vin") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Car Mass Rollup 1::CarPart")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Car Mass Rollup 1::kg"))) (name "kg") (declared-name "kg"))
      )
    )
  )
  (relationships
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "Car Mass Rollup 1::car::vin"))) (to (node (document "d0") (qualified-name "Car Mass Rollup 1::CarPart::serialNumber"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "Car Mass Rollup 1::c"))) (to (node (document "d0") (qualified-name "Car Mass Rollup 1::car"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "Car Mass Rollup 1::car::engine"))) (to (node (document "d0") (qualified-name "Car Mass Rollup 1::car::carParts"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "Car Mass Rollup 1::car::transmission"))) (to (node (document "d0") (qualified-name "Car Mass Rollup 1::car::carParts"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Car Mass Rollup 1::car"))) (to (node (document "d0") (qualified-name "Car Mass Rollup 1::CarPart"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Car Mass Rollup 1::car::carParts"))) (to (node (document "d0") (qualified-name "Car Mass Rollup 1::CarPart"))))
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
  (document "sysml/training/29_car_mass_rollup_example_2.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 4 1) (end 4 74))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 11 2) (end 11 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 24 16) (end 24 22))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 26 2) (end 26 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 26 2) (end 26 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 27 2) (end 27 62))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 28 3) (end 28 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 28 3) (end 28 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 31 2) (end 31 73))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 32 3) (end 32 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 32 3) (end 32 37))
      )
    )
  )
)
~~~
