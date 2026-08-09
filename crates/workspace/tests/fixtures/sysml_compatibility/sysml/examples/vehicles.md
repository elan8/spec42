# META
~~~ini
description=SysML Example (Mass Roll-up): Vehicles
type=file
~~~
# SOURCE
~~~sysml
package VehicleMasses {
	private import ScalarValues::*;
	private import MassRollup::*;
	
	part def CarPart :> MassedThing {			
		attribute serialNumber: String;
	}
	
	part car: CarPart :> compositeThing {	
		attribute vin redefines serialNumber;
		
		part carParts: CarPart[*] redefines subcomponents;
		
		part engine :> simpleThing, carParts {
			//...
		}
		
		part transmission :> simpleThing, carParts {
			//...
		}
	}

	// Example usage
	private import SI::*;	
	part c :> car {
		redefines mass = 1000 [kg];
		part redefines engine {
			redefines mass = 100 [kg];
		}
		
		part redefines transmission {
			redefines mass = 50 [kg];
		}	
	}
	
	// c.totalMass --> 1150.0 [kg]
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,KwRedefines,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwRedefines,Ident,Semicolon,
KwPart,Ident,ColonGt,Ident,Comma,Ident,OpenCurly,
LineComment,
CloseCurly,
KwPart,Ident,ColonGt,Ident,Comma,Ident,OpenCurly,
LineComment,
CloseCurly,
CloseCurly,
LineComment,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwRedefines,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwPart,KwRedefines,Ident,OpenCurly,
KwRedefines,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
KwPart,KwRedefines,Ident,OpenCurly,
KwRedefines,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
LineComment,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'VehicleMasses'
    (import_decl private 'ScalarValues::*')
    (import_decl private 'MassRollup::*')
    (part_def 'CarPart' :> 'MassedThing'
      (attribute_usage 'serialNumber' : 'String'))
    (part_usage 'car' : 'CarPart' :> 'compositeThing'
      (attribute_usage 'vin' :>> 'serialNumber')
      (part_usage 'carParts' : 'CarPart' :>> 'subcomponents' multiplicity)
      (part_usage 'engine' :> 'simpleThing', 'carParts'
        (line_comment))
      (part_usage 'transmission' :> 'simpleThing', 'carParts'
        (line_comment)))
    (line_comment)
    (import_decl private 'SI::*')
    (part_usage 'c' :> 'car'
      (default_ref_usage :>> 'mass' value)
      (part_usage :>> 'engine'
        (default_ref_usage :>> 'mass' value))
      (part_usage :>> 'transmission'
        (default_ref_usage :>> 'mass' value)))
    (line_comment)))
~~~
# FORMAT
~~~sysml
package VehicleMasses {
    private import ScalarValues::*;
    private import MassRollup::*;

    part def CarPart :> MassedThing {
        attribute serialNumber: String;
    }

    part car: CarPart :> compositeThing {
        attribute vin redefines serialNumber;

        part carParts: CarPart[*] redefines subcomponents;

        part engine :> simpleThing, carParts {
            //...
        }

        part transmission :> simpleThing, carParts {
            //...
        }
    }

    // Example usage
    private import SI::*;
    part c :> car {
        redefines mass = 1000 [kg];
        part redefines engine {
            redefines mass = 100 [kg];
        }

        part redefines transmission {
            redefines mass = 50 [kg];
        }
    }

    // c.totalMass --> 1150.0 [kg]
}

~~~
# EXPECTED
~~~
semantic.unresolved_name 'MassedThing'
semantic.unresolved_name 'String'
semantic.unresolved_name 'compositeThing'
semantic.unresolved_name 'subcomponents'
semantic.unresolved_name 'simpleThing'
semantic.unresolved_name 'simpleThing'
semantic.unresolved_name 'mass'
semantic.unresolved_name 'mass'
semantic.unresolved_name 'mass'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'MassedThing'
semantic.unresolved_name 'String'
semantic.unresolved_name 'compositeThing'
semantic.unresolved_name 'subcomponents'
semantic.unresolved_name 'simpleThing'
semantic.unresolved_name 'simpleThing'
semantic.unresolved_name 'mass'
semantic.unresolved_name 'mass'
semantic.unresolved_name 'mass'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "VehicleMasses"))) (name "VehicleMasses") (declared-name "VehicleMasses")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "VehicleMasses::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "VehicleMasses::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "VehicleMasses::*#import2"))) (name "*") (declared-name "*"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "VehicleMasses::CarPart"))) (name "CarPart") (declared-name "CarPart") (declared)
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "VehicleMasses::CarPart::serialNumber"))) (name "serialNumber") (declared-name "serialNumber") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "VehicleMasses::CarPart")))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "VehicleMasses::c"))) (name "c") (declared-name "c") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "VehicleMasses::c::engine"))) (name "engine") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
            (element (kind "part") (id (node (document "d0") (qualified-name "VehicleMasses::c::transmission"))) (name "transmission") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "VehicleMasses::car"))) (name "car") (declared-name "car") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "VehicleMasses::car::carParts"))) (name "carParts") (declared-name "carParts") (declared (properties (ordered false)) (multiplicity (lower unbounded) (upper unbounded) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "VehicleMasses::CarPart")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "VehicleMasses::car::engine"))) (name "engine") (declared-name "engine") (declared (properties (ordered false))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "VehicleMasses::CarPart")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "VehicleMasses::car::transmission"))) (name "transmission") (declared-name "transmission") (declared (properties (ordered false))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "VehicleMasses::CarPart")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "VehicleMasses::car::vin"))) (name "vin") (declared-name "vin") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "VehicleMasses::CarPart")))))
          )
        )
      )
    )
  )
  (relationships
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "VehicleMasses::car::vin"))) (to (node (document "d0") (qualified-name "VehicleMasses::CarPart::serialNumber"))) (provenance authored))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "VehicleMasses::c"))) (to (node (document "d0") (qualified-name "VehicleMasses::car"))) (provenance authored))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "VehicleMasses::car::engine"))) (to (node (document "d0") (qualified-name "VehicleMasses::car::carParts"))) (provenance authored))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "VehicleMasses::car::transmission"))) (to (node (document "d0") (qualified-name "VehicleMasses::car::carParts"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VehicleMasses::car"))) (to (node (document "d0") (qualified-name "VehicleMasses::CarPart"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VehicleMasses::car::carParts"))) (to (node (document "d0") (qualified-name "VehicleMasses::CarPart"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleMasses::CarPart"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleMasses::CarPart::serialNumber"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleMasses::c"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleMasses::c::engine"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleMasses::c::transmission"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleMasses::car"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleMasses::car::carParts"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleMasses::car::engine"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleMasses::car::transmission"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleMasses::car::vin"))) (status missing-prerequisite) (target "Base::dataValues"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/vehicles.md"
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
        (range (start 2 16) (end 2 26))
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
        (range (start 11 2) (end 11 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 23 16) (end 23 18))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 25 2) (end 25 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 26 2) (end 26 59))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 27 3) (end 27 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 30 2) (end 30 64))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 31 3) (end 31 31))
      )
    )
  )
)
~~~
