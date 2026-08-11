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
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "25e18695601773cbd949bf4676cde7b87b4b4c90f0df30df73d1de660528da4f") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions"))) (kind "package") (name "VehicleDefinitions") (declared-name "VehicleDefinitions") (range (start (line 0) (character 0)) (end (line 0) (character 1051))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 6) (character 1)) (end (line 6) (character 32))) (parent (node (document "d0") (qualified-name "VehicleDefinitions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 6) (character 16)) (end (line 6) (character 28))))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 7) (character 1)) (end (line 7) (character 30))) (parent (node (document "d0") (qualified-name "VehicleDefinitions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 26))))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 8) (character 1)) (end (line 8) (character 41))) (parent (node (document "d0") (qualified-name "VehicleDefinitions"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 8) (character 16)) (end (line 8) (character 37))))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::*#import3"))) (kind "import") (name "*") (declared-name "*") (range (start (line 9) (character 1)) (end (line 9) (character 23))) (parent (node (document "d0") (qualified-name "VehicleDefinitions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 9) (character 16)) (end (line 9) (character 19))))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::*#import4"))) (kind "import") (name "*") (declared-name "*") (range (start (line 10) (character 1)) (end (line 10) (character 22))) (parent (node (document "d0") (qualified-name "VehicleDefinitions"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 10) (character 16)) (end (line 10) (character 18))))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::Axle"))) (kind "part def") (name "Axle") (declared-name "Axle") (range (start (line 19) (character 1)) (end (line 19) (character 98))) (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::Axle::leftMountingPoint"))) (kind "port") (name "leftMountingPoint") (declared-name "leftMountingPoint") (range (start (line 20) (character 2)) (end (line 20) (character 38))) (parent (node (document "d0") (qualified-name "VehicleDefinitions::Axle"))) (authored (membership (kind Feature)) (relationships (typing (reference "AxleMountIF") (range none)))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::Axle::rightMountingPoint"))) (kind "port") (name "rightMountingPoint") (declared-name "rightMountingPoint") (range (start (line 21) (character 2)) (end (line 21) (character 39))) (parent (node (document "d0") (qualified-name "VehicleDefinitions::Axle"))) (authored (membership (kind Feature)) (relationships (typing (reference "AxleMountIF") (range none)))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::AxleAssembly"))) (kind "part def") (name "AxleAssembly") (declared-name "AxleAssembly") (range (start (line 18) (character 1)) (end (line 18) (character 23))) (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::AxleMountIF"))) (kind "port def") (name "AxleMountIF") (declared-name "AxleMountIF") (range (start (line 36) (character 1)) (end (line 36) (character 67))) (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::AxleMountIF::transferredTorque"))) (kind "in out parameter") (name "transferredTorque") (declared-name "transferredTorque") (range (start (line 37) (character 2)) (end (line 37) (character 39))) (parent (node (document "d0") (qualified-name "VehicleDefinitions::AxleMountIF"))) (authored (relationships (typing (reference "ISQ::torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::AxleMountIF::~AxleMountIF"))) (kind "conjugated port definition") (name "~AxleMountIF") (declared-name "~AxleMountIF") (range (start (line 36) (character 1)) (end (line 36) (character 67))) (parent (node (document "d0") (qualified-name "VehicleDefinitions::AxleMountIF"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::DriveIF"))) (kind "port def") (name "DriveIF") (declared-name "DriveIF") (range (start (line 32) (character 1)) (end (line 32) (character 56))) (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::DriveIF::driveTorque"))) (kind "in out parameter") (name "driveTorque") (declared-name "driveTorque") (range (start (line 33) (character 2)) (end (line 33) (character 32))) (parent (node (document "d0") (qualified-name "VehicleDefinitions::DriveIF"))) (authored (relationships (typing (reference "ISQ::torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::DriveIF::~DriveIF"))) (kind "conjugated port definition") (name "~DriveIF") (declared-name "~DriveIF") (range (start (line 32) (character 1)) (end (line 32) (character 56))) (parent (node (document "d0") (qualified-name "VehicleDefinitions::DriveIF"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::Lugbolt"))) (kind "part def") (name "Lugbolt") (declared-name "Lugbolt") (range (start (line 26) (character 1)) (end (line 26) (character 67))) (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::Lugbolt::tighteningTorque"))) (kind "attribute") (name "tighteningTorque") (declared-name "tighteningTorque") (range (start (line 27) (character 2)) (end (line 27) (character 44))) (parent (node (document "d0") (qualified-name "VehicleDefinitions::Lugbolt"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::torque") (range (start (line 27) (character 32)) (end (line 27) (character 43)))))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::Mounting"))) (kind "interface def") (name "Mounting") (declared-name "Mounting") (range (start (line 46) (character 1)) (end (line 46) (character 218))) (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::Mounting::_documentation"))) (kind "documentation") (name "") (range (start (line 46) (character 1)) (end (line 46) (character 218))) (parent (node (document "d0") (qualified-name "VehicleDefinitions::Mounting"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::Mounting::axleMount"))) (kind "interface end") (name "axleMount") (declared-name "axleMount") (range (start (line 48) (character 2)) (end (line 48) (character 29))) (parent (node (document "d0") (qualified-name "VehicleDefinitions::Mounting"))) (authored (relationships (typing (reference "AxleMountIF") (range none)))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::Mounting::hub"))) (kind "interface end") (name "hub") (declared-name "hub") (range (start (line 49) (character 2)) (end (line 49) (character 22))) (parent (node (document "d0") (qualified-name "VehicleDefinitions::Mounting"))) (authored (relationships (typing (reference "WheelHubIF") (range none)))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::Transmission"))) (kind "part def") (name "Transmission") (declared-name "Transmission") (range (start (line 17) (character 1)) (end (line 17) (character 23))) (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 14) (character 1)) (end (line 14) (character 53))) (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::Vehicle::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 15) (character 2)) (end (line 15) (character 30))) (parent (node (document "d0") (qualified-name "VehicleDefinitions::Vehicle"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::mass") (range (start (line 15) (character 20)) (end (line 15) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::Wheel"))) (kind "part def") (name "Wheel") (declared-name "Wheel") (range (start (line 23) (character 1)) (end (line 23) (character 44))) (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::Wheel::hub"))) (kind "port") (name "hub") (declared-name "hub") (range (start (line 24) (character 2)) (end (line 24) (character 23))) (parent (node (document "d0") (qualified-name "VehicleDefinitions::Wheel"))) (authored (membership (kind Feature)) (relationships (typing (reference "WheelHubIF") (range none)))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::WheelHubIF"))) (kind "port def") (name "WheelHubIF") (declared-name "WheelHubIF") (range (start (line 40) (character 1)) (end (line 40) (character 61))) (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::WheelHubIF::appliedTorque"))) (kind "in out parameter") (name "appliedTorque") (declared-name "appliedTorque") (range (start (line 41) (character 2)) (end (line 41) (character 34))) (parent (node (document "d0") (qualified-name "VehicleDefinitions::WheelHubIF"))) (authored (relationships (typing (reference "ISQ::torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::WheelHubIF::~WheelHubIF"))) (kind "conjugated port definition") (name "~WheelHubIF") (declared-name "~WheelHubIF") (range (start (line 40) (character 1)) (end (line 40) (character 61))) (parent (node (document "d0") (qualified-name "VehicleDefinitions::WheelHubIF"))))
    (element (id (node (document "d0") (qualified-name "VehicleDefinitions::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 1051))) (parent (node (document "d0") (qualified-name "VehicleDefinitions"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "VehicleDefinitions::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 6) (character 16)) (end (line 6) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleDefinitions::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Quantities::*") (range (start (line 7) (character 16)) (end (line 7) (character 26))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleDefinitions::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "MeasurementReferences::*") (range (start (line 8) (character 16)) (end (line 8) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleDefinitions::*#import3"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (range (start (line 9) (character 16)) (end (line 9) (character 19))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleDefinitions::*#import4"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (range (start (line 10) (character 16)) (end (line 10) (character 18))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleDefinitions::Axle::leftMountingPoint"))) (kind featureTyping) (ordinal 0)) (authored-target "AxleMountIF") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleDefinitions::AxleMountIF")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleDefinitions::Axle::rightMountingPoint"))) (kind featureTyping) (ordinal 0)) (authored-target "AxleMountIF") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleDefinitions::AxleMountIF")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleDefinitions::AxleMountIF::transferredTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::torque") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleDefinitions::DriveIF::driveTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::torque") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleDefinitions::Lugbolt::tighteningTorque"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::torque") (range (start (line 27) (character 32)) (end (line 27) (character 43))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleDefinitions::Mounting"))) (kind flowSource) (ordinal 0)) (authored-target "axleMount::transferredTorque") (range (start (line 51) (character 7)) (end (line 51) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleDefinitions::Mounting"))) (kind flowTarget) (ordinal 0)) (authored-target "hub::appliedTorque") (range (start (line 51) (character 38)) (end (line 51) (character 55))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleDefinitions::Mounting::axleMount"))) (kind featureTyping) (ordinal 0)) (authored-target "AxleMountIF") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleDefinitions::AxleMountIF")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleDefinitions::Mounting::hub"))) (kind featureTyping) (ordinal 0)) (authored-target "WheelHubIF") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleDefinitions::WheelHubIF")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleDefinitions::Vehicle::mass"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (range (start (line 15) (character 20)) (end (line 15) (character 29))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleDefinitions::Wheel::hub"))) (kind featureTyping) (ordinal 0)) (authored-target "WheelHubIF") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleDefinitions::WheelHubIF")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleDefinitions::WheelHubIF::appliedTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::torque") (range none) (outcome (status unresolved)))
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
