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
  (document "memory://snapshot/vehicle_definitions.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 15 2) (end 16 1))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 21 2) (end 22 1))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 25 2) (end 26 1))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 29 2) (end 30 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 34 1) (end 42 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 34 1) (end 42 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:3aa32456befbff78b8183a70fdff1f9ba0f041bcf9da292650a02745ce14cb03") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Axle"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::AxleAssembly"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::AxleMountIF"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::DriveIF"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Lugbolt"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Transmission"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Vehicle"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Wheel"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::WheelHubIF"))) (kind class-def) (membership (kind owning) (visibility default)))
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
