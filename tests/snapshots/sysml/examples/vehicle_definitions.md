# META
~~~ini
description=SysML Example (Vehicle): VehicleDefinitions
type=file
~~~
# SOURCE
~~~sysml
package VehicleDefinitions {
	doc
	/*
	 * Example vehicle definitions model.
	 */

	private import ScalarValues::*;
	private import Quantities::*;
	private import MeasurementReferences::*;
	private import ISQ::*;
	private import SI::*;
	
	/* PART DEFINITIONS */
	
	part def Vehicle {
		attribute mass :> ISQ::mass;
	}
	part def Transmission;	
	part def AxleAssembly;
	part def Axle {
		port leftMountingPoint: AxleMountIF;
		port rightMountingPoint: AxleMountIF;
	}
	part def Wheel {
		port hub: WheelHubIF;
	}
	part def Lugbolt {
		attribute tighteningTorque :> ISQ::torque;
	}
	
	/* PORT DEFINITIONS */
	
	port def DriveIF { 
		in driveTorque :> ISQ::torque;
	}
	
	port def AxleMountIF { 
		out transferredTorque :> ISQ::torque;
	}
	
	port def WheelHubIF { 
		in appliedTorque :> ISQ::torque;
	}
	
	/* INTERFACE DEFINITIONS */
	
	interface def Mounting {
		doc /* The definition of the interface for mounting a Wheel to an Axle. */
		end axleMount: AxleMountIF;
		end hub: WheelHubIF;
		
		flow axleMount.transferredTorque to hub.appliedTorque;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/vehicle_definitions.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 6 16) (end 6 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 16) (end 6 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 16) (end 10 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 15 20) (end 15 29))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 20 2) (end 20 38))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 21 2) (end 21 39))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 24 2) (end 24 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 27 32) (end 27 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 33 20) (end 33 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 37 27) (end 37 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 41 22) (end 41 33))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_interface_definition_member")
        (source "semantic")
        (range (start 51 2) (end 51 56))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:184050d88922ce9f5d798b369e86c011d2a2a71f8b7912b6b0c4826233b811f1") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions"))) (kind package) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t * Example vehicle definitions model.\n\t ")) (comment (text " PART DEFINITIONS ")) (comment (text " PORT DEFINITIONS ")) (comment (text " INTERFACE DEFINITIONS "))))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (path (named (kind package) (name "VehicleDefinitions")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (path (named (kind package) (name "VehicleDefinitions")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Quantities") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (path (named (kind package) (name "VehicleDefinitions")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "MeasurementReferences") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (path (named (kind package) (name "VehicleDefinitions")) (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQ") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (path (named (kind package) (name "VehicleDefinitions")) (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "SI") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Axle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Axle::leftMountingPoint"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AxleMountIF")))))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Axle::rightMountingPoint"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AxleMountIF")))))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::AxleAssembly"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::AxleMountIF"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::AxleMountIF::transferredTorque"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::torque")))))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::DriveIF"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::DriveIF::driveTorque"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::torque")))))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Lugbolt"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Lugbolt::tighteningTorque"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::torque")))))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Mounting"))) (kind interface-def) (membership (kind owning) (visibility default)) (documentation (doc (text " The definition of the interface for mounting a Wheel to an Axle. "))))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Mounting::axleMount"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AxleMountIF")))))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Mounting::hub"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 1)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "WheelHubIF")))))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Transmission"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Vehicle::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass")))))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Wheel"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Wheel::hub"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "WheelHubIF")))))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::WheelHubIF"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::WheelHubIF::appliedTorque"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::torque")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/vehicle_definitions.md") (path (named (kind package) (name "VehicleDefinitions")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_definitions.md") (path (named (kind package) (name "VehicleDefinitions")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Quantities")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_definitions.md") (path (named (kind package) (name "VehicleDefinitions")) (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "MeasurementReferences")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_definitions.md") (path (named (kind package) (name "VehicleDefinitions")) (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQ")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_definitions.md") (path (named (kind package) (name "VehicleDefinitions")) (anonymous (kind import) (ordinal 4))))) (kind namespaceImport) (ordinal 0))
      (authored-target "SI")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Axle::leftMountingPoint"))) (kind featureTyping) (ordinal 0))
      (authored-target "AxleMountIF")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::AxleMountIF")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Axle::rightMountingPoint"))) (kind featureTyping) (ordinal 0))
      (authored-target "AxleMountIF")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::AxleMountIF")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::AxleMountIF::transferredTorque"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::torque")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::DriveIF::driveTorque"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::torque")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Lugbolt::tighteningTorque"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::torque")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Mounting::axleMount"))) (kind featureTyping) (ordinal 0))
      (authored-target "AxleMountIF")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::AxleMountIF")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Mounting::hub"))) (kind featureTyping) (ordinal 0))
      (authored-target "WheelHubIF")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::WheelHubIF")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Vehicle::mass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Wheel::hub"))) (kind featureTyping) (ordinal 0))
      (authored-target "WheelHubIF")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::WheelHubIF")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::WheelHubIF::appliedTorque"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::torque")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Axle::leftMountingPoint"))) (target (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::AxleMountIF"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Axle::leftMountingPoint"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Axle::rightMountingPoint"))) (target (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::AxleMountIF"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Axle::rightMountingPoint"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Mounting::axleMount"))) (target (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::AxleMountIF"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Mounting::axleMount"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Mounting::hub"))) (target (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::WheelHubIF"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Mounting::hub"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Wheel::hub"))) (target (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::WheelHubIF"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Wheel::hub"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Axle::leftMountingPoint"))) (target (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Axle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Axle::rightMountingPoint"))) (target (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Axle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::AxleMountIF::transferredTorque"))) (target (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::AxleMountIF"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::DriveIF::driveTorque"))) (target (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::DriveIF"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Lugbolt::tighteningTorque"))) (target (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Lugbolt"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Mounting::axleMount"))) (target (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Mounting"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Mounting::hub"))) (target (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Mounting"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Vehicle::mass"))) (target (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Wheel::hub"))) (target (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Wheel"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::WheelHubIF::appliedTorque"))) (target (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::WheelHubIF"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Axle::leftMountingPoint")))
      (featured-by (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Axle")))
      (type (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::AxleMountIF")) (provenance authored))
      (effective-type (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::AxleMountIF")) (source direct))
      (supertype (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::AxleMountIF")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Axle::rightMountingPoint")))
      (featured-by (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Axle")))
      (type (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::AxleMountIF")) (provenance authored))
      (effective-type (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::AxleMountIF")) (source direct))
      (supertype (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::AxleMountIF")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::AxleMountIF")))
      (subtype (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Axle::leftMountingPoint")) (scopes any))
      (subtype (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Axle::rightMountingPoint")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Mounting")))
      (positional-ends (authored 2) (effective 2))
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
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Vehicle::mass")))
      (featured-by (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Wheel::hub")))
      (featured-by (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Wheel")))
      (type (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::WheelHubIF")) (provenance authored))
      (effective-type (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::WheelHubIF")) (source direct))
      (supertype (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::WheelHubIF")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::WheelHubIF")))
      (subtype (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Mounting::hub")) (scopes any))
      (subtype (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Wheel::hub")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::WheelHubIF::appliedTorque")))
      (featured-by (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::WheelHubIF")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/vehicle_definitions.md") (range (start 6 16) (end 6 31)) (probe (position 6 16))
    (reference (id (source (node (document "memory://snapshot/vehicle_definitions.md") (path (named (kind package) (name "VehicleDefinitions")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_definitions.md") (range (start 7 16) (end 7 29)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/vehicle_definitions.md") (path (named (kind package) (name "VehicleDefinitions")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "Quantities")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_definitions.md") (range (start 8 16) (end 8 40)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/vehicle_definitions.md") (path (named (kind package) (name "VehicleDefinitions")) (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "MeasurementReferences")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_definitions.md") (range (start 9 16) (end 9 22)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/vehicle_definitions.md") (path (named (kind package) (name "VehicleDefinitions")) (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQ")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_definitions.md") (range (start 10 16) (end 10 21)) (probe (position 10 16))
    (reference (id (source (node (document "memory://snapshot/vehicle_definitions.md") (path (named (kind package) (name "VehicleDefinitions")) (anonymous (kind import) (ordinal 4))))) (kind namespaceImport) (ordinal 0) (authored-target "SI")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_definitions.md") (range (start 20 26) (end 20 37)) (probe (position 20 26))
    (reference (id (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Axle::leftMountingPoint"))) (kind featureTyping) (ordinal 0) (authored-target "AxleMountIF")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::AxleMountIF")))))
    )
  )
  (query (document "memory://snapshot/vehicle_definitions.md") (range (start 21 27) (end 21 38)) (probe (position 21 27))
    (reference (id (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Axle::rightMountingPoint"))) (kind featureTyping) (ordinal 0) (authored-target "AxleMountIF")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::AxleMountIF")))))
    )
  )
  (query (document "memory://snapshot/vehicle_definitions.md") (range (start 37 27) (end 37 38)) (probe (position 37 27))
    (reference (id (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::AxleMountIF::transferredTorque"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::torque")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_definitions.md") (range (start 33 20) (end 33 31)) (probe (position 33 20))
    (reference (id (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::DriveIF::driveTorque"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::torque")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_definitions.md") (range (start 27 32) (end 27 43)) (probe (position 27 32))
    (reference (id (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Lugbolt::tighteningTorque"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::torque")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_definitions.md") (range (start 48 17) (end 48 28)) (probe (position 48 17))
    (reference (id (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Mounting::axleMount"))) (kind featureTyping) (ordinal 0) (authored-target "AxleMountIF")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::AxleMountIF")))))
    )
  )
  (query (document "memory://snapshot/vehicle_definitions.md") (range (start 49 11) (end 49 21)) (probe (position 49 11))
    (reference (id (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Mounting::hub"))) (kind featureTyping) (ordinal 0) (authored-target "WheelHubIF")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::WheelHubIF")))))
    )
  )
  (query (document "memory://snapshot/vehicle_definitions.md") (range (start 15 20) (end 15 29)) (probe (position 15 20))
    (reference (id (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Vehicle::mass"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_definitions.md") (range (start 24 12) (end 24 22)) (probe (position 24 12))
    (reference (id (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::Wheel::hub"))) (kind featureTyping) (ordinal 0) (authored-target "WheelHubIF")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::WheelHubIF")))))
    )
  )
  (query (document "memory://snapshot/vehicle_definitions.md") (range (start 41 22) (end 41 33)) (probe (position 41 22))
    (reference (id (source (node (document "memory://snapshot/vehicle_definitions.md") (qualified-name "VehicleDefinitions::WheelHubIF::appliedTorque"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::torque")
      (outcome (status unresolved)))
    )
  )
)
~~~
