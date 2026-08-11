# META
~~~ini
description=KerML Named Collection Members: VehicleTanks
type=file
~~~
# SOURCE
~~~kerml
package VehicleTanks {
	private import ScalarValues::*;
	private import RealFunctions::*;
	
	class V6Engine;
	
	class Tank {
		feature capacity: Real;
	}
	
	class Vehicle {
		composite tanks: Tank[1..*] ordered;
		
		feature fuelCapacity: Real = sum(tanks.capacity);
	}
	
	class Vehicle1 specializes Vehicle {
		composite tanks: Tank[4] ordered redefines Vehicle::tanks {
			feature main1[1] subsets tanks = tanks#(1);
			feature main2[1] subsets tanks = tanks#(2);
			feature aux1[1] subsets tanks = tanks#(3);
			feature aux2[1] subsets tanks = tanks#(4);
		}
		
		composite eng: V6Engine;
		
		connector eng to tanks.main1;
		connector tanks.main1 to tanks.aux1;
		
		connector eng to tanks.main2;
		connector tanks.main2 to tanks.aux2;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "vehicle_tanks.md"
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
        (range (start 2 16) (end 2 29))
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
KwClass,Ident,Semicolon,
KwClass,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwClass,Ident,OpenCurly,
KwComposite,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,Semicolon,
KwFeature,Ident,Colon,Ident,Eq,Ident,OpenParen,Ident,Dot,Ident,CloseParen,Semicolon,
CloseCurly,
KwClass,Ident,KwSpecializes,Ident,OpenCurly,
KwComposite,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwOrdered,KwRedefines,Ident,ColonColon,Ident,OpenCurly,
KwFeature,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,Semicolon,
KwFeature,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,Semicolon,
KwFeature,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,Semicolon,
KwFeature,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,Semicolon,
CloseCurly,
KwComposite,Ident,Colon,Ident,Semicolon,
KwConnector,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwConnector,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwConnector,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwConnector,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'VehicleTanks'
    (import_decl private 'ScalarValues::*')
    (import_decl private 'RealFunctions::*')
    (class_def 'V6Engine')
    (class_def 'Tank'
      (feature_def 'capacity' : 'Real'))
    (class_def 'Vehicle'
      (feature_def composite 'tanks' : 'Tank' multiplicity ordered)
      (feature_def 'fuelCapacity' : 'Real' value))
    (class_def 'Vehicle1' :> 'Vehicle'
      (feature_def composite 'tanks' : 'Tank' multiplicity :>> 'Vehicle::tanks' ordered
        (feature_def 'main1' multiplicity :> 'tanks' value)
        (feature_def 'main2' multiplicity :> 'tanks' value)
        (feature_def 'aux1' multiplicity :> 'tanks' value)
        (feature_def 'aux2' multiplicity :> 'tanks' value))
      (feature_def composite 'eng' : 'V6Engine')
      (connector_def
        (connector_end)
        (connector_end))
      (connector_def
        (connector_end)
        (connector_end))
      (connector_def
        (connector_end)
        (connector_end))
      (connector_def
        (connector_end)
        (connector_end)))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
~~~
# FORMAT
~~~sysml
package VehicleTanks {
	private import ScalarValues::*;
	private import RealFunctions::*;
	
	class V6Engine;
	
	class Tank {
		feature capacity: Real;
	}
	
	class Vehicle {
		composite tanks: Tank[1..*] ordered;
		
		feature fuelCapacity: Real = sum(tanks.capacity);
	}
	
	class Vehicle1 specializes Vehicle {
		composite tanks: Tank[4] ordered redefines Vehicle::tanks {
			feature main1[1] subsets tanks = tanks#(1);
			feature main2[1] subsets tanks = tanks#(2);
			feature aux1[1] subsets tanks = tanks#(3);
			feature aux2[1] subsets tanks = tanks#(4);
		}
		
		composite eng: V6Engine;
		
		connector eng to tanks.main1;
		connector tanks.main1 to tanks.aux1;
		
		connector eng to tanks.main2;
		connector tanks.main2 to tanks.aux2;
	}
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "cc8383cdf1b30e3f50c79173da128515ca4ca44834e970d864f6df74d57af57d") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "VehicleTanks"))) (kind "package") (name "VehicleTanks") (declared-name "VehicleTanks") (range (start (line 0) (character 0)) (end (line 0) (character 744))))
    (element (id (node (document "d0") (qualified-name "VehicleTanks::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 32))) (parent (node (document "d0") (qualified-name "VehicleTanks"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 28))))))
    (element (id (node (document "d0") (qualified-name "VehicleTanks::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 33))) (parent (node (document "d0") (qualified-name "VehicleTanks"))) (authored (membership (kind Import) (visibility "private") (import (reference "RealFunctions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 29))))))
    (element (id (node (document "d0") (qualified-name "VehicleTanks::Tank"))) (kind "classifier decl") (name "Tank") (declared-name "Tank") (range (start (line 6) (character 1)) (end (line 6) (character 42))) (parent (node (document "d0") (qualified-name "VehicleTanks"))))
    (element (id (node (document "d0") (qualified-name "VehicleTanks::V6Engine"))) (kind "classifier decl") (name "V6Engine") (declared-name "V6Engine") (range (start (line 4) (character 1)) (end (line 4) (character 16))) (parent (node (document "d0") (qualified-name "VehicleTanks"))))
    (element (id (node (document "d0") (qualified-name "VehicleTanks::Vehicle"))) (kind "classifier decl") (name "Vehicle") (declared-name "Vehicle") (range (start (line 10) (character 1)) (end (line 10) (character 113))) (parent (node (document "d0") (qualified-name "VehicleTanks"))))
    (element (id (node (document "d0") (qualified-name "VehicleTanks::Vehicle1"))) (kind "classifier decl") (name "Vehicle1") (declared-name "Vehicle1") (range (start (line 16) (character 1)) (end (line 16) (character 470))) (parent (node (document "d0") (qualified-name "VehicleTanks"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "VehicleTanks::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 1) (character 16)) (end (line 1) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleTanks::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "RealFunctions::*") (range (start (line 2) (character 16)) (end (line 2) (character 29))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
