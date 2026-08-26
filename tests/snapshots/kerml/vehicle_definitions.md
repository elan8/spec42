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
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 24) (end 15 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 18) (end 21 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 25 26) (end 25 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 29 21) (end 29 39))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:3aa32456befbff78b8183a70fdff1f9ba0f041bcf9da292650a02745ce14cb03") (contract-version "owned-cross-feature-typing-v4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions"))) (kind package) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t * Example vehicle definitions model.\n\t ")) (comment (text " BLOCKS ")) (comment (text " INTERFACE BLOCKS ")) (comment (text " ASSOCIATION BLOCKS "))))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Axle"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::AxleAssembly"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::AxleMountIF"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::AxleMountIF::transferredTorque"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Real") (direction out)))))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::DriveIF"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::DriveIF::driveTorque"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Real") (direction in)))))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Lugbolt"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Lugbolt::tighteningTorque"))) (kind default-reference) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Real")))))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Mounting"))) (kind kerml-association) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t *  mounting a Wheel to an Axle.\n\t\t "))))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Mounting::axleMount"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AxleMountIF")))))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Mounting::hub"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "WheelHubIF")))))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Transmission"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Vehicle"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Wheel"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::WheelHubIF"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::WheelHubIF::appliedTorque"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Real") (direction in)))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::AxleMountIF::transferredTorque"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::DriveIF::driveTorque"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Lugbolt::tighteningTorque"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Mounting::axleMount"))) (kind featureTyping) (ordinal 0))
      (authored-target "AxleMountIF")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::AxleMountIF")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Mounting::hub"))) (kind featureTyping) (ordinal 0))
      (authored-target "WheelHubIF")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::WheelHubIF")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::WheelHubIF::appliedTorque"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Mounting::axleMount"))) (target (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::AxleMountIF"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Mounting::axleMount"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Mounting::hub"))) (target (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::WheelHubIF"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Mounting::hub"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::AxleMountIF::transferredTorque"))) (target (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::AxleMountIF"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::DriveIF::driveTorque"))) (target (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::DriveIF"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Lugbolt::tighteningTorque"))) (target (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Lugbolt"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Mounting::axleMount"))) (target (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Mounting"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Mounting::hub"))) (target (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Mounting"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::WheelHubIF::appliedTorque"))) (target (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::WheelHubIF"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::AxleMountIF")))
      (subtype (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Mounting::axleMount")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::AxleMountIF::transferredTorque")))
      (featured-by (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::AxleMountIF")))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::DriveIF::driveTorque")))
      (featured-by (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::DriveIF")))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Lugbolt::tighteningTorque")))
      (featured-by (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Lugbolt")))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Mounting::axleMount")))
      (featured-by (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Mounting")))
      (type (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::AxleMountIF")) (provenance authored))
      (effective-type (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::AxleMountIF")) (source direct))
      (supertype (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::AxleMountIF")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Mounting::hub")))
      (featured-by (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Mounting")))
      (type (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::WheelHubIF")) (provenance authored))
      (effective-type (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::WheelHubIF")) (source direct))
      (supertype (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::WheelHubIF")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::WheelHubIF")))
      (subtype (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Mounting::hub")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::WheelHubIF::appliedTorque")))
      (featured-by (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::WheelHubIF")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/vehicle_definitions.md") (range (start 25 26) (end 25 44)) (probe (position 25 26))
    (reference (id (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::AxleMountIF::transferredTorque"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_definitions.md") (range (start 21 18) (end 21 36)) (probe (position 21 18))
    (reference (id (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::DriveIF::driveTorque"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_definitions.md") (range (start 15 24) (end 15 42)) (probe (position 15 24))
    (reference (id (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Lugbolt::tighteningTorque"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_definitions.md") (range (start 40 17) (end 40 28)) (probe (position 40 17))
    (reference (id (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Mounting::axleMount"))) (kind featureTyping) (ordinal 0) (authored-target "AxleMountIF")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::AxleMountIF")))))
    )
  )
  (query (document "memory://snapshot/vehicle_definitions.md") (range (start 41 11) (end 41 21)) (probe (position 41 11))
    (reference (id (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Mounting::hub"))) (kind featureTyping) (ordinal 0) (authored-target "WheelHubIF")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::WheelHubIF")))))
    )
  )
  (query (document "memory://snapshot/vehicle_definitions.md") (range (start 29 21) (end 29 39)) (probe (position 29 21))
    (reference (id (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::WheelHubIF::appliedTorque"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    )
  )
)
~~~
