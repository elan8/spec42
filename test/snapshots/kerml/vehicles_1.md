# META
~~~ini
description=KerML Mass Roll-up: Vehicles_1
type=file
~~~
# SOURCE
~~~kerml
package Vehicles_1 {
	private import ScalarValues::String;
	private import MassRollup_1::*;

	class Vehicle specializes MassedThing {
		feature vin: String;
		feature m redefines mass;
	
		composite engine: Engine subsets subcomponents;
		composite transmission: Transmission subsets subcomponents;
	}
	
	class Engine specializes MassedThing {
		feature serialNumber: String;
		feature m redefines mass;
		
		// ...
	}
	
	class Transmission specializes MassedThing {
		feature serialNumber: String;
		feature m redefines mass;
		
		// ...
	}
	
	// Example usage
	
	private import SI::*;
	feature v: Vehicle {
		feature m redefines Vehicle::m = 1000;
		composite engine redefines Vehicle::engine {
			feature m redefines Engine::m = 100;
		}
		composite transmission redefines Vehicle::transmission {
			feature m redefines Transmission::m = 50;
		}
	}

	// v.totalMass evaluates to 1150.0
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "vehicles_1.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 28 16) (end 28 18))
      )
    )
  )
)
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwClass,Ident,KwSpecializes,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,Semicolon,
KwFeature,Ident,KwRedefines,Ident,Semicolon,
KwComposite,Ident,Colon,Ident,KwSubsets,Ident,Semicolon,
KwComposite,Ident,Colon,Ident,KwSubsets,Ident,Semicolon,
CloseCurly,
KwClass,Ident,KwSpecializes,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,Semicolon,
KwFeature,Ident,KwRedefines,Ident,Semicolon,
LineComment,
CloseCurly,
KwClass,Ident,KwSpecializes,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,Semicolon,
KwFeature,Ident,KwRedefines,Ident,Semicolon,
LineComment,
CloseCurly,
LineComment,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwFeature,Ident,Colon,Ident,OpenCurly,
KwFeature,Ident,KwRedefines,Ident,ColonColon,Ident,Eq,DecimalValue,Semicolon,
KwComposite,Ident,KwRedefines,Ident,ColonColon,Ident,OpenCurly,
KwFeature,Ident,KwRedefines,Ident,ColonColon,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
KwComposite,Ident,KwRedefines,Ident,ColonColon,Ident,OpenCurly,
KwFeature,Ident,KwRedefines,Ident,ColonColon,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
CloseCurly,
LineComment,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'Vehicles_1'
    (import_decl private 'ScalarValues::String')
    (import_decl private 'MassRollup_1::*')
    (class_def 'Vehicle' :> 'MassedThing'
      (feature_def 'vin' : 'String')
      (feature_def 'm' :>> 'mass')
      (feature_def composite 'engine' : 'Engine' :> 'subcomponents')
      (feature_def composite 'transmission' : 'Transmission' :> 'subcomponents'))
    (class_def 'Engine' :> 'MassedThing'
      (feature_def 'serialNumber' : 'String')
      (feature_def 'm' :>> 'mass')
      (line_comment))
    (class_def 'Transmission' :> 'MassedThing'
      (feature_def 'serialNumber' : 'String')
      (feature_def 'm' :>> 'mass')
      (line_comment))
    (line_comment)
    (import_decl private 'SI::*')
    (feature_def 'v' : 'Vehicle'
      (feature_def 'm' :>> 'Vehicle::m' value)
      (feature_def composite 'engine' :>> 'Vehicle::engine'
        (feature_def 'm' :>> 'Engine::m' value))
      (feature_def composite 'transmission' :>> 'Vehicle::transmission'
        (feature_def 'm' :>> 'Transmission::m' value)))
    (line_comment)))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'MassedThing'
