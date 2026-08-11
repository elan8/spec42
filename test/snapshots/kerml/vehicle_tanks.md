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
    (element (id (node (document "d0") (qualified-name "VehicleTanks"))) (kind "package") (name "VehicleTanks") (declared-name "VehicleTanks"))
    (element (id (node (document "d0") (qualified-name "VehicleTanks::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "VehicleTanks"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "VehicleTanks::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "VehicleTanks"))) (authored (membership (kind Import) (visibility "private") (import (reference "RealFunctions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "VehicleTanks::Tank"))) (kind "classifier decl") (name "Tank") (declared-name "Tank") (parent (node (document "d0") (qualified-name "VehicleTanks"))))
    (element (id (node (document "d0") (qualified-name "VehicleTanks::V6Engine"))) (kind "classifier decl") (name "V6Engine") (declared-name "V6Engine") (parent (node (document "d0") (qualified-name "VehicleTanks"))))
    (element (id (node (document "d0") (qualified-name "VehicleTanks::Vehicle"))) (kind "classifier decl") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "VehicleTanks"))))
    (element (id (node (document "d0") (qualified-name "VehicleTanks::Vehicle1"))) (kind "classifier decl") (name "Vehicle1") (declared-name "Vehicle1") (parent (node (document "d0") (qualified-name "VehicleTanks"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "VehicleTanks::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleTanks::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "RealFunctions::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 1 16) (end 1 28)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "VehicleTanks::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 1 16) (end 1 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 2 16) (end 2 29)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "VehicleTanks::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "RealFunctions::*")
        (range (start 2 16) (end 2 29))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
