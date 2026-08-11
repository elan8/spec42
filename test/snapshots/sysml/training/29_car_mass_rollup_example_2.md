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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "29_car_mass_rollup_example_2.md"
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
        (range (start 9 20) (end 9 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 32) (end 11 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 24 16) (end 24 22))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "fac547af4e95f8ae2dd147de6dae6aa6afbdaf8143dbb6202eab769e387faed5") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Car Mass Rollup 1"))) (kind "package") (name "Car Mass Rollup 1") (declared-name "Car Mass Rollup 1") (range (start (line 0) (character 0)) (end (line 0) (character 675))))
    (element (id (node (document "d0") (qualified-name "Car Mass Rollup 1::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 32))) (parent (node (document "d0") (qualified-name "Car Mass Rollup 1"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 28))))))
    (element (id (node (document "d0") (qualified-name "Car Mass Rollup 1::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 31))) (parent (node (document "d0") (qualified-name "Car Mass Rollup 1"))) (authored (membership (kind Import) (visibility "private") (import (reference "MassRollup2::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 27))))))
    (element (id (node (document "d0") (qualified-name "Car Mass Rollup 1::CarPart"))) (kind "part def") (name "CarPart") (declared-name "CarPart") (range (start (line 4) (character 1)) (end (line 4) (character 74))) (parent (node (document "d0") (qualified-name "Car Mass Rollup 1"))) (authored (membership (kind Owning)) (relationships (specializes (reference "MassedThing") (range (start (line 4) (character 21)) (end (line 4) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "Car Mass Rollup 1::CarPart::serialNumber"))) (kind "attribute") (name "serialNumber") (declared-name "serialNumber") (range (start (line 5) (character 2)) (end (line 5) (character 33))) (parent (node (document "d0") (qualified-name "Car Mass Rollup 1::CarPart"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)) (typing (reference "String") (range (start (line 5) (character 26)) (end (line 5) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "Car Mass Rollup 1::c"))) (kind "part") (name "c") (declared-name "c") (range (start (line 25) (character 1)) (end (line 25) (character 199))) (parent (node (document "d0") (qualified-name "Car Mass Rollup 1"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "car") (range (start (line 25) (character 11)) (end (line 25) (character 14)))))))
    (element (id (node (document "d0") (qualified-name "Car Mass Rollup 1::c::engine"))) (kind "part") (name "engine") (range (start (line 27) (character 2)) (end (line 27) (character 62))) (parent (node (document "d0") (qualified-name "Car Mass Rollup 1::c"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "engine") (range (start (line 27) (character 11)) (end (line 27) (character 17)))))))
    (element (id (node (document "d0") (qualified-name "Car Mass Rollup 1::c::engine::simpleMass"))) (kind "attribute") (name "simpleMass") (declared-name "simpleMass") (range (start (line 28) (character 3)) (end (line 28) (character 38))) (parent (node (document "d0") (qualified-name "Car Mass Rollup 1::c::engine"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "simpleMass") (range (start (line 28) (character 17)) (end (line 28) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "Car Mass Rollup 1::c::simpleMass"))) (kind "attribute") (name "simpleMass") (declared-name "simpleMass") (range (start (line 26) (character 2)) (end (line 26) (character 38))) (parent (node (document "d0") (qualified-name "Car Mass Rollup 1::c"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "simpleMass") (range (start (line 26) (character 16)) (end (line 26) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "Car Mass Rollup 1::c::transmission"))) (kind "part") (name "transmission") (range (start (line 31) (character 2)) (end (line 31) (character 73))) (parent (node (document "d0") (qualified-name "Car Mass Rollup 1::c"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "transmission") (range (start (line 31) (character 17)) (end (line 31) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "Car Mass Rollup 1::c::transmission::simpleMass"))) (kind "attribute") (name "simpleMass") (declared-name "simpleMass") (range (start (line 32) (character 3)) (end (line 32) (character 37))) (parent (node (document "d0") (qualified-name "Car Mass Rollup 1::c::transmission"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "simpleMass") (range (start (line 32) (character 17)) (end (line 32) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "Car Mass Rollup 1::car"))) (kind "part") (name "car") (declared-name "car") (range (start (line 8) (character 1)) (end (line 8) (character 220))) (parent (node (document "d0") (qualified-name "Car Mass Rollup 1"))) (authored (membership (kind Feature)) (relationships (typing (reference "CarPart") (range (start (line 8) (character 11)) (end (line 8) (character 18)))) (subsetting (reference "compositeThing") (range (start (line 8) (character 22)) (end (line 8) (character 36)))))))
    (element (id (node (document "d0") (qualified-name "Car Mass Rollup 1::car::carParts"))) (kind "part") (name "carParts") (declared-name "carParts") (range (start (line 11) (character 2)) (end (line 11) (character 46))) (parent (node (document "d0") (qualified-name "Car Mass Rollup 1::car"))) (authored (membership (kind Feature)) (relationships (typing (reference "CarPart") (range (start (line 11) (character 17)) (end (line 11) (character 24)))) (redefinition (reference "subcomponents") (range (start (line 11) (character 32)) (end (line 11) (character 45)))))))
    (element (id (node (document "d0") (qualified-name "Car Mass Rollup 1::car::engine"))) (kind "part") (name "engine") (declared-name "engine") (range (start (line 13) (character 2)) (end (line 13) (character 40))) (parent (node (document "d0") (qualified-name "Car Mass Rollup 1::car"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "carParts") (range (start (line 13) (character 17)) (end (line 13) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "Car Mass Rollup 1::car::transmission"))) (kind "part") (name "transmission") (declared-name "transmission") (range (start (line 17) (character 2)) (end (line 17) (character 46))) (parent (node (document "d0") (qualified-name "Car Mass Rollup 1::car"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "carParts") (range (start (line 17) (character 23)) (end (line 17) (character 31)))))))
    (element (id (node (document "d0") (qualified-name "Car Mass Rollup 1::car::vin"))) (kind "attribute") (name "vin") (declared-name "vin") (range (start (line 9) (character 2)) (end (line 9) (character 33))) (parent (node (document "d0") (qualified-name "Car Mass Rollup 1::car"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "serialNumber") (range (start (line 9) (character 20)) (end (line 9) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "Car Mass Rollup 1::kg"))) (kind "import") (name "kg") (declared-name "kg") (range (start (line 24) (character 1)) (end (line 24) (character 23))) (parent (node (document "d0") (qualified-name "Car Mass Rollup 1"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::kg") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 24) (character 16)) (end (line 24) (character 22))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Car Mass Rollup 1::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 1) (character 16)) (end (line 1) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Car Mass Rollup 1::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "MassRollup2::*") (range (start (line 2) (character 16)) (end (line 2) (character 27))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Car Mass Rollup 1::CarPart"))) (kind specialization) (ordinal 0)) (authored-target "MassedThing") (range (start (line 4) (character 21)) (end (line 4) (character 32))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Car Mass Rollup 1::CarPart::serialNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Car Mass Rollup 1::CarPart::serialNumber"))) (kind featureTyping) (ordinal 1)) (authored-target "String") (range (start (line 5) (character 26)) (end (line 5) (character 32))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Car Mass Rollup 1::c"))) (kind subsetting) (ordinal 0)) (authored-target "car") (range (start (line 25) (character 11)) (end (line 25) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Car Mass Rollup 1::car")))))
    (reference (id (source (node (document "d0") (qualified-name "Car Mass Rollup 1::c::engine"))) (kind redefinition) (ordinal 0)) (authored-target "engine") (range (start (line 27) (character 11)) (end (line 27) (character 17))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Car Mass Rollup 1::c::engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Car Mass Rollup 1::c::engine::simpleMass"))) (kind redefinition) (ordinal 0)) (authored-target "simpleMass") (range (start (line 28) (character 17)) (end (line 28) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Car Mass Rollup 1::c::engine::simpleMass")))))
    (reference (id (source (node (document "d0") (qualified-name "Car Mass Rollup 1::c::simpleMass"))) (kind redefinition) (ordinal 0)) (authored-target "simpleMass") (range (start (line 26) (character 16)) (end (line 26) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Car Mass Rollup 1::c::simpleMass")))))
    (reference (id (source (node (document "d0") (qualified-name "Car Mass Rollup 1::c::transmission"))) (kind redefinition) (ordinal 0)) (authored-target "transmission") (range (start (line 31) (character 17)) (end (line 31) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Car Mass Rollup 1::c::transmission")))))
    (reference (id (source (node (document "d0") (qualified-name "Car Mass Rollup 1::c::transmission::simpleMass"))) (kind redefinition) (ordinal 0)) (authored-target "simpleMass") (range (start (line 32) (character 17)) (end (line 32) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Car Mass Rollup 1::c::transmission::simpleMass")))))
    (reference (id (source (node (document "d0") (qualified-name "Car Mass Rollup 1::car"))) (kind featureTyping) (ordinal 0)) (authored-target "CarPart") (range (start (line 8) (character 11)) (end (line 8) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Car Mass Rollup 1::CarPart")))))
    (reference (id (source (node (document "d0") (qualified-name "Car Mass Rollup 1::car"))) (kind subsetting) (ordinal 0)) (authored-target "compositeThing") (range (start (line 8) (character 22)) (end (line 8) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Car Mass Rollup 1::car::carParts"))) (kind featureTyping) (ordinal 0)) (authored-target "CarPart") (range (start (line 11) (character 17)) (end (line 11) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Car Mass Rollup 1::CarPart")))))
    (reference (id (source (node (document "d0") (qualified-name "Car Mass Rollup 1::car::carParts"))) (kind redefinition) (ordinal 0)) (authored-target "subcomponents") (range (start (line 11) (character 32)) (end (line 11) (character 45))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Car Mass Rollup 1::car::engine"))) (kind subsetting) (ordinal 0)) (authored-target "carParts") (range (start (line 13) (character 17)) (end (line 13) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Car Mass Rollup 1::car::carParts")))))
    (reference (id (source (node (document "d0") (qualified-name "Car Mass Rollup 1::car::transmission"))) (kind subsetting) (ordinal 0)) (authored-target "carParts") (range (start (line 17) (character 23)) (end (line 17) (character 31))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Car Mass Rollup 1::car::carParts")))))
    (reference (id (source (node (document "d0") (qualified-name "Car Mass Rollup 1::car::vin"))) (kind redefinition) (ordinal 0)) (authored-target "serialNumber") (range (start (line 9) (character 20)) (end (line 9) (character 32))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Car Mass Rollup 1::kg"))) (kind membershipImport) (ordinal 0)) (authored-target "SI::kg") (range (start (line 24) (character 16)) (end (line 24) (character 22))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Car Mass Rollup 1::c"))) (target (node (document "d0") (qualified-name "Car Mass Rollup 1::car"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Car Mass Rollup 1::c"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Car Mass Rollup 1::c::engine"))) (target (node (document "d0") (qualified-name "Car Mass Rollup 1::c::engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Car Mass Rollup 1::c::engine"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Car Mass Rollup 1::c::engine::simpleMass"))) (target (node (document "d0") (qualified-name "Car Mass Rollup 1::c::engine::simpleMass"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Car Mass Rollup 1::c::engine::simpleMass"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Car Mass Rollup 1::c::simpleMass"))) (target (node (document "d0") (qualified-name "Car Mass Rollup 1::c::simpleMass"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Car Mass Rollup 1::c::simpleMass"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Car Mass Rollup 1::c::transmission"))) (target (node (document "d0") (qualified-name "Car Mass Rollup 1::c::transmission"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Car Mass Rollup 1::c::transmission"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Car Mass Rollup 1::c::transmission::simpleMass"))) (target (node (document "d0") (qualified-name "Car Mass Rollup 1::c::transmission::simpleMass"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Car Mass Rollup 1::c::transmission::simpleMass"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Car Mass Rollup 1::car"))) (target (node (document "d0") (qualified-name "Car Mass Rollup 1::CarPart"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Car Mass Rollup 1::car"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Car Mass Rollup 1::car::carParts"))) (target (node (document "d0") (qualified-name "Car Mass Rollup 1::CarPart"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Car Mass Rollup 1::car::carParts"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Car Mass Rollup 1::car::engine"))) (target (node (document "d0") (qualified-name "Car Mass Rollup 1::car::carParts"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Car Mass Rollup 1::car::engine"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Car Mass Rollup 1::car::transmission"))) (target (node (document "d0") (qualified-name "Car Mass Rollup 1::car::carParts"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Car Mass Rollup 1::car::transmission"))) (kind subsetting) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Car Mass Rollup 1::c::engine::simpleMass")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "Car Mass Rollup 1::c::simpleMass")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "Car Mass Rollup 1::c::transmission::simpleMass")) (expression (status "unsupported") (error "declared expression form is not supported")))
  )
)
~~~
