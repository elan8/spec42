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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
RegularComment,
KwClass,Ident,Semicolon,
KwClass,Ident,Semicolon,
KwClass,Ident,Semicolon,
KwClass,Ident,Semicolon,
KwClass,Ident,Semicolon,
KwClass,Ident,OpenCurly,
Ident,OpenSquare,DecimalValue,CloseSquare,Colon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
RegularComment,
KwClass,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwClass,Ident,OpenCurly,
KwOut,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwClass,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
RegularComment,
KwAssoc,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEnd,Ident,Colon,Ident,Semicolon,
KwEnd,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'VehicleDefinitions'
    (documentation)
    (comment)
    (class_def 'Vehicle')
    (class_def 'Transmission')
    (class_def 'AxleAssembly')
    (class_def 'Axle')
    (class_def 'Wheel')
    (class_def 'Lugbolt'
      (feature_def 'tighteningTorque' multiplicity : 'ScalarValues::Real'))
    (comment)
    (class_def 'DriveIF'
      (feature_def in 'driveTorque' : 'ScalarValues::Real'))
    (class_def 'AxleMountIF'
      (feature_def out 'transferredTorque' : 'ScalarValues::Real'))
    (class_def 'WheelHubIF'
      (feature_def in 'appliedTorque' : 'ScalarValues::Real'))
    (comment)
    (association_def 'Mounting'
      (documentation)
      (feature_def end 'axleMount' : 'AxleMountIF')
      (feature_def end 'hub' : 'WheelHubIF'))))
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
# EXPECTED
~~~
semantic.unresolved_name 'ScalarValues::Real'
semantic.unresolved_name 'ScalarValues::Real'
semantic.unresolved_name 'ScalarValues::Real'
semantic.unresolved_name 'ScalarValues::Real'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'ScalarValues::Real'
semantic.unresolved_name 'ScalarValues::Real'
semantic.unresolved_name 'ScalarValues::Real'
semantic.unresolved_name 'ScalarValues::Real'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "VehicleDefinitions"))) (name "VehicleDefinitions") (declared-name "VehicleDefinitions")
      (contains
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "VehicleDefinitions::Axle"))) (name "Axle") (declared-name "Axle"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "VehicleDefinitions::AxleAssembly"))) (name "AxleAssembly") (declared-name "AxleAssembly"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "VehicleDefinitions::AxleMountIF"))) (name "AxleMountIF") (declared-name "AxleMountIF"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "VehicleDefinitions::DriveIF"))) (name "DriveIF") (declared-name "DriveIF"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "VehicleDefinitions::Lugbolt"))) (name "Lugbolt") (declared-name "Lugbolt"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "VehicleDefinitions::Mounting"))) (name "Mounting") (declared-name "Mounting"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "VehicleDefinitions::Transmission"))) (name "Transmission") (declared-name "Transmission"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "VehicleDefinitions::Vehicle"))) (name "Vehicle") (declared-name "Vehicle"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "VehicleDefinitions::Wheel"))) (name "Wheel") (declared-name "Wheel"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "VehicleDefinitions::WheelHubIF"))) (name "WheelHubIF") (declared-name "WheelHubIF"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "VehicleDefinitions::_documentation"))) (name ""))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "VehicleDefinitions::_documentation"))) (to (node (document "d0") (qualified-name "VehicleDefinitions"))))
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
  (document "kerml/vehicle_definitions.md"
    (diagnostics
    )
  )
)
~~~
