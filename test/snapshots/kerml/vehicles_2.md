# META
~~~ini
description=KerML Mass Roll-up: Vehicles_2
type=file
~~~
# SOURCE
~~~kerml
package Vehicles_2 {
	private import ScalarValues::String;
	private import MassRollup_1::*;
	
	class CarPart specializes MassedThing {		
		feature serialNumber: String;
		feature m redefines mass;
		
		composite subparts: CarPart[0..*] redefines subcomponents;
	}
	
	feature vehicle: CarPart {	
		feature vin redefines serialNumber;
		
		composite engine: CarPart subsets subparts {
			//...
		}
		
		composite transmission: CarPart subsets subparts {
			//...
		}
	}
	
	// Example usage
	
	private import SI::*;
	feature v: vehicle {
		feature m redefines CarPart::m = 1000;
		composite engine redefines vehicle::engine {
			feature m redefines CarPart::m = 100;
		}
		composite transmission redefines vehicle::transmission {
			feature m redefines CarPart::m = 50;
		}
	}
	
	// v.totalMass evaluates to 1150.0
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "vehicles_2.md"
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
        (range (start 25 16) (end 25 18))
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
KwComposite,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwRedefines,Ident,Semicolon,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenCurly,
KwFeature,Ident,KwRedefines,Ident,Semicolon,
KwComposite,Ident,Colon,Ident,KwSubsets,Ident,OpenCurly,
LineComment,
CloseCurly,
KwComposite,Ident,Colon,Ident,KwSubsets,Ident,OpenCurly,
LineComment,
CloseCurly,
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
  (package_def 'Vehicles_2'
    (import_decl private 'ScalarValues::String')
    (import_decl private 'MassRollup_1::*')
    (class_def 'CarPart' :> 'MassedThing'
      (feature_def 'serialNumber' : 'String')
      (feature_def 'm' :>> 'mass')
      (feature_def composite 'subparts' : 'CarPart' multiplicity :>> 'subcomponents'))
    (feature_def 'vehicle' : 'CarPart'
      (feature_def 'vin' :>> 'serialNumber')
      (feature_def composite 'engine' : 'CarPart' :> 'subparts'
        (line_comment))
      (feature_def composite 'transmission' : 'CarPart' :> 'subparts'
        (line_comment)))
    (line_comment)
    (import_decl private 'SI::*')
    (feature_def 'v' : 'vehicle'
      (feature_def 'm' :>> 'CarPart::m' value)
      (feature_def composite 'engine' :>> 'vehicle::engine'
        (feature_def 'm' :>> 'CarPart::m' value))
      (feature_def composite 'transmission' :>> 'vehicle::transmission'
        (feature_def 'm' :>> 'CarPart::m' value)))
    (line_comment)))
~~~
# EXPECTED
~~~
semantic.redefinition_featuring_type_overlap
semantic.redefinition_featuring_type_overlap
semantic.unresolved_name 'MassedThing'
semantic.unresolved_name 'String'
semantic.unresolved_name 'mass'
semantic.unresolved_name 'subcomponents'
~~~
# PROBLEMS
~~~
semantic.redefinition_featuring_type_overlap
semantic.redefinition_featuring_type_overlap
semantic.unresolved_name 'MassedThing'
semantic.unresolved_name 'String'
semantic.unresolved_name 'mass'
semantic.unresolved_name 'subcomponents'
~~~
# FORMAT
~~~sysml
package Vehicles_2 {
	private import ScalarValues::String;
	private import MassRollup_1::*;
	
	class CarPart specializes MassedThing {		
		feature serialNumber: String;
		feature m redefines mass;
		
		composite subparts: CarPart[0..*] redefines subcomponents;
	}
	
	feature vehicle: CarPart {	
		feature vin redefines serialNumber;
		
		composite engine: CarPart subsets subparts {
			//...
		}
		
		composite transmission: CarPart subsets subparts {
			//...
		}
	}
	
	// Example usage
	
	private import SI::*;
	feature v: vehicle {
		feature m redefines CarPart::m = 1000;
		composite engine redefines vehicle::engine {
			feature m redefines CarPart::m = 100;
		}
		composite transmission redefines vehicle::transmission {
			feature m redefines CarPart::m = 50;
		}
	}
	
	// v.totalMass evaluates to 1150.0
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "c4b6b8a2932c8d57b2390e8d872817ed56b6214751812e0f65db10417b933645") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Vehicles_2"))) (kind "package") (name "Vehicles_2") (declared-name "Vehicles_2") (range (start (line 0) (character 0)) (end (line 0) (character 813))))
    (element (id (node (document "d0") (qualified-name "Vehicles_2::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 32))) (parent (node (document "d0") (qualified-name "Vehicles_2"))) (authored (membership (kind Import) (visibility "private") (import (reference "MassRollup_1::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 28))))))
    (element (id (node (document "d0") (qualified-name "Vehicles_2::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 25) (character 1)) (end (line 25) (character 22))) (parent (node (document "d0") (qualified-name "Vehicles_2"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 25) (character 16)) (end (line 25) (character 18))))))
    (element (id (node (document "d0") (qualified-name "Vehicles_2::CarPart"))) (kind "classifier decl") (name "CarPart") (declared-name "CarPart") (range (start (line 4) (character 1)) (end (line 4) (character 169))) (parent (node (document "d0") (qualified-name "Vehicles_2"))))
    (element (id (node (document "d0") (qualified-name "Vehicles_2::String"))) (kind "import") (name "String") (declared-name "String") (range (start (line 1) (character 1)) (end (line 1) (character 37))) (parent (node (document "d0") (qualified-name "Vehicles_2"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::String") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 36))))))
    (element (id (node (document "d0") (qualified-name "Vehicles_2::v"))) (kind "feature decl") (name "v") (declared-name "v") (range (start (line 26) (character 1)) (end (line 26) (character 260))) (parent (node (document "d0") (qualified-name "Vehicles_2"))))
    (element (id (node (document "d0") (qualified-name "Vehicles_2::vehicle"))) (kind "feature decl") (name "vehicle") (declared-name "vehicle") (range (start (line 11) (character 1)) (end (line 11) (character 201))) (parent (node (document "d0") (qualified-name "Vehicles_2"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Vehicles_2::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "MassRollup_1::*") (range (start (line 2) (character 16)) (end (line 2) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicles_2::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (range (start (line 25) (character 16)) (end (line 25) (character 18))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicles_2::String"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::String") (range (start (line 1) (character 16)) (end (line 1) (character 36))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
