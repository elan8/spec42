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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Vehicles_3"))) (name "Vehicles_3") (declared-name "Vehicles_3")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Vehicles_3::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Vehicles_3::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Vehicles_3::*#import2"))) (name "*") (declared-name "*"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Vehicles_3::CarPart"))) (name "CarPart") (declared-name "CarPart"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "Vehicles_3::e"))) (name "e") (declared-name "e"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "Vehicles_3::t"))) (name "t") (declared-name "t"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "Vehicles_3::v"))) (name "v") (declared-name "v"))
      )
    )
  )
  (relationships
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
  (document "kerml/vehicles_3.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 1) (end 1 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 1) (end 2 32))
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
        (range (start 30 1) (end 30 22))
      )
    )
  )
)
~~~
