# META
~~~ini
description=KerML Vehicle: VehicleDefinitions
type=file
~~~
# SOURCE
~~~kerml
package VehicleDefinitions {
	doc
	/*
	 * Example vehicle definitions model.
	 */

	
	/* BLOCKS */
	
	class Vehicle;	
	class Transmission;	
	class AxleAssembly;
	class Axle;	
	class Wheel;
	class Lugbolt {
		tighteningTorque[1] : ScalarValues::Real;
	}
	
	/* INTERFACE BLOCKS */
	
	class DriveIF { 
		in driveTorque: ScalarValues::Real;
	}
	
	class AxleMountIF { 
		out transferredTorque : ScalarValues::Real;
	}
	
	class WheelHubIF { 
		in appliedTorque : ScalarValues::Real;
	}
	
	/* ASSOCIATION BLOCKS */
	
	assoc Mounting {
		doc
		/*
		 *  mounting a Wheel to an Axle.
		 */
	
		end axleMount: AxleMountIF;
		end hub: WheelHubIF;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "vehicle_definitions.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package VehicleDefinitions {
	doc
	/*
	 * Example vehicle definitions model.
	 */

	
	/* BLOCKS */
	
	class Vehicle;	
	class Transmission;	
	class AxleAssembly;
	class Axle;	
	class Wheel;
	class Lugbolt {
		tighteningTorque[1] : ScalarValues::Real;
	}
	
	/* INTERFACE BLOCKS */
	
	class DriveIF { 
		in driveTorque: ScalarValues::Real;
	}
	
	class AxleMountIF { 
		out transferredTorque : ScalarValues::Real;
	}
	
	class WheelHubIF { 
		in appliedTorque : ScalarValues::Real;
	}
	
	/* ASSOCIATION BLOCKS */
	
	assoc Mounting {
		doc
		/*
		 *  mounting a Wheel to an Axle.
		 */
	
		end axleMount: AxleMountIF;
		end hub: WheelHubIF;
	}
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "1652e7d6af92c08caeabfd71905dad0a96d0fbca10acc71ca53ca44f68c9efab") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions"))) (kind "package") (name "VehicleDefinitions") (declared-name "VehicleDefinitions"))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::Axle"))) (kind "classifier decl") (name "Axle") (declared-name "Axle") (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::AxleAssembly"))) (kind "classifier decl") (name "AxleAssembly") (declared-name "AxleAssembly") (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::AxleMountIF"))) (kind "classifier decl") (name "AxleMountIF") (declared-name "AxleMountIF") (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::DriveIF"))) (kind "classifier decl") (name "DriveIF") (declared-name "DriveIF") (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::Lugbolt"))) (kind "classifier decl") (name "Lugbolt") (declared-name "Lugbolt") (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::Mounting"))) (kind "kermlDecl") (name "Mounting") (declared-name "Mounting") (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::Transmission"))) (kind "classifier decl") (name "Transmission") (declared-name "Transmission") (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::Vehicle"))) (kind "classifier decl") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::Wheel"))) (kind "classifier decl") (name "Wheel") (declared-name "Wheel") (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::WheelHubIF"))) (kind "classifier decl") (name "WheelHubIF") (declared-name "WheelHubIF") (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
  )
  (references
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
)
~~~