semantic.unresolved_name 'String'
semantic.unresolved_name 'mass'
semantic.unresolved_name 'subcomponents'
semantic.unresolved_name 'subcomponents'
semantic.unresolved_name 'MassedThing'
semantic.unresolved_name 'String'
semantic.unresolved_name 'mass'
semantic.unresolved_name 'MassedThing'
semantic.unresolved_name 'String'
semantic.unresolved_name 'mass'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'MassedThing'
semantic.unresolved_name 'String'
semantic.unresolved_name 'mass'
semantic.unresolved_name 'subcomponents'
semantic.unresolved_name 'subcomponents'
semantic.unresolved_name 'MassedThing'
semantic.unresolved_name 'String'
semantic.unresolved_name 'mass'
semantic.unresolved_name 'MassedThing'
semantic.unresolved_name 'String'
semantic.unresolved_name 'mass'
~~~
# FORMAT
~~~sysml
package Vehicles_1 {
	private import ScalarValues::String;
	private import MassRollup_1::*;

	class Vehicle specializes MassedThing {
		feature vin: String;
		feature m redefines mass;
	
		composite engine: Engine subsets subcomponents;
		composite transmission: Transmission subsets subcomponents;
	}
	
	class Engine specializes MassedThing {
		feature serialNumber: String;
		feature m redefines mass;
		
		// ...
	}
	
	class Transmission specializes MassedThing {
		feature serialNumber: String;
		feature m redefines mass;
		
		// ...
	}
	
	// Example usage
	
	private import SI::*;
	feature v: Vehicle {
		feature m redefines Vehicle::m = 1000;
		composite engine redefines Vehicle::engine {
			feature m redefines Engine::m = 100;
		}
		composite transmission redefines Vehicle::transmission {
			feature m redefines Transmission::m = 50;
		}
	}

	// v.totalMass evaluates to 1150.0
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "02f511967dbd700da35471f9ae3ce58e2885ed4093bd5c2cf900f5359e876ef6") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Vehicles_1"))) (kind "package") (name "Vehicles_1") (declared-name "Vehicles_1") (range (start (line 0) (character 0)) (end (line 0) (character 890))))
    (element (id (node (document "d0") (qualified-name "Vehicles_1::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 32))) (parent (node (document "d0") (qualified-name "Vehicles_1"))) (authored (membership (kind Import) (visibility "private") (import (reference "MassRollup_1::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 28))))))
    (element (id (node (document "d0") (qualified-name "Vehicles_1::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 28) (character 1)) (end (line 28) (character 22))) (parent (node (document "d0") (qualified-name "Vehicles_1"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 28) (character 16)) (end (line 28) (character 18))))))
    (element (id (node (document "d0") (qualified-name "Vehicles_1::Engine"))) (kind "classifier decl") (name "Engine") (declared-name "Engine") (range (start (line 12) (character 1)) (end (line 12) (character 114))) (parent (node (document "d0") (qualified-name "Vehicles_1"))))
    (element (id (node (document "d0") (qualified-name "Vehicles_1::String"))) (kind "import") (name "String") (declared-name "String") (range (start (line 1) (character 1)) (end (line 1) (character 37))) (parent (node (document "d0") (qualified-name "Vehicles_1"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::String") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 36))))))
    (element (id (node (document "d0") (qualified-name "Vehicles_1::Transmission"))) (kind "classifier decl") (name "Transmission") (declared-name "Transmission") (range (start (line 19) (character 1)) (end (line 19) (character 120))) (parent (node (document "d0") (qualified-name "Vehicles_1"))))
    (element (id (node (document "d0") (qualified-name "Vehicles_1::Vehicle"))) (kind "classifier decl") (name "Vehicle") (declared-name "Vehicle") (range (start (line 4) (character 1)) (end (line 4) (character 208))) (parent (node (document "d0") (qualified-name "Vehicles_1"))))
    (element (id (node (document "d0") (qualified-name "Vehicles_1::v"))) (kind "feature decl") (name "v") (declared-name "v") (range (start (line 29) (character 1)) (end (line 29) (character 264))) (parent (node (document "d0") (qualified-name "Vehicles_1"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Vehicles_1::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "MassRollup_1::*") (range (start (line 2) (character 16)) (end (line 2) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicles_1::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (range (start (line 28) (character 16)) (end (line 28) (character 18))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicles_1::String"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::String") (range (start (line 1) (character 16)) (end (line 1) (character 36))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
