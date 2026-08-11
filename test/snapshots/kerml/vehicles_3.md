# META
~~~ini
description=KerML Mass Roll-up: Vehicles_3
type=file
~~~
# SOURCE
~~~kerml
package Vehicles_3 {
	private import ScalarValues::*;
	private import MassRollup_2::*;
	
	class CarPart specializes MassedThing {			
		feature serialNumber: String;
		feature m redefines MassedThing::mass;
		
		feature subparts redefines carParts;	
	}
	
	composite feature carParts: CarPart[0..*] subsets massedThings;
	
	feature vehicle subsets carParts {	
		feature vin redefines serialNumber;
		
		feature redefines engine;
		feature redefines transmission;
	}
	
	composite feature engine subsets carParts {
		//...
	}
	
	composite feature transmission subsets carParts {
		//...
	}

	// Example usage
	
	private import SI::*;
	feature v: vehicle {
		feature m redefines CarPart::m = 1000;
		composite :>> engine = e;
		composite :>> transmission = t;
	}
	
	feature e :> engine {
		feature m redefines CarPart::m = 100;
	}
	
	feature t :> transmission {
		feature m redefines CarPart::m = 50;
	}
	
	// v.totalMass evaluates to 1150.0
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "vehicles_3.md"
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
        (range (start 2 16) (end 2 28))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 11 1) (end 11 354))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 30 16) (end 30 18))
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
KwClass,Ident,KwSpecializes,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,Semicolon,
KwFeature,Ident,KwRedefines,Ident,ColonColon,Ident,Semicolon,
KwFeature,Ident,KwRedefines,Ident,Semicolon,
CloseCurly,
KwComposite,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,Semicolon,
KwFeature,Ident,KwSubsets,Ident,OpenCurly,
KwFeature,Ident,KwRedefines,Ident,Semicolon,
KwFeature,KwRedefines,Ident,Semicolon,
KwFeature,KwRedefines,Ident,Semicolon,
CloseCurly,
KwComposite,KwFeature,Ident,KwSubsets,Ident,OpenCurly,
LineComment,
CloseCurly,
KwComposite,KwFeature,Ident,KwSubsets,Ident,OpenCurly,
LineComment,
CloseCurly,
LineComment,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwFeature,Ident,Colon,Ident,OpenCurly,
KwFeature,Ident,KwRedefines,Ident,ColonColon,Ident,Eq,DecimalValue,Semicolon,
KwComposite,ColonGtGt,Ident,Eq,Ident,Semicolon,
KwComposite,ColonGtGt,Ident,Eq,Ident,Semicolon,
CloseCurly,
KwFeature,Ident,ColonGt,Ident,OpenCurly,
KwFeature,Ident,KwRedefines,Ident,ColonColon,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
KwFeature,Ident,ColonGt,Ident,OpenCurly,
KwFeature,Ident,KwRedefines,Ident,ColonColon,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
LineComment,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'Vehicles_3'
    (import_decl private 'ScalarValues::*')
    (import_decl private 'MassRollup_2::*')
    (class_def 'CarPart' :> 'MassedThing'
      (feature_def 'serialNumber' : 'String')
      (feature_def 'm' :>> 'MassedThing::mass')
      (feature_def 'subparts' :>> 'carParts'))
    (feature_def composite 'carParts' : 'CarPart' multiplicity :> 'massedThings')
    (feature_def 'vehicle' :> 'carParts'
      (feature_def 'vin' :>> 'serialNumber')
      (feature_def :>> 'engine')
      (feature_def :>> 'transmission'))
    (feature_def composite 'engine' :> 'carParts'
      (line_comment))
    (feature_def composite 'transmission' :> 'carParts'
      (line_comment))
    (line_comment)
    (import_decl private 'SI::*')
    (feature_def 'v' : 'vehicle'
      (feature_def 'm' :>> 'CarPart::m' value)
      (feature_def composite :>> 'engine' value)
      (feature_def composite :>> 'transmission' value))
    (feature_def 'e' :> 'engine'
      (feature_def 'm' :>> 'CarPart::m' value))
    (feature_def 't' :> 'transmission'
      (feature_def 'm' :>> 'CarPart::m' value))
    (line_comment)))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'MassedThing'
