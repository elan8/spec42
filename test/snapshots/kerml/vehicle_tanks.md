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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "VehicleTanks"))) (name "VehicleTanks") (declared-name "VehicleTanks")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "VehicleTanks::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "VehicleTanks::*#import"))) (name "*") (declared-name "*"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "VehicleTanks::Tank"))) (name "Tank") (declared-name "Tank"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "VehicleTanks::V6Engine"))) (name "V6Engine") (declared-name "V6Engine"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "VehicleTanks::Vehicle"))) (name "Vehicle") (declared-name "Vehicle"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "VehicleTanks::Vehicle1"))) (name "Vehicle1") (declared-name "Vehicle1"))
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
  (document "kerml/vehicle_tanks.md"
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
