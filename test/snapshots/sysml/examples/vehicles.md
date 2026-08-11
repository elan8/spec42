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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "vehicles.md"
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
        (range (start 4 21) (end 4 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 2) (end 5 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 26) (end 5 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 8 22) (end 8 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 26) (end 9 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 38) (end 11 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 13 17) (end 13 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 17 23) (end 17 34))
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
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 27 3) (end 27 32))
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "ba97ef78975a2484e4f6454572093c677dbda4d7d913eaff4a1fb8383de23264") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "VehicleMasses"))) (kind "package") (name "VehicleMasses") (declared-name "VehicleMasses") (range (start (line 0) (character 0)) (end (line 0) (character 683))))
    (element (id (node (document "d0") (qualified-name "VehicleMasses::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 32))) (parent (node (document "d0") (qualified-name "VehicleMasses"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 28))))))
    (element (id (node (document "d0") (qualified-name "VehicleMasses::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 30))) (parent (node (document "d0") (qualified-name "VehicleMasses"))) (authored (membership (kind Import) (visibility "private") (import (reference "MassRollup::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 26))))))
    (element (id (node (document "d0") (qualified-name "VehicleMasses::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 23) (character 1)) (end (line 23) (character 22))) (parent (node (document "d0") (qualified-name "VehicleMasses"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 23) (character 16)) (end (line 23) (character 18))))))
    (element (id (node (document "d0") (qualified-name "VehicleMasses::CarPart"))) (kind "part def") (name "CarPart") (declared-name "CarPart") (range (start (line 4) (character 1)) (end (line 4) (character 74))) (parent (node (document "d0") (qualified-name "VehicleMasses"))) (authored (membership (kind Owning)) (relationships (specializes (reference "MassedThing") (range (start (line 4) (character 21)) (end (line 4) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "VehicleMasses::CarPart::serialNumber"))) (kind "attribute") (name "serialNumber") (declared-name "serialNumber") (range (start (line 5) (character 2)) (end (line 5) (character 33))) (parent (node (document "d0") (qualified-name "VehicleMasses::CarPart"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)) (typing (reference "String") (range (start (line 5) (character 26)) (end (line 5) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "VehicleMasses::c"))) (kind "part") (name "c") (declared-name "c") (range (start (line 24) (character 1)) (end (line 24) (character 178))) (parent (node (document "d0") (qualified-name "VehicleMasses"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "car") (range (start (line 24) (character 11)) (end (line 24) (character 14)))))))
    (element (id (node (document "d0") (qualified-name "VehicleMasses::c::engine"))) (kind "part") (name "engine") (range (start (line 26) (character 2)) (end (line 26) (character 59))) (parent (node (document "d0") (qualified-name "VehicleMasses::c"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "engine") (range (start (line 26) (character 17)) (end (line 26) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "VehicleMasses::c::transmission"))) (kind "part") (name "transmission") (range (start (line 30) (character 2)) (end (line 30) (character 64))) (parent (node (document "d0") (qualified-name "VehicleMasses::c"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "transmission") (range (start (line 30) (character 17)) (end (line 30) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "VehicleMasses::car"))) (kind "part") (name "car") (declared-name "car") (range (start (line 8) (character 1)) (end (line 8) (character 258))) (parent (node (document "d0") (qualified-name "VehicleMasses"))) (authored (membership (kind Feature)) (relationships (typing (reference "CarPart") (range (start (line 8) (character 11)) (end (line 8) (character 18)))) (subsetting (reference "compositeThing") (range (start (line 8) (character 22)) (end (line 8) (character 36)))))))
    (element (id (node (document "d0") (qualified-name "VehicleMasses::car::carParts"))) (kind "part") (name "carParts") (declared-name "carParts") (range (start (line 11) (character 2)) (end (line 11) (character 52))) (parent (node (document "d0") (qualified-name "VehicleMasses::car"))) (authored (membership (kind Feature)) (relationships (typing (reference "CarPart") (range (start (line 11) (character 17)) (end (line 11) (character 24)))) (redefinition (reference "subcomponents") (range (start (line 11) (character 38)) (end (line 11) (character 51)))))))
    (element (id (node (document "d0") (qualified-name "VehicleMasses::car::engine"))) (kind "part") (name "engine") (declared-name "engine") (range (start (line 13) (character 2)) (end (line 13) (character 53))) (parent (node (document "d0") (qualified-name "VehicleMasses::car"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "simpleThing") (range (start (line 13) (character 17)) (end (line 13) (character 28)))) (subsetting (reference "carParts") (range (start (line 13) (character 30)) (end (line 13) (character 38)))))))
    (element (id (node (document "d0") (qualified-name "VehicleMasses::car::transmission"))) (kind "part") (name "transmission") (declared-name "transmission") (range (start (line 17) (character 2)) (end (line 17) (character 59))) (parent (node (document "d0") (qualified-name "VehicleMasses::car"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "simpleThing") (range (start (line 17) (character 23)) (end (line 17) (character 34)))) (subsetting (reference "carParts") (range (start (line 17) (character 36)) (end (line 17) (character 44)))))))
    (element (id (node (document "d0") (qualified-name "VehicleMasses::car::vin"))) (kind "attribute") (name "vin") (declared-name "vin") (range (start (line 9) (character 2)) (end (line 9) (character 39))) (parent (node (document "d0") (qualified-name "VehicleMasses::car"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "serialNumber") (range (start (line 9) (character 26)) (end (line 9) (character 38)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "VehicleMasses::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 1) (character 16)) (end (line 1) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleMasses::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "MassRollup::*") (range (start (line 2) (character 16)) (end (line 2) (character 26))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleMasses::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (range (start (line 23) (character 16)) (end (line 23) (character 18))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleMasses::CarPart"))) (kind specialization) (ordinal 0)) (authored-target "MassedThing") (range (start (line 4) (character 21)) (end (line 4) (character 32))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleMasses::CarPart::serialNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleMasses::CarPart::serialNumber"))) (kind featureTyping) (ordinal 1)) (authored-target "String") (range (start (line 5) (character 26)) (end (line 5) (character 32))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleMasses::c"))) (kind subsetting) (ordinal 0)) (authored-target "car") (range (start (line 24) (character 11)) (end (line 24) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleMasses::car")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleMasses::c::engine"))) (kind redefinition) (ordinal 0)) (authored-target "engine") (range (start (line 26) (character 17)) (end (line 26) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleMasses::c::engine")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleMasses::c::transmission"))) (kind redefinition) (ordinal 0)) (authored-target "transmission") (range (start (line 30) (character 17)) (end (line 30) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleMasses::c::transmission")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleMasses::car"))) (kind featureTyping) (ordinal 0)) (authored-target "CarPart") (range (start (line 8) (character 11)) (end (line 8) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleMasses::CarPart")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleMasses::car"))) (kind subsetting) (ordinal 0)) (authored-target "compositeThing") (range (start (line 8) (character 22)) (end (line 8) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleMasses::car::carParts"))) (kind featureTyping) (ordinal 0)) (authored-target "CarPart") (range (start (line 11) (character 17)) (end (line 11) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleMasses::CarPart")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleMasses::car::carParts"))) (kind redefinition) (ordinal 0)) (authored-target "subcomponents") (range (start (line 11) (character 38)) (end (line 11) (character 51))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleMasses::car::engine"))) (kind subsetting) (ordinal 0)) (authored-target "simpleThing") (range (start (line 13) (character 17)) (end (line 13) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleMasses::car::engine"))) (kind subsetting) (ordinal 1)) (authored-target "carParts") (range (start (line 13) (character 30)) (end (line 13) (character 38))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleMasses::car::carParts")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleMasses::car::transmission"))) (kind subsetting) (ordinal 0)) (authored-target "simpleThing") (range (start (line 17) (character 23)) (end (line 17) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleMasses::car::transmission"))) (kind subsetting) (ordinal 1)) (authored-target "carParts") (range (start (line 17) (character 36)) (end (line 17) (character 44))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleMasses::car::carParts")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleMasses::car::vin"))) (kind redefinition) (ordinal 0)) (authored-target "serialNumber") (range (start (line 9) (character 26)) (end (line 9) (character 38))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "VehicleMasses::c"))) (target (node (document "d0") (qualified-name "VehicleMasses::car"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleMasses::c"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "VehicleMasses::c::engine"))) (target (node (document "d0") (qualified-name "VehicleMasses::c::engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleMasses::c::engine"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "VehicleMasses::c::transmission"))) (target (node (document "d0") (qualified-name "VehicleMasses::c::transmission"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleMasses::c::transmission"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleMasses::car"))) (target (node (document "d0") (qualified-name "VehicleMasses::CarPart"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleMasses::car"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleMasses::car::carParts"))) (target (node (document "d0") (qualified-name "VehicleMasses::CarPart"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleMasses::car::carParts"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "VehicleMasses::car::engine"))) (target (node (document "d0") (qualified-name "VehicleMasses::car::carParts"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleMasses::car::engine"))) (kind subsetting) (ordinal 1)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "VehicleMasses::car::transmission"))) (target (node (document "d0") (qualified-name "VehicleMasses::car::carParts"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleMasses::car::transmission"))) (kind subsetting) (ordinal 1)))
  )
  (evaluation
  )
)
~~~
