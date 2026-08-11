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
  (document "vehicle_definitions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 16) (end 6 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 16) (end 10 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 15 20) (end 15 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 27 32) (end 27 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 33 2) (end 33 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 37 2) (end 37 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 41 2) (end 41 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 51 7) (end 51 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 51 38) (end 51 55))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "25e18695601773cbd949bf4676cde7b87b4b4c90f0df30df73d1de660528da4f") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions"))) (kind "package") (name "VehicleDefinitions") (declared-name "VehicleDefinitions"))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "VehicleDefinitions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "VehicleDefinitions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::*#import2"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "VehicleDefinitions"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::*#import3"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "VehicleDefinitions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::*#import4"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "VehicleDefinitions"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::Axle"))) (kind "part def") (name "Axle") (declared-name "Axle") (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::Axle::leftMountingPoint"))) (kind "port") (name "leftMountingPoint") (declared-name "leftMountingPoint") (parent (node (document "d0") (qualified-name "VehicleDefinitions::Axle"))) (authored (membership (kind Feature)) (relationships (typing (reference "AxleMountIF")))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::Axle::rightMountingPoint"))) (kind "port") (name "rightMountingPoint") (declared-name "rightMountingPoint") (parent (node (document "d0") (qualified-name "VehicleDefinitions::Axle"))) (authored (membership (kind Feature)) (relationships (typing (reference "AxleMountIF")))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::AxleAssembly"))) (kind "part def") (name "AxleAssembly") (declared-name "AxleAssembly") (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::AxleMountIF"))) (kind "port def") (name "AxleMountIF") (declared-name "AxleMountIF") (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::AxleMountIF::transferredTorque"))) (kind "in out parameter") (name "transferredTorque") (declared-name "transferredTorque") (parent (node (document "d0") (qualified-name "VehicleDefinitions::AxleMountIF"))) (authored (relationships (typing (reference "ISQ::torque")))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::AxleMountIF::~AxleMountIF"))) (kind "conjugated port definition") (name "~AxleMountIF") (declared-name "~AxleMountIF") (parent (node (document "d0") (qualified-name "VehicleDefinitions::AxleMountIF"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::DriveIF"))) (kind "port def") (name "DriveIF") (declared-name "DriveIF") (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::DriveIF::driveTorque"))) (kind "in out parameter") (name "driveTorque") (declared-name "driveTorque") (parent (node (document "d0") (qualified-name "VehicleDefinitions::DriveIF"))) (authored (relationships (typing (reference "ISQ::torque")))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::DriveIF::~DriveIF"))) (kind "conjugated port definition") (name "~DriveIF") (declared-name "~DriveIF") (parent (node (document "d0") (qualified-name "VehicleDefinitions::DriveIF"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::Lugbolt"))) (kind "part def") (name "Lugbolt") (declared-name "Lugbolt") (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::Lugbolt::tighteningTorque"))) (kind "attribute") (name "tighteningTorque") (declared-name "tighteningTorque") (parent (node (document "d0") (qualified-name "VehicleDefinitions::Lugbolt"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::torque")))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::Mounting"))) (kind "interface def") (name "Mounting") (declared-name "Mounting") (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::Mounting::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "VehicleDefinitions::Mounting"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::Mounting::axleMount"))) (kind "interface end") (name "axleMount") (declared-name "axleMount") (parent (node (document "d0") (qualified-name "VehicleDefinitions::Mounting"))) (authored (relationships (typing (reference "AxleMountIF")))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::Mounting::hub"))) (kind "interface end") (name "hub") (declared-name "hub") (parent (node (document "d0") (qualified-name "VehicleDefinitions::Mounting"))) (authored (relationships (typing (reference "WheelHubIF")))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::Transmission"))) (kind "part def") (name "Transmission") (declared-name "Transmission") (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::Vehicle::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "VehicleDefinitions::Vehicle"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::mass")))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::Wheel"))) (kind "part def") (name "Wheel") (declared-name "Wheel") (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::Wheel::hub"))) (kind "port") (name "hub") (declared-name "hub") (parent (node (document "d0") (qualified-name "VehicleDefinitions::Wheel"))) (authored (membership (kind Feature)) (relationships (typing (reference "WheelHubIF")))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::WheelHubIF"))) (kind "port def") (name "WheelHubIF") (declared-name "WheelHubIF") (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::WheelHubIF::appliedTorque"))) (kind "in out parameter") (name "appliedTorque") (declared-name "appliedTorque") (parent (node (document "d0") (qualified-name "VehicleDefinitions::WheelHubIF"))) (authored (relationships (typing (reference "ISQ::torque")))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::WheelHubIF::~WheelHubIF"))) (kind "conjugated port definition") (name "~WheelHubIF") (declared-name "~WheelHubIF") (parent (node (document "d0") (qualified-name "VehicleDefinitions::WheelHubIF"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "VehicleDefinitions::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleDefinitions::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Quantities::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleDefinitions::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "MeasurementReferences::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleDefinitions::*#import3"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleDefinitions::*#import4"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleDefinitions::Axle::leftMountingPoint"))) (kind featureTyping) (ordinal 0)) (authored-target "AxleMountIF") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleDefinitions::AxleMountIF")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleDefinitions::Axle::rightMountingPoint"))) (kind featureTyping) (ordinal 0)) (authored-target "AxleMountIF") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleDefinitions::AxleMountIF")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleDefinitions::AxleMountIF::transferredTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::torque") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleDefinitions::DriveIF::driveTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::torque") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleDefinitions::Lugbolt::tighteningTorque"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::torque") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleDefinitions::Mounting"))) (kind flowSource) (ordinal 0)) (authored-target "axleMount::transferredTorque") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleDefinitions::Mounting"))) (kind flowTarget) (ordinal 0)) (authored-target "hub::appliedTorque") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleDefinitions::Mounting::axleMount"))) (kind featureTyping) (ordinal 0)) (authored-target "AxleMountIF") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleDefinitions::AxleMountIF")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleDefinitions::Mounting::hub"))) (kind featureTyping) (ordinal 0)) (authored-target "WheelHubIF") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleDefinitions::WheelHubIF")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleDefinitions::Vehicle::mass"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleDefinitions::Wheel::hub"))) (kind featureTyping) (ordinal 0)) (authored-target "WheelHubIF") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleDefinitions::WheelHubIF")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleDefinitions::WheelHubIF::appliedTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::torque") (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleDefinitions::Axle::leftMountingPoint"))) (target (node (document "d0") (qualified-name "VehicleDefinitions::AxleMountIF"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleDefinitions::Axle::leftMountingPoint"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleDefinitions::Axle::rightMountingPoint"))) (target (node (document "d0") (qualified-name "VehicleDefinitions::AxleMountIF"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleDefinitions::Axle::rightMountingPoint"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleDefinitions::Mounting::axleMount"))) (target (node (document "d0") (qualified-name "VehicleDefinitions::AxleMountIF"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleDefinitions::Mounting::axleMount"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleDefinitions::Mounting::hub"))) (target (node (document "d0") (qualified-name "VehicleDefinitions::WheelHubIF"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleDefinitions::Mounting::hub"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleDefinitions::Wheel::hub"))) (target (node (document "d0") (qualified-name "VehicleDefinitions::WheelHubIF"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleDefinitions::Wheel::hub"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 10 16) (end 10 18)) (probe (position 10 16))
      (reference
        (source (document "d0") (qualified-name "VehicleDefinitions::*#import4"))
        (kind namespaceImport) (ordinal 0) (authored-target "SI::*")
        (range (start 10 16) (end 10 18))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 16) (end 9 19)) (probe (position 9 16))
      (reference
        (source (document "d0") (qualified-name "VehicleDefinitions::*#import3"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQ::*")
        (range (start 9 16) (end 9 19))
        (outcome (status unresolved))
      )
    )
    (query (range (start 15 20) (end 15 29)) (probe (position 15 20))
      (reference
        (source (document "d0") (qualified-name "VehicleDefinitions::Vehicle::mass"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
        (range (start 15 20) (end 15 29))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 16) (end 7 26)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "VehicleDefinitions::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "Quantities::*")
        (range (start 7 16) (end 7 26))
        (outcome (status unresolved))
      )
    )
    (query (range (start 27 32) (end 27 43)) (probe (position 27 32))
      (reference
        (source (document "d0") (qualified-name "VehicleDefinitions::Lugbolt::tighteningTorque"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::torque")
        (range (start 27 32) (end 27 43))
        (outcome (status unresolved))
      )
    )
    (query (range (start 6 16) (end 6 28)) (probe (position 6 16))
      (reference
        (source (document "d0") (qualified-name "VehicleDefinitions::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 6 16) (end 6 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 51 38) (end 51 55)) (probe (position 51 38))
      (reference
        (source (document "d0") (qualified-name "VehicleDefinitions::Mounting"))
        (kind flowTarget) (ordinal 0) (authored-target "hub::appliedTorque")
        (range (start 51 38) (end 51 55))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 16) (end 8 37)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "VehicleDefinitions::*#import2"))
        (kind namespaceImport) (ordinal 0) (authored-target "MeasurementReferences::*")
        (range (start 8 16) (end 8 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 51 7) (end 51 34)) (probe (position 51 7))
      (reference
        (source (document "d0") (qualified-name "VehicleDefinitions::Mounting"))
        (kind flowSource) (ordinal 0) (authored-target "axleMount::transferredTorque")
        (range (start 51 7) (end 51 34))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
