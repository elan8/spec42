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
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions"))) (kind "package") (name "VehicleDefinitions") (declared-name "VehicleDefinitions") (range (start (line 0) (character 0)) (end (line 0) (character 639))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::Axle"))) (kind "classifier decl") (name "Axle") (declared-name "Axle") (range (start (line 12) (character 1)) (end (line 12) (character 12))) (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::AxleAssembly"))) (kind "classifier decl") (name "AxleAssembly") (declared-name "AxleAssembly") (range (start (line 11) (character 1)) (end (line 11) (character 20))) (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::AxleMountIF"))) (kind "classifier decl") (name "AxleMountIF") (declared-name "AxleMountIF") (range (start (line 24) (character 1)) (end (line 24) (character 70))) (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::DriveIF"))) (kind "classifier decl") (name "DriveIF") (declared-name "DriveIF") (range (start (line 20) (character 1)) (end (line 20) (character 58))) (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::Lugbolt"))) (kind "classifier decl") (name "Lugbolt") (declared-name "Lugbolt") (range (start (line 14) (character 1)) (end (line 14) (character 63))) (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::Mounting"))) (kind "kermlDecl") (name "Mounting") (declared-name "Mounting") (range (start (line 34) (character 1)) (end (line 34) (character 127))) (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::Transmission"))) (kind "classifier decl") (name "Transmission") (declared-name "Transmission") (range (start (line 10) (character 1)) (end (line 10) (character 20))) (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::Vehicle"))) (kind "classifier decl") (name "Vehicle") (declared-name "Vehicle") (range (start (line 9) (character 1)) (end (line 9) (character 15))) (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::Wheel"))) (kind "classifier decl") (name "Wheel") (declared-name "Wheel") (range (start (line 13) (character 1)) (end (line 13) (character 13))) (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::WheelHubIF"))) (kind "classifier decl") (name "WheelHubIF") (declared-name "WheelHubIF") (range (start (line 28) (character 1)) (end (line 28) (character 64))) (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 639))) (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
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