semantic.unresolved_name 'String'
semantic.unresolved_name 'MassedThing::mass'
semantic.unresolved_name 'massedThings'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'MassedThing'
semantic.unresolved_name 'String'
semantic.unresolved_name 'MassedThing::mass'
semantic.unresolved_name 'massedThings'
~~~
# FORMAT
~~~sysml
package Vehicles_3 {
	private import ScalarValues::*;
	private import MassRollup_2::*;
	
	class CarPart specializes MassedThing {			
		feature serialNumber: String;
		feature m redefines MassedThing::mass;
		
		feature subparts redefines carParts;	
	}
	
	composite feature carParts: CarPart[0..*] subsets massedThings;
	
	feature vehicle subsets carParts {	
		feature vin redefines serialNumber;
		
		feature redefines engine;
		feature redefines transmission;
	}
	
	composite feature engine subsets carParts {
		//...
	}
	
	composite feature transmission subsets carParts {
		//...
	}

	// Example usage
	
	private import SI::*;
	feature v: vehicle {
		feature m redefines CarPart::m = 1000;
		composite :>> engine = e;
		composite :>> transmission = t;
	}
	
	feature e :> engine {
		feature m redefines CarPart::m = 100;
	}
	
	feature t :> transmission {
		feature m redefines CarPart::m = 50;
	}
	
	// v.totalMass evaluates to 1150.0
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "53074dc88e83c311a26d3392ceb8d69868fa1a9b5d7c0f272194136eae285c84") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Vehicles_3"))) (kind "package") (name "Vehicles_3") (declared-name "Vehicles_3") (range (start (line 0) (character 0)) (end (line 0) (character 938))))
    (element (id (node (document "d0") (qualified-name "Vehicles_3::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 32))) (parent (node (document "d0") (qualified-name "Vehicles_3"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 28))))))
    (element (id (node (document "d0") (qualified-name "Vehicles_3::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 32))) (parent (node (document "d0") (qualified-name "Vehicles_3"))) (authored (membership (kind Import) (visibility "private") (import (reference "MassRollup_2::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 28))))))
    (element (id (node (document "d0") (qualified-name "Vehicles_3::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 30) (character 1)) (end (line 30) (character 22))) (parent (node (document "d0") (qualified-name "Vehicles_3"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 30) (character 16)) (end (line 30) (character 18))))))
    (element (id (node (document "d0") (qualified-name "Vehicles_3::CarPart"))) (kind "classifier decl") (name "CarPart") (declared-name "CarPart") (range (start (line 4) (character 1)) (end (line 4) (character 162))) (parent (node (document "d0") (qualified-name "Vehicles_3"))))
    (element (id (node (document "d0") (qualified-name "Vehicles_3::e"))) (kind "feature decl") (name "e") (declared-name "e") (range (start (line 37) (character 1)) (end (line 37) (character 65))) (parent (node (document "d0") (qualified-name "Vehicles_3"))))
    (element (id (node (document "d0") (qualified-name "Vehicles_3::t"))) (kind "feature decl") (name "t") (declared-name "t") (range (start (line 41) (character 1)) (end (line 41) (character 70))) (parent (node (document "d0") (qualified-name "Vehicles_3"))))
    (element (id (node (document "d0") (qualified-name "Vehicles_3::v"))) (kind "feature decl") (name "v") (declared-name "v") (range (start (line 31) (character 1)) (end (line 31) (character 127))) (parent (node (document "d0") (qualified-name "Vehicles_3"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Vehicles_3::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 1) (character 16)) (end (line 1) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicles_3::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "MassRollup_2::*") (range (start (line 2) (character 16)) (end (line 2) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicles_3::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (range (start (line 30) (character 16)) (end (line 30) (character 18))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
